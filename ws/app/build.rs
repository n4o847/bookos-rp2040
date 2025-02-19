use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let profile = env::var("PROFILE").unwrap();

    let boot2 = Path::new("../target/thumbv6m-none-eabi")
        .join(profile)
        .join("boot2.padded.bin")
        .canonicalize()?;
    println!("cargo::rustc-env=BOOT2={}", boot2.display());

    let script = fs::canonicalize("memmap.ld")?;
    println!("cargo::rustc-link-arg=-T{}", script.display());
    println!("cargo::rerun-if-changed={}", script.display());

    Ok(())
}
