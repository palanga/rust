use std::{env, fs};

pub fn get_args() -> (String, String) {
    let mut args = env::args();

    args.next(); // discard program name
    let query = args.next().expect("No search string provided");
    let file_path = args.next().expect("No file path provided");

    (query, file_path)
}

pub fn read_file(file_path: String) -> String {
    fs::read_to_string(file_path).expect("Couldnt read file")
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&lowercase_query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
