mod boilerplate_gen;
mod file_create;
mod lib;
mod topic_select;

use file_create::generate_practice_files;
use lib::Config;
use std::error::Error;
use std::io;
use topic_select::TOPIC_COUNT;

pub fn main() -> Result<(), Box<dyn Error>> {
    run();
    Ok(())
}

fn run() {
    let user_input = get_user_input();

    let config = Config::new(user_input);

    let topics = topic_select::generate_study_topics(&config);

    match generate_practice_files(&topics.dsa_selection) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };
}

fn get_user_input() -> u32 {
    let mut user_input = String::new();

    loop {
        println!(
            "How many data structures/algorithms today? (1-{}): ",
            TOPIC_COUNT
        );

        io::stdin()
            .read_line(&mut user_input)
            .expect("Dawg, wtf did you enter?");

        match user_input.trim().parse::<u32>() {
            Ok(num) => {
                if num < 1 || num > TOPIC_COUNT {
                    // Have to clear because read_line appends to buffer
                    user_input.clear();
                    continue;
                }
                return num;
            }
            Err(_) => {
                user_input.clear();
                continue;
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let config = Config::new(8);

        let dsa_vec = topic_select::generate_study_topics(&config);

        assert_eq!(dsa_vec.dsa_selection.len(), 8);
    }
}
