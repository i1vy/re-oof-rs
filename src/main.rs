use std::path::Path;
use std::{env, fs};

fn main() {
	let localappdata = env::var("LOCALAPPDATA").expect("couldn't find LOCALAPPDATA env");
	let versions = Path::new(&localappdata).join("Roblox\\Versions");

	// exit if roblox is not in localappdata
	if !versions.exists() {
		println!("roblox is probably not installed");
		std::process::exit(1);
	}

	// the versions directory
	let versions = fs::read_dir(versions).expect("no versions found");

	// for every version found
	for version in versions {
		let versionname = version.as_ref().unwrap().file_name().into_string().unwrap();

		// if it is actually a version thing
		if versionname.starts_with("version") {
			println!("{:?}", version.unwrap().file_name())
		}
	}
	
}
