use std::{fs, io};

pub fn force_create(name: &str) -> Result<(), io::Error> {
    match fs::create_dir(name) {
        Ok(_) => {
            println!("{} directory created successfully.", name);
            Ok(())
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            println!("The {} directory already exists.", name);
            fs::remove_dir_all(name).expect("Failed to remove file");
            force_create(name)
        }
        Err(err) => {
            println!("Failed to create {} directory: {:?}", name, err);
            Err(err)
        }
    }
}

pub fn create(name: &str) -> Result<(), io::Error> {
    match fs::create_dir(name) {
        Ok(_) => {
            println!("{} directory created successfully.", name);
            Ok(())
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            println!("The {} directory already exists.", name);
            Ok(())
        }
        Err(err) => {
            println!("Failed to create {} directory: {:?}", name, err);
            Err(err)
        }
    }
}
