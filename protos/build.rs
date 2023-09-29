use std::env;
use std::path::Path;

fn main() {
    let mut codegen = protobuf_codegen::Codegen::new();
    let protobuf_gen = codegen
        .cargo_out_dir("genprotos")
        .inputs(&["./configuration.proto"])
        .include("./");

    match env::var("BAZEL_PROTOC") {
        Ok(protoc) => {
            protobuf_gen.protoc_path(Path::new(&protoc));
        }
        Err(_) => {
            protobuf_gen.protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap());
        }
    }

    protobuf_gen.run().expect("Codegen failed.");
}
