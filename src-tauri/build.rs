fn main() -> anyhow::Result<()> {
    generate_struct()?;
    tauri_build::build();

    Ok(())
}

fn generate_struct() -> anyhow::Result<()> {
    // https://github.com/oxidecomputer/typify/issues/889
    let mut settings = typify::TypeSpaceSettings::default();
    settings.with_crate("super", typify::CrateVers::Any, None);
    let mut typespace = typify::TypeSpace::new(&settings);
    typespace.add_root_schema(serde_json::from_str(include_str!(
        "../docs/public/schema/Manifest.json"
    ))?)?;
    let out_dir = std::env::var("OUT_DIR")?;
    let out_path = std::path::Path::new(&out_dir).join("models_generated.rs");
    std::fs::write(
        &out_path,
        typespace
            .to_stream()
            .to_string()
            .replace(":: super", "super"),
    )?;
    println!(
        "cargo::rustc-env=MODELS_GENERATED_PATH={}",
        out_path.display()
    );
    Ok(())
}
