use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    p1(input);
    p2(input);
}

fn p1(input: &str) {
    let columns = input.lines().fold(
        HashMap::<usize, (&str, Vec<i64>)>::new(),
        |mut acc, line| {
            line.split_ascii_whitespace()
                .enumerate()
                .for_each(|(i, c)| {
                    let col = acc.entry(i).or_default();
                    if let Ok(num) = c.parse::<i64>() {
                        col.1.push(num);
                    } else {
                        col.0 = c;
                    }
                });

            acc
        },
    );
    let res: i64 = columns
        .into_values()
        .map(|(op, nums)| match op {
            "+" => nums.into_iter().reduce(|a, b| a + b).unwrap(),
            "-" => nums.into_iter().reduce(|a, b| a - b).unwrap(),
            "*" => nums.into_iter().reduce(|a, b| a * b).unwrap(),
            _ => unreachable!(),
        })
        .sum();
    println!("{res:?}");
}

fn p2(input: &str) {
    let line = input.lines().last().unwrap();
    let (widths, _) =
        line.chars()
            .enumerate()
            .fold((Vec::new(), 0), |(mut map, mut end), (i, c)| {
                if c == '+' || c == '*' {
                    if let Some((_, _, end2)) = map.last_mut() {
                        *end2 = end - 2;
                    }
                    map.push((c, end, end - 2));
                    end += 1;
                } else {
                    end += 1;
                    if line.len() - 1 == i {
                        if let Some((_, _, end2)) = map.last_mut() {
                            *end2 = end - 1;
                        }
                    }
                }
                (map, end)
            });
    let mut sum = 0;
    for (c, start, end) in widths.into_iter() {
        let columns = input
            .lines()
            .fold(HashMap::<usize, Vec<i64>>::new(), |mut acc, line| {
                let line = &line[start..=end];
                for (chi, ch) in line.chars().rev().enumerate() {
                    let chcol = acc.entry(chi).or_default();
                    if let Ok(num) = ch.to_string().parse::<i64>() {
                        chcol.push(num);
                    }
                }

                acc
            });
        let nums = columns
            .into_values()
            .map(|nums| {
                nums.iter()
                    .map(|c| c.to_string())
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let res = match c {
            '+' => nums.into_iter().sum(),
            '*' => nums.into_iter().product(),
            _ => 0,
        };
        sum += res;
    }

    println!("{sum:?}");
}
