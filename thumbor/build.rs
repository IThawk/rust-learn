use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("src/pd")
        .compile_protos(&["src/abi.proto"], &["src"])?;
    Ok(())
}
