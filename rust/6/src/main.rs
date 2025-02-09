use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let int_re = Regex::new(r"(\d+)").unwrap();
    let mut lights: HashSet<(i64, i64)> = HashSet::new();
    for line in contents.lines() {
        let nums: Vec<i64> = int_re
            .captures_iter(line)
            .map(|x| x[0].parse().unwrap())
            .collect();
        let x_0 = nums[0];
        let y_0 = nums[1];
        let x_1 = nums[2];
        let y_1 = nums[3];

        for x in x_0..=x_1 {
            for y in y_0..=y_1 {
                if line.contains("turn on") {
                    lights.insert((x, y));
                } else if line.contains("turn off") {
                    lights.remove(&(x, y));
                } else {
                    if lights.contains(&(x, y)) {
                        lights.remove(&(x, y));
                    } else {
                        lights.insert((x, y));
                    }
                }
            }

        }
    }

    return lights.len() as i64;
}

fn part2(contents: String) -> i64 {
    let int_re = Regex::new(r"(\d+)").unwrap();
    let mut lights: HashMap<(i64, i64), i64> = HashMap::new();

    for line in contents.lines() {
        let nums: Vec<i64> = int_re
            .captures_iter(line)
            .map(|x| x[0].parse().unwrap())
            .collect();
        let x_0 = nums[0];
        let y_0 = nums[1];
        let x_1 = nums[2];
        let y_1 = nums[3];

        for x in x_0..=x_1 {
            for y in y_0..=y_1 {
                if line.contains("turn on") {
                    *lights.entry((x, y)).or_insert(0) += 1;
                } else if line.contains("turn off") {
                    if let Some(val) = lights.get_mut(&(x, y)) {
                        *val = (*val - 1).max(0);
                    }
                } else {
                    *lights.entry((x, y)).or_insert(0) += 2;
                }
            }
        }
    }

    return lights.values().sum::<i64>();
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
    let day = "6".to_string();

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
        "\nPart 1:\nLights on: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTotal brightness: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}