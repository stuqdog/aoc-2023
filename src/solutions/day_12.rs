use std::collections::HashMap;

struct Solution {
    seen: HashMap<(usize, usize, usize), u64>,
    part_two: bool,
    pattern: Vec<char>,
    actual_counts: Vec<usize>,
}

impl Solution {
    fn default(part_two: bool) -> Self {
        Self {
            seen: HashMap::new(),
            part_two,
            pattern: Vec::new(),
            actual_counts: Vec::new(),
        }
    }
    fn valid_count(
        &mut self,
        pattern_idx: usize,
        counts_completed: usize,
        cur_count: usize,
    ) -> u64 {
        let mut ret = 0;
        let key = (pattern_idx, counts_completed, cur_count);
        if let Some(v) = self.seen.get(&key) {
            return *v;
        }
        if pattern_idx == self.pattern.len() {
            if counts_completed == self.actual_counts.len() && cur_count == 0 {
                return 1;
            }
            if counts_completed == self.actual_counts.len() - 1
                && cur_count == *self.actual_counts.last().unwrap()
            {
                return 1;
            }
            return 0;
        }
        match self.pattern[pattern_idx] {
            '.' => {
                if counts_completed < self.actual_counts.len()
                    && self.actual_counts[counts_completed] == cur_count
                {
                    ret += self.valid_count(pattern_idx + 1, counts_completed + 1, 0);
                } else if cur_count == 0 {
                    ret += self.valid_count(pattern_idx + 1, counts_completed, 0);
                }
            }
            '#' => ret += self.valid_count(pattern_idx + 1, counts_completed, cur_count + 1),
            _ => {
                ret += self.valid_count(pattern_idx + 1, counts_completed, cur_count + 1);
                if cur_count == 0 {
                    ret += self.valid_count(pattern_idx + 1, counts_completed, 0);
                }
                if counts_completed < self.actual_counts.len()
                    && cur_count == self.actual_counts[counts_completed]
                {
                    ret += self.valid_count(pattern_idx + 1, counts_completed + 1, 0);
                }
            }
        }
        self.seen.insert(key, ret);
        ret
    }

    fn valid_combos(&mut self, input: &str) -> u64 {
        self.seen = HashMap::new();
        let (mut pattern, mut counts) = input.split_once(' ').unwrap();
        let mut grown_pattern = "".to_string();
        let mut grown_counts = "".to_string();
        if self.part_two {
            for i in 0..5 {
                grown_pattern.push_str(&pattern);
                if i != 4 {
                    grown_pattern.push('?');
                }
                grown_counts.push(',');
                grown_counts.push_str(&counts);
            }
            pattern = &grown_pattern;
            counts = &grown_counts;
        }
        let counts = counts
            .split(',')
            .skip_while(|s| s == &"")
            .map(|s| s.parse::<usize>().unwrap_or(0))
            .collect::<Vec<usize>>();
        self.actual_counts = counts;
        self.pattern = pattern.chars().collect::<Vec<char>>();
        self.valid_count(0, 0, 0)
    }
}

