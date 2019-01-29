extern crate walkdir;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use walkdir::WalkDir;

fn main() -> Result<(), std::io::Error> {
    let mut args: Vec<String> = std::env::args().collect();
    let root = match args.pop() {
        Some(p) => std::path::PathBuf::from(p),
        None => std::env::current_dir()?,
    };

    let mut filenames: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();

    for entry in WalkDir::new(root.as_path())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        println!("{:?}", f_name);
        let list = filenames.entry(f_name).or_insert(Vec::new());
        list.push(entry.into_path());
    }

    for k in filenames.keys() {
        match filenames.get(k) {
            Some(path_list) => {
                println!("found: {:?} {:?}", path_list.iter().count(), k);
                for p in path_list.iter() {
                    println!("    {:?}", p.to_string_lossy());
                }
            }
            None => continue,
        }
    }
    Ok(())
}

fn is_similar(a: &String, b: &String) -> bool {
    true
}
