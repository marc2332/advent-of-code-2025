fn main() {
    let input = include_str!("./input.txt");

    p1(input);
    p2(input);
}

fn p1(input: &str) {
    let codes = input.trim().split(",");

    let mut invalid_sum = 0i64;

    for range in codes {
        let (first, last) = range.trim().split_once("-").unzip();
        let first = first.unwrap().parse::<i64>().unwrap();
        let last = last.unwrap().parse::<i64>().unwrap();

        for id_num in first..=last {
            let id = id_num.to_string();

            let first_half = &id[..id.len() / 2];
            let second_half = &id[id.len() / 2..];

            if first_half == second_half {
                invalid_sum += id_num;
            }
        }
    }

    println!("{invalid_sum:?}");
}

fn p2(input: &str) {
    let codes = input.trim().split(",");

    let mut invalid_sum = 0i64;

    for range in codes {
        let (first, last) = range.trim().split_once("-").unzip();
        let first = first.unwrap().parse::<i64>().unwrap();
        let last = last.unwrap().parse::<i64>().unwrap();

        for id_num in first..=last {
            let id = id_num.to_string();

            let mut carry = String::new();

            for c in id.chars() {
                carry.push(c);

                if id
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(carry.len())
                    .all(|t| carry == t.iter().collect::<String>())
                    && id.len() != carry.len()
                {
                    invalid_sum += id_num;
                    break;
                }
            }
        }
    }

    println!("{invalid_sum:?}");
}
