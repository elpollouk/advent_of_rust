use std::collections::HashSet;

fn problem1 (frequencies: &[i32]) -> i32 {
    let mut current_frequency = 0;

    for f in frequencies.iter() {
        current_frequency += f;
    }

    current_frequency
}

fn problem2 (frequencies: &[i32]) -> i32 {
    let mut current_frequency = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    let mut i = 0;

    while !seen.contains(&current_frequency) {
        seen.insert(current_frequency);
        current_frequency += frequencies[i % frequencies.len()];
        i += 1;
    }

    current_frequency
}

pub fn solve() {
    println!("Day 01");
    println!("  Test 1 = {}", problem1(&[1, -2, 3, 1]));
    println!("  Test 2 = {}", problem2(&[7, 7, -2, -7, -4]));
}