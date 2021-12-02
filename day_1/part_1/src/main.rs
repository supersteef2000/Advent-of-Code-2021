use std::fs::read_to_string;

fn main() {
    let list = read_to_string("src/input").unwrap();
    let numbers = list.lines();
    let mut vec = Vec::new();
    for number in numbers {
        let number: i32 = number.parse().unwrap();
        vec.push(number);
    }
    let max = vec.len();
    let mut count = 0;
    for current in 0..max {
        if current > 0 {
            let previous = current - 1;
            if vec[current] > vec[previous] {
                count = count + 1;
            }
        }
    }
    println!("{}", count)
}