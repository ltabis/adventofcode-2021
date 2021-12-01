use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./src/input.txt")?
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut iterator = input.into_iter().peekable();
    let mut sweep = 0;
    while iterator.peek().is_some() {
        let next = iterator.next().unwrap_or(0);
        let peek = iterator.peek().unwrap_or(&0).clone();

        sweep += [0, 1][(peek > next) as usize];
    }

    println!("number of sweeps: {}", sweep);

    Ok(())
}