pub fn main() {
    let input = ".?????...? 1,1,1
#????????.#?#?????? 2,1,1,5,1
???##?###????? 1,2,3,4
?#?????##????#?? 1,9
?.?.??#?...????? 1,2,1
.#.#???..??#???#?? 1,1,1,1,1,4
?#??#??#..#?#???. 1,4,1,1,2
??######???.??.. 7,1,1
.?#???#?#?#??.?.?.#? 2,7,1,1,1
.?#.???.???.#??? 2,1,1,3,4
?????????#?. 2,1,3
??.?.?.#??##?#?#?. 1,9
?....?.?#?#???? 1,1,7
?.??.????#?#??##??#. 1,9,2
?.??#...#? 1,1,1
?###??????#??#? 7,5
??????##????# 1,7,2
??????.????.. 1,2
??##??#??#.. 5,2
#????#?#???????#??#. 8,8
???#..????###??? 2,2,3,2
?##?#?#?????#?.#??? 2,3,1,1,4
??#?..?#???? 3,3,1
??#.?#??.?#????? 1,4,2,1
???.?#????? 1,5
.?#.???##??? 2,6
...#.?#?##. 1,1,2
????.?????? 1,2,1
?#?????????# 2,2,1,2
??##??????. 5,1,1
???..??#???#????## 1,1,1,9
??#??##??# 1,3,1
??#????#??.?????##. 9,3
??.?#??#??#.???#?.?? 2,2,1,1,4,1
?#??#?#??#??#????. 11,1,1,1
.#...??###? 1,5
???#?#?#??????? 1,1,1,6,1
.???#.??##?..#??# 3,4,2,1
???#?.???# 1,1,1
???#??###?#?#???? 1,12,1
??.#?????? 2,2,2
#?#??..#?# 5,1,1
#?..?????##?#.???#?? 2,2,5,6
????#?.??##?.??#?? 6,5,1,1
.??.#?##????. 1,6
#??#?####??#?.?? 1,1,8,1
?#.??#?#?#??#???? 1,1,9,1
.???????#?. 4,3
.?.?#???#?#?#??#.? 1,10,2
????#??#??#..?. 1,1,1,2,1
.??#??#????.??? 7,2
??#?#?????.?? 3,2,1,1
???#.??#????#??# 2,3,4
??.??#?.#??#??#?? 1,1,2,4,1
????#.????.?#???? 5,3,1,1,1
?????##??????? 1,1,9
?????.???#??.? 3,6
.?????##????. 5,1
??????????. 3,1,2
?####.??#???.? 4,5
?.#??.??????.?#??#? 2,5,5
?..#?????????? 1,1,5
???????#.???? 4,1
?.??.?????#????. 2,2,1,1,2
????#..?????#.?? 3,5,1
???????????#???# 7,5,1
.???#..???? 1,1,2
??.?.?.???#? 2,1,2
??##???#?????. 3,4
??.????..??###??? 1,4
.??#????#?# 4,3
???.????#.?#?##???# 1,1,1,1,1,8
??????#?.?.??? 3,2
?.???..????? 2,3
?..??#??????? 1,2,1,1
?.#???.?#??? 1,1,3
??????#???.??????? 7,1,1,1
???#?#???????#?. 4,3,2,2
??###?.??#?#? 3,5
?#???.??????????# 5,1,1,1,4
???#.????#???.# 1,1,4,2,1
???.#???#?? 2,7
?#.?#?###??#?. 1,10
.?..????#?#?? 1,2,2,1
.??.?????# 1,3,1
???#??.????#??? 6,1,3,1
??#?#?#???# 3,1,3
?????????????????? 3,4
#????#????##?????? 13,2
????#??.#?#?##?#??? 1,2,8,2
.??#?##??.?#?????#?? 5,9
?##..#???? 3,1
?##.##???#??.##???# 2,3,4,6
#?????????#? 1,1,3,2
???#?????????? 2,2,1,1
??#??##??#?.? 1,5,1,1
?#??.??##???????.?? 1,1,9,1,1
??#?##??##?.??#??? 10,1,2,1
..??#..?#?.? 1,2
??#?.?.????? 1,1,1,1
#.?##??.?.? 1,2,1
..?..??#????#..? 1,8
.?#??.??????. 3,5
??.????#???#????? 1,1,3,2,1
??.#??..?? 1,3,2
#?.#??????.?? 2,2,3,1
????.???.? 3,1
#???.??????# 1,2,1,2
?.?.??.?#??.??? 1,1,1,1,3
?.#????#.? 1,1,1
.????#??.? 2,4,1
???##??#?..#??#??? 5,1,4
??#??##?.???#? 5,4
??#??#???##????.? 3,3,6
.?##??.#?..? 2,2
?????#.?#?#???#??? 3,9
.#.??#???? 1,3
??????.??.? 5,1,1
.??.???????????. 1,7
??????##???#?##??.#? 2,13,2
??#???????#?? 3,1,1,3
?????.?#??? 2,3
????.????????? 1,6
#?#.??#?.#?? 1,1,2,1
?#?????#?###?????# 4,7,2,1
?????#???.?.? 3,1
#?#?.#?##??. 3,6
??#.????????? 1,1,3,1
??????#????.?? 1,1,2,2
??????#?.??##??? 1,3
?.###???#???? 4,2,1
??.????.??.#.#??#? 2,1,1,1,2,2
??##.???##?##?.##?#? 3,2,5,4
??.?????????#?? 1,2,5
.##????????. 4,1
??..?##.???? 1,2,2,1
.?#?.????.#?##?... 2,2,1,3
#?????.???#?#??? 1,1,1,7
?#.#???.#??#???.???. 2,1,1,1,4,2
??????????##???#??? 1,1,7
???#?#???.##.. 3,2
.??##?#.??#?. 6,1
?##?.###?#?? 3,5
????###?..???..? 1,6,3
..#????#?#??? 1,3,2
#?#????????# 4,1,4
?#???????????#?? 2,3,1,3
.#???#?..??#?????# 1,2,9
??.?#.???. 1,1,1
?#?#??.???.#? 1,2,2,1
??##??#???#? 3,5
?#???????. 3,1
.#?????.??? 3,2
.#?#?##??????#?? 1,4,6
??.??.?##?.?? 1,1,3,1
??##??.#.?.?# 4,1,1,2
#???????#?? 1,1,4
????#??.?#???? 1,4,1,3
#?.?##???.? 1,3
????#?????##?##???.? 8,6
??.?#?.???.#??? 2,1,1,2
??#??#.??.???. 2,1,3
.?.?.??.?##### 1,1,5
???#??.???#??#.?? 1,3,1,5,1
..#????##??#?#?.? 1,10
?#.????#??. 2,5
#.????????#?#??#??? 1,1,7,2,1
.?.?...?##?#?#?#?. 1,9
???????????# 1,1,6
??#???????#?..?? 9,1
..?#?????? 1,3
.?.???????? 2,4
.??.#??.??? 2,1,1
??#.????.? 1,1,1
.#.??.????#???.???? 1,2,1,1,3,3
??#??????.??? 5,1,2
????????.??.?? 1,4,1,1
???????#?? 3,3
??#?????#???##. 2,1,2,2
??#???????# 2,6
?#?#??#??#.?.#?? 6,1,2
##?##?#?.??#?? 8,1,1
.?#??###?????#?? 8,3
????#.#?#????? 3,6
#??.????#??#.#??..?? 2,6,1,3,1
??????.????? 2,1
????.?????. 1,2,1
???????#..#? 1,2,2,2
????#??#?? 4,4
???.?????##?##???? 1,14
??#.???#.?#??.???? 2,2,1,1,1,1
?.???#?.?. 1,5
#.#?#.?##????? 1,3,5,1
.?..?#???????#???#? 1,14
.#?#??##??#?..??#?. 3,2,3,4
.??#?#?.????.???.?# 6,1,1,1,1
.?#.????.#.? 1,1,1,1
?#?.??#??#?? 1,8
??..?????? 1,5
???.?????.? 2,4
??#???.##?#?.??. 5,2,1,1
??#???#?....???#?.?? 3,3,4,2
?..???##..?.?. 1,5,1
?#????????.?? 3,2,1
?#??????.## 4,1,2
.??.###.?#??. 1,3,3
?.??????#?#??? 1,1,8
?#.?.??????#?.???? 2,1,6,1
.???????#?#???. 2,6
???#????.?#??#??? 4,1,1,5
??#???##???#? 7,2
#??????.?? 2,2
?.?#.?##???.?#?## 1,2,4
.?#.??????#? 1,2,2,1
#?#?#?#?##.??#?#.? 1,3,4,2,1,1
???#?????#?#?? 1,6
?????##?#?#?##??? 1,11
#?????????##?.#??.? 1,2,1,6,2,1
???#?.???????.?.. 1,3,2,2,1
.??.#?????? 1,2,1
??#?????#.? 6,1
.????#??#??????. 12,1
?.?...#??...?###??.? 2,4
??.??.?##??..?? 2,1,5,1
?#????.#??###? 2,1,6
#.???#?.?#?#??.? 1,5,5
#?#?????????#? 11,1
#???#??#?#???.#..#? 12,1,1
.??#?#?.????? 5,1
?????.?????#????? 1,1,1,6
.??#?.?????#???#??? 2,1,1,1,4
?#???##????.#???? 7,1,4
.?#?#??#.. 3,2
..????????.#?????#.? 4,1,7,1
??.????#.#??.?# 1,5,3,2
?#??????.???##.? 3,3,2
.#.?#???#???##??#?# 1,8,3,3
??#.??#?..####?#?##? 1,1,4,4,5
?#?????????#?# 5,5,1
???????.?? 1,3,1
???????..??.? 1,2,1,1
.?##.#???? 2,2
?#?.?#??#.#?? 1,5,2
?.???.???##??????. 3,9
?#.?????.?.???.??## 2,2,1,1,3,3
???????#??.??#????. 5,2,3,3
?????.???.? 4,1,1
???????.?#?##?.? 1,1,5,1
.??????.#??.??? 4,3,1
??.?#??#?? 2,2
?.#?.??#?.??#??.. 1,2,4,1,2
??#?.??????? 2,1,1,1
???????###????.#??. 3,9,2
.#??.#??.###??#????# 3,3,7,1,1
.??????.?? 2,2
????#.#??#?##???? 1,1,11
#???????##.? 1,1,4
.?#?#???#?.???? 8,2
?.??#?.#?.???..????? 1,3,2,1,5
???#????#.#####?? 1,3,2,5,1
??#?#?????? 6,2
??.????#??#? 1,6,1
#????.##?#. 2,2,4
?#?#????????????? 9,1,1
????##?#???? 6,4
?????.?????? 5,1,1,1
????????????????###. 1,8,7
??.????????#??#?.?? 1,13,1
????.#???.####? 2,1,5
??#?#?????????#. 7,1,1
??.#??##?? 1,1,5
?.???#??#???#??#?# 1,1,9,1,1
????#???#?#. 1,7
???.???#???##.#?? 1,1,7,1
??#.????#. 1,2,1
.##.??#?#.? 2,4,1
??#?.?.#.???? 3,1,1,2
#???#??#?#?.???.? 10,1,1
???..#??## 3,5
.?????#??##???#. 6,2,1
.???#???????????. 5,5
?????#?.??? 1,2,2
?#?#??..?.?#??.. 6,1,4
???.?#.???? 1,2,1
????.#??????##???.. 1,1,2,8
.?????????.????##? 5,3,5
?##???#????#?#??? 3,11
?????##?.?#? 2,2,2
?.????#????? 1,5,1
???#.??#???.?#? 1,2,4,1,2
##???#??..#?????? 3,1,1,5
??????.??#.? 1,2,1,1
?#????###??#?##? 2,9
#.??#?.??####? 1,1,1,7
?????#??.??#?????#? 5,9
?#.??#.???##?? 1,3,4,1
.#..??#???????.???? 1,4,1,2,1,1
##?#.??###?## 2,1,7
?#?#?#?.?? 3,1,2
.??..???.???..#?? 1,1,1,1
.?????????.?????#??? 2,3,3
#.#?.???.????.???#. 1,2,1,4,1,2
??????.?##?.. 4,3
?.##?#??.#?###?###?# 6,9,1
?.????.?.??????#??? 4,1,1,2,1
.?.????.?#???. 1,5
.?#????#??#.??#????? 3,4,5,1
?#.#?##???#?????#? 1,9,1,1
.???.??????#??#?. 2,7,2
?????#??##?##?????? 2,1,9,1
###?????????.?. 6,1,1,1
??????????..#? 1,1,6,1
????.??.?#.??.####?? 1,1,1,1,2,6
?????#?#?##?#??# 1,13
????#??#?????? 5,1,1,1
..???##.#????#. 1,3,3,1
??.?#?????? 2,5,1
.??..??????#???#?? 1,2,3,3
#?..?#?#??#??#?? 1,10
?#??#?#?#???????? 7,5,2
#?.??#??????.#.#? 1,3,2,1,2
??#.???#.???????? 3,4,5
..????..?##...? 2,3
.??.??...? 2,1,1
##...??#?.????## 2,4,5
###.??#??#?#?.?# 3,8,1
.??#??????#?#..??. 1,4,1,1,1,1
????????#??##??? 2,1,1,6,1
???.##?.??#. 3,3,1,1
????????#????##??#?# 9,1,3,1,1
??.??#??## 1,3,2
????#?#??#?.???. 1,1,5,1,1
??#???#.???? 2,1,1,1
?????#???#??#??#?#.? 1,1,1,8,1
??.?????.?.#??.?? 2,1
??..?#???# 1,3,1
.?##??????????? 7,1
??????.??????.#.? 1,2,2,1,1
.??????.??? 4,1
?#????#.?.??? 2,1,1,3
#.??#??#.##????. 1,3,2,2,3
??.#?#?.?.??#?????. 1,1,1,1,1,4
????#?????????#?.?? 1,3,7,1
????????.?? 4,2,2
.????????? 2,1
#???#?#?.?????????? 1,5,2,1,3
.?#??#??##?#. 2,7
??#?.????#??###?.?? 1,5,4,2
??.???.?##?? 1,3,4
?.?????.???? 1,5,2
??????#??? 3,1,1
#..?#????????##? 1,12
#???#?.??? 3,1,1
????#?.?.????????? 1,1,2,1,4,3
#?????#??#??.??????? 1,8,1,1,4
..?#??##?? 1,3
.#???.?##?##????? 1,10
??#??#?..??#??.??#? 7,5,2
??..??..#??? 1,1,1,1
???.????##?#.? 2,7
.?##???????.?#?.?.?# 2,3,3,1
.?.??#??????#?#??? 1,1,1,3,6
#?.???#?????#.#???. 2,4,2,2,1
?#?#???.?#????#?? 4,6
????.?????#?????#?? 1,2,2,9
??.??#?#.???#??.?##? 1,5,2,1,2
????#.?.???#??..? 1,6
??.?.?###?#???.????? 1,1,6,1,2,1
??#.###.#???. 3,3,1,2
????#?..#?#???? 1,1,1,2,1
.??#??#????????##?# 1,1,5,1,6
#??##.??#???#?? 5,8
???????#??? 3,4
.??.#.??.??.???? 1,1,1,2,1
??#??.?#?.?? 4,1,2
.???????.???#?????.? 3,6
???#????#????#??#??. 3,14
????????..?????.? 3,1,2
?????#?#?.#??? 1,4,1
#???.??##.???? 4,2,1
?.?#????#? 2,3
?.?.???##???. 1,7
...#####?.??##? 6,3
?#???#?????? 1,3,2,1
???.##?###? 1,7
#?#???#????? 7,2
.??#???.?#???.???# 5,3,4
?#?#??#??#.?#???.#?? 1,5,1,2,1,1
??????##??##??#??. 4,8,1,1
??#????..??##???#??? 4,1,9
?.??#?.?.? 2,1
?#????#?.?? 7,2
???????##?????? 4,4,2
??##?.?#???.#?.????? 1,3,3,1,1,5
.?.?????#????? 1,1,3,1
?????.??#?????###? 5,1,2,1,3
???#??.#??????.?. 1,4,1,5
??#????????#.? 2,1,4
?.??????#..? 3,3
?????##..#.? 2,2,1
.#?..????#####???#. 1,13
.??.?????#?#?.#? 1,1,6,1
??????????#??. 6,2
???.?..??? 3,1,1
??#???????#?????? 3,6,2
?.???.????.??#? 1,2
????.???????????? 2,8
.#.??????. 1,5
?????.???. 2,1,2
??.??#???#..?###???. 1,5,1,6
#????#.??##?.??# 3,1,3,3
???#???.#?.?.?????.? 6,2,1,3,1,1
.###.???.? 3,1
???????..??#? 6,2
????#??????#? 5,2,2
????????.??#?? 2,3,1
#?#.???.??? 3,1,2
?###?###???.?? 8,2
?#??.#..#?.?#???. 1,1,2,3
?????.?#????? 5,1,1
.?#??????.?.?.?? 3,1
.#??##??.???#?#.. 7,3
???#?.?????? 2,4
???#.???.. 3,1
?.???#??#.????# 1,1,1,1,2
??????##.??#????#?? 7,1,2,5
?????.?.?? 1,1,1
#??#?????.????. 1,3,3,2,1
?.???##????.??#?#.. 5,2,3,1
?#.#?????? 1,1,1
??????#?##?#?.?#?. 1,1,1,2,1,3
??#????.##? 3,2
??.?.#.??#? 2,1,1
?????##?.?#??? 3,3,1,1
???#.?#???? 2,3
???????..?#??#??#??? 3,2,6,1,1
.??#?#??.????#??#??# 7,1,7
???#.?#?.# 2,2,1
??.?##?#.??# 5,2
?.????#?#.? 4,1
????.???#.? 1,3
?#????#??#???#? 1,1,2,2,3
.?????#??#??#??. 1,10
?#?#??#?????#?#?#??? 13,1,1,1
?#??.?#?#.???? 1,1,3,1
???#.???#?? 1,1,5
#???????##???? 1,1,7
?#.?????###? 1,1,6
?..??.????# 1,3,1
?.#??.????#? 3,1,3
?.#?#??#??????? 1,1,4,1,2
.???????..#.??????#. 4,1,1,1,2,1
???#..?#..?.??##?# 1,1,2,1,5
??????????.#. 1,1,3,1
????..????????????# 1,1,1,3,1,2
.#.??#???? 1,1,1
??????????????????? 4,1,4,1,1
??????????#??#?????? 4,8,1
.???#?.??? 2,1,2
???.???#???.?.??#? 2,6,4
?.?.??????#?#?#? 8,2
??#?.????? 1,4
??#???#??????#???. 2,5,2,1
?#???.#??? 2,2
#??.##?####??#? 1,11
#????##??.????.?##.? 4,4,1,1,3,1
???...?#??#??? 2,6
???????????? 1,1,3
?#.????#??.#..?? 1,1,4,1,1
.?.?????##??##???? 1,1,12
????????..??## 1,5,3
?????#?#..#??? 7,1,1
.?##???.#?#.??#? 2,1,3,1
.??##??#????#.???..? 8,2,1
?#????.??.##???? 3,1,3
#??#????#????? 5,4,1
##?#?????#? 4,2
???#??#??.. 2,3
?#????#????.#.###? 2,5,1,3
?.?????#?#??.????#?? 1,4,2,5,1
#????#??.??? 2,2,1,1
.?????#??.????.? 4,3,1,1,1
?#..?#??.# 2,1,1
#?.??#??.?##?? 1,4,5
??????##??.? 1,1,4
???..#?.????.? 1,2,2
???#?.#?.?#??#..#? 5,2,3,1,2
?.#?#????#?????????? 10,1
??????#??.? 3,3
????#?#??#??###?? 5,7
?????.?.?.#?.? 3,1,1
?????#?#####?# 3,9
?#??.?..??. 3,1,1
?#?????#?#??#.??? 4,6,3
?????#?.??. 1,4,1
????#?###???..? 10,1
.???#??#?.??#?# 1,1,1,3,1
#????#??#.? 1,6,1
#????#?.#?#?.#.?? 3,1,4,1,2
#.?.?.?#.??.???#? 1,1,2,1,5
??????..?????#??.?. 1,1,1,4,1,1
????..?..?????# 1,1,1,1,2
??.#???.?#? 1,3,1
????.??#?. 1,2
?#??#??????.??? 8,1
??????###??#.??.?? 1,6,2,1,1
????..##??#?#??????? 2,11
?.???##??????. 2,7
.#??#?.?????? 2,1,4,1
?.?#??#??##?#???#??# 1,8,8
#????.#?????#??????? 1,1,1,7,1,1
?...?#?.????##?? 1,1,7
..???.???????#? 2,6
#?#???#?????????? 5,5,1,2
??.??????? 1,1,1
..?.???.??? 1,2
#.????.?#????###???? 1,2,2,7
??.?..??????#???? 1,1,2,6
.?#????##.#.??#???? 8,1,5
#??##?.?#???# 6,3,1
???#?????????????? 9,1,1,1
?#????.???#??# 3,1,3,2
????.??????????.? 1,1,2,1,3
?..?#?????..?#?.??# 1,2,2,1,3,2
??#???#?#??#??#??? 4,3,6
.????#.???????? 2,1,1,1,1
.??????.#?#.# 4,1,3,1
.?#?#?#??#?????#? 1,6,1,1
???.????#.????? 2,4,1
.?.??#???? 1,1
?.?#????#?##?#?? 3,7
..???##??#.#.???? 7,1,1,1
?.???#??????###??.#? 1,1,1,4,3,1
????#???.???..# 2,1,1,1,1
??????.?.#???. 4,2
.??#??.?##.#??? 5,2,1,1
?.?.???##?????#???.? 1,1,11,1
??##??#?????.?????? 10,3
.???#.??????????# 1,1,5,3
??.#??##???##??##?? 10,3
?#.???#??.? 1,5,1
??????.?...??#??#?.. 3,7
?.???????????. 2,1,2
??????????..?###??. 5,3
??????#?.?? 1,5
.?##..#??? 3,2
??#?..???#??? 3,1,2,1
????????.#??## 1,1,2,2
#??????#?..# 1,3,1,1
??.#?.?????# 1,1,1,1
???#?????.?. 5,1,1,1
?????????#? 2,7
?##?.##????? 3,2,1,2
?#.????.?###? 2,2,4
?.#?.?#???? 2,1,2
??#???.???.??. 2,1,3,1
.?#?#????? 1,1,2
.???.???#?###?? 2,7
?#?#??????????# 6,1,1,1,1
??????.???? 1,2,1
..??.?#???#.??#??#?. 2,5,2,3
?????#?.???##??#?? 2,2,6,1
???..??.??...#?#..?. 2,3
#?.?#??#?? 1,2,2
?????????.??#???.??# 2,5,1,1,1,2
#????##?#?#?.?? 2,3,1,2,1
#????????????#??? 1,2,11
?????????..? 1,5,1
???????..?????.#???? 1,4,1,1,3,1
???.????????????#? 1,1,1,1,2,5
??.????###?#? 1,8
.?##???.??????# 5,4,1
.????????.??.???? 2,2
?#?#.????#??? 3,6
???#??.#??#.?#?.?? 1,3,1,1,1,1
??..?.??#? 1,1,1
???..????#.?#? 1,1,3,1
???#?????????.? 1,2,1,1,1
?????.###?#?.???? 1,6
??##???#####.??.?.? 11,1,1,1
????.#?#??..??. 3,1,3,1
???#????????????? 2,1,2,5,1
?#????.?.#??? 1,1,1,4
#?????#???#?#. 1,1,2,3
????#????#??#???? 1,12,1
..#.?#.??#?.?.#.??? 1,1,3,1,1,3
???????##?# 1,2,4
????????#????? 4,4
#??...?#?.#.???? 1,1,2,1,4
?#?.???.###?.#???? 3,1,3,2,1
?#???????? 2,1,1
.#???#?????###? 1,4,3
???#?.??.?????#?. 4,6
????##??#??#?##?. 1,3,1,4
?.???#?##??#?? 6,1
?????????.??????#?# 1,1,5,1,1,4
??????#??##???.?# 1,10,1
.?#...??.??.???? 1,1,1,4
#?#.#?##?..#??#? 1,1,4,1,2
.#??.????##?..?? 1,1,1,3,1
??????#??#?? 7,1,1
??##?#??.#?#. 5,3
?...????.##??# 1,2,5
.#?.???..? 1,1,1
???.?..????#???. 3,5
???????#???#??#??#?? 1,9,1
??#?##???.#??.#.???? 4,1,1,1,1,3
.?????..????##?? 1,7
??????###??##?#????? 1,2,14
???#???.#???.??? 1,4,2
??.?#??????????? 1,11,1
??.?.????..???##??.? 1,1,5
#??????#??.??????. 2,5,1,3,1
??#?.?.???#?##? 2,1,6
.????..????? 2,2
????##???..?##?..? 1,4,3
..????.?.??????????? 1,2,1,6,3
#??#?###??#????#. 4,6,3
#?##?#???.????##???. 9,9
.#?????.?..? 1,1,1,1
???##??#??###?##?? 8,8
??.????#??.?#..# 2,7,1,1
??????#??????????#? 2,4,10
??##?#?????##??.#?? 11,1,2
??..?.#?#?? 1,1,5
????###?#??.?#??.?. 6,4
??.##????#?..? 1,2,5,1
??????#??##??????. 8,2
????##???.##??#???? 6,5,1
???????##?.?..?????. 6,3
#????##..?? 3,2,1
??#??##??#?..? 1,5,2,1
???#???.???#??? 1,3,5
.??#.#???.?#?.#?.?? 3,4,3,2,1
.???##...????? 5,1
?#??.??#?.#???##.#?. 1,1,1,6,2
.?#??#??#?#?##??# 4,9
???#????.???.??#.?? 8,2,3,1
.??#.?.?##.. 2,3
????##?#?.? 1,5,1
?.?.??????###? 1,2,7
?#????????????? 3,1,4,2
..#??#????????? 4,1
???.?.??????#??#?? 1,1,2,1,5,1
.???#??#?#????????#? 11,3
.???#??.#??.? 3,2,1
????#????#???#?# 1,4,1,3,1
?####?#????.??##??? 10,1,5
#???.??.?#. 1,1,2
??####?????#? 7,1
????????##?.#???.? 2,6,4
?#????##.?????? 3,1,2,1,1
??#?#??????.#? 6,1,1
?????????.#.# 7,1,1
??????????????? 6,5
.?.?.##??? 1,3
.##?.#??.???#? 3,1,5
???#???#???????? 4,1,2,1
..???????? 1,1
???????????? 7,1
..????.???????? 3,1,3
???.???????#???##?? 1,2,1,2,4
.?###?.???? 4,1,1
?.?#?#??#?###???#??? 2,12,1
????#??.?.?????.?? 2,1,1
?????#???????#???? 1,6,4
?.???.?#???? 2,4
??###..????##???##?? 5,9
??????????. 1,3,1
#??????###?? 5,4
#?#..?#??#?##??? 1,1,3,7
??#?.?###?????##?. 3,5,4
??#???????##.?##? 10,2
#.??###?#? 1,5
?...##??##?##??. 1,9
.??#???#.???????? 6,2,1
#??#?#??.#??#?##??. 8,9
?..?.#???????#?? 1,1,3,4
??.?#??#?.?.#??? 5,4
?..???????#??##???? 1,1,5,3,2
.?#?????.. 2,1
???###..??? 4,1
????#??????###???#?? 1,2,1,1,9
??#????.????.#??#??? 5,3,4
?#???#..?.??????.# 2,1,1,2,2,1
??...??##?#???#?? 1,9
???.#??.????#? 3,2
?????...???.#???. 2,1,1,1
???#???##???#??..?## 1,1,4,4,2
??????#??..??. 2,2,1
?????##?????????# 1,6,1,3
.#.#.?#???.? 1,1,4
?????.???.???? 1,1,3
???.??#??.????#??? 1,3,1,8
??##???????.?##??? 3,1,2,4
???#???????.?#. 9,1
????#.??????. 3,1,4
.????.#?##??#. 1,1,7
?.???????? 1,1,2
.????#.?##?#?#???. 5,8,1
?#??.??.?#?? 3,2,2
?????????? 2,1,1
#.??????##??????## 1,2,3,5
?#??.?#?#?? 1,5
.##???#?#?????#??? 8,1,4,1
??.?????.??#?????# 2,5,4,2
?..?.??.??#? 2,4
???..??.???##? 1,1,6
#??#?..???.???.#??. 5,1,1,2,1,1
?????#?#??#?# 3,4
#???.?..??##?# 1,1,1,6
.#????.???#??? 5,1,3
#.???#???? 1,1,4
.?????..?????#? 2,1,1,1,1
.#???#??.?#?. 1,3,2
????.????#??? 1,2,4
?##?#???###??..?##? 5,6,3
??????#??#???. 1,1,1,5
#???????????. 5,2
#.??.????????.??? 1,2,7,1,1
??#.??.?????###? 1,1,2,5
.#?.?..#???????? 1,4
?###?##.?? 4,2,1
.????.?#?##.? 1,4
?.##?.?..?? 2,1,1
?#?.?#.???? 1,2,1
.???##????????. 6,4,1
.##???????##???#??#? 2,1,9
#???????## 2,4
#????#.#.?#? 6,1,2
??#?#??#?? 3,2
##.??.??.?.#?? 2,1,1,1
?#????????#?##?#??#? 2,1,1,1,7
?????#????????. 1,7,1,1
??.???#???#?#? 5,4
.????.#?####?#????? 1,1,11
???.??.?????#????.?? 1,1,8,1,2
??????.?.#?#????# 3,1,8
#?.??????..# 2,2,1,1
???#?????? 2,2
?????#??.????##? 7,5
.???????#??? 1,8
?.???????.??? 5,1
.????##???#???????? 1,5,3,3
.?#???###?#???#? 1,8,2
?##??#????#???#?.?? 3,5,2,1,1
??#?#.???? 1,3,1
?????????????? 1,3,2,4
??#??.?????#??#? 3,9
#.#????##?..? 1,2,5
?#????#??????? 1,1,7,1
.??#??#????###??? 8,3
?.???#??#?? 1,3
?#??.???#?????????? 4,6,4
?.?.?#.?#???#??????# 1,1,2,8,1
??#???????? 1,4
.#????????. 3,1,1
.#?#?#????????? 1,1,1,6,1
???#???#?????? 3,1,1,2
?.#??#???????#????.? 1,1,5,5,1
.#????#.????# 4,1,1,1
#.?.#?#?.??#?#?#? 1,1,4,7
.?????.???. 1,2
??.???#????#?#..??? 2,2,2,5,2
.????.????##?? 2,6
??#?.??####??. 3,5
?#?#?????#?????# 7,4,1
??.#????#?? 1,4,1
?????##???#??.? 5,1,3,1
????????.#??? 3,2,1,1
???.???#?. 1,1,1
#?.#?.?##. 2,1,3
???#??.???#. 4,2
?#?#????#.??#..????. 9,1,1,1,1
?#??.??#??#? 2,1,3,2
?##???#??????##.?.? 2,11
????#?.?????. 5,4
???#?.?????#???? 3,3,4
????####??.??#????? 8,5
???#??#??? 1,3,3
???#?#????#??#??..# 14,1
???..?.???? 1,2
#...??????.?#??.? 1,5,3
???#?#??????.??##?? 3,1,5,1,2,1
?##??.?.?#? 3,1,1
##?????#?#??????#.? 8,1,4
.##???##??#?###??.?? 4,11,2
###.???????.???? 3,1,2
?...??????????#???? 1,5,3
????.?.#??##???.? 4,1,2,1,1
#??.?#?.#???. 2,3,3
?#??#?#?..???????? 1,4,1,1,1,1
.????.?#?#.?????.?# 3,3,1,1,2
##?.???.#????#?#???? 2,2,1,1,8
.?????##????#?.#.? 8,2,1
?#??###???#??????#? 12,1
.?..??.??????#??.#?. 1,7,2
?#????#?...?#?? 1,3,1
??????????????? 3,1,3
??????.????##? 1,2,6
.???.?##???? 1,3,1
?####?.#????.#? 5,3,1
?.#?#????????..# 1,3,4,2,1
#?.?.????#?? 2,1,2,3
??#???????#???##?# 10,5
.???#?????.? 1,6
?.#?.???##?????????? 1,2,6,7
???????#?#??#? 1,1,1,7
?#??#?#??..##???.. 8,4
????????.#? 2,1,1
#.#??..?.#???? 1,3,3,1
#???.??#???? 2,1,4,1
????.????? 3,1,3
?????.##???.??? 1,2,2,1,1
??#??.#?????#?? 3,2,3
.??#???.??###??? 1,1,6
?????.???.? 1,2
?.?#?????#???. 8,1
.???????#?#??????# 1,6,3
????##?#????.. 1,4,2
?.??.?.?????? 1,1,5
?????.###.? 4,3,1
???..??#.##? 1,3,2
#.#??#??.?????#.?? 1,4,1,1,1,1
.#?????.?##???.?# 2,3,4,1,1
???#????.??????.??.? 4,1,1,1,2
...???.?.#. 2,1
???#?.##.???.????? 3,2,2,1,1
??????#??.##???????. 4,9
#?#????????? 5,4
.#????##???.?? 1,7,2
???#.?.?#. 1,1,1
???#?##.??#??#??.##. 6,6,2
???????#?#?#?###?#. 1,11
????#.#??.? 2,1
?.#??.?????#??????? 3,2,8
?#.???????? 2,4,1
??.?#??#??? 2,7
.?????????#? 3,4,2
?.???.???#.? 3,3
.#?#??##??? 4,3
?.?????????? 3,3
?????.?.#????????? 1,1,1,1,5,1
?#?.???????#?#???#? 2,3,3,2
.?#?#?????.????..? 8,3
.?#?#.?..???? 3,4
?.?????.?.##.??.? 5,1,2,1
??????#???.? 3,4,1
#?.????.?####?? 2,1,1,4
????#?????????#? 2,3,6
?.#??#????##?????#? 1,2,1,2,4
??..????##???? 2,8
?##?####??# 8,1
?.?#####?????#.?. 6,3
###??#..##??.??#?. 6,3,2
?????#??.???#? 2,4
?.??????#?#?#?? 1,6
????#?????? 5,1,1
#??#?#.#?????#.???? 1,2,1,3,1,3
??#?????##???.. 3,6
???#??#?????? 4,2,1
?.??###.??? 4,1
???.???????.??? 3,3,2,1
????????#?????#?.?? 4,2,1,1,1,1
????.???????#?? 1,1,1,4
##??....##??? 4,3
.?#???#?????.?#???. 8,1,2,2
?.?##?????.. 1,5,1
##???#????? 2,4,2
??.?.????##?????. 1,10
?##???###??.?????#. 10,1,1
???????#???? 3,1
?.??..???#.? 1,1,2,1
#????#.... 2,1
?#?.?????? 3,1,2
??.#?.??#?.. 1,1,4
?.??.#???????#?? 1,7
???#??#??#??#?? 4,2,2,3
???..#?##? 1,4
.???#??..??. 4,2
?#?.????#???#? 2,1,2,2
???????#.???#?#?. 4,1,2,1,1
?????.??#??? 2,5
??????.?####?? 3,6
???##??????? 9,1
????##?#?.??.#??.??? 1,7,1,1,1,1
#????###??##?.?.?? 2,8,1
???.??..?. 1,1,1
.?#??#????#?#?#? 1,11
?.???#.???????? 1,2,6
?????##?.??#??## 8,3,2
?.?????????? 4,3
???.???#?##???????? 2,1,6,3
..??????##?#????? 8,2,1
.?.#??#???##..#??#? 1,9,2,1
?.?#.??#?# 1,1,5
.#.??#?#?#? 1,5
.#??????.?# 1,2,2
.?????????.???.# 1,4,1,3,1
#?????##?#?????????? 1,1,8,1,1,2
?##?#???.???????##? 7,10
#.?.##.?#? 1,2,1
?##??..???#? 4,2
??#?#????# 6,2
#?.##?#??? 1,6
???#.?#???###.????# 2,1,7,5
??????#?##??.? 3,7
?#?#??#??????.. 3,1,1,2
.??#??#?.?.??????? 1,5,1,3,1,1
#??.#??.#??#??????? 3,3,1,5,2
.##??????????????? 3,2,2,5
??##??.#?####.. 4,6
?.???###?.#..?? 6,1,1
???????????? 1,1,7
???#?#?#??????##??? 9,3,1
??????.???. 5,3
.????#??????##??? 6,8
..?#??#???#?#?.??## 5,4,2
?????#???###?.#?.??? 1,9,2,2
#?.??#?#?? 2,3,1
#?#???.???##? 3,3
#????#???????.??#? 12,1,1
#???.?????###?#? 1,2,2,5
#?.?????#?##???##?? 1,1,11
??#??#????#?# 1,1,1,6
..#??.??#???? 3,1,1
???##??????#?##?. 6,4
.???????.#.??#?????? 4,1,1,8
..??.????#?.?# 1,2,2,1
.????#??.?#?.??##??. 5,1,1,4
?#.??????? 2,1,2
????#?##??#????? 9,2
?????????#. 2,1,1
?.?#????##????#? 1,9,1,1
..?#??#??.???? 6,2
?##??????.??????# 6,2,1,1
????#??#??#?#?.. 5,7
?#?#???.?###???????# 5,7,1,2
??#?#??#???? 1,1,2,1
?.?.?#???### 1,1,2,3
?????.???????#?.??.? 2,1,9,2,1
.??##???#.?.????#? 6,1,2,1
??.?????????.??.. 2,4,1
??#???#?##?? 4,5
???#??####?.????? 10,1
??????#????#??#? 1,10,1
##?.????????# 2,3,1,3
?#??.?.??###?.#?## 1,1,1,1,3,4
???#?????##? 1,2,1,3
?#???.?.##???.?????. 4,1,3,5
#..?..?.?##?#?? 1,1,6
?.??.?????..? 2,2
???#??.?#??#? 5,2,3
##??????#??? 3,3
??#.?.??.#????#?#? 3,1,2,5
?.?????????#???????? 10,2
???.?#.?????????#. 2,1,10
???#??##??????# 9,1,2
??##?#?????###.? 8,3
?##????.?.???#??. 2,2,1,4,1
.?#?###???????????? 7,1,1,2
#????#???#?#?.????# 1,1,5,2,1,1
.??????.???????. 6,1,1,1,1
??????.??#. 1,1,3
??.??##??.? 1,2
??????#???.?.??. 1,5,1,1
???##.??#??? 4,1
..???#??#? 4,2
#????.#?.?????#???? 5,1,1,1,2,1
?.??#???.#?.????. 4,2
?.#.#?.?.??#??? 1,2,1,1,3
?.???###??##?..???. 7,2,1,1
?##?.????????? 2,3,3
???????????????##??? 1,1,5,1,3,1
?#.?????#????#??#??? 1,2,1,7,2
###?.???#?#????????? 4,7,5
.??#?????. 1,1,1
..###??#??? 3,3
???#???#?.#?.#??# 5,2,2,4
??.???#.???????.. 4,3
?#???.?????? 1,1
??..??????? 1,3,1
??#???????.??.?#? 7,1,2,2
?????????.???#?. 3,1,1,3
.????#.???##?? 5,6
??????.??? 1,2,2
?????????? 2,1,3
?????.??#?? 2,4
?.?.??#??????.? 1,4,3
?#?#???????????# 12,1
#?.?#????? 1,7
????#?#?#????# 1,6,1,1
...?#??????????##?# 8,4
??????#??? 3,4
??.????????????? 1,1,2,1,2
?#??#?#??#.?#???? 7,2,3
????..???#??#??. 1,2,1,6
?????.??#?????.??. 1,5
?#.????????? 1,1,1,3
??##?.#?#??? 3,5";

    let mut part_one = 0;
    let mut part_two = 0;
    let mut p1 = Solution::default(false);
    let mut p2 = Solution::default(true);
    for input in input.lines() {
        part_one += p1.valid_combos(input);
        part_two += p2.valid_combos(input);
    }

    println!("part one {part_one}");
    println!("part two {part_two}")
}