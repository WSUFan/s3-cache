fn main() {
    protobuf_codegen::Codegen::new()
        .out_dir("src/protos/")
        .inputs(&["src/protos/configuration.proto"])
        .include("src/protos")
        .run()
        .expect("Codegen failed.");
}
