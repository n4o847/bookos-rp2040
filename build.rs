use std::{
    env,
    fs::{self, File},
    io::{self, BufRead},
    process::Command,
};

fn make_elf() -> io::Result<()> {
    let mut command = Command::new(env!("CARGO"));
    command
        .arg("build")
        .arg("--release")
        .args(["--package", "boot2"]);

    println!("cargo::warning=[make_elf] {:?}", command);

    let output = command.output()?;

    for line in output.stderr.lines() {
        println!("cargo::warning=[make_elf] {}", line?);
    }

    if !output.status.success() {
        return Err(io::Error::other("failed to make elf"));
    }

    Ok(())
}

fn make_objdump() -> io::Result<()> {
    let target = File::create("target/thumbv6m-none-eabi/release/boot2.objdump")?;

    let mut command = Command::new("arm-none-eabi-objdump");
    command
        .arg("--disassemble-all")
        .arg("target/thumbv6m-none-eabi/release/boot2")
        .stdout(target);

    println!("cargo::warning=[make_objdump] {:?}", command);

    let output = command.output()?;

    for line in output.stderr.lines() {
        println!("cargo::warning=[make_objdump] {}", line?);
    }

    if !output.status.success() {
        return Err(io::Error::other("failed to make objdump"));
    }

    Ok(())
}

fn make_bin() -> io::Result<()> {
    let mut command = Command::new("arm-none-eabi-objcopy");
    command
        .args(["--output-target", "binary"])
        .args(["--only-section=.text"])
        .arg("target/thumbv6m-none-eabi/release/boot2")
        .arg("target/thumbv6m-none-eabi/release/boot2.bin");

    println!("cargo::warning=[make_bin] {:?}", command);

    let output = command.output()?;

    for line in output.stderr.lines() {
        println!("cargo::warning=[make_bin] {}", line?);
    }

    if !output.status.success() {
        return Err(io::Error::other("failed to make bin"));
    }

    Ok(())
}

struct Crc32 {
    table: [u32; 256],
}

impl Crc32 {
    fn new() -> Self {
        let mut table = [0u32; 256];

        for i in 0..256 {
            let mut c = (i << 24) as u32;
            for _ in 0..8 {
                c = if c & 0x80000000 != 0 {
                    0x04c11db7 ^ (c << 1)
                } else {
                    c << 1
                };
            }
            table[i] = c;
        }

        Self { table }
    }

    fn checksum(&self, buf: &[u8]) -> u32 {
        let mut c = 0xffffffff;
        for &byte in buf {
            c = (c << 8) ^ self.table[((c >> 24) ^ byte as u32) as usize];
        }
        c
    }
}

fn make_pad_checksum() -> io::Result<()> {
    let bin = fs::read("target/thumbv6m-none-eabi/release/boot2.bin")?;
    assert!(bin.len() <= 252);

    let mut padded = bin.clone();
    padded.resize(252, 0);

    // See [2.8.1.3.1. Checksum]
    let crc32 = Crc32::new();
    let checksum = crc32.checksum(&padded);

    println!(
        "cargo::warning=[make_pad_checksum] checksum: {:#08x}",
        checksum
    );

    padded.extend(checksum.to_le_bytes());

    fs::write("target/thumbv6m-none-eabi/release/boot2.padded.bin", padded)?;

    Ok(())
}

fn main() -> io::Result<()> {
    println!("cargo::rerun-if-changed=boot2");

    make_elf()?;
    make_objdump()?;
    make_bin()?;
    make_pad_checksum()?;

    let boot2 = fs::canonicalize("target/thumbv6m-none-eabi/release/boot2.padded.bin")?;
    println!("cargo::rustc-env=BOOT2={}", boot2.display());
    println!("cargo::warning=[main] BOOT2={:?}", boot2.display());

    let script = fs::canonicalize("memmap.ld")?;
    println!("cargo::rustc-link-arg=-T{}", script.display());
    println!("cargo::rerun-if-changed={}", script.display());

    Ok(())
}
