use crate::utils::solution::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day21 {}

impl Solution for Day21 {
    fn part_one(&self, input: &str) -> Option<String> {
        let boss = Boss::from_str(input).unwrap();

        Shop::items()
            .filter(|p| p.can_defeat(&boss))
            .min_by_key(Player::value)
            .map(|p| p.value().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let boss = Boss::from_str(input).unwrap();

        Shop::items()
            .filter(|p| !p.can_defeat(&boss))
            .max_by_key(Player::value)
            .map(|p| p.value().to_string())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Attack {
    Damage(i32),
    Armor(i32),
}

impl Attack {
    fn as_damage(&self) -> i32 {
        match self {
            Self::Damage(n) => *n,
            _ => 0,
        }
    }

    fn as_armor(&self) -> i32 {
        match self {
            Self::Armor(n) => *n,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Item {
    cost: i32,
    attack: Attack,
}

impl Item {
    const fn new(cost: i32, attack: Attack) -> Self {
        Self { cost, attack }
    }
}

struct Boss {
    hitpoints: i32,
    damage: i32,
    armor: i32,
}

impl FromStr for Boss {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().filter_map(|l| match l.split_once(':') {
            Some((_, b)) => b.trim().parse::<i32>().ok(),
            _ => None,
        });

        Ok(Self {
            hitpoints: lines.next().unwrap(),
            damage: lines.next().unwrap(),
            armor: lines.next().unwrap(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Player {
    hitpoints: i32,
    weapon: Item,
    armor: Option<Item>,
    rings: [Option<Item>; 2],
}

impl Player {
    const DEFAULT_HITPOINTS: i32 = 100;

    fn new(hitpoints: i32, weapon: Item, armor: Option<Item>, rings: [Option<Item>; 2]) -> Self {
        Self {
            hitpoints,
            weapon,
            armor,
            rings,
        }
    }

    fn damage_score(&self) -> i32 {
        self.weapon.attack.as_damage()
            + self.rings[0].map_or(0, |r| r.attack.as_damage())
            + self.rings[1].map_or(0, |r| r.attack.as_damage())
    }

    fn armor_score(&self) -> i32 {
        self.armor.map_or(0, |a| a.attack.as_armor())
            + self.rings[0].map_or(0, |r| r.attack.as_armor())
            + self.rings[1].map_or(0, |r| r.attack.as_armor())
    }

    fn value(&self) -> i32 {
        self.weapon.cost
            + self.armor.map_or(0, |a| a.cost)
            + self.rings[0].map_or(0, |r| r.cost)
            + self.rings[1].map_or(0, |r| r.cost)
    }

    fn can_defeat(&self, boss: &Boss) -> bool {
        let player_damage = (self.damage_score() - boss.armor).max(1);
        let num_player_attacks =
            (boss.hitpoints / player_damage) + (boss.hitpoints % player_damage).min(1);

        let boss_damage = (boss.damage - self.armor_score()).max(1);
        let num_boss_attacks =
            (self.hitpoints / boss_damage) + (self.hitpoints % boss_damage).min(1);

        num_player_attacks <= num_boss_attacks
    }
}

struct Shop {}

impl Shop {
    const WEAPONS: [Item; 5] = [
        Item::new(8, Attack::Damage(4)),
        Item::new(10, Attack::Damage(5)),
        Item::new(25, Attack::Damage(6)),
        Item::new(40, Attack::Damage(7)),
        Item::new(74, Attack::Damage(8)),
    ];

    const ARMOR: [Item; 5] = [
        Item::new(13, Attack::Armor(1)),
        Item::new(31, Attack::Armor(2)),
        Item::new(53, Attack::Armor(3)),
        Item::new(75, Attack::Armor(4)),
        Item::new(102, Attack::Armor(5)),
    ];

    const RINGS: [Item; 6] = [
        Item::new(25, Attack::Damage(1)),
        Item::new(50, Attack::Damage(2)),
        Item::new(100, Attack::Damage(3)),
        Item::new(20, Attack::Armor(1)),
        Item::new(40, Attack::Armor(2)),
        Item::new(80, Attack::Armor(3)),
    ];

    fn items() -> impl Iterator<Item = Player> {
        let weapons = Self::WEAPONS
            .iter()
            .combinations(1)
            .chain(Self::WEAPONS.iter().combinations(Self::WEAPONS.len() + 1))
            .chain(Self::WEAPONS.iter().combinations(Self::WEAPONS.len() + 1));
        let armor = Self::ARMOR
            .iter()
            .combinations(0)
            .chain(Self::ARMOR.iter().combinations(1))
            .chain(Self::ARMOR.iter().combinations(Self::ARMOR.len() + 1));
        let rings = Self::RINGS
            .iter()
            .combinations(0)
            .chain(Self::RINGS.iter().combinations(1))
            .chain(Self::RINGS.iter().combinations(2));

        [weapons, armor, rings]
            .into_iter()
            .multi_cartesian_product()
            .filter_map(|items| {
                let weapon: Vec<_> = items[0].to_vec();
                let armor: Vec<_> = items[1].to_vec();
                let rings: Vec<_> = items[2].to_vec();

                let r1 = rings.first().copied().copied();
                let r2 = rings.last().copied().copied();

                if r1.is_some() && r2.is_some() && r1 == r2 {
                    None
                } else {
                    Some(Player::new(
                        Player::DEFAULT_HITPOINTS,
                        weapon.first().copied().copied().unwrap(),
                        armor.first().copied().copied(),
                        [r1, r2],
                    ))
                }
            })
    }
}

crate::verify!(Day21, crate::my_input!("2015", "21"), "123", "201");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let player = Player::new(
            8,
            Item::new(0, Attack::Damage(5)),
            Some(Item::new(0, Attack::Armor(5))),
            [None, None],
        );

        let boss = Boss {
            hitpoints: 12,
            damage: 7,
            armor: 2,
        };

        assert!(player.can_defeat(&boss));
    }
}
