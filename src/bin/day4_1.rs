use std::fs;

fn read_file() -> String {
    let content = fs::read_to_string("day4.in")
        .expect("file should be readable");

    return content;
}

fn main() {
    let content = read_file();

    let output: u32 = content.trim()
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
        })
        .map(|line| {
            let mut count = 0;
            let (winners, nums) = line.split_once("|").unwrap();
            let winners = winners.trim().split_whitespace().collect::<Vec<&str>>();

            for num in nums.trim().split_whitespace() {
                if winners.contains(&num){
                    count += 1;
                }
            }
            if count == 0 {
                return count
            } else {
                return u32::pow(2,count - 1);
            }
        })
        .sum();

        println!("{output}");
}



                                 
