use crate::mana_symbol::ManaSymbol;
use regex::Regex;
use std::cell::LazyCell;

pub const MANA_COST_PARSER: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"(\{[WUBRGXYCS\/2]+\})|\{[0123456789]+\}").unwrap());

/// A mana cost representated as a sequence of ManaSymbol. A well formed ManaCost does not repeat any ManaSymbol as they include their multiplicity or value already.
pub struct ManaCost {
    symbols: Vec<ManaSymbol>,
}

impl ManaCost {
    pub fn parse_string(s: &str) -> Option<Self> {
        let mut symbols = Vec::new();

        let mut caps = MANA_COST_PARSER.captures(s).unwrap();

        for (n, capture) in caps.iter().enumerate() {
            println!("{n} {}", capture.unwrap().as_str());
        }

        Some(ManaCost { symbols })
    }

    pub fn print_symbols(&self) -> String {
        let mut out = String::new();
        for s in self.symbols.iter() {
            out.push_str(&s.to_string());
        }
        out
    }

    /// The mana value associated with a ManaCost.
    pub fn mana_value(&self) -> usize {
        let mut mv = 0;
        for symbol in self.symbols.iter() {
            match symbol {
                ManaSymbol::X(_) | ManaSymbol::Y(_) => (),
                ManaSymbol::Generic(n)
                | ManaSymbol::White(n)
                | ManaSymbol::Blue(n)
                | ManaSymbol::Black(n)
                | ManaSymbol::Red(n)
                | ManaSymbol::Green(n)
                | ManaSymbol::Colorless(n)
                | ManaSymbol::Snow(n)
                | ManaSymbol::WB(n)
                | ManaSymbol::UB(n)
                | ManaSymbol::UR(n)
                | ManaSymbol::BR(n)
                | ManaSymbol::BG(n)
                | ManaSymbol::RG(n)
                | ManaSymbol::RW(n)
                | ManaSymbol::GW(n)
                | ManaSymbol::GU(n)
                | ManaSymbol::WU(n)
                | ManaSymbol::WC(n)
                | ManaSymbol::UC(n)
                | ManaSymbol::BC(n)
                | ManaSymbol::RC(n)
                | ManaSymbol::GC(n) => mv += n,
                ManaSymbol::W2(n)
                | ManaSymbol::U2(n)
                | ManaSymbol::B2(n)
                | ManaSymbol::R2(n)
                | ManaSymbol::G2(n) => mv += n * 2,
            }
        }
        mv
    }

    /// The mana value associated with a ManaCost on the stack when values for X and Y have been chosen.
    pub fn mana_value_on_stack(&self, x: usize, y: usize) -> usize {
        let mut mv = 0;
        for symbol in self.symbols.iter() {
            match symbol {
                ManaSymbol::X(n) => mv += n * x,
                ManaSymbol::Y(n) => mv += n * y,
                ManaSymbol::Generic(n)
                | ManaSymbol::White(n)
                | ManaSymbol::Blue(n)
                | ManaSymbol::Black(n)
                | ManaSymbol::Red(n)
                | ManaSymbol::Green(n)
                | ManaSymbol::Colorless(n)
                | ManaSymbol::Snow(n)
                | ManaSymbol::WB(n)
                | ManaSymbol::UB(n)
                | ManaSymbol::UR(n)
                | ManaSymbol::BR(n)
                | ManaSymbol::BG(n)
                | ManaSymbol::RG(n)
                | ManaSymbol::RW(n)
                | ManaSymbol::GW(n)
                | ManaSymbol::GU(n)
                | ManaSymbol::WU(n)
                | ManaSymbol::WC(n)
                | ManaSymbol::UC(n)
                | ManaSymbol::BC(n)
                | ManaSymbol::RC(n)
                | ManaSymbol::GC(n) => mv += n,
                ManaSymbol::W2(n)
                | ManaSymbol::U2(n)
                | ManaSymbol::B2(n)
                | ManaSymbol::R2(n)
                | ManaSymbol::G2(n) => mv += n * 2,
            }
        }
        mv
    }
}

#[test]
fn test_parse() {
    ManaCost::parse_string("{U/W}{R}{G}{R}{2/B}{15}{C}");
}
