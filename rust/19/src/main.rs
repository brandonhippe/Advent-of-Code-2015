use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn reverse_molecule(molecule: String, backtracks: Vec<(String, String)>) -> Option<i64> {
    if molecule == "e".to_string() {
        return Some(0);
    }

    for (k, v) in &backtracks {
        if molecule.contains(k) {
            if let Some(s) = reverse_molecule(molecule.replacen(k, &v, 1), backtracks.clone()) {
                return Some(s + 1);
            }
        }
    }

    return None;
}

fn part1(contents: String) -> i64 {
    let replace_re = Regex::new(r"(\w+) => (\w+)").unwrap();
    let mut start_molecule: Option<String> = None;
    let mut replace_mapping: HashMap<String, Vec<String>> = HashMap::new();
    for line in contents.lines().filter_map(|l| {let line = l.trim(); if line.len() == 0 {None} else {Some(line)}}) {
        if replace_re.is_match(line) {
            let caps = replace_re.captures(line).unwrap();
            replace_mapping.entry(caps.get(1).unwrap().as_str().to_string()).or_insert(Vec::new()).push(caps.get(2).unwrap().as_str().to_string());
        } else {
            start_molecule = Some(line.to_string());
        }
    }

    assert!(start_molecule.is_some());
    let molecule: String = start_molecule.unwrap();
    let mut pos_molecules: HashSet<String> = HashSet::new();
    let replace_mol_re = Regex::new(&replace_mapping.keys().fold("".to_string(), |re, k| {
        if re.len() > 0 {
            format!("{}|{}", re, k)
        } else {
            k.to_string()
        }
    })).unwrap();

    for ix in (0..molecule.len()).filter(|ix| replace_mol_re.is_match_at(&molecule, *ix)) {
        let match_str = replace_mol_re.find_at(&molecule, ix).unwrap();
        for repl in replace_mapping.get(match_str.as_str()).unwrap() {
            pos_molecules.insert({
                format!("{}{}{}", molecule[..match_str.start()].to_string(), repl, molecule[match_str.end()..].to_string())
            });
        }
    }
    return pos_molecules.len() as i64;
}

fn part2(contents: String) -> i64 {
    let replace_re = Regex::new(r"(\w+) => (\w+)").unwrap();
    let mut goal_molecule: Option<String> = None;
    let mut backtracks: Vec<(String, String)> = Vec::new();
    for line in contents.lines().filter_map(|l| {let line = l.trim(); if line.len() == 0 {None} else {Some(line)}}) {
        if replace_re.is_match(line) {
            let caps = replace_re.captures(line).unwrap();
            backtracks.push((caps.get(2).unwrap().as_str().to_string(), caps.get(1).unwrap().as_str().to_string()));
        } else {
            goal_molecule = Some(line.to_string());
        }
    }
    
    assert!(goal_molecule.is_some());
    backtracks.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    return reverse_molecule(goal_molecule.unwrap(), backtracks).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 4);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 3);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "19".to_string();

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
        "\nPart 1:\nMolecules that can be created: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSteps to make molecule: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}