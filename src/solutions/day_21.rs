use std::collections::{HashMap, HashSet};

pub fn main() {
    let input = "...................................................................................................................................
.##.....#........#........#...........#......#....##.....##....................#...#.......#.....................#......#.#........
...........................#.......#...#.#...#....#...........................#................##.......#....................#.#...
..............#..............##...#.....#.#...#..#.......#...................................#..#......#.......#....#...........##.
..........##...#..#......#...#.#..........................................#....................#.....................#.....#.......
..#............##.....................##.........#..............................#.............##.........##..................#.....
..............#.#..............................#...............................#....#...#..#.......................#...##..........
.....................#...#....#.......#.............#.#...........##...................#.#...........##......#.#......#.....#.#....
.....#.......#..........#...##.#.....#.......#.#.....#.......................#....#..#....................#.#.#....#.......#.......
..#............#.#..........#.....#.....#.......#..............................................#.................#.................
......#........##..........#........#............##......................................................#.........#...............
......#...#.....#....##.................#.......................#..#..............................#.......#..............#.....#...
.......#..#.......#..........#.............................#.##.........................................#...#.........#............
....................#.............#.#..............................#......#...........##...##..............#..#.....#..............
........#.........#...................#................##..#...#...#.......#.....................#...#........#.#.......#..........
.......................#..#.....##...........................#.#.....................#............#.#.............#.#...#..........
..#....#....#...............#........#.........................#......#...........................#.##..#.###..#....##.......#.....
...........#..##..............#...##............................#..#.....................#............#............................
...##.......#........#.#.....#.........#.#.........#..............##....................#.##.....#............#.................#..
...#......#....#............#............#...............#.#......#.............#.............#........#.#.##.........#....#....#..
.#...###.#....#..................#...#.................#......#........#...#.....#.......#...................#..........##.........
...#..........#........#.......#.................#......#...............#...................#..................#...#....#..........
...........#....#..................#...#.......#..........#....#......#......#...#...........#..........................#..#.......
....###...............#......#........#...............#...##...#...............##..............#....#..........#..#....#...........
.............##......#.#............#........#....................#.#.........#..............#................#.#..............#...
.###....#...............#.........#.#...............................#...#......###....................#.##.........#...............
......#..............#.........#.............#..#..#.......####.....................................#..#...........................
....#...#........#..........................#........#....#.#..#.............#...#...................#.............................
......#...#.................................#.....#...........#..................................................#....#.#..........
..........#.....#.................................##..#........#..................##.......................#.......................
..#..#.##.#.#..............#..#.................#..................#........#.........#..............#....................#.##.....
.........#.......#.....#.#...............#...#....#..#.............#...#...............................#...#.##...#.....#......#...
..#..#..........#.......................#............#...#............#............#..#...............#............................
.#..................##.#..............#..................#.....#..............#....#...#..##............................#...#...#..
.#.....#..#.........................#....#........#........#........#..##........#..........#.#.........##..#..........#........#..
...................#...##................#..........#.......#...............##....................................#...#............
................#...#.................#.....#..............#...............#......#...#....##...................#.....#.....#......
....................##............................#..##..##..##...........#.......#.#.#..#....#...................#................
..#........#.#..#....#..............#............................................##.................................#..............
..........#.......#...............................#.......#..#....#...#.......###.......#..........#.........#........#........#...
........................................#.............#.##........................#...............#..............#.................
...#...........................#.................#.......#..#.....#...#.........................#...#..........#.............#.....
..................................#...#..............#......#..##..#..#........#..#.#..........##....##..................#....#....
..#..........#....................#......#....#.........##....#..........#......##...#.....#........##...................#.........
..........#..............#.............................#...............................#.......................................#...
.............#..........#.......#...#.#..#..#..#...#.......#..#..........#..#......................................................
...#.....#.................##..##....#.#..#...................#......#......#...........#..#....#...................#.......###....
........#....................#....#...........#.............#...............#...........#...#.........................##..#.#......
.....................#...........#..#..#....#..............#..........#..#.........#..##..........#............................#...
...#......#.........................#.......#.......#...#.........#........................#.#.........#......#............#.......
....................#.#..#..................................................#......#............#.##........##..............#......
.......#...........#...#.#..#....#.#.......................##.................................#................................#...
.......#............#...#......#..#.#................#................................#...##....#..#...........#.............#.....
.........................#....................#.#..##..#..............#...#....#...##..............................................
.........................##..............#........#.#...........#........#.............#...........................#..........#....
.#....#..........................#......#..#...#...........#.......#..........................................#....................
....................#........#....#..#...........#.....................##.##.....#...#..................#..........#...............
.....................#....................#.#..................##.....#............#.......##..#...........#.......................
.....................#..#........#...#.#..................#....#.....................................#..........##....#............
...........................#.#....#...............#...............#........#........#...#..........#..........#....................
.................###......#.......#....................#.............#............#....................#.....#.......##............
.........##......#.....................#.....#..........#.....................#......#..........#.....................#............
..................#..#........##..###.....................#..........#......#...#......#.#......#......#.............#....#........
.........#....#.#.............#....#........#.....#..........#...........................#.......#....#...#........................
...........#.................................................#....................#.......#...........#...#.......#................
.................................................................S.................................................................
..................#...............#..........#..........####.............##......#.#...#.......###........#..................#.....
.......................#...#..##...................##..........................#...##..........##........#...##....................
............#................#.............................#................#......#..........##...................................
.............#....#.....##...............#....#..##...#.....#............................##..............#.........................
.............###.....#..........#.....................................#....#.#.........#................................#..........
..............#..........................................#........##...#.#.#...............#..........#............##............#.
............#....#............#.........#...........#.............#............................#..##..#............................
...#...................#.........#...#............#..#........................#.#.#..#..#.............#.....#......................
.................#........................#.........#..........#..#............#.#.....#...........#....#....#.................#...
.....#.................#....#........#.#......#............##.........#.#.........##...........#..#............#..#..............#.
.....................................................#..................................#..............#..#.......#..............#.
.......................................#...........#.........................................#...........#.....#..............#....
...#.#..#..........#...#.##.#..........................................#...............#.#.......#.......................#...#..#..
....##.##.#.........#...#......#.#.#..........................................#.........#......##...#......#................#......
.......................#...#......#....#...#...#.......#......................#...........................#.#......................
.......#..#.................#...#...#...........#........#.........#......#.#.#...##.............#....................#..#.........
........................................................#.............................#..........#...#.................#...........
............###......................#.................................#..................................##...............#.......
..........................................#.#........................#......#.#..#..................#..#......................#.#..
.........##........................#...#....#.....#.#......#...................#.............#.........#............#.#.#..#.......
......#......................##...........#....#.##.....#......#..#...#..#.........#..#...#....#.......................##....#.....
..............................#......#........#...#............#.........#.##..........................#.........#..........#.#....
...##...#....................#.....#................................##.....#.......................#...........#..#....#...........
.......#....#...#...................#.#...........#......#..........#......#...........#.............#....................#......#.
.................................#......#.#...............#.......#..........#.#..#.#.............#..........###....#..#...........
..............#..#....#..............................................#......#.........#...........................#................
.#..........#....#....#..........#..#.........................#...#.....#.....................##...#...............#.....#.........
.......#.......#.#.......................#...............#.........#.......................##.....................#......#...#.....
.....................................#....#.........#.....#...................##............#...............................#.#....
........#...#...#.....................................##...............#.......#...#.......#...............#..#.........#.....#....
....#............#........#............................#......#.....#.#..#..............................#...#..#...#...............
.....#..................#..............#............#.............#.#.#.#......#......#................#.....#.....................
.......#...........#.........................#...............##....#..##.......#........#.................#..............#...#.....
..#..#.......#.#......#......#...........#.#..................#...#..##..#...........#........................###..........#.......
........#......................#........#........##.#......................##..#...#............................#...........#......
...............#.....#........#.#...................#.......#..............#............#........................#.....#..#..#.....
........##...#....#....#.......................................#.........#..##......................##..#......#...................
..#..........#.....#....#.#..#.....................................#...#.#..#..#.#.#..#............#.............##................
..................#......#...................##........#.......##...#.............#............................##.........#........
......................#........#.#.#...........#.......#.#.....................................#.............#....##.......#....#..
.............#.....#...........#..#.............##...#..#.............................................#...........#.#...#.....#....
..........................#..#..................#.......#..#.........#......#..#..................#....#..........#.#...#..........
.....#.........##.....#.......#.......#.........#..#.##...#...................#..........................#......#.#................
....#.#.................#...........#.......................................#.....#.........#.............#..................#.....
....#.....#.................#.#....#...#...............#.....#.............#.............#..................#.......#.....#....#...
.....#..........................#..#.........................#........#....#..#.................##...............................#.
.......#..................................................#...............................#......#.............#....#..............
..#...#......##......................#....................#.........#........................#.......#....#.......#...#....#.......
........#..........#....................#....#.............##...........##...........#..........................................#..
.###................#..........#......................#.......#.......#.............#..............................#......#....#...
...#......#...#.#........#.....#.........##.........................#.#....................#......................#.#...........##.
..........#.........................#....#.....##............#.....................#...............................................
..#...........##...............................#.#.................#..............#.............#..#................#..#...........
......#............#..#...........#.............#.........#....#..#.............#....#............#...........#.....#..............
...##..#..........#...#..................#.....#...............#.....#.................................###..............#..........
..............#.#.................#...........#................#....#...........#.........#.........#...#..............#...........
................#.#...#.......#.........#.#..#...............................#.#....................#......#.......#.....#.........
....................#...#....#...#............#....#............................#.#.#......##.#...##......#.#.#.....#..........#...
.........#..............#..#......#............#.................................#..........#......#.........#...........#..#..##..
...#.#....#.....##..................#.##..##....#.......#.......#.............................................................#....
.....#..#.......#...................#.........................................#...#.................#......................#....#..
............................................................................#......................................................
......................#.....#......#...#..........#........#..............................#.#..#..........#........##..............
..................##...#......#.#.#...#..##.........#......#.................##.....#...#.......#...#....#..#....#......##.........
..................................................................................................................................."
.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let input_len = input.len() as i64;

    let num_steps = 26501365;
    let num_grids = num_steps / input_len;

    let mut start_pos = (0, 0, 0, 0);
    'outer: for (r, row) in input.iter().enumerate() {
        for (col, c) in row.iter().enumerate() {
            if c == &'S' {
                start_pos = (r as i32, col as i32, 0, 0);
                break 'outer;
            }
        }
    }

    // for part two:
    // key insight: our starting position (65, 65) is at the direct center of the grid.
    // key insight: there are only garden plots for ever so long as you go straight in one
    // direction from the start forever.
    // key insight: the number of steps takes you exactly to the edge of a grid iteration
    // provided you go in one direction forever.
    // From these two, we can extrapolate out that we basically have an diamond with squiggly
    // edges but that aligns perfectly within a specific number of grid overlays (goes all the way
    // to the edge and doesn't go over it).
    // From this, we can basically construct a mini version of the grid that shows us what
    // each _type_ of grid will look like based on distance from center grid, and then multiply
    // that out to reach the final value.
    //
    // Note that this means the algorithm is not generic! it likely doesn't give us the correct
    // answer for the example, e.g., because the two insights above don't hold. but given theG
    // actual input received, this is correct.
    //
    for part_one in [true, false].iter() {
        let mut cur_positions = HashSet::new();
        cur_positions.insert(start_pos);
        let mut visited_even: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        let mut visited_odd: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        let steps = if *part_one {
            64
        } else {
            input_len / 2 + input_len * 2
        };
        for i in 0..steps {
            let mut new_poses = HashSet::new();
            for pos in cur_positions {
                if i % 2 == 0 {
                    if visited_even.contains(&pos) {
                        continue;
                    } else {
                        visited_even.insert(pos);
                    }
                } else {
                    if visited_odd.contains(&pos) {
                        continue;
                    } else {
                        visited_odd.insert(pos);
                    }
                }
                for dir in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                    let mut new_pos = (pos.0 + dir.0, pos.1 + dir.1, pos.2, pos.3);
                    if new_pos.1 < 0 {
                        new_pos.1 += input[0].len() as i32;
                        new_pos.3 -= 1;
                    }
                    if new_pos.1 == input[0].len() as i32 {
                        new_pos.1 = 0;
                        new_pos.3 += 1;
                    }
                    if new_pos.0 < 0 {
                        new_pos.0 += input.len() as i32;
                        new_pos.2 -= 1;
                    }
                    if new_pos.0 == input.len() as i32 {
                        new_pos.0 = 0;
                        new_pos.2 += 1;
                    }
                    if *part_one && (new_pos.2 != 0 || new_pos.3 != 0) {
                        continue;
                    }
                    let ch = input[new_pos.0 as usize][new_pos.1 as usize];
                    if ch != '#' {
                        new_poses.insert(new_pos);
                    }
                }
            }
            cur_positions = new_poses;
        }
        let solution;
        if *part_one {
            visited_even.extend(cur_positions.iter());
            solution = visited_even.len() as i64;
        } else {
            visited_odd.extend(cur_positions.iter());
            let part_two_map = visited_odd.iter().fold(HashMap::new(), |mut acc, v| {
                acc.entry((v.2, v.3))
                    .and_modify(|i| *i += 1)
                    .or_insert(1i64);
                acc
            });
            println!("{part_two_map:?}");
            let num_values = part_two_map.iter().fold(HashMap::new(), |mut acc, (_, v)| {
                acc.entry(v).and_modify(|i| *i += 1).or_insert(1);
                acc
            });
            println!("{num_values:?}");
            // why these grids? check the outputs in comments below!
            let grid_count_occurrences = [(1, -2), (-1, 2), (-1, -2), (1, 2)]
                .iter()
                .fold(0, |acc, i| acc + part_two_map.get(i).unwrap_or(&0));
            let single_occurrences = [(0, 2), (2, 0), (0, -2), (-2, 0)]
                .iter()
                .fold(0, |acc, i| acc + part_two_map.get(i).unwrap_or(&0));

            let grid_count_minus_one_occurrences = [(-1, 1), (1, -1), (1, 1), (-1, -1)]
                .iter()
                .fold(0, |acc, i| acc + part_two_map.get(i).unwrap_or(&0));

            let minus_one_squared_occurrences = part_two_map.get(&(0, 0)).unwrap_or(&0);

            let grid_count_squared_occurrences = part_two_map.get(&(0, 1)).unwrap_or(&0);
            solution = num_grids * grid_count_occurrences
                + single_occurrences
                + (num_grids - 1) * grid_count_minus_one_occurrences
                + (num_grids - 1) * (num_grids - 1) * minus_one_squared_occurrences
                + num_grids * num_grids * grid_count_squared_occurrences;
        }
        println!("{solution} ");
    }
}

