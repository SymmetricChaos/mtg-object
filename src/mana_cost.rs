use crate::mana::Mana;
use regex::Regex;
use std::{cell::LazyCell, collections::BTreeMap};

pub const NUMBER: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"[0123456789]+").unwrap());
pub const MANA_COST_PARSER: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"\{([WUBRGXYCS\/0123456789]+)\}").unwrap());

/// A mana cost representated as a sequence of ManaSymbol. A well formed ManaCost does not repeat any ManaSymbol as they include their multiplicity or value already.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaCost {
    symbols: BTreeMap<Mana, i32>,
}

impl ManaCost {
    // Convenience method.
    // Negative values should not be used.
    fn add(&mut self, mana: Mana, n: i32) {
        self.symbols
            .entry(mana)
            .and_modify(|curr| *curr += n)
            .or_insert(n);
    }

    /// Create a ManaCost from a &str.
    pub fn parse_string(s: &str) -> Option<Self> {
        let mut cost = ManaCost {
            symbols: BTreeMap::new(),
        };

        for (_, [m]) in MANA_COST_PARSER.captures_iter(s).map(|c| c.extract()) {
            if NUMBER.is_match(m) {
                cost.add(Mana::Generic, i32::from_str_radix(m, 10).unwrap());
            } else {
                match m {
                    "W" => cost.add(Mana::White, 1),
                    "U" => cost.add(Mana::Blue, 1),
                    "B" => cost.add(Mana::Black, 1),
                    "R" => cost.add(Mana::Red, 1),
                    "G" => cost.add(Mana::Green, 1),
                    "C" => cost.add(Mana::Colorless, 1),
                    "S" => cost.add(Mana::Snow, 1),
                    "X" => cost.add(Mana::X, 1),
                    "Y" => cost.add(Mana::Y, 1),
                    // Twobrid
                    "W/2" => cost.add(Mana::W2, 1),
                    "U/2" => cost.add(Mana::U2, 1),
                    "B/2" => cost.add(Mana::B2, 1),
                    "R/2" => cost.add(Mana::R2, 1),
                    "G/2" => cost.add(Mana::G2, 1),
                    // Hybrid
                    "W/U" => cost.add(Mana::WU, 1),
                    "W/B" => cost.add(Mana::WB, 1),
                    "U/B" => cost.add(Mana::UB, 1),
                    "U/R" => cost.add(Mana::UR, 1),
                    "B/R" => cost.add(Mana::BR, 1),
                    "B/G" => cost.add(Mana::BG, 1),
                    "R/G" => cost.add(Mana::RG, 1),
                    "R/W" => cost.add(Mana::RW, 1),
                    "G/W" => cost.add(Mana::GW, 1),
                    "G/U" => cost.add(Mana::GU, 1),
                    // Onebrid
                    "W/C" => cost.add(Mana::WC, 1),
                    "U/C" => cost.add(Mana::UC, 1),
                    "B/C" => cost.add(Mana::BC, 1),
                    "R/C" => cost.add(Mana::RC, 1),
                    "G/C" => cost.add(Mana::GC, 1),
                    _ => return None,
                };
            }
        }

        Some(cost)
    }

    /// Print the symbols of a ManaCost in text form using an arbitrary order.
    pub fn print_symbols(&self) -> String {
        let mut out = String::new();
        for (s, n) in self.symbols.iter() {
            if *s == Mana::Generic {
                out.push_str(&format!("{{{}}}", n));
            } else {
                out.push_str(&s.to_string().repeat(
                    usize::try_from(*n).expect("negative mana multiplicity for mana symbol"),
                ));
            }
        }
        out
    }

    /// The mana value associated with a ManaCost.
    pub fn mana_value(&self) -> i32 {
        let mut mv = 0;
        for (s, n) in self.symbols.iter() {
            mv += s.value() * n;
        }
        mv
    }

    /// The mana value associated with a ManaCost on the stack when values for X and Y have been chosen.
    pub fn mana_value_on_stack(&self, x: i32, y: i32) -> i32 {
        let mut mv = 0;
        for (s, n) in self.symbols.iter() {
            if *s == Mana::X {
                mv += x * n;
            } else if *s == Mana::Y {
                mv += y * n;
            } else {
                mv += s.value() * n;
            }
        }
        mv
    }
}

#[test]
fn test_parse() {
    let c = ManaCost::parse_string("{R}{G}{R}{15}{C}{W/U}").unwrap();
    println!("{}", c.print_symbols());
}
