fn main() {
    let calories = include_str!("input.txt");
    let max = calories
        .split("\n\n")
        .map(|elf_calories|{
            return elf_calories
                .split("\n")
                .flat_map(str::parse::<usize>)
                .sum::<usize>();
        })
        .max().unwrap();

        print!("Max is {max}");
}
