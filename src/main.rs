use std::fmt;

use rand::prelude::*;

trait Average {
    fn average(&mut self) -> f64;
}

impl<Iter> Average for Iter
where
    Iter: Iterator<Item = f64>,
{
    fn average(&mut self) -> f64 {
        let (sum, n) = Iterator::enumerate(self)
            .map(|(n, item)| ((n + 1) as f64, item))
            .fold((0.0, 0.0), |(acc, _), (n, item)| (acc + item, n));
        sum / n
    }
}

#[derive(Clone, Debug, Copy)]
struct Dice {
    d1: u8,
    d2: u8,
}

impl Dice {
    fn new(d1: u8, d2: u8) -> Self {
        if d1 <= d2 {
            Self { d1, d2 }
        } else {
            Self { d1: d2, d2: d1 }
        }
    }

    fn roll() -> Self {
        let mut rng = thread_rng();
        Self::new(rng.gen_range(1, 7), rng.gen_range(1, 7))
    }

    fn sum(self) -> u8 {
        self.d1 + self.d2
    }

    fn modify(self, power: Power) -> Self {
        match power {
            Power::None => self,
            Power::Reroll => Self::roll(),
            Power::FlipOne => match self.d1 {
                1 | 2 | 3 => Self::new(4, self.d2),
                _ => self,
            },
        }
    }

    fn gold(self, bet: u8) -> u8 {
        if bet <= self.sum() {
            bet
        } else {
            2
        }
    }
}

#[derive(Debug)]
struct Outcome(Vec<f64>);

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Bet | Exp")?;
        writeln!(f, "--- | ---")?;
        for (i, outcome) in self.0.iter().enumerate() {
            writeln!(f, " {:>2} | {:>16.2}", i + 2, outcome)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum Power {
    None,
    Reroll,
    FlipOne,
}

trait Strategy {
    fn choose_power(bet: u8, dice: Dice) -> Power;

    fn outcome(bet: u8) -> u8 {
        let dice = Dice::roll();
        dice.modify(Self::choose_power(bet, dice)).gold(bet)
    }

    fn avg_outcome(trials: u64) -> Outcome {
        Outcome(
            (2..=12)
                .map(|bet| (0..trials).map(|_| Self::outcome(bet) as f64).average())
                .collect(),
        )
    }
}

struct RerollIfLosing();
impl Strategy for RerollIfLosing {
    fn choose_power(bet: u8, dice: Dice) -> Power {
        if dice.sum() < bet {
            Power::Reroll
        } else {
            Power::None
        }
    }
}

struct RerollIfLosingOrFlip();
impl Strategy for RerollIfLosingOrFlip {
    fn choose_power(bet: u8, dice: Dice) -> Power {
        if bet <= dice.sum() {
            Power::None
        } else if bet <= dice.modify(Power::FlipOne).sum() {
            Power::FlipOne
        } else {
            Power::Reroll
        }
    }
}

struct AlwaysFlip();
impl Strategy for AlwaysFlip {
    fn choose_power(_bet: u8, _dice: Dice) -> Power {
        Power::FlipOne
    }
}

struct NoPower();
impl Strategy for NoPower {
    fn choose_power(_bet: u8, _dice: Dice) -> Power {
        Power::None
    }
}

// 1. Bet a number // bet
// 2. roll two 6-sided dice. Dice::roll()
// 2.a. If you roll the number or higher, you get that many gold. Dice::gold(bet)
// 2.b. If you roll lower than the number, you get 2 gold.
// What number should you bet for the best expected return?
// You can get a power in the game that lets you either
// 1. Reroll both dice once
// 2. flip one die to show 4.
// 3. Do nothing
// With that power, what number should you bet for the best expected return?

fn main() {
    let trials: u64 = 1_000_000;
    println!("n = {}", trials);
    println!("No change:\n{}", NoPower::avg_outcome(trials));
    println!("Reroll if losing:\n{}", RerollIfLosing::avg_outcome(trials));
    println!("Flip if sum is < 5:\n{}", AlwaysFlip::avg_outcome(trials));
    println!(
        "If losing, flip (if applicable) or reroll:\n{}",
        RerollIfLosingOrFlip::avg_outcome(trials)
    );
}
