use sha1::{Digest, Sha1};
use std::fs;
use std::io::{BufReader, Read};

fn generate_file_sha1(file_path: &str) -> String {
    let mut hasher = Sha1::new();
    let file = fs::File::open(file_path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 4096];
    let metadata = fs::metadata(file_path).expect("Failed to read metadata");
    let literal_string = b"blob ";
    let size = metadata.len();
    hasher.update(literal_string);
    hasher.update(size.to_string().as_bytes());
    hasher.update(&[0]);
    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read file");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn run(files: Vec<String>) {
    for file in files {
        let sha1 = generate_file_sha1(&file);
        println!("{} {}", sha1, file);
    }
    todo!()
}
