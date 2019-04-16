use std::fs::File;
// use std::io::Read;
use std::io::{BufReader, BufRead};

// pub fn read_first_line(year: u32, day: u8) -> Vec<u8> {
//     let mut r = Vec::with_capacity(10_000);
//     let path = format!("./inputs/{}/day{}.txt", year, day);
//     let fp = File::open(&path).expect(&format!("Can't Open {}", path));
//     r.extend_from_slice(
//         &fp.bytes()
//             .map(|e| e.unwrap())
//             .take_while(|e| *e != b'\n')
//             .collect::<Vec<u8>>()
//     );
//     r
// }

pub fn read_line_by_line(year: u32, day: u8) -> Vec<String> {
    let mut r = Vec::with_capacity(10_000);
    let path = format!("./inputs/{}/day{}.txt", year, day);
    let fp = File::open(&path).expect(&format!("Can't Open {}", path));
    let fp = BufReader::new(fp);
    for line in fp.lines() {
        r.push(line.unwrap());
    }
    r
}
