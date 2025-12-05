use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Solution {}", solve(&input));
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn solve(input: &str) -> i16 {
    let actions = input.lines().map(|action| (&action[..1], &action[1..]));

    let mut state = 50;
    let mut password = 0;

    for (direction, distance) in actions {
        let distance: i16 = distance.parse().unwrap();
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction value!"),
        };
        state = rotate(state, direction, distance);
        println!("state {state}, {direction:?}, {distance}");
        if state == 0 {
            password += 1;
        }
    }
    password
}

fn rotate(state: i16, direction: Direction, distance: i16) -> i16 {
    match direction {
        Direction::Left => {
            let value = state - distance;
            // This works for the small inputs we get
            // if value < 0 { 99 % value.abs() } else { value }
            value % 100
        }
        Direction::Right => {
            let value = state + distance;
            value % 100
            // if value > 99 { value % 100 } else { value }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Direction, rotate, solve};

    const SAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_sample_data() {
        let solution = solve(SAMPLE_INPUT);
        assert_eq!(solution, 3);
    }

    #[test]
    fn test_rotate() {
        assert_eq!(rotate(11, Direction::Right, 8), 19);
    }

    #[test]
    fn test_rotate_underflow() {
        assert_eq!(rotate(5, Direction::Left, 101), 3);
    }

    #[test]
    fn test_rotate_big_underflow() {
        assert_eq!(rotate(5, Direction::Left, 109), 96);
    }

    #[test]
    fn test_rotate_overflow() {
        assert_eq!(rotate(99, Direction::Right, 4), 3);
    }
}
