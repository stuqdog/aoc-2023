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

    let (time, distance) = input.split_once("\n").unwrap();
    let distance = distance
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let time = time
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let part_two = helper(time, distance);
    println!("part two: {part_two}");
}
