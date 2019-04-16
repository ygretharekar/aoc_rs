pub fn run(input: &[u8]){
    println!("Hello World");
    println!("{}", solve_part1(input));
    println!("{}", solve_part2(input));
}

fn solve_part1(buf: &[u8]) -> i64 {
    
    let mut floor:i64 = 0;
    
    for i in 0..buf.len() {
        if b'(' == buf[i] {
            floor += 1;
        } else if b')' == buf[i] {
            floor -= 1;
        } else {
            panic!("invalid input: {}", buf[i])
        }
    }

    floor
}

fn solve_part2(buf: &[u8]) -> i64 {
    
    let mut floor:i64 = 0;
    
    for i in 0..buf.len() {
        if b'(' == buf[i] {
            floor += 1;
        } else if b')' == buf[i] {
            floor -= 1;
        } else {
            panic!("invalid input: {}", buf[i])
        }

        if floor == -1 {
            return i as i64 + 1;
        }
    }

    return 0;
}
