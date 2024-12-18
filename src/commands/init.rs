use std::{fs, io};

fn create_rgit_dir() {
    match fs::create_dir(".rgit") {
        Ok(_) => {
            println!(".rgit directory created successfully.");
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            println!("The .rgit directory already exists.");
            fs::remove_dir_all(".rgit").expect("Failed to remove file");
            create_rgit_dir();
        }
        Err(err) => {
            println!("Failed to create .rgit directory: {:?}", err);
            panic!();
        }
    }
}

pub fn run() {
    create_rgit_dir();
    todo!();
}
