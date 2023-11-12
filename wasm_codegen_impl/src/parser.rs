use std::fs;

use crate::value_type::{ParamType, ReturnType, ValueType};

const BIG_CALL_MIN_PARAMS: usize = 16;
const BIG_CALL_MAX_PARAMS: usize = 32;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TopLevelSection {
    Imports,
    Exports,
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub param_type: ValueType,
}

#[derive(Debug, PartialEq)]
pub struct Func {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Option<ValueType>,
    pub big_call: bool,
}

#[derive(Debug, PartialEq)]
pub struct MultiFunc {
    pub name: String,
    pub funcs: Vec<Func>,
}

#[derive(Debug, PartialEq)]
pub enum AnyFunc {
    Normal(Func),
    MultiFunc(MultiFunc),
}

impl AnyFunc {
    fn name(&self) -> &str {
        match self {
            AnyFunc::Normal(n) => &n.name,
            AnyFunc::MultiFunc(m) => &m.name,
        }
    }
}

#[derive(Debug)]
struct Section {
    kind: TopLevelSection,
    funcs: Vec<AnyFunc>,
}

#[derive(Debug, Default)]
struct AllSections {
    imports: Option<Vec<AnyFunc>>,
    exports: Option<Vec<AnyFunc>>,
}

#[derive(Debug, Default)]
struct Parser {
    current_section: Option<Section>,
    all: AllSections,
    current_multi_func: Option<MultiFunc>,
}

impl Parser {
    fn change_section(&mut self, to: TopLevelSection) {
        let section = self.current_section.take();
        if let Some(section) = section {
            assert_ne!(section.kind, to);
            self.collect_section(section);
        }
        self.current_section.replace(Section {
            kind: to,
            funcs: vec![],
        });
    }

    fn start_multi_func(&mut self, name: String) {
        assert!(self.current_multi_func.is_none());
        self.current_multi_func.replace(MultiFunc {
            name,
            funcs: vec![],
        });
    }

    fn finish_multi_func(&mut self) {
        let m = self.current_multi_func.take().unwrap();

        self.current_section
            .as_mut()
            .unwrap()
            .funcs
            .push(AnyFunc::MultiFunc(m));
    }

    fn add_func(&mut self, func: Func) {
        if let Some(m) = self.current_multi_func.as_mut() {
            m.funcs.push(func);
            return;
        }

        self.current_section
            .as_mut()
            .unwrap()
            .funcs
            .push(AnyFunc::Normal(func));
    }

    fn collect_section(&mut self, section: Section) {
        match section.kind {
            TopLevelSection::Exports => {
                self.all.exports.replace(section.funcs);
            }
            TopLevelSection::Imports => {
                self.all.imports.replace(section.funcs);
            }
        }
    }

