fn main() {
    build_helper::generate_bindings!(guest, "../wasm.interface");
}
