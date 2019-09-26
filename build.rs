use std::process::Command;
use which::which;

fn main() {
    println!("start generating proto");
    let grpc_rust_plugin_path = which("grpc_rust_plugin")
        .expect("`grpc_rust_plugin` is not found in path")
        .to_str()
        .expect("path string failed")
        .to_string();

    let outout = Command::new("protoc")
        .arg("--rust_out=./src/")
        .arg("--grpc_out=./src/")
        .arg(format!("--plugin=protoc-gen-grpc={}", grpc_rust_plugin_path).as_str())
        .arg("proto/server.proto")
        .output()
        .expect("protoc command failed");
    println!("{}", outout.status);
    println!("{}", String::from_utf8_lossy(&outout.stdout));
    println!("{}", String::from_utf8_lossy(&outout.stderr));
}
