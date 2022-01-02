fn main() {
    let mut cfg = prost_build::Config::new();
    cfg.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    cfg.out_dir("src/pb")
        .compile_protos(&["idl/request.proto"], &["idl"])
        .unwrap();
}
