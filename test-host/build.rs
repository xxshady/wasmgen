fn main() {
    build_helper::generate_bindings!(
        host,
        "host_gen.rs",
        @interfaces
        main: "../wasm.interface"
        extra: [
            "../extra_wasm.interface",
        ]
    );
}
