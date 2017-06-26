use std::cmp::Ordering;

extern crate rand;
use rand::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,

    /* For the count to work, it has to always be *the last* variant */
    VariantCount
}
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
                panic!("Ordering {:?} vs. {:?} not implemented", self, other)
            }
        }
    }
}

impl Rand for RPS {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0 as isize, VariantCount as isize) {
            n if n == (Rock as isize) => Rock,
            n if n == (Paper as isize) => Paper,
            n if n == (Scissors as isize) => Scissors,
            n => panic!("Undefined RPS variant {} not implemented!", n),
        }
    }
}
