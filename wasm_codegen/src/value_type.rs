type RustType = String;

pub struct ParamType<'a>(pub &'a str);
pub struct ReturnType<'a>(pub &'a str);

#[derive(Debug, PartialEq, Clone)]
pub struct ValueType {
    pub name: RustType,
    pub de: Option<RustType>,
    pub kind: ValueKind,
    pub repr: ValueRepr,
    pub can_be_param: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ValueTypePool {
    types: Vec<ValueType>,
}

impl ValueTypePool {
    pub fn append_types(&mut self, types: Vec<String>) {
        self.types.append(&mut Self::parse_types(types));
    }

    pub fn from_param_type(&self, param_type: ParamType) -> Option<ValueType> {
        self.types
            .iter()
            .find(|t| t.name == param_type.0 && t.can_be_param)
            .cloned()
    }

    pub fn from_return_type(&self, return_type: ReturnType) -> Option<ValueType> {
        self.types.iter().find(|t| t.name == return_type.0).cloned()
    }

    fn parse_types(types: Vec<String>) -> Vec<ValueType> {
        types
            .into_iter()
            .map(|raw| {
                let pat = {
                    let mut colon = true;
                    let mut ignore_whitespace = false;
                    move |c: char| -> bool {
                        match (c, colon, ignore_whitespace) {
                            (':', true, _) => {
                                colon = false;
                                ignore_whitespace = true;
                                false
                            }
                            (' ', false, false) => {
                                colon = true;
                                true
                            }
                            ('A'..='Z' | 'a'..='z' | '0'..='9' | '_', false, true) => {
                                ignore_whitespace = false;
                                false
                            }
                            _ => false,
                        }
                    }
                };

                let mut rust_type = None;
                let mut de = None;
                let mut kind = None;
                let mut repr = None;
                let mut can_be_param = None;
                for part in raw.split(pat) {
                    match part.split(": ").collect::<Vec<&str>>()[..] {
                        [prop, value] => {
                            let value = value.trim();
                            match prop.trim() {
                                "type" => {
                                    rust_type.replace(value.to_string());
                                }
                                "de" => {
                                    de.replace(value.to_string());
                                }
                                "kind" => {
                                    kind.replace(value.into());
                                }
                                "repr" => {
                                    repr.replace(value.into());
                                }
                                "can_be_param" => {
                                    can_be_param.replace(value == "true");
                                }
                                prop => {
                                    panic!("Unknown prop: {prop}");
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                ValueType {
                    name: rust_type.unwrap(),
                    de,
                    kind: kind.unwrap(),
                    repr: repr.unwrap(),
                    can_be_param: can_be_param.unwrap(),
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueKind {
    /// Custom type
    FatPtr,
    /// Small value that fits in wasm type
    /// (e.g. i32 or f32)
    Native,
    /// Did not found better way for bool values :deadge:
    /// other than representing it as i32 and checking == 1
    Bool,
    /// Improved (de)serialization of strings
    String,
}

impl From<&str> for ValueKind {
    fn from(value: &str) -> Self {
        match value {
            "FatPtr" => Self::FatPtr,
            "Native" => Self::Native,
            "Bool" => Self::Bool,
            "String" => Self::String,
            _ => panic!("Unknown ValueKind: {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueRepr {
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    FatPtr,
}

impl From<ValueRepr> for &str {
    fn from(value: ValueRepr) -> Self {
        match value {
            ValueRepr::I32 => "i32",
            ValueRepr::U32 => "u32",
            ValueRepr::I64 => "i64",
            ValueRepr::U64 => "u64",
            ValueRepr::F32 => "f32",
            ValueRepr::F64 => "f64",
            ValueRepr::FatPtr => "super::__shared::FatPtr",
        }
    }
}

impl From<&str> for ValueRepr {
    fn from(value: &str) -> Self {
        match value {
            "I32" => Self::I32,
            "U32" => Self::U32,
            "I64" => Self::I64,
            "U64" => Self::U64,
            "F32" => Self::F32,
            "F64" => Self::F64,
            "FatPtr" => Self::FatPtr,
            _ => panic!("Unknown ValueRepr: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool() {
        let mut pool = ValueTypePool::default();
        pool.append_types(vec![
            "type: i8 kind: Native repr: I32 can_be_param: true".to_string(),
            "type: i16 kind: Native repr: I32 can_be_param: true".to_string(),
            "type: &String de: String kind: String repr: FatPtr can_be_param: true".to_string(),
            "type: shared::Custom kind: FatPtr repr: FatPtr can_be_param: true".to_string(),
        ]);
        dbg!(pool);
    }
}
