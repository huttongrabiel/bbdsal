mod lib;
mod topic_select;
mod boilerplate_gen;

use lib::Config;

pub fn main() {

}

fn run() {
    let config = Config::new(7);
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
