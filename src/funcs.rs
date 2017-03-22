use model::*;
use files::*;

use std::path::Path;
use std::fs::{DirEntry, File};
use std::io::Read;
use std::process::{exit, Command};

/// exit
pub fn go_die() -> ! {
	println!("Have a nice day :)");
	exit(0);
}

/// clear the screen
// pub fn clear_screen() {
// 	Command::new("cls").output().expect("Failed to run command: cls");
// }

/// print help
pub fn print_help() {
	println!("Commands:");
	println!("data           -- print the meta data stored in the cofiguration file.");
	println!("ls             -- print all the files.");
	println!("exit           -- exit project manager.");
	println!("help           -- print this doc.");
	println!("line           -- see how many lines of code is here in your project.");
}

/// print meta data
/// stored in the cofiguration file
pub fn print_meta(cfg: &Config) {
	println!("Mame:");
	println!("\t{}", cfg.proj_name());
	println!("Path:");
	println!("\t{}", cfg.path());
	println!("Ignored:");
	for i in cfg.ignored() {
		println!("\t{} ", i);
	}
	println!("Ignored Suffix:");
	for i in cfg.ignored_suffix() {
		println!("\t{} ", i);
	}
	println!("Build Script:");
	for i in cfg.build() {
		println!("\t{} ", i);
	}
}

/// list the files
#[allow(unused_must_use)]
pub fn print_files(cfg: &Config) {
	visit_files(&cfg, Path::new("."), &|e: &DirEntry| {
		println!("{}", e.path().display());
	});
}

/// print how many lines of code is here
#[allow(unused_must_use)]
pub fn print_code_line_sum(cfg: &Config) {
	static mut sum: u64 = 0;
	visit_files(&cfg, Path::new("."), &|e: &DirEntry| {
		let mut bytes: Vec<u8> = Vec::new();
		let mut lines: u64 = 1;
		let path = e.path();
		let mut f = File::open(path.clone()).unwrap();
		let mut size: u64 = 0;
		match f.read_to_end(&mut bytes) {
			Ok(s) => {
				size = s as u64;
				let mut cnt = 0;
				for i in &bytes {
					if *i == '\n' as u8 { cnt += 1; }
				}
				lines += cnt;
			},
			_ => { },
		}
		println!("In {:<26} => {} lines, {} per line.", path.display(), lines, size / lines);
		unsafe {
			sum += lines;
		}
	});
	unsafe {
		println!("Total: {} lines of code.", sum);
	}
}

pub fn print_git_data(cfg: &Config) {
	// println!("In project {}:", cfg.proj_name());
	let status = match Command::new("git")
			.arg("status")
			.output() {
		Ok(o) => {
			println!("Git root detected in {}.", cfg.proj_name());
			o.stdout
		},
		_ => {
			println!("{} is not a git repository.", cfg.proj_name());
			return;
		}
	};
	let info = String::from_utf8(status.clone()).unwrap();
	for ln in info.lines() {
		// let ln = ln.trim();
		if !ln.starts_with("  (use \"git ") &&
				!ln.starts_with("no changes added to commit") &&
				!ln.trim().is_empty() {
			println!("{}", ln);
		}
	}
}

#[allow(dead_code)]
pub fn build_proj(cfg: &Config) {
	for i in cfg.build() {
		println!("Running: {}", i);
		match Command::new("call").arg(&i).output() {
			Ok(o) => println!("{}", String::from_utf8(o.stdout).unwrap()),
			_ => {
				println!("Error while running this command!");
				break;
			},
		}
	}
}

