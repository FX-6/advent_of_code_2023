pub fn part1(input: String) -> u32 {
    let mut res: u32 = 0;
    let mut current_number = String::from("");
    let mut add_current_number = false;

    for (line_no, line) in input.lines().enumerate() {
        for (char_no, char) in line.chars().enumerate() {
            if !char.is_digit(10) {
                if add_current_number {
                    res += current_number.parse::<u32>().unwrap();
                }
                current_number = "".to_string();
                add_current_number = false;
                continue;
            }

            current_number.push(char);

            // Top Left
            if line_no > 0 && char_no > 0 {
                let char_to_check = input
                    .lines()
                    .nth(line_no - 1)
                    .unwrap()
                    .chars()
                    .nth(char_no - 1)
                    .unwrap();
                if char_to_check != '.' && !char_to_check.is_digit(10) {
                    add_current_number = true;
                }
            }

            // Top Center
            if line_no > 0 {
                let char_to_check = input
                    .lines()
                    .nth(line_no - 1)
                    .unwrap()
                    .chars()
                    .nth(char_no)
                    .unwrap();
                if char_to_check != '.' && !char_to_check.is_digit(10) {
                    add_current_number = true;
                }
            }

            // Top Right
            if line_no > 0 && char_no < line.len() - 1 {
                let char_to_check = input
                    .lines()
                    .nth(line_no - 1)
                    .unwrap()
                    .chars()
                    .nth(char_no + 1)
                    .unwrap();
                if char_to_check != '.' && !char_to_check.is_digit(10) {
                    add_current_number = true;
                }
            }

            // Center Left
            if char_no > 0 {
                let char_to_check = line.chars().nth(char_no - 1).unwrap();
                if char_to_check != '.' && !char_to_check.is_digit(10) {
                    add_current_number = true;
                }
            }

            // Center Right
            if char_no < line.len() - 1 {
                let char_to_check = line.chars().nth(char_no + 1).unwrap();
                if char_to_check != '.' && !char_to_check.is_digit(10) {
                    add_current_number = true;
                }
            }

            // Bottom Left
            if line_no < input.lines().count() - 1 && char_no > 0 {
                let char_to_check = input
                    .lines()
                    .nth(line_no + 1)
                    .unwrap()
                    .chars()
                    .nth(char_no - 1)
                    .unwrap();
                if char_to_check != '.' && !char_to_check.is_digit(10) {
                    add_current_number = true;
                }
            }

            // Bottom Center
            if line_no < input.lines().count() - 1 {
                let char_to_check = input
                    .lines()
                    .nth(line_no + 1)
                    .unwrap()
                    .chars()
                    .nth(char_no)
                    .unwrap();
                if char_to_check != '.' && !char_to_check.is_digit(10) {
                    add_current_number = true;
                }
            }

            // Bottom Right
            if line_no < input.lines().count() - 1 && char_no < line.len() - 1 {
                let char_to_check = input
                    .lines()
                    .nth(line_no + 1)
                    .unwrap()
                    .chars()
                    .nth(char_no + 1)
                    .unwrap();
                if char_to_check != '.' && !char_to_check.is_digit(10) {
                    add_current_number = true;
                }
            }

            // Clear (+ add) number if it ends at the lineend
            if char_no == line.len() - 1 {
                if add_current_number {
                    res += current_number.parse::<u32>().unwrap();
                }
                current_number = "".to_string();
                add_current_number = false;
            }
        }
    }

    res
}

pub fn part2(_input: String) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let res = part1(String::from(
            "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n",
        ));

        assert_eq!(res, 4361);
    }
}
