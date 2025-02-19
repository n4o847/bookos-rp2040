mod crc32;

use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

use clap::{Args, Parser, Subcommand};
use crc32::Crc32;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(BuildArgs),
}

#[derive(Args)]
struct BuildArgs {
    #[clap(short, long)]
    release: bool,
}

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn ws_dir() -> PathBuf {
    Path::new(CARGO_MANIFEST_DIR).join("../ws")
}

fn ws_target_dir() -> PathBuf {
    ws_dir().join("target")
}

fn ws_bin_exe(name: &str, release: bool) -> PathBuf {
    ws_target_dir()
        .join("thumbv6m-none-eabi")
        .join(if release { "release" } else { "debug" })
        .join(name)
}

fn build(package: &str, release: bool) -> ExitStatus {
    let mut command = Command::new("cargo");

    command.arg("build");

    if release {
        command.arg("--release");
    }

    command.arg("--package").arg(package);

    // This produces a fat binary, so we don't use it
    // command.env("RUSTFLAGS", "--emit asm");

    command.current_dir(ws_dir());

    eprintln!();
    eprintln!("[xtask] Running command: {:?}", command);

    command.status().expect("Failed to execute command")
}

fn objdump(name: &str, release: bool) -> ExitStatus {
    let mut command = Command::new("arm-none-eabi-objdump");

    command.arg("--disassemble-all");

    let input = ws_bin_exe(name, release);
    command.arg(&input);

    let output = input.with_extension("objdump");
    let output_file = File::create(&output).expect("Failed to create objdump file");
    eprintln!();
    eprintln!("[xtask] Writing objdump to {}", output.display());
    command.stdout(output_file);

    command.current_dir(ws_dir());

    eprintln!();
    eprintln!("[xtask] Running command: {:?}", command);

    command.status().expect("Failed to execute command")
}

fn objcopy(release: bool) -> ExitStatus {
    const SECTION: &str = ".text";

    let mut command = Command::new("arm-none-eabi-objcopy");

    command.arg("--output-target").arg("binary");
    command.arg("--only-section").arg(SECTION);

    let input = ws_bin_exe("boot2", release);
    command.arg(&input);

    let output = input.with_extension("bin");
    command.arg(&output);

    command.current_dir(ws_dir());

    eprintln!();
    eprintln!("[xtask] Running command: {:?}", command);

    command.status().expect("Failed to execute command")
}

fn make_pad_checksum(release: bool) {
    let mut bin = fs::read(ws_bin_exe("boot2.bin", release)).expect("Failed to read boot2.bin");
    assert!(bin.len() <= 252);

    bin.resize(252, 0);

    // See [2.8.1.3.1. Checksum]
    let crc32 = Crc32::new();
    let checksum = crc32.checksum(&bin);
    eprintln!();
    eprintln!("[xtask] Checksum: {:#08x}", checksum);

    bin.extend(checksum.to_le_bytes());

    let output = ws_bin_exe("boot2.padded.bin", release);
    fs::write(&output, &bin).expect("Failed to write boot2.padded.bin");
    eprintln!();
    eprintln!("[xtask] Writing padded binary to {}", output.display());
}

impl BuildArgs {
    fn run(self) {
        if !build("boot2", self.release).success() {
            panic!("[xtask] Failed to build boot2");
        }

        if !objdump("boot2", self.release).success() {
            panic!("[xtask] Failed to disassemble boot2");
        }

        if !objcopy(self.release).success() {
            panic!("[xtask] Failed to create boot2.bin");
        }

        make_pad_checksum(self.release);

        if !build("app", self.release).success() {
            panic!("[xtask] Failed to build app");
        }

        if !objdump("app", self.release).success() {
            panic!("[xtask] Failed to disassemble app");
        }
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Build(args) => args.run(),
    }
}
