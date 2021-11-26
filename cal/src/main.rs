use std::fs;

fn main() {
    println!("File being read: {}", "../cal_files/main.cal");

    let contents = fs::read_to_string("../cal_files/main.cal")
        .expect("An error occured while trying to read file");

    let lines: Vec<&str> = contents.split('\n').collect();

    for line in lines {
        println!("{}", line);
    }
}
