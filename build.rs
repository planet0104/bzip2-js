fn main() {
    cc::Build::new()
        .file("src\\emscripten.c")
        .compile("emscripten");
}
