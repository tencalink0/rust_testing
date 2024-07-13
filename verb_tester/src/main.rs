use std::fs;
//use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = "files/verbs1.txt";

    let verbs = read_verbs_from_file(path);
    //let verbs = read_verbs_from_file(path).unwrap_or_else();
    //println!("{:?}", verbs);
    println!("{}", verbs);
}

fn read_verbs_from_file<P>(filename: P) -> Vec<&str>
where 
    P: AsRef<Path>, 
{
    let file = fs::read_to_string(filename);
    let file: String = match file {
        Ok(text) => text,
        Err(error) => { 
            println!("{}", error);
            String::new()
        }
    };

    let lines: Vec<&str> = &file.lines().collect();

    for line in lines {
        println!("{}", line)
    }

    lines
}
