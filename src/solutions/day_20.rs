use std::collections::{HashMap, VecDeque};

struct FlipFlop {
    is_on: bool,
    send_to: Vec<String>,
}

struct Conjunction {
    most_recent_high: HashMap<String, bool>,
    send_to: Vec<String>,
}

struct Broadcast {
    send_to: Vec<String>,
}

enum Modules {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

impl Modules {
    fn of_string(s: &str) -> (String, Self) {
        let (name, send_to) = s.split_once(" -> ").unwrap();
        let name = name.to_string();
        let send_to = send_to
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut name = name.chars();
        let type_ = name.next().unwrap();
        let name = name.collect::<String>();
        match type_ {
            '&' => (
                name,
                Self::Conjunction(Conjunction {
                    most_recent_high: HashMap::new(),
                    send_to,
                }),
            ),
            '%' => (
                name,
                Self::FlipFlop(FlipFlop {
                    is_on: false,
                    send_to,
                }),
            ),
            _ => (
                "broadcaster".to_string(),
                Self::Broadcast(Broadcast { send_to }),
            ),
        }
    }

    fn send_to(&self) -> Vec<String> {
        match self {
            Self::Broadcast(b) => b.send_to.clone(),
            Self::FlipFlop(f) => f.send_to.clone(),
            Self::Conjunction(c) => c.send_to.clone(),
        }
    }
}

pub fn main() {
    let input = "%rp -> gq, sd
&kh -> cs
%jz -> pl, jb
%dx -> tx
%dh -> bm, sd
&zv -> ns, dx, hl, hn, fm
%xb -> ds, sk
%hv -> sk, kr
%db -> zv, zz
&sk -> rg, hh, hv, kr, kh, zl, zn
%tc -> jz
%dj -> ts, pl
%jk -> sd, vh
%fm -> dx, zv
%dp -> sd, cc
%vh -> sd
&lz -> cs
%kr -> rg
%jb -> bf, pl
%kz -> sk
%ts -> pl, bs
%gr -> ns, zv
%kc -> sd, kf
%jd -> zv
%bs -> vg
%zk -> rp
%vf -> zk
%mm -> ms, sk
%qc -> pl, dj
%fk -> qc
%bm -> vf, sd
%ds -> kz, sk
%sn -> zv, jd
%zn -> mm
%ct -> fk
%np -> sk, xb
&tg -> cs
%tx -> cm, zv
%zl -> hh
%zz -> px, zv
%ms -> zl, sk
%ns -> db
%px -> zv, sn
broadcaster -> fm, hv, kc, bv
&hn -> cs
%hh -> np
%kf -> dh
%vg -> pl, tc
%bv -> ct, pl
&pl -> bv, fk, ct, bs, tg, tc
%cm -> zv, hl
%cc -> sd, jk
%bf -> pl
%hl -> gr
&cs -> rx
%gq -> dp
%rg -> zn
&sd -> zk, kf, gq, lz, kc, vf";

    let mut modules = HashMap::new();
    input.lines().for_each(|l| {
        let (name, m) = Modules::of_string(l);
        modules.insert(name, m);
    });

    input.lines().for_each(|l| {
        let (name, m) = Modules::of_string(l);
        let targets = match m {
            Modules::FlipFlop(f) => f.send_to.clone(),
            Modules::Broadcast(b) => b.send_to.clone(),
            Modules::Conjunction(c) => c.send_to.clone(),
        };
        for target in targets {
            let module = modules.get_mut(&target);
            match module {
                Some(Modules::Conjunction(c)) => {
                    c.most_recent_high.insert(name.clone(), false);
                }
                _ => (),
            }
        }
    });

    let mut starting_pulses = VecDeque::new();
    for to_send in modules.get("broadcaster").unwrap().send_to() {
        starting_pulses.push_back((to_send, false, "broadcaster".to_string()));
    }
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    // false is low, true is high
    let mut i = 0;
    let mut done = false;
    let pulses_we_care_about = match modules.get("cs") {
        Some(Modules::Conjunction(c)) => {
            c.most_recent_high.keys().fold(Vec::new(), |mut acc, s| {
                acc.push(s.clone());
                acc
            })
        }
        _ => vec![],
    };
    let mut pulses_we_care_about =
        pulses_we_care_about
            .iter()
            .fold(HashMap::new(), |mut acc, p| {
                acc.insert(p.clone(), 0);
                acc
            });

    while !done {
        i += 1;
        if i == 1001 {
            println!("part one {}", low_pulse_count * high_pulse_count);
        }
        let mut pulses_to_send = starting_pulses.clone();
        low_pulse_count += 1;
        while let Some((target, is_high, from)) = pulses_to_send.pop_front() {
            if is_high {
                high_pulse_count += 1;
            } else {
                low_pulse_count += 1;
            }
            match modules.get_mut(&target) {
                Some(Modules::Conjunction(c)) => {
                    c.most_recent_high.insert(from, is_high);
                    let next_pulse = !c.most_recent_high.values().fold(true, |acc, b| acc && *b);
                    for next in &c.send_to {
                        if next == "rx" && !next_pulse {
                            done = true;
                        }
                        if let Some(v) = pulses_we_care_about.get(&target) {
                            if v == &0 && next_pulse {
                                pulses_we_care_about.insert(target.clone(), i);
                                done = pulses_we_care_about
                                    .values()
                                    .fold(true, |acc, v| acc && v > &0);
                            }
                        }
                        pulses_to_send.push_back((next.clone(), next_pulse, target.clone()));
                    }
                }
                Some(Modules::FlipFlop(f)) => {
                    if is_high {
                        continue;
                    }
                    f.is_on = !f.is_on;
                    let pulse = f.is_on;
                    for next in &f.send_to {
                        pulses_to_send.push_back((next.clone(), pulse, target.clone()));
                    }
                }
                Some(Modules::Broadcast(b)) => {
                    for next in &b.send_to {
                        pulses_to_send.push_back((next.clone(), is_high, target.clone()));
                    }
                }
                None => (),
            }
        }
    }
    // assuming that the values are all primes so we don't have to do LCM math. Fortunately
    // that assumption is correct! but to make this more generic we'd want to be more careful
    // about finding the LCM.
    let part_two: i64 = pulses_we_care_about
        .values()
        .fold(1, |acc, v| acc * (*v as i64));

    println!("part two {part_two}");
}
