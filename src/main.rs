fn main() {
    let calories = include_str!("input.txt");
    let mut max = 0;
    let elf_calories = calories
        .split("\n\n");
//        .flat_map(|x| x.split("\n"));
        for t in elf_calories {

            let numbers = t.split("\n");
            let mut count_elf = 0;
            for number in numbers {
                count_elf = &number.parse().unwrap() + count_elf;
            }
            if count_elf > max {
                max = count_elf;
            }
        }

        print!("Max is {max}");
}
