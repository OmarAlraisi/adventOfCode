use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let lines = input.split('\n').into_iter();
    let mut sum = 0;
    lines.for_each(|line| {
        let first = line.chars().find(|x| x.is_digit(10));
        let last = line.chars().rev().find(|x| x.is_digit(10));

        match (first, last) {
            (Some(f), Some(l)) => {
                sum += f.to_digit(10).unwrap() * 10 + l.to_digit(10).unwrap();
            },
            _ => ()
        }
    });

    println!("{}", sum);
}
