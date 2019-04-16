pub fn run(input: Vec<String>){
    let part1: u64 = solve_part1(&input);
    println!("{}", part1);
    
    let (part2a, part2b): (u64, u64)  = solve_part2(input);
    println!("{}, {}", part2a, part2b);
}

fn solve_part1(input: &Vec<String>) -> u64 {

    let mut sum: u64 = 0;

    for line in input.iter() {
        let str_num: Vec<&str> = line.split("x").collect();
        let mut str_num: Vec<u64> = str_num.iter().map(|e| e.parse().unwrap()).collect();
        str_num.sort();

        assert_eq!(str_num.len(), 3);

        sum += 3*str_num[0]*str_num[1] + 2*str_num[0]*str_num[2] + 2*str_num[1]*str_num[2];
    }

    sum
}

fn solve_part2(input: Vec<String>) -> (u64, u64) {

    let mut sum: u64 = 0;
    let mut cube: u64 = 0;

    for line in input.iter() {
        let str_num: Vec<&str> = line.split("x").collect();
        let mut str_num: Vec<u64> = str_num.iter().map(|e| e.parse().unwrap()).collect();
        str_num.sort();

        assert_eq!(str_num.len(), 3);

        sum += 2*str_num[0] + 2*str_num[1];
        cube = str_num[0] * str_num[1] * str_num[2];
        sum += cube;
    }

    (sum, cube)
}