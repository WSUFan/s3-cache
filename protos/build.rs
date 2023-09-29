fn main() {
    protobuf_codegen::Codegen::new()
        .out_dir("./")
        .inputs(&["./configuration.proto"])
        .include("./")
        .run()
        .expect("Codegen failed.");
}
