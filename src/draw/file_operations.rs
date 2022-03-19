use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn write_to_file(file_name: &str, file_data: &str) {
    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(file_data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    };
}

pub fn open_new_file(file_name: &str) -> File {
    let path = Path::new(&file_name);

    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path).unwrap()
}

pub fn open_file(file_name: &str) -> File {
    let path = Path::new(file_name.as_str());
    File::open(path).expect("file not found!")
}

pub fn write_line_to_file(mut file: &File, line: &str) {
    if let Err(why) = writeln!(file, "{}", line) {
        panic!("couldn't write to file: {}", why);
    }
}