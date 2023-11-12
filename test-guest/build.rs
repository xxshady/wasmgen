fn main() {
    build_helper::generate_bindings!(guest, "../wasm.interface", "guest_gen.rs");
}
