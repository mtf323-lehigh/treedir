/* treedir */

use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::io;

#[derive(Debug)]
pub struct Tree {
    root_node: Node,
}

impl Tree {
    pub fn new(node: Node) -> Tree {
	Tree { root_node: node }
    }

    pub fn root(&self) -> &Node {
	&self.root_node
    }

    pub fn root_mut(&mut self) -> &mut Node {
	&mut self.root_node
    }

    pub fn root_dir(&self) -> PathBuf {
	self.root_node.entry_path.clone()
    }

    /*
    pub fn search(&self, query: Path, current_dir: Option<Path>)
		  -> Option<&Node> {
	let components = query.components();
	let current_dir = match current_dir {
	    Some(p) => match components.next() {
		Component::RootDir => root_node.entry_path,
		Component::ParentDir => 
		Component::CurDir | _ => p.to_path_buf()
	    },
	    None => root_node.entry_path
	};

	for comp in components {
	    
	}
    }
     */
}

#[derive(Debug)]
pub struct Node {
    entry_path: PathBuf,
    entry_type: EntryType,
    children: Rc<Vec<Node>>,
}

#[derive(Debug)]
enum EntryType {
    File,
    Directory,
    Symlink,
    Other
}

#[derive(Debug)]
pub enum Error {
    GeneralError,
    IOError(io::Error),
}

pub fn generate_tree(path: &Path) -> Result<Tree, Error> {
    let mut children: Vec<Node> = Vec::new();

    /*
    let entries = match fs::read_dir(path) {
	Ok(i) => i,
	Err(_) => { return Err(Error::GeneralError); }
    };
     */

    let path = path.canonicalize()
	.map_err(|e| Error::IOError(e))?;
    let entries = fs::read_dir(&path)
	.map_err(|e| Error::IOError(e))?;

    for entry in entries {
	let entry = entry.map_err(|_e| Error::GeneralError)?;
	let node = process_entry(&entry)?;
	children.push(node);
    }

    let entry_path = path.canonicalize()
	.map_err(|e| Error::IOError(e))?;
    let root_node = Node {
	entry_path: entry_path,
	entry_type: EntryType::Directory,
	children: Rc::new(children)
    };
    let tree = Tree::new(root_node);
    Ok(tree)
}

fn process_entry(entry: &DirEntry) -> Result<Node, Error> {
    let t = entry.file_type().map_err(|_e| Error::GeneralError)?;
    let entry_type = {
	if t.is_file() { EntryType::File }
	else if t.is_dir() { EntryType::Directory }
	else if t.is_symlink() { EntryType::Symlink }
	else { EntryType::Other }
    };

    let entry_path = entry.path().canonicalize()
	.map_err(|e| Error::IOError(e))?;

    /*let node = match entry_type {
	EntryType::File => {
	    let children: Vec<Node> = Vec::new();
	    Node { entry_path, entry_type, Rc::new(children) }
	},
	EntryType::Directory => {
	    let children = process_directory(entry)?;
	    Node { entry_path, entry_type, Rc::new(children) }
	},
	EntryType::Symlink => {
	    let children: Vec<Node> = Vec::new();
	    Node { entry_path, entry_type, Rc::new(children) }
	},
	_ => { return Err(Error::GeneralError); }
    };*/

    let children: Vec<Node> = match entry_type {
	EntryType::File => Vec::new(),
	EntryType::Directory => process_directory(entry)?,
	EntryType::Symlink => Vec::new(),
	_ => Vec::new()
    };

    let node = Node {
	entry_path: entry_path,
	entry_type: entry_type,
	children: Rc::new(children)
    };

    //println!("{:?}", node.entry_path);
    Ok(node)
}

fn process_directory(entry: &DirEntry) -> Result<Vec<Node>, Error> {
    let mut children: Vec<Node> = Vec::new();

    let path = entry.path().canonicalize()
	.map_err(|e| Error::IOError(e))?;
    
    let entries = fs::read_dir(&path)
	.map_err(|e| Error::IOError(e))?;
    for entry in entries {
	let entry = entry.map_err(|_e| Error::GeneralError)?;
	let node = process_entry(&entry)?;
	children.push(node);
    }

    Ok(children)
}
