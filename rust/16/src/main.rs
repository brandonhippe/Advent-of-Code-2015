use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use regex::Regex;

fn part1(contents: String) -> i64 {
    let correct: HashMap<&str, i64> = HashMap::from_iter(
        "children: 3
        cats: 7
        samoyeds: 2
        pomeranians: 3
        akitas: 0
        vizslas: 0
        goldfish: 5
        trees: 3
        cars: 2
        perfumes: 1".lines().map(|l| {
            let mut sides = l.trim().split(": ");
            (sides.next().unwrap(), sides.next().unwrap().parse::<i64>().unwrap())
        }));

    let int_re = Regex::new(r"-?\d+").unwrap();
    let field_re = Regex::new(r"(\w+): (\d+)").unwrap();
    for (ix, line) in contents.lines().enumerate().map(|(ix, l)| ((ix + 1) as i64, l)) {
        let curr: HashMap<&str, i64> = HashMap::from_iter(
            field_re.captures_iter(line).map(|c| {
                let (_, [k, v_str]) = c.extract();
                (k, v_str.parse::<i64>().unwrap())
            })
        );

        if curr.iter().all(|(k, v)| correct.get(k).unwrap() == v){
            return ix;
        }
    }
    return -1;
}

fn part2(contents: String) -> i64 {
    let correct: HashMap<&str, i64> = HashMap::from_iter(
        "children: 3
        cats: 7
        samoyeds: 2
        pomeranians: 3
        akitas: 0
        vizslas: 0
        goldfish: 5
        trees: 3
        cars: 2
        perfumes: 1".lines().map(|l| {
            let mut sides = l.trim().split(": ");
            (sides.next().unwrap(), sides.next().unwrap().parse::<i64>().unwrap())
        }));

    let int_re = Regex::new(r"-?\d+").unwrap();
    let field_re = Regex::new(r"(\w+): (\d+)").unwrap();
    for (ix, line) in contents.lines().enumerate().map(|(ix, l)| ((ix + 1) as i64, l)) {
        let curr: HashMap<&str, i64> = HashMap::from_iter(
            field_re.captures_iter(line).map(|c| {
                let (_, [k, v_str]) = c.extract();
                (k, v_str.parse::<i64>().unwrap())
            })
        );

        if curr.iter().all(|(k, v)| {
            let comp = correct.get(k).unwrap();
            match *k {
                "cats" => v > comp,
                "trees" => v > comp,
                "pomeranians" => v < comp,
                "goldfish" => v < comp,
                _ => comp == v
            }
        }){
            return ix;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "16".to_string();

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
        "\nPart 1:\nGift from: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nGift actually from: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}