use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;

fn part1(contents: String) -> i64 {
    let reindeer_regex = Regex::new(r"\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds").unwrap();
    return contents.lines().map(|line| {
        let caps = reindeer_regex.captures(line).unwrap();
        let speed = caps[1].parse::<i64>().unwrap();
        let fly_time = caps[2].parse::<i64>().unwrap();
        let rest_time = caps[3].parse::<i64>().unwrap();
        let cycle_time = fly_time + rest_time;
        let cycles = 2503 / cycle_time;
        let rem = 2503 % cycle_time;

        speed * (fly_time * cycles + rem.min(fly_time))
    }).max().unwrap();
}

fn part2(contents: String) -> i64 {
    let reindeer_regex = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds").unwrap();
    let mut points: HashMap<String, i64> = HashMap::new();
    let mut reindeers: HashMap<String, (i64, i64, i64, i64)> = HashMap::new();

    for line in contents.lines() {
        let caps = reindeer_regex.captures(line).unwrap();
        let reindeer = caps[1].to_string();
        let speed = caps[2].parse::<i64>().unwrap();
        let fly_time = caps[3].parse::<i64>().unwrap();
        let rest_time = caps[4].parse::<i64>().unwrap();
        reindeers.insert(reindeer.clone(), (0, speed, fly_time, rest_time));
    }

    for t in 0..2503 {
        for k in reindeers.clone().keys() {
            let (mut pos, mut speed, mut fly_time, mut rest_time) = reindeers.get_mut(k).unwrap();
            if t % (fly_time + rest_time) < fly_time {
                pos += speed;
            }
            reindeers.insert((*k).to_string(), (pos, speed, fly_time, rest_time));
        }
        
        let max_dist = reindeers.values().map(|r| r.0).max().unwrap();
        for r in reindeers.keys().filter(|r| reindeers.get(&r.to_string()).unwrap().0 == max_dist) {
            *points.entry((*r).to_string()).or_insert(0) += 1;
        }
    }

    return points.into_values().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 2660);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1564);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "14".to_string();

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
        "\nPart 1:\nMax distance: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMost points: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}