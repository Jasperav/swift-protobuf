use std::path::Path;
use std::process::Command;

fn main() {
    println!(
        "Protoc: {:#?}",
        Command::new("which").arg("protoc").output().unwrap()
    );
    println!(
        "Protoc swift: {:#?}",
        Command::new("which")
            .arg("protoc-gen-swift")
            .output()
            .unwrap()
    );

    let current_dir = std::env::current_dir().unwrap();
    let proto_dir = current_dir.join("src");

    println!("Protodir is {:#?}", proto_dir);

    assert!(proto_dir.exists());

    protobuf_strict::write_protos(&proto_dir);

    let protos = proto_dir.join("protos");

    assert!(protos.exists());

    let path = protos.to_str().unwrap();

    macro_rules! write {
        ($output: expr, $args: expr) => {
            let output_path = current_dir
                .join($output)
                .join("Sources")
                .join($output)
                .join("protos");

            println!("Output path is {:#?}", output_path);

            let _ = std::fs::remove_dir_all(&output_path);

            std::fs::create_dir_all(&output_path).expect("Creating dirs failed");

            assert!(output_path.exists());

            let output_path = output_path.to_str().unwrap();

            for proto in &protobuf_strict::protos() {
                println!(
                    "Will generate a new proto, path: {:#?}, output_path: {:#?}, proto: {}",
                    path, output_path, proto
                );

                let proto = format!("{proto}.proto");

                assert!(Path::new(path).join(&proto).exists());

                let status = Command::new("protoc")
                    .arg(format!("--proto_path={}", path))
                    .arg(format!("--swift_out={}", output_path))
                    .arg("--swift_opt=Visibility=Public")
                    .arg(&proto)
                    .args(&$args)
                    .status()
                    .expect("Failed to generate proto");

                assert!(status.success());
            }
        };
    }

    let empty: [&str; 0] = [];

    write!("generated", empty);

    println!("Generated the normal protos");

    let uuids: String = protobuf_strict::get_uuids().join("|");
    let args = [
        format!("--swift_opt=Uuids={}", uuids),
        "--swift_opt=RemoveBoilerplateCode=true".to_string(),
    ];

    write!("bgenerated", args);
}
