use std::collections::HashMap;

fn main() {
    let input = load_input("./src/input.txt").expect("couldn't parse input.");

    println!("ex01: {}", find_sum_of_risk_levels(&input));
}

#[derive(Debug)]
struct Map {
    grid: HashMap<Point, u8>,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn find_sum_of_risk_levels(map: &Map) -> usize {
    map.grid.iter().fold(0, |acc, (point, depth)| {
        acc + if is_lowest_point(map, point, *depth) {
            compute_risk(*depth)
        } else {
            0
        } as usize
    })
}

fn is_lowest_point(map: &Map, point: &Point, depth: u8) -> bool {
    find_adgacent_locations(map, point)
        .iter()
        .all(|(_, d)| depth < **d)
}

fn find_adgacent_locations<'a>(map: &'a Map, point: &'a Point) -> Vec<(&'a Point, &'a u8)> {
    let mut adgacent = Vec::with_capacity(4);

    if point.x < map.max_x - 1 {
        adgacent.push(
            map.grid
                .get_key_value(&Point {
                    x: point.x + 1,
                    y: point.y,
                })
                .unwrap(),
        );
    }
    if point.x != 0 {
        adgacent.push(
            map.grid
                .get_key_value(&Point {
                    x: point.x - 1,
                    y: point.y,
                })
                .unwrap(),
        );
    }
    if point.y < map.max_y - 1 {
        adgacent.push(
            map.grid
                .get_key_value(&Point {
                    x: point.x,
                    y: point.y + 1,
                })
                .unwrap(),
        );
    }
    if point.y != 0 {
        adgacent.push(
            map.grid
                .get_key_value(&Point {
                    x: point.x,
                    y: point.y - 1,
                })
                .unwrap(),
        );
    }

    adgacent
}

fn compute_risk(depth: u8) -> u8 {
    depth + 1
}

fn load_input(src: &str) -> std::io::Result<Map> {
    let input = std::fs::read_to_string(src)?;
    Ok(Map {
        grid: input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Point { x, y }, c.to_digit(10).unwrap() as u8))
            })
            .collect(),
        max_x: input.lines().next().unwrap().len(),
        max_y: input.lines().count(),
    })
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_load_input() {
        let input = load_input("./src/test.txt").expect("couldn't parse input.");

        println!("{:?}", input);
    }

    #[test]
    fn test_example() {
        let input = load_input("./src/test.txt").expect("couldn't parse input.");

        assert_eq!(find_sum_of_risk_levels(&input), 15);
    }
}