// actual values: {(1, -1): 6807, (-2, 1): 994, (0, 1): 7808, (0, -2): 5871, (-1, -1): 6823, (-1, 1): 6794, (0, 2): 5843, (2, -1): 989, (-1, -2): 996, (-2, -1): 996, (-1, 2): 994, (1, 0): 7808, (1, -2): 989, (0, -1): 7808, (-2, 0): 5858, (1, 2): 999, (-1, 0): 7808, (0, 0): 7759, (2, 0): 5856, (2, 1): 999, (1, 1): 6808}

// grid counts with 2.5x : {989: 2, 994: 2, 996: 2, 999: 2, 5843: 1, 5856: 1, 5858: 1, 5871: 1, 6794: 1, 6807: 1, 6808: 1, 6823: 1, 7759: 1, 7808: 4}
// grid counts with 4.5x : {989: 4, 994: 4, 996: 4, 999: 4, 5843: 1, 5856: 1, 5858: 1, 5871: 1, 6794: 3, 6807: 3, 6808: 3, 6823: 3, 7759: 9, 7808: 16}
// grid counts with 6.5x : {989: 6, 994: 6, 996: 6, 999: 6, 5843: 1, 5856: 1, 5858: 1, 5871: 1, 6794: 5, 6807: 5, 6808: 5, 6823: 5, 7759: 25, 7808: 36}
