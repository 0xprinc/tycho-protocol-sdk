use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Resolver", "abi/resolver.json")?
        .generate()?
        .write_to_file("src/abi/resolver.rs")?;
    Abigen::new("Factory", "abi/factory.json")?
        .generate()?
        .write_to_file("src/abi/factory.rs")?;
    Ok(())
}
