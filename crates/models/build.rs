fn main() -> anyhow::Result<()> {
    generate_struct()?;

    Ok(())
}

fn generate_struct() -> anyhow::Result<()> {
    // https://github.com/oxidecomputer/typify/issues/889
    let mut settings = typify::TypeSpaceSettings::default();
    settings.with_crate("super", typify::CrateVers::Any, None);
    let mut typespace = typify::TypeSpace::new(&settings);

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let schema_dir = manifest_dir.join("../../docs/public/schema");

    let mut defs = std::collections::HashMap::new();
    for schema in std::fs::read_dir(&schema_dir)? {
        let schema = schema?;
        if schema.file_type()?.is_file() {
            println!("cargo::rerun-if-changed={}", schema.path().display());
            let json = std::fs::read_to_string(schema.path())?;
            let root: schemars::schema::RootSchema = serde_json::from_str(&json)?;
            for (k, v) in root.definitions.clone() {
                defs.insert(k, v);
            }
        }
    }
    typespace.add_ref_types(defs)?;

    for schema in std::fs::read_dir(&schema_dir)? {
        let schema = schema?;
        if schema.file_type()?.is_file() {
            let json = std::fs::read_to_string(schema.path())?;
            let root: schemars::schema::Schema = serde_json::from_str(&json)?;
            typespace.add_type(&root)?;
        }
    }

    let out_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let out_path = std::path::Path::new(&out_dir)
        .join("src")
        .join("schema_generated.rs");
    std::fs::write(
        &out_path,
        prettyplease::unparse(&syn::parse2::<syn::File>(typespace.to_stream()).unwrap())
            .replace("::super", "super"),
    )?;
    println!("cargo::rustc-env=SCHEMA_RS_PATH={}", out_path.display());
    Ok(())
}
