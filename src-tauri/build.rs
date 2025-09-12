fn main() -> anyhow::Result<()> {
    generate_struct(
        "generated_manifest.rs",
        "MANIFEST_SCHEMA_PATH",
        include_str!("../docs/public/schema/Manifest.json"),
    )?;
    generate_struct(
        "generated_registry.rs",
        "REGISTRY_SCHEMA_PATH",
        include_str!("../docs/public/schema/Registry.json"),
    )?;
    tauri_build::build();

    Ok(())
}

fn generate_struct(filename: &str, env_var_name: &str, schema: &str) -> anyhow::Result<()> {
    // https://github.com/oxidecomputer/typify/issues/889
    let mut settings = typify::TypeSpaceSettings::default();
    settings.with_crate("super", typify::CrateVers::Any, None);
    let mut typespace = typify::TypeSpace::new(&settings);
    typespace.add_root_schema(serde_json::from_str(schema)?)?;
    let out_dir = std::env::var("OUT_DIR")?;
    let out_path = std::path::Path::new(&out_dir).join(filename);
    std::fs::write(
        &out_path,
        typespace
            .to_stream()
            .to_string()
            .replace(":: super", "super"),
    )?;
    println!("cargo::rustc-env={}={}", env_var_name, out_path.display());
    Ok(())
}
