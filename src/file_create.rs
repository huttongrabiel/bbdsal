use crate::boilerplate_gen;
use crate::topic_select::DSATopic;
use std::{fs, path::Path};

pub fn generate_practice_files(
    topics: &Vec<DSATopic>,
) -> Result<(), &'static str> {
    if !Path::exists(Path::new("./.history")) {
        create_history_file();
    } else {
        update_history_file();
    }

    let parent_dir = match create_practice_dir() {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };

    let sub_dir = match create_practice_sub_dir(&parent_dir) {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };

    create_practice_files(&topics, sub_dir)
}

fn create_practice_dir() -> Result<String, &'static str> {
    let path = String::from("./bbdsalPractice");

    // Check if dir already exists, otherwise this will panic which we don't
    // want.
    if Path::exists(Path::new(&path)) {
        return Ok(path);
    }

    match fs::create_dir(&path) {
        Ok(_) => Ok(path),
        Err(_) => return Err("Failed to create dir"),
    }
}

fn create_practice_sub_dir(
    parent_path: &String,
) -> Result<String, &'static str> {
    let number = history_file_number();
    let path = format!("{}/practice{}", parent_path, number);

    if Path::exists(Path::new(&path)) {
        return Ok(path);
    }

    match fs::create_dir(&path) {
        Ok(_) => Ok(path),
        Err(_) => return Err("Could not create practice subdirectory."),
    }
}

fn create_practice_files(
    topics: &Vec<DSATopic>,
    dir_path: String,
) -> Result<(), &'static str> {
    for topic in topics {
        let content = boilerplate_gen::generate_boiler_plate(topic);
        match fs::write(format!("{}/{}.c", dir_path, topic), content) {
            Ok(_) => (),
            Err(_) => {
                return Err("Failed to create practice files for topics.")
            }
        }
    }
    Ok(())
}

fn create_history_file() {
    let contents = format!("WARNING: DO NOT DELETE THIS FILE!!\n{}\n", 0);

    match fs::write("./.history", contents) {
        Ok(_) => (),
        Err(_) => panic!("Failed to create .history file, please run again"),
    }
}

fn history_file_number() -> u32 {
    let number =
        fs::read_to_string("./.history").expect("Unable to read .history file");

    let number: Vec<&str> = number
        .lines()
        .filter(|line| line.parse::<u32>().is_ok())
        .collect();

    let number = number.get(0).unwrap().parse::<u32>().unwrap();

    number
}

fn update_history_file() {
    let number = history_file_number() + 1;
    let contents = format!("WARNING: DO NOT DELETE THIS FILE!!\n{}\n", number);

    match fs::write("./.history", contents) {
        Ok(_) => (),
        Err(_) => panic!("Could not update .history file!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    #[test]
    fn dir_create() {
        let path = match create_practice_dir() {
            Ok(path) => path,
            Err(_) => panic!("Directory not found!!"),
        };

        assert!(Path::exists(Path::new(&path)));
    }

    #[test]
    fn history_file() {
        create_history_file();

        assert!(Path::exists(Path::new("./.history")));
    }
}
