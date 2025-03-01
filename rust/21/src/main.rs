use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct Item {
    cost: i64,
    damage: f64,
    armor: f64,
    name: String,
}

impl Item {
    fn new(item_str: &str) -> Item {
        let split_re = Regex::new(r"\s\s+").unwrap();
        let mut splits = split_re.split(item_str).map(|l| l.trim());
        Item {
            name: splits.next().unwrap().to_string(),
            cost: splits.next().unwrap().parse::<i64>().unwrap(),
            damage: splits.next().unwrap().parse::<f64>().unwrap(),
            armor: splits.next().unwrap().parse::<f64>().unwrap(),
        }
    }
}


fn shop() -> Vec<Vec<Item>> {
    let shop_strs: Vec<String> = Vec::from_iter("
        Weapons:    Cost  Damage  Armor
        Dagger        8     4       0
        Shortsword   10     5       0
        Warhammer    25     6       0
        Longsword    40     7       0
        Greataxe     74     8       0

        Armor:      Cost  Damage  Armor
        Leather      13     0       1
        Chainmail    31     0       2
        Splintmail   53     0       3
        Bandedmail   75     0       4
        Platemail   102     0       5

        Rings:      Cost  Damage  Armor
        Damage +1    25     1       0
        Damage +2    50     2       0
        Damage +3   100     3       0
        Defense +1   20     0       1
        Defense +2   40     0       2
        Defense +3   80     0       3
    ".lines().filter_map(|l| {
        let line = l.trim().to_string();
        if line.len() > 0 {
            Some(line)
        } else {
            None
        }
    }));

    let mut items: Vec<Vec<Item>> = Vec::new();
    for line in shop_strs {
        if line.contains(':') {
            items.push(Vec::new());
        } else {
            items.iter_mut().last().unwrap().push(Item::new(&line));
        }
    }

    return items;
}

#[derive(Debug)]
struct Player {
    hp: f64,
    damage: f64,
    armor: f64,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            hp: 100.0,
            damage: 0.0,
            armor: 0.0,
        }
    }
}

fn sim_fight(player: &Player, boss: &Player) -> bool {
    let mut hp = player.hp;
    let mut boss_hp = boss.hp;

    while hp > 0.0 {
        boss_hp -= 1_f64.max(player.damage - boss.armor);
        if boss_hp <= 0.0 {
           break;
        }
        hp -= 1_f64.max(boss.damage - player.armor);
    }

    hp > 0.0
}

fn part1(contents: String) -> i64 {
    let shop = shop();
    let boss: Player = {
        let mut stats = contents.lines().filter_map(|l| {
            let line = l.trim();
            if line.len() > 0 {
                Some(line.split_whitespace().last().unwrap().parse::<f64>().unwrap())
            } else {
                None
            }
        });
        Player {
            hp: stats.next().unwrap(),
            damage: stats.next().unwrap(),
            armor: stats.next().unwrap(),
        }
    };
    
    let mut least_gold: i64 = i64::MAX;
    for weapon in &shop[0] {
        for armor in &shop[1] {
            for rings in shop[2].iter().combinations(2) {
                let mut cost = weapon.cost;
                let mut player = Player {
                    damage: weapon.damage,
                    ..Default::default()
                };
                if sim_fight(&player, &boss) {
                    least_gold = least_gold.min(cost);
                }

                cost += armor.cost;
                player.armor += armor.armor;
                if sim_fight(&player, &boss) {
                    least_gold = least_gold.min(cost);
                }
                
                for ring in &rings {
                    cost += ring.cost;
                    player.damage += ring.damage;
                    player.armor += ring.armor;
                    if sim_fight(&player, &boss) {
                        least_gold = least_gold.min(cost);
                    }
                    cost -= ring.cost;
                    player.damage -= ring.damage;
                    player.armor -= ring.armor;
                }

                cost += rings.iter().map(|r| r.cost).sum::<i64>();
                player.damage += rings.iter().map(|r| r.damage).sum::<f64>();
                player.armor += rings.iter().map(|r| r.armor).sum::<f64>();
                if sim_fight(&player, &boss) {
                    least_gold = least_gold.min(cost);
                }
                cost -= rings.iter().map(|r| r.cost).sum::<i64>();
                player.damage -= rings.iter().map(|r| r.damage).sum::<f64>();
                player.armor -= rings.iter().map(|r| r.armor).sum::<f64>();

                cost -= armor.cost;
                player.armor -= armor.armor;
                
                for ring in &rings {
                    cost += ring.cost;
                    player.damage += ring.damage;
                    player.armor += ring.armor;
                    if sim_fight(&player, &boss) {
                        least_gold = least_gold.min(cost);
                    }
                    cost -= ring.cost;
                    player.damage -= ring.damage;
                    player.armor -= ring.armor;
                }

                cost += rings.iter().map(|r| r.cost).sum::<i64>();
                player.damage += rings.iter().map(|r| r.damage).sum::<f64>();
                player.armor += rings.iter().map(|r| r.armor).sum::<f64>();
                if sim_fight(&player, &boss) {
                    least_gold = least_gold.min(cost);
                }
            }
        } 
    }

    return least_gold;
}

