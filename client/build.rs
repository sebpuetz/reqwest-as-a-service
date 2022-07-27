fn main() -> anyhow::Result<()> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/")
        .compile(&["../proto/api.proto"], &["../proto"])?;
    Ok(())
}
