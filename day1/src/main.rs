use std::fs::read_to_string;

// Part 1
fn get_left_number(line: &str) -> i32 {
    let mut left_number: i32 = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            left_number = c.to_digit(10).unwrap() as i32;
            return left_number;
        }
    }
    left_number
}

// Part 1
fn get_right_number(line: &str) -> i32 {
    let mut left_number: i32 = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            left_number = c.to_digit(10).unwrap() as i32;
        }
    }
    left_number
}

// Part 2
fn line_to_number_reversed(line: &str) -> i32 {
    // just reverse the string
    let line = line.chars().rev().collect::<String>();

    for index in 0..line.len() {
        let substr = line.get(index..).unwrap();

        match substring_to_number_reversed(substr) {
            Ok(number) => return number,
            Err(_) => continue,
        }
    }
    return 0;
}

// Part 2
fn line_to_number(line: &str) -> i32 {
    for index in 0..line.len() {
        let substr = line.get(index..).unwrap();

        match substring_to_number(substr) {
            Ok(number) => return number,
            Err(_) => continue,
        }
    }
    return 0;
}

// Part 2
fn substring_to_number_reversed(line: &str) -> Result<i32, &str> {
    if line.len() < 1 {
        return Err("No first letter");
    }
    let mut char_feed = line.chars();
    let c = char_feed.next().unwrap();

    if c.is_digit(10) {
        return Ok(c.to_digit(10).unwrap() as i32);
    }

    /*
    one ->      eno
    nine ->     enin
    three ->    eerht
    five ->     evif
    seven ->    neves
    two ->      owt
    four ->     ruof
    eight ->    thgie
    six ->      xis
     */

    match c {
        'e' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'n' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'o' => return Ok(1),
                        'i' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'n' => return Ok(9),
                                _ => return Err("-ine but not nine"),
                            }
                        }
                        _ => return Err("-ne but not one or nine"),
                    }
                }
                'e' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'r' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'h' => {
                                    let c4 = char_feed.next().unwrap();
                                    match c4 {
                                        't' => return Ok(3),
                                        _ => return Err("-hree but not three"),
                                    }
                                }
                                _ => return Err("-ree but not three"),
                            }
                        }
                        _ => return Err("-ee but not three"),
                    }
                }
                'v' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'i' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'f' => return Ok(5),
                                _ => return Err("-ive but not five"),
                            }
                        }
                        _ => return Err("-ve but not five"),
                    }
                }
                _ => return Err("e but not one or nine or three"),
            }
        }
        'n' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'e' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'v' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'e' => {
                                    let c4 = char_feed.next().unwrap();
                                    match c4 {
                                        's' => return Ok(7),
                                        _ => return Err("-even but not seven"),
                                    }
                                }
                                _ => return Err("-ven but not seven"),
                            }
                        }
                        _ => return Err("-en but not seven"),
                    }
                }
                _ => return Err("-n but not seven"),
            }
        }
        'o' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'w' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        't' => return Ok(2),
                        _ => return Err("-wo but not two"),
                    }
                }
                _ => return Err("-o but not two"),
            }
        }
        'r' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'u' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'o' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'f' => return Ok(4),
                                _ => return Err("-our but not four"),
                            }
                        }
                        _ => return Err("-ur but not four"),
                    }
                }
                _ => return Err("-r but not four"),
            }
        }
        't' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'h' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'g' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'i' => {
                                    let c4 = char_feed.next().unwrap();
                                    match c4 {
                                        'e' => return Ok(8),
                                        _ => return Err("-ight but not eight"),
                                    }
                                }
                                _ => return Err("-ght but not eight"),
                            }
                        }
                        _ => return Err("-ht but not eight"),
                    }
                }
                _ => return Err("-t but not eight"),
            }
        }
        'x' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'i' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        's' => return Ok(6),
                        _ => return Err("-ix but not six"),
                    }
                }
                _ => return Err("-x but not six"),
            }
        }
        _ => return Err("No first letter"),
    }
}

// Part 2
fn substring_to_number(line: &str) -> Result<i32, &str> {
    if line.len() < 1 {
        return Err("No first letter");
    }
    let mut char_feed = line.chars();
    let c = char_feed.next().unwrap();

    if c.is_digit(10) {
        return Ok(c.to_digit(10).unwrap() as i32);
    }

    match c {
        'o' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'n' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'e' => return Ok(1),
                        _ => return Err("on but not one"),
                    }
                }
                _ => return Err("o but not one"),
            }
        }
        't' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'w' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'o' => return Ok(2),
                        _ => return Err("tw but not two"),
                    }
                }
                'h' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'r' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'e' => {
                                    let c4 = char_feed.next().unwrap();
                                    match c4 {
                                        'e' => return Ok(3),
                                        _ => return Err("thre but not three"),
                                    }
                                }
                                _ => return Err("thr but not three"),
                            }
                        }
                        _ => return Err("th but not three"),
                    }
                }
                _ => return Err("t but not two or three"),
            }
        }
        'f' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'o' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'u' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'r' => return Ok(4),
                                _ => return Err("fou but not four"),
                            }
                        }
                        _ => return Err("fo but not four"),
                    }
                }
                'i' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'v' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'e' => return Ok(5),
                                _ => return Err("fiv but not five"),
                            }
                        }
                        _ => return Err("fi but not five"),
                    }
                }
                _ => return Err("f but not four or five"),
            }
        }
        's' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'i' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'x' => return Ok(6),
                        _ => return Err("si but not six"),
                    }
                }
                'e' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'v' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'e' => {
                                    let c4 = char_feed.next().unwrap();
                                    match c4 {
                                        'n' => return Ok(7),
                                        _ => return Err("seve but not seven"),
                                    }
                                }
                                _ => return Err("sev but not seven"),
                            }
                        }
                        _ => return Err("se but not seven"),
                    }
                }
                _ => return Err("s but not six or seven"),
            }
        }
        'e' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'i' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'g' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'h' => {
                                    let c4 = char_feed.next().unwrap();
                                    match c4 {
                                        't' => return Ok(8),
                                        _ => return Err("eigh but not eight"),
                                    }
                                }
                                _ => return Err("eig but not eight"),
                            }
                        }
                        _ => return Err("ei but not eight"),
                    }
                }
                _ => return Err("e but not eight"),
            }
        }
        'n' => {
            let c1 = char_feed.next().unwrap();
            match c1 {
                'i' => {
                    let c2 = char_feed.next().unwrap();
                    match c2 {
                        'n' => {
                            let c3 = char_feed.next().unwrap();
                            match c3 {
                                'e' => return Ok(9),
                                _ => return Err("nin but not nine"),
                            }
                        }
                        _ => return Err("ni but not nine"),
                    }
                }
                _ => return Err("n but not nine"),
            }
        }
        _ => return Err("No first letter"),
    }
}

fn main() {
    let file = read_to_string("./input.txt").unwrap();

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;
    for line in file.lines() {
        let pt_1_left_number = get_left_number(line);
        let pt_1_right_number = get_right_number(line);
        let pt_1_combined = pt_1_left_number * 10 + pt_1_right_number;
        sum_part_1 += pt_1_combined;

        let pt_2_left_number = line_to_number(line);
        let pt_2_right_number = line_to_number_reversed(line);
        let pt_2_combined = pt_2_left_number * 10 + pt_2_right_number;
        sum_part_2 += pt_2_combined;
    }

    println!("Part 1: {}", sum_part_1);
    println!("Part 2: {}", sum_part_2);
}
