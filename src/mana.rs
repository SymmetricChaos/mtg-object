use std::fmt::{self, Display};

/// All possible mana symbols.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mana {
    Generic,
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
    Snow,
    // Variables
    X,
    Y,
    // Hybrid mana
    WU,
    WB,
    UB,
    UR,
    BR,
    BG,
    RG,
    RW,
    GW,
    GU,
    // Twobrid mana
    W2,
    U2,
    B2,
    R2,
    G2,
    // Onebrid mana
    WC,
    UC,
    BC,
    RC,
    GC,
}

impl Display for Mana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mana::Generic => "{#}", // numeric value must be handled sepeately
                Mana::White => "{W}",
                Mana::Blue => "{U}",
                Mana::Black => "{B}",
                Mana::Red => "{R}",
                Mana::Green => "{G}",
                Mana::Colorless => "{C}",
                Mana::X => "{X}",
                Mana::Y => "{Y}",
                Mana::Snow => "{S}",
                Mana::W2 => "{W/2}",
                Mana::U2 => "{U/2}",
                Mana::B2 => "{B/2}",
                Mana::R2 => "{R/2}",
                Mana::G2 => "{G/2}",
                Mana::WU => "{W/U}",
                Mana::WB => "{W/B}",
                Mana::UB => "{U/B}",
                Mana::UR => "{U/R}",
                Mana::BR => "{B/R}",
                Mana::BG => "{B/G}",
                Mana::RG => "{R/G}",
                Mana::RW => "{R/W}",
                Mana::GW => "{G/W}",
                Mana::GU => "{G/U}",
                Mana::WC => "{W/C}",
                Mana::UC => "{U/C}",
                Mana::BC => "{B/C}",
                Mana::RC => "{R/C}",
                Mana::GC => "{G/C}",
            }
        )
    }
}

impl Mana {
    pub fn value(&self) -> i32 {
        match self {
            Mana::W2 | Mana::U2 | Mana::B2 | Mana::R2 | Mana::G2 => 2,
            Mana::X | Mana::Y => 0,
            _ => 1,
        }
    }
}
