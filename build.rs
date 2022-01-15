use vergen::{vergen, Config, SemverKind};

fn compile_protobuf() {
    let mut cfg = prost_build::Config::new();
    cfg.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    cfg.out_dir("src/pb")
        .compile_protos(&["idl/request.proto"], &["idl"])
        .unwrap();
}

fn generate_build_info() -> anyhow::Result<()> {
    let mut config = Config::default();
    // Change the SEMVER output to the lightweight variant
    *config.git_mut().semver_kind_mut() = SemverKind::Normal;
    // Add a `-dirty` flag to the SEMVER output
    *config.git_mut().semver_dirty_mut() = Some("-dirty");
    // Generate the instructions
    if let Err(e) = vergen(config) {
        eprintln!("error occurred while generating instructions: {:?}", e);
        let mut config = Config::default();
        *config.git_mut().enabled_mut() = false;
        vergen(config)
    } else {
        println!("generate_build_info success...");
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    compile_protobuf();
    generate_build_info()
}
