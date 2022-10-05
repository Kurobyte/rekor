use chrono::{Datelike, Local};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

// extern crate dirs;

pub fn get_config_path() -> PathBuf {
    let some_dir: Option<PathBuf> = dirs::config_dir();
    match some_dir {
        Some(path) => {
            let today = Local::now();
            let app_folder = Path::new(&path).join("rekor");
            let file = app_folder.join(format!("track-{0}-{1}.csv", today.year(), today.month()));
            let create_res = fs::create_dir_all(app_folder);
            // dbg!(create_res);
            if create_res.is_err() {
                panic!("Can't create program config dir.")
            }

            file
        }
        None => panic!("Can't find config dir."),
    }
}

pub fn write_raw(text: String) {
    let f_path = get_config_path();
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(f_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
