use std::path::Path;
use std::env;

fn main() {
	let localappdata = env::var("LOCALAPPDATA").expect("couldn't find LOCALAPPDATA env");
	let versions = Path::new(&localappdata).join("Roblox\\Versions");

	if !versions.exists() {
		println!("roblox is probably not installed");
		std::process::exit(1);
	}

	
	
}
