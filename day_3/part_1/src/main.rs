use std::fs::read_to_string;

fn main() {
    let list = read_to_string("src/input").unwrap();
    let numbers = list.lines();
    let mut string = String::new();
    let mut inv_string = String::new();
    let mut vec0 = Vec::new();
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut vec3 = Vec::new();
    let mut vec4 = Vec::new();
    let mut vec5 = Vec::new();
    let mut vec6 = Vec::new();
    let mut vec7 = Vec::new();
    let mut vec8 = Vec::new();
    let mut vec9 = Vec::new();
    let mut vec10 = Vec::new();
    let mut vec11 = Vec::new();
    for number in numbers {
        let chars = number.chars();
        let vec = Vec::from_iter(chars);
        vec0.push(vec[0]);
        vec1.push(vec[1]);
        vec2.push(vec[2]);
        vec3.push(vec[3]);
        vec4.push(vec[4]);
        vec5.push(vec[5]);
        vec6.push(vec[6]);
        vec7.push(vec[7]);
        vec8.push(vec[8]);
        vec9.push(vec[9]);
        vec10.push(vec[10]);
        vec11.push(vec[11]);
    }
    string.push(which_is_it_bro_come_on_tell_me(vec0.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec1.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec2.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec3.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec4.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec5.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec6.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec7.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec8.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec9.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec10.clone()));
    string.push(which_is_it_bro_come_on_tell_me(vec11.clone()));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec0)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec1)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec2)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec3)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec4)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec5)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec6)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec7)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec8)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec9)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec10)));
    inv_string.push(mr_gorbachev_invert_this_char(which_is_it_bro_come_on_tell_me(vec11)));
    let gamma = i32::from_str_radix(&*string, 2).unwrap();
    let epsilon = i32::from_str_radix(&*inv_string, 2).unwrap();
    println!("{}", gamma * epsilon);
}

fn which_is_it_bro_come_on_tell_me(vec: Vec<char>) -> char {
    let mut ones = 0;
    let mut zeroes = 0;
    let result: char;
    for char in vec {
        if char == '1' {
            ones += 1;
        } else {
            zeroes += 1;
        }
    }
    if ones > zeroes {
        result = '1';
    } else {
        result = '0';
    }
    return result;
}

fn mr_gorbachev_invert_this_char(char: char) -> char {
    let new_char: char;
    if char == '1' {
        new_char = '0';
    } else {
        new_char = '1';
    }
    return new_char;
}