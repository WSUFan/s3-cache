fn main() {
    protobuf_codegen::Codegen::new()
        .out_dir("src/genprotos/")
        .inputs(&["protos/configuration.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}
