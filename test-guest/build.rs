fn main() {
    build_helper::generate_bindings!(
        guest,
        "guest_gen.rs",
        @interfaces
        main: "../wasm.interface"
        extra: [
            "../extra_wasm.interface",
        ]
    );
}
