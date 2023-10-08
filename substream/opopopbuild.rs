use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("TX", "abi/tx.json")?
        .generate()?
        .write_to_file("src/abi/tx.rs")?;

    Ok(())
}
