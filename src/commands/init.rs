use std::{fs, io};

fn force_create_dir(name: &str) -> Result<(), io::Error> {
    match fs::create_dir(name) {
        Ok(_) => {
            println!("{} directory created successfully.", name);
            Ok(())
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            println!("The {} directory already exists.", name);
            fs::remove_dir_all(name).expect("Failed to remove file");
            force_create_dir(name)
        }
        Err(err) => {
            println!("Failed to create {} directory: {:?}", name, err);
            Err(err)
        }
    }
}

fn force_create_file(name: &str) -> Result<(), io::Error> {
    match fs::File::create(name) {
        Ok(_) => {
            println!("{} file created successfully.", name);
            Ok(())
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            println!("The {} file already exists.", name);
            fs::remove_file(name).expect("Failed to remove file");
            force_create_file(name)
        }
        Err(err) => {
            println!("Failed to create {} file: {:?}", name, err);
            Err(err)
        }
    }
}

fn create_rgit_dir() {
    match force_create_dir(".rgit") {
        Ok(_) => {
            force_create_file(".rgit/HEAD").expect("Failed to create HEAD file");
            force_create_dir(".rgit/config").expect("Failed to create config file");
            force_create_dir(".rgit/objects").expect("Failed to create objects directory");
            force_create_dir(".rgit/objects/info").expect("Failed to create info file");
            force_create_dir(".rgit/objects/pack").expect("Failed to create pack file");
            force_create_dir(".rgit/refs").expect("Failed to create refs directory");
            force_create_dir(".rgit/refs/heads").expect("Failed to create heads directory");
            force_create_dir(".rgit/refs/tags").expect("Failed to create tags directory");
            println!("rGit directory created successfully.");
        }
        Err(err) => {
            println!("Failed to create rGit directory: {:?}", err);
            panic!();
        }
    }
}

pub fn run() {
    create_rgit_dir();
    todo!();
}
