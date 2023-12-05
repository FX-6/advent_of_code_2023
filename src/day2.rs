use regex::Regex;

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

pub fn part1(input: String) -> u32 {
    let mut res: u32 = 0;

    let red_regex = Regex::new(r"(?<amount>\d*) red").unwrap();
    let green_regex = Regex::new(r"(?<amount>\d*) green").unwrap();
    let blue_regex = Regex::new(r"(?<amount>\d*) blue").unwrap();

    for (game_no, game_str) in input.lines().enumerate() {
        if game_str.is_empty() {
            continue;
        }

        let mut game_red_cubes: u32 = 0;
        let mut game_green_cubes: u32 = 0;
        let mut game_blue_cubes: u32 = 0;

        for round in game_str.split(";") {
            if round.contains("red") {
                let round_red_cubes = red_regex
                    .captures(round)
                    .unwrap()
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                if round_red_cubes > game_red_cubes {
                    game_red_cubes = round_red_cubes;
                }
            }

            if round.contains("green") {
                let round_green_cubes = green_regex
                    .captures(round)
                    .unwrap()
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                if round_green_cubes > game_green_cubes {
                    game_green_cubes = round_green_cubes;
                }
            }

            if round.contains("blue") {
                let round_blue_cubes = blue_regex
                    .captures(round)
                    .unwrap()
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                if round_blue_cubes > game_blue_cubes {
                    game_blue_cubes = round_blue_cubes;
                }
            }
        }

        if game_red_cubes <= RED_CUBES
            && game_green_cubes <= GREEN_CUBES
            && game_blue_cubes <= BLUE_CUBES
        {
            res += game_no as u32 + 1;
        }
    }

    res
}

pub fn part2(input: String) -> u32 {
    let mut res: u32 = 0;

    let red_regex = Regex::new(r"(?<amount>\d*) red").unwrap();
    let green_regex = Regex::new(r"(?<amount>\d*) green").unwrap();
    let blue_regex = Regex::new(r"(?<amount>\d*) blue").unwrap();

    for game_str in input.lines() {
        if game_str.is_empty() {
            continue;
        }

        let mut game_red_cubes: u32 = 0;
        let mut game_green_cubes: u32 = 0;
        let mut game_blue_cubes: u32 = 0;

        for round in game_str.split(";") {
            if round.contains("red") {
                let round_red_cubes = red_regex
                    .captures(round)
                    .unwrap()
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                if round_red_cubes > game_red_cubes {
                    game_red_cubes = round_red_cubes;
                }
            }

            if round.contains("green") {
                let round_green_cubes = green_regex
                    .captures(round)
                    .unwrap()
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                if round_green_cubes > game_green_cubes {
                    game_green_cubes = round_green_cubes;
                }
            }

            if round.contains("blue") {
                let round_blue_cubes = blue_regex
                    .captures(round)
                    .unwrap()
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                if round_blue_cubes > game_blue_cubes {
                    game_blue_cubes = round_blue_cubes;
                }
            }
        }

        res += game_red_cubes * game_green_cubes * game_blue_cubes;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let res = part1(String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
        ));

        assert_eq!(res, 8);
    }

    #[test]
    fn test_part2() {
        let res = part2(String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
        ));

        assert_eq!(res, 2286);
    }
}
