use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

fn main() {
	let localappdata = env::var("LOCALAPPDATA").expect("couldn't find LOCALAPPDATA env");
	let versions = Path::new(&localappdata).join("Roblox").join("Versions");

	// embed oof file
	let oof: &'static [u8] = include_bytes!("ouch.ogg");

	// exit if roblox is not in localappdata
	if !versions.exists() {
		println!("roblox is probably not installed");
		std::process::exit(1);
	}

	println!("patching...");

	let versions = fs::read_dir(versions).expect("no versions found");

	for version in versions {
		let version = version.unwrap();
		let versionname = version.file_name().into_string().unwrap();

		if versionname.starts_with("version") {
			let soundspath = Path::new(&version.path()).join("content").join("sounds");
			
			if soundspath.exists() {
				let ouchpath = Path::new(&soundspath).join("ouch.ogg");
				let mut file = File::create(&ouchpath).unwrap();
				file.write_all(oof).expect("couldn't write to file");
			}
		}
	}

	println!("successfully patched oof");
	
}
