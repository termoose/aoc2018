use std::ops::Rem;

const INPUT: usize = 556061;

fn get_sum(input: &Vec<u32>, first: u32, second: u32) -> u32 {
    let length = input.len() as u32;
    input[Rem::rem(first, length) as usize] + input[Rem::rem(second, length) as usize]
}

fn get_val(input: &Vec<u32>, elf: u32) -> u32 {
    let length = input.len() as u32;
    input[Rem::rem(elf, length) as usize]
}

fn to_digits(input: u32) -> Vec<u32> {
    input.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}

fn main() {
    let mut recipes: Vec<u32> = vec![3, 7];
    let mut elf1: u32 = 0;
    let mut elf2: u32 = 1;

    let input_digits = &to_digits(INPUT as u32)[..];

    loop {
        let sum = get_sum(&recipes, elf1, elf2);
        let sum_digits = to_digits(sum);

        // Add recepies
        recipes.extend(sum_digits);

        // Move elfs
        elf1 = Rem::rem(elf1 + 1 + get_val(&recipes, elf1), recipes.len() as u32);
        elf2 = Rem::rem(elf2 + 1 + get_val(&recipes, elf2), recipes.len() as u32);

        //println!("Len: {}", recipes.len());
        // if recipes.len() >= INPUT + 10 {
        //     println!("Recipes: {}", recipes.len());
        //     println!("{} {}: {:?}", elf1, elf2, &recipes[INPUT..]);

        if recipes.len() > 15 {
            let padded_slice = &recipes[(recipes.len()-7)..];
            
            let pos = padded_slice
                .windows(input_digits.len())
                .position(|w| w == input_digits);
            match pos {
                Some(n) => {
                    println!("Len: {} Pos: {}: {:?}", recipes.len(), n, padded_slice);
                    println!("Index: {}", recipes.len() + n);
                    break;
                }
                None => {}
            }
        }
        //     break;
        // }
        //println!("{} {}: {:?}", elf1, elf2, recipes);
    }
    // let vector = vec![1,2,3,4,5,6];
    // println!("Slice: {:?}", &vector[1..5]);
    // let digits = to_digits(18);
    // recipes.extend(digits);
    // println!("Recipes: {:?}", recipes);
    // println!("Sum: {}", get_sum(&recipes, elf1, elf2));
    // println!("Digits: {:?}", to_digits(18));
}
