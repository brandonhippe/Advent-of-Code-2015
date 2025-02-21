use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn expand_string(string: String) -> String {
    let mut new_string = String::new();
    let mut p_char = string.chars().next().unwrap();
    let mut count = 1;
    
    for c in string.chars().skip(1) {
        if c != p_char {
            new_string.push_str(&format!("{}{}", count, p_char));
            count = 0;
            p_char = c;
        }
        count += 1;
    }
    new_string.push_str(&format!("{}{}", count, p_char));
    
    return new_string;
}

fn part1(contents: String) -> i64 {
    let mut line = contents.trim().to_string();
    for _ in 0..40 {
        line = expand_string(line);
    }
    return line.len() as i64;
}

fn part2(contents: String) -> i64 {
    let mut line = contents.trim().to_string();
    for _ in 0..50 {
        line = expand_string(line);
    }
    return line.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 0);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "10".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nLength: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLength: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}