extern crate walkdir;

use std::collections::HashMap;
use walkdir::WalkDir;

fn main() -> Result<(), std::io::Error>{
    let mut args: Vec<String> = std::env::args().collect();
    let root = match args.pop() {
    	Some(p) => std::path::PathBuf::from(p),
    	None => std::env::current_dir()?,
    };

    let mut filenames = HashMap::new();

    for entry in WalkDir::new(root.as_path())
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter > 1{
	        println!("{:?} : {:?}", *counter, f_name);
	    }

    }
    Ok(())
}


