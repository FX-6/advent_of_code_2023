pub fn part1(input: String) -> u32 {
    let mut res: u32 = 0;

    for line in input.lines() {
        for char in line.chars() {
            if char.is_digit(10) {
                res += char.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_digit(10) {
                res += char.to_digit(10).unwrap();
                break;
            }
        }
    }

    res
}

pub fn part2(input: String) -> u32 {
    let mut res: u32 = 0;
    let numbers = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        'outer: for i in 0..line.len() {
            for (text, number) in &numbers {
                if i + text.len() > line.len() {
                    continue;
                }

                if line[i..(i + text.len())].contains(text) {
                    res += number * 10;
                    break 'outer;
                }
            }
        }

        'outer: for j in 0..line.len() {
            let i = line.len() - j;

            for (text, number) in &numbers {
                if i < text.len() {
                    continue;
                }

                if line[i - text.len()..i].contains(text) {
                    res += number;
                    break 'outer;
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let res = part1(String::from(
            "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n",
        ));

        assert_eq!(res, 142);
    }

    #[test]
    fn test_part2() {
        let res = part2(String::from(
            "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n",
        ));

        assert_eq!(res, 281);
    }
}
