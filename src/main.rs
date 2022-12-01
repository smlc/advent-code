fn main() {
    let calories = include_str!("input.txt");
    let mut calories_by_elf : Vec<usize> = calories
        .split("\n\n")
        .map(|elf_calories|{
            return elf_calories
                .split("\n")
                .flat_map(str::parse::<usize>)
                .sum::<usize>();
        })
        .collect();

        calories_by_elf.sort_by(|a, b| b.cmp(a));
        //print!("{:?}", calories_by_elf);
        let max : usize = calories_by_elf.iter()
        .take(3)
        .sum();
        
        print!("Max is {max}");
}
