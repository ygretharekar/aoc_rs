use std::collections::HashMap;

pub fn run(input: &[u8]) {
    println!("{}", solve_part1(input));
}


fn solve_part1(input: &[u8]) -> i64 {
    // println!("{:?}", input);
    let mut grid = HashMap::new();
    let mut pos = (0, 0);
    let mut homes = 0;

    grid.insert(pos, true);
    homes += 1;

    for dir in input {
        match dir {
            b'^' => pos = (pos.0, pos.1 + 1),
            b'>' => pos = (pos.0 + 1, pos.1),
            b'<' => pos = (pos.0 - 1, pos.1),
            b'v' => pos = (pos.0, pos.1 - 1),
            _ => panic!("invalid input: {}", dir),
        }

        grid.entry(pos).or_insert_with(
            || {
                homes += 1;
                return true;
            }
        );
    }

    homes
}

fn solve_part2(input: &[u8]) -> i64 {
    // println!("{:?}", input);
    let mut santa_grid = HashMap::new();
    let mut robo_grid = HashMap::new();
    let mut pos = (0, 0);
    let mut santa_homes = 0;
    let mut robo_homes = 0;

    santa_grid.insert(pos, true);
    robo_grid.insert(pos, true);
    santa_homes += 1;
    robo_homes += 1;

    for i in 0..input.len() {
        if i%2 == 0 {
            match input[i] {
                b'^' => pos = (pos.0, pos.1 + 1),
                b'>' => pos = (pos.0 + 1, pos.1),
                b'<' => pos = (pos.0 - 1, pos.1),
                b'v' => pos = (pos.0, pos.1 - 1),
                _ => panic!("invalid input: {}", input[i])
            }

            santa_grid.entry(pos).or_insert_with(
                || {
                    santa_homes += 1;
                    return true;
                }
            );
        } else {
            match input[i] {
                b'^' => pos = (pos.0, pos.1 + 1),
                b'>' => pos = (pos.0 + 1, pos.1),
                b'<' => pos = (pos.0 - 1, pos.1),
                b'v' => pos = (pos.0, pos.1 - 1),
                _ => panic!("invalid input: {}", input[i])
            }

            robo_grid.entry(pos).or_insert_with(
                || {
                    robo_homes += 1;
                    return true;
                }
            );
        }
    }

    santa_homes
}

