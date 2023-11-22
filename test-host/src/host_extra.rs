use crate::host_shared::extra_interfaces::extra_wasm;
use crate::host_shared::{self, GetBigCallPtr};

pub(crate) fn extra_linker_func<
    U: host_shared::GetExtra + host_shared::GetBigCallPtr + host_shared::Imports,
>(
    linker: &mut wasmtime::Linker<U>,
) {
    linker
        .func_wrap(
            "__custom_imports",
            stringify!(extra),
            #[allow(unused_mut)]
            |mut caller: wasmtime::Caller<U>, func_index: u32| -> u64 {
                #[allow(clippy::unnecessary_cast)]
                {
                    match func_index {
                        0u32 => {
                            let (a,) = host_shared::read_big_call_args(&mut caller)
                                .with_borrow(|big_call_args| {
                                    (
                                        <i32 as host_shared::__shared::NumAsU64Arr>::from_bytes(
                                            big_call_args[0usize..8usize].try_into().unwrap(),
                                        ) as i32,
                                    )
                                });
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().get_extra_wasm().extra_a(a);
                            0
                        }
                        1u32 => {
                            let (b,) = host_shared::read_big_call_args(&mut caller)
                                .with_borrow(|big_call_args| {
                                    (
                                        <i32 as host_shared::__shared::NumAsU64Arr>::from_bytes(
                                            big_call_args[0usize..8usize].try_into().unwrap(),
                                        ) == 1,
                                    )
                                });
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().get_extra_wasm().extra_b(b);
                            0
                        }
                        2u32 => {
                            let (option_i32,) = host_shared::read_big_call_args(&mut caller)
                                .with_borrow(|big_call_args| {
                                    (
                                        host_shared::read_from_guest(
                                                &mut caller,
                                                <host_shared::__shared::FatPtr as host_shared::__shared::NumAsU64Arr>::from_bytes(
                                                    big_call_args[0usize..8usize].try_into().unwrap(),
                                                ),
                                            )
                                            .unwrap(),
                                    )
                                });
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller
                                .data()
                                .get_extra_wasm()
                                .extra_option_i32(option_i32);
                            0
                        }
                        _ => {
                            panic!(
                                "Unknown multi func index: {func_index} in func: {}",
                                stringify!(extra)
                            );
                        }
                    }
                }
            },
        )
        .unwrap();
}