    fn parse(mut self, input: String) -> Interface {
        let mut word = String::new();
        let mut current_func: Option<Func> = None;
        let mut current_param_name: Option<String> = None;
        let mut return_type_parsing = false;
        let mut return_type_parsing_arrow = false;

        for char in input.chars() {
            match char {
                ':'
                // params
                |'(' | ')' | ','
                // top level sections
                | '{' | '}'
                // multi funcs
                | '[' | ']' => {
                    match (std::mem::take(&mut word).trim(), char) {
                        // top level sections
                        ("imports", '{') => {
                            // println!("starting with imports");
                            self.change_section(TopLevelSection::Imports);
                        }
                        ("exports", '{') => {
                            // println!("starting with exports");
                            self.change_section(TopLevelSection::Exports);
                        }
                        (unknown, '{') => {
                            panic!("Unknown top-level section: {unknown:?}");
                        }

                        // func names
                        (func_name, '(') => {
                            assert!(current_func.is_none());

                            if self
                                .current_section
                                .as_ref()
                                .unwrap()
                                .funcs
                                .iter()
                                .any(|f| f.name() == func_name)
                            {
                                panic!("Detected func name duplicate: {func_name:?}");
                            }

                            current_func.replace(Func {
                                name: func_name.to_string(),
                                params: vec![],
                                ret: None,
                                big_call: false, // will be changed then in parameters parsing
                            });
                        }

                        // params
                        ("", ')') => {
                            assert!(current_func.is_some());
                            // println!("empty params");
                            return_type_parsing = true;
                        }

                        (param_name, ':') => {
                            assert!(current_param_name.is_none());

                            let Some(func) = current_func.as_ref() else {
                                panic!("Expected current func");
                            };
                            if func.params.iter().any(|p| p.name == param_name) {
                                panic!("Detected func param name duplicate: {param_name:?} in func: {:?}", func.name);
                            }

                            current_param_name.replace(param_name.to_string());
                        }

                        (param_type, ',' | ')') => {
                            let (Some(func), Some(param_name)) =
                                (current_func.as_mut(), current_param_name.take())
                            else {
                                panic!("Expected current func and param name");
                            };

                            func.params.push(Param {
                                name: param_name,
                                param_type: ParamType(param_type).into(),
                            });

                            let current_count = func.params.len();
                            if current_count >= BIG_CALL_MIN_PARAMS || self.current_multi_func.is_some() {
                                func.big_call = true;
                            }
                            if current_count > BIG_CALL_MAX_PARAMS {
                                panic!(
                                    "Too many args (> {BIG_CALL_MAX_PARAMS}) in func: {}",
                                    func.name
                                );
                            }

                            if char == ')' {
                                return_type_parsing = true;
                            }
                        }

                        // custom multi func that uses single wasmtime func under the hood
                        // multi_example[
                        //   first(a: I32)
                        //   second(b: I32) -> I32
                        // ]
                        (multi_func, '[') => {
                            // println!("starting multi func: {multi_func}");
                            self.start_multi_func(multi_func.to_string());
                        }

                        // TODO: test
                        ("", ']') => {
                            // println!("multi func finish?");
                            self.finish_multi_func();
                        }

                        _ => {}
                    }
                }
                _ => {
                    if return_type_parsing {
                        match char {
                            '-' => {
                                return_type_parsing_arrow = true;
                            }
                            '>' => {
                                assert!(return_type_parsing_arrow);
                                return_type_parsing_arrow = false;
                            }
                            '\n' => {
                                return_type_parsing = false;
                                let Some(mut func) = current_func.take() else {
                                    panic!("Expected current func");
                                };
                                let word = &std::mem::take(&mut word);
                                let return_type = word.trim();
                                if !return_type.is_empty() {
                                    let return_type = &return_type.split("->").last().unwrap();
                                    let return_type = return_type.trim();
                                    func.ret.replace(ReturnType(return_type).into());
                                }

                                self.add_func(func);
                            }
                            _ => {
                                assert!(!return_type_parsing_arrow);
                            }
                        }
                    }

                    word += &char.to_string();
                }
            }
        }

        if let Some(section) = self.current_section.take() {
            self.collect_section(section);
        }

        Interface {
            imports: self.all.imports.unwrap(),
            exports: self.all.exports.unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Interface {
    pub imports: Vec<AnyFunc>,
    pub exports: Vec<AnyFunc>,
}

fn parse(input: String) -> Interface {
    let parser = Parser::default();
    parser.parse(input)
}

pub fn read_and_parse_interface(path: &str) -> Interface {
    let input = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read interface file at: {path:?}, error: {e}"));
    parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_check(input: &str, interface: Interface) {
        let parsed = parse(input.to_string());
        assert_eq!(parsed, interface);
    }

    #[test]
    fn empty() {
        parse_and_check(
            "
            imports {
                
            }

            exports {

            }
        ",
            Interface {
                imports: vec![],
                exports: vec![],
            },
        );
    }

    #[test]
    fn normal_funcs() {
        parse_and_check(
            "
            imports {
                test_import(a: I32)
            }

            exports {
                test_export(b: F32)
            }
        ",
            Interface {
                imports: vec![AnyFunc::Normal(Func {
                    name: "test_import".to_string(),
                    params: vec![Param {
                        name: "a".to_string(),
                        param_type: ValueType::I32,
                    }],
                    ret: None,
                    big_call: false,
                })],

                exports: vec![AnyFunc::Normal(Func {
                    name: "test_export".to_string(),
                    params: vec![Param {
                        name: "b".to_string(),
                        param_type: ValueType::F32,
                    }],
                    ret: None,
                    big_call: false,
                })],
            },
        );
    }

    #[test]
    fn multi_func() {
        parse_and_check(
            "
            imports {
                test_multi_import[
                    first()
                    second()
                    third()
                ]
                test_import()
            }

            exports {
                test_export()
            }
        ",
            Interface {
                imports: vec![
                    AnyFunc::MultiFunc(MultiFunc {
                        name: "test_multi_import".to_string(),
                        funcs: vec![
                            Func {
                                name: "first".to_string(),
                                params: vec![],
                                ret: None,
                                big_call: false,
                            },
                            Func {
                                name: "second".to_string(),
                                params: vec![],
                                ret: None,
                                big_call: false,
                            },
                            Func {
                                name: "third".to_string(),
                                params: vec![],
                                ret: None,
                                big_call: false,
                            },
                        ],
                    }),
                    AnyFunc::Normal(Func {
                        name: "test_import".to_string(),
                        params: vec![],
                        ret: None,
                        big_call: false,
                    }),
                ],

                exports: vec![AnyFunc::Normal(Func {
                    name: "test_export".to_string(),
                    params: vec![],
                    ret: None,
                    big_call: false,
                })],
            },
        );
    }
}
