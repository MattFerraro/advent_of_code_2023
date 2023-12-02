use std::cmp;
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./input.txt").unwrap();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;
    for (idx, line) in file.lines().enumerate() {
        let game_num_and_game: Vec<&str> = line.split(":").collect();
        let game = game_num_and_game[1];

        let mut game_was_good = true;

        let mut red_required = 0;
        let mut green_required = 0;
        let mut blue_required = 0;

        for draw in game.split(";") {
            let mut observed_red = 0;
            let mut observed_green = 0;
            let mut observed_blue = 0;

            for ball_count in draw.split(",") {
                // a string like " 10 red"
                let number_and_color: Vec<&str> = ball_count.split(" ").collect();
                let number: u64 = number_and_color[1].parse().unwrap();
                let color = number_and_color[2];

                match color {
                    "red" => {
                        observed_red = number;
                    }
                    "green" => {
                        observed_green = number;
                    }
                    "blue" => {
                        observed_blue = number;
                    }
                    _ => {
                        println!("what color is this? {}", color);
                    }
                }
            }

            red_required = cmp::max(red_required, observed_red);
            green_required = cmp::max(green_required, observed_green);
            blue_required = cmp::max(blue_required, observed_blue);

            if observed_red > max_red || observed_green > max_green || observed_blue > max_blue {
                game_was_good = false;
            }
        }

        let game_power = red_required * green_required * blue_required;
        sum_part_2 += game_power;

        if game_was_good {
            sum_part_1 += idx + 1;
        }
    }

    println!("Sum part 1: {}", sum_part_1);
    println!("Sum part 2: {}", sum_part_2);
}
