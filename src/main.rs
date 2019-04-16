mod aoc2015;
mod commons;

use commons::read_input;

fn main() {
    aoc2015::day1::run(&read_input::read_first_line(2015, 1));
}
