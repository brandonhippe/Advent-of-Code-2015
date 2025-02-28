use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use itertools::Itertools;
use std::iter::zip;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
    amt: i64,
}

impl Ingredient {
    fn new(ing_str: &str) -> Ingredient {
        let int_re = Regex::new(r"-?\d+").unwrap();
        let mut ints = int_re.find_iter(ing_str);
        let mut name = ing_str.split_whitespace().next().unwrap().to_string();
        name = name[0..(name.len() - 1)].to_string();

        Ingredient {
            name: name,
            capacity: ints.next().unwrap().as_str().parse::<i64>().unwrap(),
            durability: ints.next().unwrap().as_str().parse::<i64>().unwrap(),
            flavor: ints.next().unwrap().as_str().parse::<i64>().unwrap(),
            texture: ints.next().unwrap().as_str().parse::<i64>().unwrap(),
            calories: ints.next().unwrap().as_str().parse::<i64>().unwrap(),
            amt: 1,
        }
    }

    fn score(&self) -> Vec<i64> {
        vec![self.amt * self.capacity, self.amt * self.durability, self.amt * self.flavor, self.amt * self.texture]
    }
}

fn part1(contents: String) -> i64 {
    let mut ingredients: Vec<Ingredient> = contents.lines().map(|l| Ingredient::new(l)).collect();
    let mut max_score: i64 = 0;

    for splits in (0..=100).combinations(ingredients.len() - 1) {
        let mut p_split: usize = 0;
        let mut score_vec: Vec<i64> = vec![0; 4];
        for (mut ing, split) in zip(ingredients.clone(), splits) {
            ing.amt = (split - p_split) as i64;
            score_vec = zip(score_vec, ing.score()).map(|(mut sc, ing)| {
                sc += ing;
                sc
            }).collect();
            p_split = split;
        }

        let mut ing: &mut Ingredient = ingredients.iter_mut().last().unwrap();
        ing.amt = (100 - p_split) as i64;
        score_vec = zip(score_vec, ing.score()).map(|(mut sc, ing)| {
            sc += ing;
            sc
        }).collect();

        max_score = max_score.max(score_vec.iter().map(|v| {
            if *v < 0 {
                0
            } else {
                *v
            }
        }).fold(1, |acc, v| acc * v));
    }

    return max_score;
}

fn part2(contents: String) -> i64 {
    let mut ingredients: Vec<Ingredient> = contents.lines().map(|l| Ingredient::new(l)).collect();
    let mut max_score: i64 = 0;
    
    for splits in (0..=100).combinations(ingredients.len() - 1) {
        let mut p_split: usize = 0;
        let mut score_vec: Vec<i64> = vec![0; 4];
        let mut calories: i64 = 0;

        for (mut ing, split) in zip(ingredients.clone(), splits) {
            ing.amt = (split - p_split) as i64;
            score_vec = zip(score_vec, ing.score()).map(|(mut sc, ing)| {
                sc += ing;
                sc
            }).collect();
            calories += ing.amt * ing.calories;
            p_split = split;
        }

        let mut ing: &mut Ingredient = ingredients.iter_mut().last().unwrap();
        ing.amt = (100 - p_split) as i64;
        score_vec = zip(score_vec, ing.score()).map(|(mut sc, ing)| {
            sc += ing;
            sc
        }).collect();
        calories += ing.amt * ing.calories;
        
        if calories == 500 {
            max_score = max_score.max(score_vec.iter().map(|v| {
                if *v < 0 {
                    0
                } else {
                    *v
                }
            }).fold(1, |acc, v| acc * v));
        }

    }

    return max_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 62842880);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 57600000);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "15".to_string();

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
        "\nPart 1:\nBest cookie: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nBest 500 calorie cookie: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}