use std::cmp;


fn main() {
    part_one();
    
    part_two();
}
fn part_one() {
    let mut char_position: [usize; 26] = [5000; 26];

    let mut windows_start = 0;
    let mut windows_end = 0;

    let input_string = include_str!("input06.txt");
    for (i, c) in input_string.bytes().enumerate() {
        if char_position[(c - b'a') as usize] == 5000 {
            char_position[(c - b'a') as usize] = i;
        } else {
            windows_start = cmp::max(windows_start, char_position[(c - b'a') as usize] + 1);
            char_position[(c - b'a') as usize] = i;
        }
        
        windows_end = i;

        if windows_end - windows_start == 4 {
            println!("Position {}", i);
            break;
        }
    }

    let character_found = &input_string[windows_start..windows_end];
    println!("Result : {}", character_found)
}
fn part_two() {
    let mut char_position: [usize; 26] = [5000; 26];

    let mut windows_start = 0;
    let mut windows_end = 0;

    let input_string = include_str!("input06.txt");
    for (i, c) in input_string.bytes().enumerate() {
        if char_position[(c - b'a') as usize] == 5000 {
            char_position[(c - b'a') as usize] = i;
        } else {
            windows_start = cmp::max(windows_start, char_position[(c - b'a') as usize] + 1);
            char_position[(c - b'a') as usize] = i;
        }
    
        windows_end = i;
        if windows_end - windows_start + 1 == 14 {
            println!("Position {}", i + 1);
            break;
        }
        
    }

    let character_found = &input_string[windows_start..windows_end];
    println!("Result : {}", character_found)
}