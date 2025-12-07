use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");

    p1(input);
    p2(input);
}

fn p1(input: &str) {
    let mut marked = HashSet::new();
    let mut splits = 0;
    let lines = input.lines().collect::<Vec<_>>();
    lines.iter().enumerate().for_each(|(l, line)| {
        for (i, ch) in line.chars().enumerate() {
            if ch == 'S' {
                marked.insert((l, i));
            } else {
                let mut impacted = false;
                if l > 0 {
                    impacted = marked.contains(&(l - 1, i));
                }
                if impacted {
                    if ch == '^' {
                        let mut split = false;
                        if i > 0 {
                            marked.insert((l, i - 1));
                            split = true;
                        }
                        if i < line.len() {
                            marked.insert((l, i + 1));
                            split = true;
                        }
                        if split {
                            splits += 1;
                        }
                    } else {
                        marked.insert((l, i));
                    }
                }
            }
        }
    });
    println!("{}", splits);
}

fn p2(input: &str) {
    let mut tracker = HashMap::new();
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .iter()
        .filter(|line| line.contains("^") || line.contains("S"))
        .for_each(|line| {
            for (i, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    tracker.insert(i, 1);
                } else {
                    let entry = tracker.get(&i).cloned().unwrap_or_default();
                    if ch == '^' {
                        if i > 0 {
                            *tracker.entry(i - 1).or_default() += entry;
                        }
                        if i < line.len() {
                            *tracker.entry(i + 1).or_default() += entry;
                        }
                        tracker.insert(i, 0);
                    }
                }
            }
        });
    println!("{:#?}", tracker.into_values().sum::<i64>());
}
