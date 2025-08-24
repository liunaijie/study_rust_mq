use std::fs;

static OUT_DIR:&str = "src";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "proto/basic/basic.proto",
        "proto/hello.proto",
        "proto/goodbye.proto",
    ];

    fs::create_dir_all(OUT_DIR).unwrap();
    tonic_build::configure()
        .build_server(true)
        .out_dir(OUT_DIR)
        .compile(&protos, &["proto"])
        .unwrap();
    Ok(())
}