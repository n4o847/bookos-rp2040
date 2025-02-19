use std::{fs, io};

fn main() -> io::Result<()> {
    let script = fs::canonicalize("boot2.ld")?;
    println!("cargo::rustc-link-arg=-T{}", script.display());
    println!("cargo::rerun-if-changed={}", script.display());

    Ok(())
}
