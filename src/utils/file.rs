use std::{fs, io};

pub fn create(name: &str) -> Result<(), io::Error> {
    match fs::File::create(name) {
        Ok(_) => {
            println!("{} file created successfully.", name);
            Ok(())
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            println!("The {} file already exists.", name);
            Ok(())
        }
        Err(err) => {
            println!("Failed to create {} file: {:?}", name, err);
            Err(err)
        }
    }
}

pub fn force_create(name: &str) -> Result<(), io::Error> {
    match fs::File::create(name) {
        Ok(_) => {
            println!("{} file created successfully.", name);
            Ok(())
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            println!("The {} file already exists.", name);
            fs::remove_file(name).expect("Failed to remove file");
            force_create(name)
        }
        Err(err) => {
            println!("Failed to create {} file: {:?}", name, err);
            Err(err)
        }
    }
}
