use std::process::Command;

fn main() {
    let command = "cargo install protobuf-codegen";

    // Create a Command struct
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to start command");

    let status = child.wait().expect("Failed to wait for command");
    if !status.success() {
        panic!("Command {} failed with error: {:?}", command, status.code());
    }
    
    protobuf_codegen::Codegen::new()
        .out_dir("src/protos/")
        .inputs(&["src/protos/configuration.proto"])
        .include("src/protos")
        .run()
        .expect("Codegen failed.");
}
