use treedir::{Tree, generate_tree};
use std::path::{Path, PathBuf};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if(args.len() != 2) {
	println!("Usage: treedir <path_to_dir>");
	return;
    }

    let path_string = &args[1];
    let path = Path::new(path_string);
    let tree = generate_tree(path);
    println!("{:#?}", tree);

    println!("PID: {}", std::process::id());
}
