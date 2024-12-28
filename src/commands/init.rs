use crate::utils::{dir, file};

fn create_rgit_dir() {
    match dir::force_create(".rgit") {
        Ok(_) => {
            file::force_create(".rgit/HEAD").expect("Failed to create HEAD file");
            dir::force_create(".rgit/config").expect("Failed to create config file");
            dir::force_create(".rgit/objects").expect("Failed to create objects directory");
            dir::force_create(".rgit/objects/info").expect("Failed to create info file");
            dir::force_create(".rgit/objects/pack").expect("Failed to create pack file");
            dir::force_create(".rgit/refs").expect("Failed to create refs directory");
            dir::force_create(".rgit/refs/heads").expect("Failed to create heads directory");
            dir::force_create(".rgit/refs/tags").expect("Failed to create tags directory");
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