fn part2(contents: String) -> i64 {
    let shop = shop();
    let boss: Player = {
        let mut stats = contents.lines().filter_map(|l| {
            let line = l.trim();
            if line.len() > 0 {
                Some(line.split_whitespace().last().unwrap().parse::<f64>().unwrap())
            } else {
                None
            }
        });
        Player {
            hp: stats.next().unwrap(),
            damage: stats.next().unwrap(),
            armor: stats.next().unwrap(),
        }
    };
    
    let mut most_gold: i64 = 0;
    for weapon in &shop[0] {
        for armor in &shop[1] {
            for rings in shop[2].iter().combinations(2) {
                let mut cost = weapon.cost;
                let mut player = Player {
                    damage: weapon.damage,
                    ..Default::default()
                };
                if !sim_fight(&player, &boss) {
                    most_gold = most_gold.max(cost);
                }

                cost += armor.cost;
                player.armor += armor.armor;
                if !sim_fight(&player, &boss) {
                    most_gold = most_gold.max(cost);
                }
                
                for ring in &rings {
                    cost += ring.cost;
                    player.damage += ring.damage;
                    player.armor += ring.armor;
                    if !sim_fight(&player, &boss) {
                        most_gold = most_gold.max(cost);
                    }
                    cost -= ring.cost;
                    player.damage -= ring.damage;
                    player.armor -= ring.armor;
                }

                cost += rings.iter().map(|r| r.cost).sum::<i64>();
                player.damage += rings.iter().map(|r| r.damage).sum::<f64>();
                player.armor += rings.iter().map(|r| r.armor).sum::<f64>();
                if !sim_fight(&player, &boss) {
                    most_gold = most_gold.max(cost);
                }
                cost -= rings.iter().map(|r| r.cost).sum::<i64>();
                player.damage -= rings.iter().map(|r| r.damage).sum::<f64>();
                player.armor -= rings.iter().map(|r| r.armor).sum::<f64>();

                cost -= armor.cost;
                player.armor -= armor.armor;
                
                for ring in &rings {
                    cost += ring.cost;
                    player.damage += ring.damage;
                    player.armor += ring.armor;
                    if !sim_fight(&player, &boss) {
                        most_gold = most_gold.max(cost);
                    }
                    cost -= ring.cost;
                    player.damage -= ring.damage;
                    player.armor -= ring.armor;
                }

                cost += rings.iter().map(|r| r.cost).sum::<i64>();
                player.damage += rings.iter().map(|r| r.damage).sum::<f64>();
                player.armor += rings.iter().map(|r| r.armor).sum::<f64>();
                if !sim_fight(&player, &boss) {
                    most_gold = most_gold.max(cost);
                }
            }
        } 
    }

    return most_gold;
}

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "21".to_string();

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
        "\nPart 1:\nLeast cost to win: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMost cost to lose: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}