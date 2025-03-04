use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use itertools::Itertools;
use mod_exp::mod_exp;

fn part1(contents: String) -> i64 {
    let loc: (i64, i64) = Regex::new(r"\d+").unwrap().find_iter(&contents).map(|m| m.as_str().parse::<i64>().unwrap()).collect_tuple().unwrap();
    let triangle_line: i64 = loc.0 + loc.1 - 1;
    let ix = (triangle_line * (triangle_line + 1) / 2) - loc.0;
    return (20151125 * mod_exp(252533, ix, 33554393)) % 33554393;
}

fn part2(contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        assert_eq!(part1("3, 2".to_string()), 8057251);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "25".to_string();

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
        "\nPart 1:\nCode to give to the machine: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}