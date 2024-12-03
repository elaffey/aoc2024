use core::iter::Iterator;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    std::fs::read_to_string("samples/day1.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            // let a: &str = line;
            let v: Vec<&str> = line.split_whitespace().collect();
            let a: i32 = v[0].parse().unwrap();
            let b: i32 = v[1].parse().unwrap();
            left.push(a);
            right.push(b);
            // dbg!(a, b);
        });

    left.sort();
    right.sort();

    let mut total = 0;
    for (i, l) in left.iter().enumerate() {
        let r = right[i];
        let dist = (l - r).abs();
        total += dist;
    }
    dbg!(total);
    // dbg!(left);
}

#[allow(dead_code)]
pub fn part2() {
    let mut left: HashMap<i32, i32> = HashMap::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    let mut left1: Vec<i32> = Vec::new();
    let mut right1: Vec<i32> = Vec::new();

    std::fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            let a: i32 = v[0].parse().unwrap();
            let b: i32 = v[1].parse().unwrap();
            *left.entry(a).or_insert(0) += 1;
            *right.entry(b).or_insert(0) += 1;
            left1.push(a);
            right1.push(b);
            // dbg!(a, b)
        });
    let mut ans = 0;
    for l in left1 {
        let tmp: i32 = *right.get(&l).unwrap_or(&0);
        ans += l * tmp;
    }
    dbg!(ans);
}
