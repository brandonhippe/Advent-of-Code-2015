use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Player {
    hp: i64,
    mana: i64,
    damage: i64,
    armor: i64,
    effects: VecDeque<Vec<String>>,
    mana_spent: i64,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            hp: 50,
            mana: 500,
            damage: 0,
            armor: 0,
            effects: VecDeque::new(),
            mana_spent: 0,
        }
    }
}

impl Player {
    fn apply_effects(&mut self) -> bool {
        if self.hp <= 0 {
            return true;
        }
        for to_apply in &self.effects {
            for effect in to_apply {
                match effect.as_str() {
                    "Missile" => self.hp -= 4,
                    "Drain" => {
                        if self.damage == 0 {
                            self.hp += 2;
                        } else {
                            self.hp -= 2;
                        }
                    },
                    "Shield" => self.armor += 7,
                    "Poison" => self.hp -= 3,
                    "Recharge" => self.mana += 101,
                    _ => panic!("Unknown effect!")
                }
            }
        }

        self.effects.pop_front();

        return self.hp <= 0;
    }
}

fn wizard_combat(contents: String, p2: bool) -> i64 {
    let player = Player {..Default::default()};
    let boss = Player {
        hp: contents.lines().next().unwrap().split_whitespace().last().unwrap().parse::<i64>().unwrap(),
        damage: contents.lines().skip(1).next().unwrap().split_whitespace().last().unwrap().parse::<i64>().unwrap(),
        ..Default::default()
    };

    let mut least_mana = i64::MAX;
    let mut test_states: VecDeque<(i64, Player, Player)> = VecDeque::from([(0, player.clone(), boss.clone())]);

    while let Some((turn_ix, mut player, mut boss)) = test_states.pop_front() {
        player.hp -= ((turn_ix % 2 == 0) && p2) as i64;
        if player.apply_effects() {
            continue;
        }
        if boss.apply_effects() {
            least_mana = least_mana.min(player.mana_spent);
            continue;
        }
        if player.mana_spent >= least_mana {
            continue;
        }
        
        if turn_ix % 2 == 0 {
            // Player turn
            if player.mana >= 53 && !boss.effects.iter().any(|t| t.contains(&"Missile".to_string())) {
                // Missile
                let mut new_player_effects: VecDeque<Vec<String>> = player.effects.clone();
                let mut new_boss_effects: VecDeque<Vec<String>> = boss.effects.clone();
                while new_boss_effects.len() < 1 {
                    new_boss_effects.push_back(Vec::new());
                }
                new_boss_effects[0].push("Missile".to_string());

                test_states.push_back((
                    turn_ix + 1,
                    Player {
                        hp: player.hp,
                        mana: player.mana - 53,
                        mana_spent: player.mana_spent + 53,
                        effects: new_player_effects,
                        ..Default::default()
                    },
                    Player {
                        hp: boss.hp,
                        damage: boss.damage,
                        effects: new_boss_effects,
                        ..Default::default()
                    }
                ));
            }
            
            if player.mana >= 73 && !boss.effects.iter().any(|t| t.contains(&"Drain".to_string())) {
                // Drain
                let mut new_player_effects: VecDeque<Vec<String>> = player.effects.clone();
                let mut new_boss_effects: VecDeque<Vec<String>> = boss.effects.clone();
                while new_player_effects.len() < 1 {
                    new_player_effects.push_back(Vec::new());
                }
                new_player_effects[0].push("Drain".to_string());
                while new_boss_effects.len() < 1 {
                    new_boss_effects.push_back(Vec::new());
                }
                new_boss_effects[0].push("Drain".to_string());

                test_states.push_back((
                    turn_ix + 1,
                    Player {
                        hp: player.hp,
                        mana: player.mana - 73,
                        mana_spent: player.mana_spent + 73,
                        effects: new_player_effects,
                        ..Default::default()
                    },
                    Player {
                        hp: boss.hp,
                        damage: boss.damage,
                        effects: new_boss_effects,
                        ..Default::default()
                    }
                ));
            }
            
            if player.mana >= 113 && !player.effects.iter().any(|t| t.contains(&"Shield".to_string())) {
                // Shield
                let mut new_player_effects: VecDeque<Vec<String>> = player.effects.clone();
                let mut new_boss_effects: VecDeque<Vec<String>> = boss.effects.clone();
                while new_player_effects.len() < 6 {
                    new_player_effects.push_back(Vec::new());
                }
                new_player_effects[5].push("Shield".to_string());

                test_states.push_back((
                    turn_ix + 1,
                    Player {
                        hp: player.hp,
                        mana: player.mana - 113,
                        mana_spent: player.mana_spent + 113,
                        effects: new_player_effects,
                        ..Default::default()
                    },
                    Player {
                        hp: boss.hp,
                        damage: boss.damage,
                        effects: new_boss_effects,
                        ..Default::default()
                    }
                ));
            }

            if player.mana >= 173 && !boss.effects.iter().any(|t| t.contains(&"Poison".to_string())) {
                // Poison
                let mut new_player_effects: VecDeque<Vec<String>> = player.effects.clone();
                let mut new_boss_effects: VecDeque<Vec<String>> = boss.effects.clone();
                while new_boss_effects.len() < 6 {
                    new_boss_effects.push_back(Vec::new());
                }
                new_boss_effects[5].push("Poison".to_string());

                test_states.push_back((
                    turn_ix + 1,
                    Player {
                        hp: player.hp,
                        mana: player.mana - 173,
                        mana_spent: player.mana_spent + 173,
                        effects: new_player_effects,
                        ..Default::default()
                    },
                    Player {
                        hp: boss.hp,
                        damage: boss.damage,
                        effects: new_boss_effects,
                        ..Default::default()
                    }
                ));
            }

            if player.mana >= 229 && !player.effects.iter().any(|t| t.contains(&"Recharge".to_string())) {
                // Recharge
                let mut new_player_effects: VecDeque<Vec<String>> = player.effects.clone();
                let mut new_boss_effects: VecDeque<Vec<String>> = boss.effects.clone();
                while new_player_effects.len() < 5 {
                    new_player_effects.push_back(Vec::new());
                }
                new_player_effects[4].push("Recharge".to_string());

                test_states.push_back((
                    turn_ix + 1,
                    Player {
                        hp: player.hp,
                        mana: player.mana - 229,
                        mana_spent: player.mana_spent + 229,
                        effects: new_player_effects,
                        ..Default::default()
                    },
                    Player {
                        hp: boss.hp,
                        damage: boss.damage,
                        effects: new_boss_effects,
                        ..Default::default()
                    }
                ));
            }
        } else {
            // Boss turn
            test_states.push_back((
                turn_ix + 1, 
                Player {
                    hp: player.hp - 1_i64.max(boss.damage - player.armor),
                    mana: player.mana,
                    mana_spent: player.mana_spent,
                    effects: player.effects.clone(),
                    ..Default::default()
                },
                Player {
                    hp: boss.hp,
                    damage: boss.damage,
                    effects: boss.effects.clone(),
                    ..Default::default()
                }
            ));
        }
    }

    return least_mana;
}

fn part1(contents: String) -> i64 {
    wizard_combat(contents, false)
}

fn part2(contents: String) -> i64 {
    wizard_combat(contents, true)
}

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "22".to_string();

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
        "\nPart 1:\nLeast mana: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLeast mana: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}