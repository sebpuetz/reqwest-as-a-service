#[cfg(not(feature = "with-client"))]
fn main() -> anyhow::Result<()> {
    tonic_build::configure()
        .out_dir("src/")
        .build_client(false)
        .compile(&["./proto/api.proto"], &["./proto"])?;
    Ok(())
}

#[cfg(feature = "with-client")]
fn main() -> anyhow::Result<()> {
    tonic_build::configure()
        .out_dir("src/")
        .compile(&["./proto/api.proto"], &["./proto"])?;
    Ok(())
}
