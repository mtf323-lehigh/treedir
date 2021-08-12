/*
main.rs

Copyright 2021 Michael Fitzgerald

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
