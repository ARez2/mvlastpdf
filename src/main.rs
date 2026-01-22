use std::{fs::DirEntry, path::PathBuf};

use users::get_current_username;

fn main() {
    #[cfg(target_os = "linux")]
    let downloads = PathBuf::from(format!(
        "/home/{}/Downloads/",
        get_current_username()
            .expect("Could not get username")
            .to_str()
            .unwrap()
    ));
    #[cfg(target_os = "windows")]
    let downloads = PathBuf::from(format!(
        "C:\\Users\\{}\\Downloads/",
        get_current_username()
            .expect("Could not get username")
            .to_str()
            .unwrap()
    ));

    let args: Vec<String> = std::env::args().collect();
    let mut target = PathBuf::from(args[args.len() - 1].clone());
    let target_is_file = target.is_file();

    let mut entries = std::fs::read_dir(downloads)
        .unwrap()
        .map(|f| f.unwrap())
        .collect::<Vec<DirEntry>>();
    entries.sort_by_key(|a| a.metadata().unwrap().modified().unwrap());
    entries.reverse();

    let mut moved_something = false;
    for file in &entries {
        if let Some(ext) = file.path().extension() {
            if ext == "pdf" {
                println!("Moving {:?} to {:?}", file.path(), target);
                if !target.exists() {
                    std::fs::create_dir_all(&target).unwrap();
                }
                if !target_is_file {
                    target = target.join(file.file_name());
                }
                std::fs::rename(file.path(), target).unwrap();
                moved_something = true;
                break;
            }
        }
    }

    if !moved_something {
        println!("Did not find any PDF in ~/Downloads/");
    }
}
