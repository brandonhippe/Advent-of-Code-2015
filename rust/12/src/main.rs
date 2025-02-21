use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;

fn json_sum(parse: &json::JsonValue) -> i64 {
    let obj_map: HashMap<&str, &json::JsonValue> = HashMap::from_iter(parse.entries());
    if obj_map.into_values().filter(|v| *v == "red").collect::<Vec<_>>().len() != 0 {
        return 0;
    }
    
    if let Some(val) = parse.as_i64() {
        return val;
    }

    let mut total: i64 = 0;
    for (_k, v) in parse.entries() {
        total += json_sum(v);
    }
    for v in parse.members() {
        total += json_sum(v);
    }

    return total;
}

fn part1(contents: String) -> i64 {
    let num_regex = Regex::new(r"-?\d+").unwrap();
    return num_regex.find_iter(&contents).map(|m| m.as_str().parse::<i64>().unwrap()).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    return json_sum(&json::parse(&contents).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 6);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 4);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "12".to_string();

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
        "\nPart 1:\nSum: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of non-red: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}