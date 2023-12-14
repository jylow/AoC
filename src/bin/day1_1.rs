use std::fs;

fn file_reader() -> String {
    let contents = fs::read_to_string("day1_1.in")
        .expect("should be able to read the file");

    return contents;
}

fn main() {
    let contents = file_reader();

    let output: u32 = contents
        .lines()
        .map(|line: &str| {
            let mut iter = 
                line.chars().filter_map(|character: char| {
                    character.to_digit(10)
                });
            let first: u32 = 
                iter.next().expect("should be a number");

            match iter.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>() 
            .expect("should be a valid number")
        })
        .sum::<u32>();

    println!("{}", output);
}

