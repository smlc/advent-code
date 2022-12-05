use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let lines = include_str!("input03.txt")
            .split("\n");
    let mut priorities_total = 0;
    
           
    for line in lines {
        let chars = line.chars();
        let line_lenght = line.len();
        let mut set: HashSet<char> = HashSet::new(); 
        for (i, char) in chars.enumerate() {
            
            if i >= line_lenght / 2  {
                if  set.contains(&char) {
                    set.remove(&char);
                    println!("Contain {}", char);
                    priorities_total = char_to_int(char) + priorities_total;
                }

            } else {
                set.insert(char);
            }
        }
    }

    println!("Total priority : {}", priorities_total);
}

fn part_two() {
    let input: Vec<&str> = include_str!("input03.txt")
        .lines()
        .collect();
    let priorities_total :usize = input
        .chunks(3)
        .map(test_by_group)
        .sum();
    
    println!("Total priority : {}", priorities_total);
}

fn test_by_group(chunk: &[&str]) -> usize {
    let mut historgramme: [u8; 53] = [0; 53];
    let mut sum = 0;

    for (i, sack) in chunk.iter().enumerate() {
        for element in sack.chars() {
            let item = char_to_int(element) as usize;
            
            if i < 2 {
                // On the two first row, if we meet the char we set the bit first two bit to 1.
                historgramme[item as usize] |= 1 << i;
            } else if historgramme[item as usize] == 0b011{
                // If the two first bit is set that mean we get char on the two first row.
                println!("char {element} for {item}");
                sum += item;
                break;
            }
        }
    }

    return sum

}
fn char_to_int(c : char) -> u32 {
    let as_int = c as u32;
    if c.is_uppercase() {
        return as_int - 38; 
    } else {
        return  as_int - 96; 
    }
}