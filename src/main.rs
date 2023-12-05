use clap::Parser;

mod day1;
mod day2;
mod day3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The day of the challenge (1 to 25)
    #[arg(short, long)]
    day: u8,

    /// The part of the challenge (1 or 2)
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();

    match (args.day, args.part) {
        (1, 1) => println!("Answer: {}", day1::part1(get_input(1))),
        (1, 2) => println!("Answer: {}", day1::part2(get_input(1))),
        (2, 1) => println!("Answer: {}", day2::part1(get_input(2))),
        (2, 2) => println!("Answer: {}", day2::part2(get_input(2))),
        (3, 1) => println!("Answer: {}", day3::part1(get_input(3))),
        (3, 2) => println!("Answer: {}", day3::part2(get_input(3))),
        (_, _) => println!("Not implemented yet"),
    }
}

fn get_input(day: u8) -> String {
    let file_path = format!("./inputs/day{}.txt", day);
    let file_contents =
        std::fs::read_to_string(&file_path).expect(&format!("Couldn't find file {}", file_path));

    file_contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let input = get_input(0);
        assert_eq!(input, "Hello World\n");
    }
}
