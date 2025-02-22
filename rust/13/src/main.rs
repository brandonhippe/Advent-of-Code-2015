use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

fn max_happiness(changes: &HashMap<&str, HashMap<&str, i64>>) -> i64 {
    let mut happiness: i64 = 0;
    let mut to_check: VecDeque<VecDeque<&str>> = VecDeque::from([VecDeque::from([*changes.clone().keys().next().unwrap()])]);

    while let Some(mut order) = to_check.pop_front() {
        let last_person = order.pop_back().unwrap();
        if order.len() == changes.len() - 1 {
            // Calculate happiness
            order.push_back(last_person);
            let mut ord_happiness: i64 = 0;

            loop {
                ord_happiness += changes.get(order[0]).unwrap().get(order[1]).unwrap();
                ord_happiness += changes.get(order[1]).unwrap().get(order[0]).unwrap();

                if order[0] == last_person {
                    break;
                }
                order.rotate_left(1);
            }

            happiness = happiness.max(ord_happiness);
            continue;
        }

        for next in changes.get(last_person).unwrap().keys().filter(|p| !order.contains(*p)) {
            let mut new_order = order.clone();
            new_order.push_back(last_person);
            new_order.push_back(next);
            to_check.push_back(new_order);
        }
    }

    return happiness;
}

fn part1(contents: String) -> i64 {
    let seat_regex = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (?P<p2>\w+)").unwrap();
    let mut changes: HashMap<&str, HashMap<&str, i64>> = HashMap::new();
    for (_, [p1, change_dir, units, p2]) in seat_regex.captures_iter(&contents).map(|c| c.extract()) {
        let change: i64 = match change_dir {
            "gain" => units.parse::<i64>().unwrap(),
            "lose" => -units.parse::<i64>().unwrap(),
            _ => panic!("Unknown happiness change")
        };

        changes.entry(p1).or_insert(HashMap::new()).insert(p2, change);
    }

    return max_happiness(&changes);
}

fn part2(contents: String) -> i64 {
    let seat_regex = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (?P<p2>\w+)").unwrap();
    let mut changes: HashMap<&str, HashMap<&str, i64>> = HashMap::new();
    for (_, [p1, change_dir, units, p2]) in seat_regex.captures_iter(&contents).map(|c| c.extract()) {
        let change: i64 = match change_dir {
            "gain" => units.parse::<i64>().unwrap(),
            "lose" => -units.parse::<i64>().unwrap(),
            _ => panic!("Unknown happiness change")
        };

        changes.entry(p1).or_insert(HashMap::new()).insert(p2, change);
    }

    for k in changes.clone().keys() {
        changes.entry(k).or_insert(HashMap::new()).insert("you", 0);
        changes.entry("you").or_insert(HashMap::new()).insert(k, 0);
    }

    return max_happiness(&changes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 330);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "13".to_string();

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
        "\nPart 1:\nBest arrangement: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nBest arrangement: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}