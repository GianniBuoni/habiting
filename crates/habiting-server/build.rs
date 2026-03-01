use tonic_prost_build::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_protos("../../proto/habiting.proto")?;
    Ok(())
}
