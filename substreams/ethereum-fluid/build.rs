use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ERC20", "abi/ERC20.json")?
        .generate()?
        .write_to_file("src/abi/ERC20.rs")?;
    
    Abigen::new("DexReservesResolver", "abi/dexReservesResolver.json")?
        .generate()?
        .write_to_file("src/abi/dexReservesResolver.rs")?;

    Abigen::new("StorageRead", "abi/storageRead.json")?
        .generate()?
        .write_to_file("src/abi/storageRead.rs")?;

    Ok(())
}
