use anyhow::Result;
use nix::fcntl::Flock;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::{env, path::Path};
use syzlang_parser::parser::{Consts, Parsed, Statement};

fn chosen_oss() -> Vec<String> {
	let oss: Vec<&str> = vec![
		#[cfg(feature = "akaros")]
		"akaros",
		#[cfg(feature = "darwin")]
		"darwin",
		#[cfg(feature = "freebsd")]
		"freebsd",
		#[cfg(feature = "fuchsia")]
		"fuchsia",
		#[cfg(feature = "linux")]
		"linux",
		#[cfg(feature = "netbsd")]
		"netbsd",
		#[cfg(feature = "openbsd")]
		"openbsd",
		#[cfg(feature = "trusty")]
		"trusty",
		#[cfg(feature = "windows")]
		"windows",
	];
	oss.into_iter()
		.map(|x| x.to_string())
		.collect::<Vec<String>>()
}

fn generate(skdir: &mut PathBuf, outdir: &Path) -> Result<()> {
	let mut code = String::from("");

	skdir.push("sys");
	println!("skdir {skdir:?}");

	for os in chosen_oss().into_iter() {
		println!("os {os:?}");
		let mut gdir = outdir.to_path_buf();
		gdir.push(&os);

		println!("gdir {gdir:?}");
		if !gdir.is_dir() {
			std::fs::create_dir(&gdir)?;
		}

		let mut consts = Consts::default();
		let mut stmts = Vec::new();

		skdir.push(&os);
		println!("skdir {skdir:?}");
		std::thread::sleep(std::time::Duration::from_millis(1000));
		let paths = std::fs::read_dir(&skdir).unwrap();
		for p in paths {
			let p = p?;
			println!("reading {p:?}");
			let ft = p.file_type()?;
			if ft.is_file() {
				let path = p.path();
				println!("is file {path:?}");
				let ext = path.extension();
				if ext == Some(std::ffi::OsStr::new("const")) {
					println!("parsing const");
					consts.create_from_file(&path)?;
				} else if ext == Some(std::ffi::OsStr::new("txt")) {
					println!("parsing stmts");
					let mut stmt = Statement::from_file(&path)?;
					stmts.append(&mut stmt);
				}
			}
		}

		let inscode = format!(
			r#"
/// Data files for {os} operating system
pub mod {os} {{
	lazy_static::lazy_static! {{
		#[derive(Default)]
		pub static ref PARSED: std::sync::RwLock<syzlang_parser::parser::Parsed> = {{
			let parsed = include_bytes!(concat!(env!("OUT_DIR"), "/{os}/parsed.json"));
			let parsed = std::str::from_utf8(parsed).unwrap();
			std::sync::RwLock::new(serde_json::from_str(parsed).unwrap())
		}};
	}}
}}
		"#
		);
		code.push_str(inscode.as_str());

		let parsed = Parsed::new(consts, stmts)?;
		let out = serde_json::to_string(&parsed)?;
		gdir.push("parsed.json");
		std::fs::write(&gdir, out)?;
		gdir.pop();

		skdir.pop(); // os
	}
	skdir.pop(); // sys

	let mut gdir = outdir.to_path_buf();
	gdir.push("data.rs");
	println!("writing to {gdir:?}");
	std::fs::write(gdir, code)?;

	Ok(())
}

fn download_syzkaller(skdir: &PathBuf, max: usize) {
	if !skdir.exists() {
		println!("Directory does not exist, downloading");
		let c = Command::new("git")
			.arg("clone")
			.arg("--quiet")
			.arg("--branch")
			.arg("master")
			.arg("https://github.com/google/syzkaller.git")
			.arg(skdir)
			.output()
			.expect("Unable to download syzkaller from git");
		println!("c1 {c:?}");
		assert!(c.stderr == b"");
	} else {
		let c = Command::new("git")
			.arg("-C")
			.arg(skdir)
			.arg("checkout")
			.arg("master")
			.output()
			.expect("unable to checkout master");
		println!("c1.5 {c:?}");
		if !c.status.success() {
			if max > 0 {
				std::fs::remove_dir_all(skdir).expect("Unable to remove syzkaller directory");
				download_syzkaller(skdir, max - 1)
			} else {
				panic!("unable to download Syzkaller");
			}
		}
	}
}

fn acquire_lock(scratch: &Path) -> anyhow::Result<Flock<File>> {
	let mut lock = PathBuf::from(scratch);
	lock.push("build.lock");
	let lock = std::fs::OpenOptions::new()
		.create(true)
		.truncate(true)
		.write(true)
		.open(lock)
		.unwrap();
	let lock = nix::fcntl::Flock::lock(lock, nix::fcntl::FlockArg::LockExclusive)
		.expect("Unable to acquire lock");
	Ok(lock)
}


fn main() -> Result<()> {
	println!("Build started");
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let out_dir = PathBuf::from(out_dir);

	// Get a scratch directory to download code
	let mut skdir = scratch::path("sk");
	println!("using path {skdir:?}");
	let mut skversion = out_dir.clone();
	skversion.push("skversion.txt");

	let oss = chosen_oss();
	if oss.is_empty() {
		std::fs::write(skversion, b"").unwrap();
		let mut gdir = out_dir.to_path_buf();
		gdir.push("data.rs");
		std::fs::write(gdir, b"")?;
	} else {
		// Get a lock so that multiple build.rs processes don't interfere with
		// eachother
		let lock = skdir.clone();
		let lock = acquire_lock(&lock)?;

		skdir.push("syzkaller");
		println!("output will be in {skdir:?}");
		download_syzkaller(&skdir, 1);

		println!("pulling newest version of Syzkaller");
		let c = Command::new("git")
			.arg("-C")
			.arg(&skdir)
			.arg("pull")
			.output()
			.expect("unable to pull newest version from syzkaller");

		println!("c2 {c:?}");
		assert!(c.status.success());

		let checked = "1834ff143d083ae2c374f2a18d887575887321a9";
		let c = Command::new("git")
			.arg("-C")
			.arg(&skdir)
			.arg("checkout")
			.arg(checked)
			.output()
			.expect("unable to get specific version {checked}");

		println!("c3 {c:?}");
		assert!(c.status.success());

		// Get last hash from git
		let hash = Command::new("git")
			.arg("-C")
			.arg(&skdir)
			.arg("rev-parse")
			.arg("HEAD")
			.output()
			.expect("unable to get git hash");

		println!("c4 {hash:?}");
		assert!(hash.status.success());
		let hash = std::str::from_utf8(&hash.stdout).unwrap();
		let hash = hash.trim_end();
		println!("git has = {hash}");
		assert!(hash == checked);

		let is_match = if skversion.is_file() {
			let data = std::fs::read(&skversion).unwrap();
			let data = std::str::from_utf8(&data).unwrap();
			data == hash
		} else {
			println!("skversion.txt does not exit, generating fresh");
			false
		};
		println!("skversion.txt returned {is_match}");

		if !is_match {
			println!("Generating fresh");
			generate(&mut skdir, &out_dir).unwrap();
			std::fs::write(skversion, hash).unwrap();
		} else {
			println!("Hashes matches, aborting fresh build");
		}

		lock.unlock().expect("unable to unlock lock");
		println!("Lock unlocked");
	}

	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-env-changed=CARGO_CFG_FEATURE");
	Ok(())
}
