fn main() {
    let input = include_str!("./input.txt");

    p1(input);
    p2(input);
}

fn p1(input: &str) {
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut res = 0;

    for (x, row) in rows.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == '@' {
                let map: [(isize, isize); 8] = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ];

                let mut count = 0;

                for (x_o, y_o) in map {
                    let Some(x) = (x as isize).checked_add(x_o) else {
                        continue;
                    };
                    let Some(y) = (y as isize).checked_add(y_o) else {
                        continue;
                    };
                    if let Some(c) = rows.get(x as usize).and_then(|row| row.get(y as usize))
                        && *c == '@'
                    {
                        count += 1;
                    }
                }

                if count < 4 {
                    res += 1;
                }
            }
        }
    }

    println!("p1: {res}");
}

fn p2(input: &str) {
    let mut raw_rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut res = 0;

    loop {
        let mut res_iter = 0;
        let rows = raw_rows.clone();

        for (x, row) in rows.iter().enumerate() {
            for (y, c) in row.iter().enumerate() {
                if *c == '@' {
                    let map: [(isize, isize); 8] = [
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ];

                    let mut count = 0;

                    for (x_o, y_o) in map {
                        let Some(x) = (x as isize).checked_add(x_o) else {
                            continue;
                        };
                        let Some(y) = (y as isize).checked_add(y_o) else {
                            continue;
                        };
                        if let Some(c) = rows.get(x as usize).and_then(|row| row.get(y as usize))
                            && *c == '@'
                        {
                            count += 1;
                        }
                    }

                    if count < 4 {
                        res_iter += 1;
                        raw_rows[x][y] = 'x';
                    }
                }
            }
        }

        res += res_iter;
        if res_iter == 0 {
            break;
        }
    }

    println!("p2: {res}");
}
