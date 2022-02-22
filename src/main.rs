#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

fn print_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Direction is North!"),
        d if d == Direction::South => println!("Direction is South!"),
        d => println!("Direction is not North or South, but {:?}!", d),
    }
}

fn main() {
    let option = None.unwrap_or_else(|| {
        3.0 + 0.14
    });

    assert_eq!(option, 3.14);

    let result = "13.37".parse::<f64>()
        .unwrap_or_else(|err| {
            panic!("{:?}: Cannot parse number into a floating-point number.", err)
        });
    
    assert_eq!(result, 13.37);
}