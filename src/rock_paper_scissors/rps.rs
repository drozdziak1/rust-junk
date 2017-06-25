use std::cmp::Ordering;

extern crate rand;
use rand::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RPS {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}
static VARIANT_COUNT: isize = 3;
use RPS::*;

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match *self {
            Rock if *other == Scissors => Some(Ordering::Greater),
            Rock if *other == Paper => Some(Ordering::Less),

            Paper if *other == Rock => Some(Ordering::Greater),
            Paper if *other == Scissors => Some(Ordering::Less),

            Scissors if *other == Paper => Some(Ordering::Greater),
            Scissors if *other == Rock => Some(Ordering::Less),

            _ if *self == *other => Some(Ordering::Equal),

            _ => {
                // None doesn't make sense anyway
                panic!(
                    "Ordering {:?} vs. {:?} not implemented",
                    self,
                    other
                )
            }
        }
    }
}

impl Rand for RPS {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0 as isize, VARIANT_COUNT) {
            0 => Rock,
            1 => Paper,
            2 => Scissors,
            n => panic!("Undefined RPS variant {} not implemented!", n),
        }
    }
}

