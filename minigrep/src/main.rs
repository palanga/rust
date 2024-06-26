use std::env;
use minigrep::{get_args, read_file, search, search_case_insensitive};

fn main() {
    let (query, file_path) = get_args();
    let contents = read_file(file_path);
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    if ignore_case {
        search_case_insensitive(&query, &contents).iter().for_each(|line| println!("{line}"));
    } else {
        search(&query, &contents).iter().for_each(|line| println!("{line}"));
    }
}
