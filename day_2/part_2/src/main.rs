use std::fs::read_to_string;
use std::iter::StepBy;

fn main() {
    let list = read_to_string("src/input").unwrap();
    let commands = list.lines();
    let mut num_vec: Vec<i32> = Vec::new();
    let mut str_vec: Vec<&str> = Vec::new();
    for command in commands {
        let instructions = command.split_whitespace();
        let strings: StepBy<_> = instructions.clone().step_by(2);
        let numbers: StepBy<_> = instructions.skip(1).step_by(2);
        let fake_str_vec = Vec::from_iter(strings);
        let fake_num_vec = Vec::from_iter(numbers);
        for str in fake_str_vec {
            str_vec.push(str);
        }
        for num in fake_num_vec {
            let num: i32 = num.parse().unwrap();
            num_vec.push(num);
        }
    }
    let max = num_vec.len();
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for current in 0..max {
        match str_vec[current] {
            "forward" => {
                horizontal += num_vec[current];
                depth += aim * num_vec[current]
            },
            "up" => aim -= num_vec[current],
            "down" => aim += num_vec[current],
            _ => println!("something just went very wrong and it's {}'s fault", str_vec[current])
        }
    }
    println!("{}", depth * horizontal)
}