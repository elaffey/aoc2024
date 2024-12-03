use core::{iter::Iterator, num};

#[allow(dead_code)]
pub fn part1() {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();

    let mut results = Vec::new();
    let mut i = 0;
    while let Some(start) = input[i..].find("mul(") {
        i += start + 4;
        let mut end_index = None;

        for (j, c) in input[i..].chars().enumerate() {
            if c == ')' {
                end_index = Some(i + j);
                break;
            }
        }

        if let Some(end) = end_index {
            let slice = &input[i..end];
            if let Some((first, second)) = slice.split_once(',') {
                results.push((first.to_string(), second.to_string()));
            }
        }
    }

    let mut ans = 0;
    for (first, second) in results {
        if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
            ans += num1 * num2;
        };
    }
    dbg!(ans);
}

#[allow(dead_code)]
pub fn part2() {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();

    let mut i = 0;
    let mut ans = 0;
    let mut enabled = true;
    let mut prev = 0;

    while let Some(start) = input[i..].find("mul(") {
        i += start + 4;
        let mut end_index = None;

        for (j, c) in input[i..].chars().enumerate() {
            if c == ')' {
                end_index = Some(i + j);
                break;
            }
        }

        if let Some(end) = end_index {
            let slice = &input[i..end];
            if let Some((first, second)) = slice.split_once(',') {
                if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                    let sub = &input[..i];
                    let last_do = sub.rfind("do()");
                    let last_dont = sub.rfind("don't()");

                    if let (Some(last_do), Some(last_dont)) = (last_do, last_dont) {
                        if last_do > last_dont {
                            enabled = true;
                        } else {
                            enabled = false;
                        }
                    } else if last_do.is_some() {
                        enabled = true;
                    } else if last_dont.is_some() {
                        enabled = false
                    }
                    // dbg!(num1, num2, last_do, last_dont, enabled);
                    if enabled {
                        ans += num1 * num2;
                    }
                };
            }
        }
        prev = i;
    }
    dbg!(ans);
}
