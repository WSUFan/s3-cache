fn main() {
    protobuf_codegen::Codegen::new()
        .out_dir("protos/")
        .inputs(&["protos/configuration.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}
