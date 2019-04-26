// use crypto::md5::Md5;

pub fn run(input: &str) {
    println!("{}", input);
    println!("{}", solve_part1(input.as_bytes()));
}

fn solve_part1(input: &[u8]) -> u64 {
    // let mut i: u64 = 0;
    let mut buf = Vec::new();

    buf.extend_from_slice(input);

    // let md5: Md5 = Md5::new();

    1 as u64
}