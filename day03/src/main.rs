fn main() {
    let input = include_str!("./input.txt");

    p1(input);
    p2(input);
}

fn p1(input: &str) {
    let banks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0i32;

    for bank in banks {
        let mut candidate = 0;
        for (i, battery) in bank.iter().enumerate() {
            for other_battery in bank[i + 1..].iter() {
                let result = format!("{battery}{other_battery}").parse::<i32>().unwrap();
                if result > candidate {
                    candidate = result;
                }
            }
        }
        sum += candidate;
    }

    println!("p1: {sum}");
}

fn p2(input: &str) {
    let banks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    // With some help from https://github.com/aarroyoc
    let res: i64 = banks
        .into_iter()
        .map(|bank| {
            let mut consumed = 0;
            let mut n = 0;
            for d in 1..=12 {
                let max_digit = bank[consumed..bank.len() - (12 - d)].iter().max().unwrap();
                consumed += bank[consumed..]
                    .iter()
                    .position(|c| c == max_digit)
                    .unwrap()
                    + 1;
                n = n * 10 + max_digit;
            }
            n
        })
        .sum();

    println!("p2: {res:?}");
}
