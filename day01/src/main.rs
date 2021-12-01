use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./src/input.txt")?
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    ex01(&input);
    ex02(&input);

    Ok(())
}

fn ex01(input: &Vec<u64>) {
    let mut iterator = input.into_iter().peekable();
    let mut increases = 0;
    while iterator.peek().is_some() {
        let next = iterator.next().unwrap_or(&0).clone();
        let peek = *iterator.peek().unwrap_or(&&0).clone();

        increases += [0, 1][(peek > next) as usize];
    }

    println!("[ex01] number of increases: {}", increases);
}

fn ex02(input: &Vec<u64>) {
    let mut iterator = input.into_iter().peekable();
    let mut increases = 0;

    let mut i0 = iterator.next().unwrap_or(&0).clone();
    let mut i1 = iterator.next().unwrap_or(&0).clone();
    let mut i2 = iterator.next().unwrap_or(&0).clone();
    let mut i3 = *iterator.peek().unwrap_or(&&0).clone();

    loop {
        let a = i0 + i1 + i2;
        let b = i1 + i2 + i3;

        increases += [0, 1][(b > a) as usize];

        i0 = i1;
        i1 = i2;
        i2 = iterator.next().unwrap_or(&0).clone();
        let peek = iterator.peek();

        i3 = if peek.is_none() {
            break;
        } else {
            *peek.unwrap().clone()
        };
    }

    println!("[ex02] number of increases: {}", increases);
}
