use core::iter::Iterator;

#[allow(dead_code)]
pub fn part1() {
    let mut count = 0;
    std::fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let tmp: Vec<i32> = line
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect();
            let increasing = tmp[1] > tmp[0];
            let mut safe = true;
            for i in 1..tmp.len() {
                if increasing {
                    let diff = tmp[i] - tmp[i - 1];
                    if diff <= 0 || diff > 3 {
                        safe = false;
                    }
                } else {
                    let diff = tmp[i - 1] - tmp[i];
                    if diff <= 0 || diff > 3 {
                        safe = false;
                    }
                }
            }
            if safe {
                count += 1;
            }
            // dbg!(tmp, safe);
            // let nums =
        });
    dbg!(count);
}

fn check(nums: &Vec<i32>, increasing: bool) -> (bool, usize, usize) {
    let mut s = true;
    let mut i1 = 0;
    let mut i2 = 0;

    for i in 1..nums.len() {
        if increasing {
            let diff = nums[i] - nums[i - 1];
            if diff <= 0 || diff > 3 {
                s = false;
                i1 = i;
                i2 = i - 1;
            }
        } else {
            let diff = nums[i - 1] - nums[i];
            if diff <= 0 || diff > 3 {
                s = false;
                i1 = i;
                i2 = i - 1;
            }
        }
    }

    (s, i1, i2)
}

#[allow(dead_code)]
pub fn part2() {
    let mut count = 0;
    std::fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect();
            let increasing = nums.last().unwrap() > nums.first().unwrap();
            let mut extra_safe = true;
            let (s1, i1, i2) = check(&nums, increasing);
            if !s1 {
                let mut nums_sans_i1 = nums.clone();
                nums_sans_i1.remove(i1);
                let (s1, _, _) = check(&nums_sans_i1, increasing);
                if !s1 {
                    let mut nums_sans_i2 = nums.clone();
                    nums_sans_i2.remove(i2);
                    let (s2, _, _) = check(&nums_sans_i2, increasing);
                    if !s2 {
                        extra_safe = false;
                    }
                }
            }
            if extra_safe {
                count += 1;
            }
        });
    dbg!(count);
}
