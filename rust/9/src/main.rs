use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn min_dist<'a>(start: &'a str, goal: &str, distances: &HashMap<&str, HashMap<&str, i64>>, visited: &mut HashSet<&'a str>) -> i64 {
    if start == goal {
        return if visited.len() == distances.len() - 1 {0} else {i64::MAX >> 1};
    }

    visited.insert(start);

    let mut min_d: i64 = i64::MAX;
    for (k, v) in distances.get(start).unwrap().iter().filter(|(k, _)| !visited.contains(*k)) {
        min_d = min_d.min(v + min_dist(k, goal, distances, &mut visited.clone()));
    }

    visited.remove(start);
    return min_d;
}

fn part1(contents: String) -> i64 {
    let connect_re = Regex::new(r"(?P<p1>\w+) to (?P<p2>\w+) = (?P<d>\d+)").unwrap();
    let mut distances: HashMap<&str, HashMap<&str, i64>> = HashMap::new();

    for line in contents.lines() {
        let line_match = connect_re.captures(line).unwrap();
        let p1 = line_match.name("p1").unwrap().as_str();
        let p2 = line_match.name("p2").unwrap().as_str();
        let d = line_match.name("d").unwrap().as_str().parse::<i64>().unwrap();
        distances.entry(p1).or_insert(HashMap::new()).insert(p2, d);
        distances.entry(p2).or_insert(HashMap::new()).insert(p1, d);
    }

    return distances.clone().keys().combinations(2).map(|positions| {
        min_dist(*positions[0], *positions[1], &distances, &mut HashSet::new())
    }).min().unwrap();
}

fn part2(contents: String) -> i64 {
    let connect_re = Regex::new(r"(?P<p1>\w+) to (?P<p2>\w+) = (?P<d>\d+)").unwrap();
    let mut distances: HashMap<&str, HashMap<&str, i64>> = HashMap::new();

    for line in contents.lines() {
        let line_match = connect_re.captures(line).unwrap();
        let p1 = line_match.name("p1").unwrap().as_str();
        let p2 = line_match.name("p2").unwrap().as_str();
        let d = -line_match.name("d").unwrap().as_str().parse::<i64>().unwrap();
        distances.entry(p1).or_insert(HashMap::new()).insert(p2, d);
        distances.entry(p2).or_insert(HashMap::new()).insert(p1, d);
    }

    return -distances.clone().keys().combinations(2).map(|positions| {
        min_dist(*positions[0], *positions[1], &distances, &mut HashSet::new())
    }).min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 605);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 982);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "9".to_string();

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
        "\nPart 1:\nShortest traversal: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLongest traversal: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}