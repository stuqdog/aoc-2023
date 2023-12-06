// binary search would be faster but I'm lazy!
fn helper(time: i64, distance: i64) -> i64 {
    let mut time_ = time.clone();
    let mut time_held = 0;
    while time_held * time_ < distance {
        time_held += 1;
        time_ -= 1;
    }

    time - (time_held * 2) + 1
}

pub fn main() {
    let input = "Time:        46     85     75     82
Distance:   208   1412   1257   1410";

    let (times, distances) = input.split_once("\n").unwrap();
    let times: Vec<i64> = times
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let distances: Vec<i64> = distances
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut part_one = 1;
    for (idx, time) in times.into_iter().enumerate() {
        let forgiveness_range = helper(time, distances[idx]);
        part_one *= forgiveness_range;
    }

    println!("part one: {part_one}");

    let (time, dist) = input.split_once("\n").unwrap();
    let dist = dist.chars().fold(0 as i64, |acc, c| match c.to_digit(10) {
        Some(i) => acc * 10 + i as i64,
        None => acc,
    });
    let time = time.chars().fold(0 as i64, |acc, c| match c.to_digit(10) {
        Some(i) => acc * 10 + i as i64,
        None => acc,
    });

    let part_two = helper(time, dist);
    println!("part two: {part_two}");
}
