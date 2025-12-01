fn main() {
    let input = include_str!("./input.txt");

    p1(input);
    p2(input);
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn p1(input: &str) {
    let mut rotations = input
        .trim()
        .lines()
        .map(|l| {
            let direction = l.chars().next().unwrap();
            let distance = l[1..].parse::<i32>().unwrap();
            if direction == 'L' {
                Rotation::Left(distance)
            } else {
                Rotation::Right(distance)
            }
        })
        .collect::<Vec<_>>();

    let mut current = 50;
    let mut zeros = 0;

    for rotation in rotations {
        match rotation {
            Rotation::Left(d) => {
                // current = 15
                // d = 50
                // final = 100  - 35
                let n = d.abs() % 100;

                if current - n < 0 && current >= 0 {
                    current += 100 - n;
                } else {
                    current -= n;
                }
            }
            Rotation::Right(d) => {
                let n = d.abs() % 100;
                if current + n > 99 && current <= 99 {
                    current -= 100 - n;
                } else {
                    current += n;
                }
            }
        }

        if current == 0 {
            zeros += 1;
        }
    }
    println!("p1 {}, zeros = {}", current, zeros);
}

fn p2(input: &str) {
    let mut rotations = input
        .trim()
        .lines()
        .map(|l| {
            let direction = l.chars().next().unwrap();
            let distance = l[1..].parse::<i32>().unwrap();
            if direction == 'L' {
                Rotation::Left(distance)
            } else {
                Rotation::Right(distance)
            }
        })
        .collect::<Vec<_>>();

    let mut current = 50;
    let mut zeros = 0;

    for rotation in rotations {
        match rotation {
            Rotation::Left(d) => {
                let n = d.abs() % 100;

                if current - n < 0 && current >= 0 {
                    if current > 0 {
                        zeros += 1;
                    }
                    current += 100 - n;
                } else {
                    current -= n;
                    if current == 0 {
                        zeros += 1;
                    }
                }

                let n2 = (d as f32 / 100.).floor() as i32;

                zeros += n2;
            }
            Rotation::Right(d) => {
                let n = d.abs() % 100;
                if current + n > 99 && current <= 99 {
                    if current <= 99 {
                        zeros += 1;
                    }
                    current -= 100 - n;
                } else {
                    current += n;
                    if current == 0 {
                        zeros += 1;
                    }
                }
                let n2 = (d as f32 / 100.).floor() as i32;

                zeros += n2;
            }
        }
    }
    println!("p2 {}, zeros = {}", current, zeros);
}
