use std::{cmp::Ordering, collections::BinaryHeap};

use crate::utils::solution::Solution;

const DEFAULT_PLAYER_HEALTH: i32 = 50;
const DEFAULT_PLAYER_MANA: i32 = 500;

pub struct Day22 {}

impl Solution for Day22 {
    fn part_one(&self, _input: &str) -> Option<String> {
        let mut heap = BinaryHeap::<Node>::new();

        let start = Node::new(PlayerState::new(14, 250), BossState::new(10, 9));

        println!("START: {:#?}", start);

        for (i, item) in start.neighbors().iter().enumerate() {
            println!("\tNEIGHBOR {}: {:#?}", i, item);
        }

        for (j, item) in start
            .neighbors()
            .last()
            .unwrap()
            .neighbors()
            .iter()
            .enumerate()
        {
            println!("\t\tNEIGHBOR 4.{}: {:#?}", j, item);
        }

        // heap.push(start);

        // while let Some(node) = heap.pop() {
        //     if node.game_state.boss_state.health <= 0 {
        //         println!("{:#?}", node);
        //         break;
        //     }

        //     heap.extend(node.neighbors());
        // }

        todo!()
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

#[derive(Clone, Debug)]
struct Spell {
    name: &'static str,
    cost: i32,
    duration: i32,
    cast_effect: fn(&Spell, GameState) -> GameState,
}

impl Spell {
    const fn new(
        name: &'static str,
        cost: i32,
        duration: i32,
        cast_effect: fn(&Spell, GameState) -> GameState,
    ) -> Self {
        Self {
            name,
            cost,
            duration,
            cast_effect,
        }
    }
}

const SPELLS: [Spell; 5] = [
    Spell::new("Magic missile", 53, 0, |spell, mut game_state| {
        game_state.player_state.mana -= spell.cost;
        game_state.boss_state.health -= 4;

        game_state
    }),
    Spell::new("Drain", 73, 0, |spell, mut game_state| {
        game_state.player_state.mana -= spell.cost;
        game_state.boss_state.health -= 2;
        game_state.player_state.health += 2;

        game_state
    }),
    Spell::new("Shield", 113, 6, |spell, mut game_state| {
        game_state.player_state.mana -= spell.cost;
        game_state.active_spells.push(SpellEffect {
            name: spell.name.to_string(),
            duration: spell.duration,
            effect: |name, mut game_state| {
                if let Some(spell_state) = game_state.find_active_spell_by_name(name) {
                    spell_state.duration -= 1;
                    game_state.player_state.armor = spell_state.duration.signum() * 7;
                }

                game_state
            },
        });

        game_state
    }),
    Spell::new("Poison", 173, 6, |spell, mut game_state| {
        game_state.player_state.mana -= spell.cost;
        game_state.active_spells.push(SpellEffect {
            name: spell.name.to_string(),
            duration: spell.duration,
            effect: |name, mut game_state| {
                if let Some(spell_state) = game_state.find_active_spell_by_name(name) {
                    spell_state.duration -= 1;
                    game_state.boss_state.health -= 3;
                }

                game_state
            },
        });

        game_state
    }),
    Spell::new("Recharge", 229, 5, |spell, mut game_state| {
        game_state.player_state.mana -= spell.cost;
        game_state.active_spells.push(SpellEffect {
            name: spell.name.to_string(),
            duration: spell.duration,
            effect: |name, mut game_state| {
                if let Some(spell_state) = game_state.find_active_spell_by_name(name) {
                    spell_state.duration -= 1;
                    game_state.player_state.mana += 101;
                }

                game_state
            },
        });

        game_state
    }),
];

#[derive(Clone, Copy, Debug)]
struct PlayerState {
    health: i32,
    armor: i32,
    mana: i32,
}

impl PlayerState {
    fn new(health: i32, mana: i32) -> Self {
        PlayerState {
            health,
            armor: 0,
            mana,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct BossState {
    health: i32,
    damage: i32,
}

impl BossState {
    fn new(health: i32, damage: i32) -> Self {
        BossState { health, damage }
    }
}

#[derive(Clone, Debug)]
struct SpellEffect {
    name: String,
    duration: i32,
    effect: fn(&str, GameState) -> GameState,
}

#[derive(Clone, Debug)]
struct GameState {
    player_state: PlayerState,
    boss_state: BossState,
    active_spells: Vec<SpellEffect>,
}

impl GameState {
    fn find_active_spell_by_name(&mut self, name: &str) -> Option<&mut SpellEffect> {
        self.active_spells.iter_mut().find(|item| item.name == name)
    }

    fn apply_spell_effects(self) -> Self {
        self.active_spells
            .iter()
            .fold(self.clone(), |acc, active_spell| {
                (active_spell.effect)(&active_spell.name, acc)
            })
    }
}

#[derive(Clone, Debug)]
struct Node {
    cost: i32,
    game_state: GameState,
}

impl Node {
    /// Generates a zero-cost [`Node`] with nothing going on in-game
    fn new(player_state: PlayerState, boss_state: BossState) -> Self {
        Self {
            cost: 0,
            game_state: GameState {
                player_state,
                boss_state,
                active_spells: Vec::default(),
            },
        }
    }

    fn neighbors(&self) -> Vec<Node> {
        SPELLS
            .iter()
            .filter_map(|spell| -> Option<Node> {
                let mut next_node = self.clone();

                // Apply each of the active spell effects
                next_node.game_state = next_node.game_state.apply_spell_effects();

                // Prune spells that have ended
                next_node
                    .game_state
                    .active_spells
                    .retain(|effect| effect.duration > 0);

                if next_node
                    .game_state
                    .active_spells
                    .iter()
                    .any(|spell_state| spell_state.name == spell.name)
                    || next_node.game_state.player_state.mana < spell.cost
                {
                    return None; // Can't have the same effect active twice
                }

                // Cast the spell in the current `map` invocation
                next_node.cost += spell.cost;
                next_node.game_state = (spell.cast_effect)(spell, next_node.game_state);

                Some(next_node)
            })
            .collect()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

crate::verify!(Day22, crate::my_input!("2015", "22"), "", "");
