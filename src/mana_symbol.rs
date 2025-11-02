use std::fmt::{self, Display};

/// All possible mana symbols along with their multiplicity (or value for Generic).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ManaSymbol {
    Generic(usize),
    White(usize),
    Blue(usize),
    Black(usize),
    Red(usize),
    Green(usize),
    Colorless(usize),
    Snow(usize),
    // Variables
    X(usize),
    Y(usize),
    // Hybrid mana
    WU(usize),
    WB(usize),
    UB(usize),
    UR(usize),
    BR(usize),
    BG(usize),
    RG(usize),
    RW(usize),
    GW(usize),
    GU(usize),
    // Twobrid mana
    W2(usize),
    U2(usize),
    B2(usize),
    R2(usize),
    G2(usize),
    // Onebrid mana
    WC(usize),
    UC(usize),
    BC(usize),
    RC(usize),
    GC(usize),
}

impl Display for ManaSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ManaSymbol::Generic(n) => format!("{{{n}}}"),
                ManaSymbol::White(n) => "{W}".repeat(*n),
                ManaSymbol::Blue(n) => "{U}".repeat(*n),
                ManaSymbol::Black(n) => "{B}".repeat(*n),
                ManaSymbol::Red(n) => "{R}".repeat(*n),
                ManaSymbol::Green(n) => "{G}".repeat(*n),
                ManaSymbol::Colorless(n) => "{C}".repeat(*n),
                ManaSymbol::X(n) => "{X}".repeat(*n),
                ManaSymbol::Y(n) => "{Y}".repeat(*n),
                ManaSymbol::Snow(n) => "{S}".repeat(*n),
                ManaSymbol::W2(n) => "{W/2}".repeat(*n),
                ManaSymbol::U2(n) => "{U/2}".repeat(*n),
                ManaSymbol::B2(n) => "{B/2}".repeat(*n),
                ManaSymbol::R2(n) => "{R/2}".repeat(*n),
                ManaSymbol::G2(n) => "{G/2}".repeat(*n),
                ManaSymbol::WU(n) => "{W/U}".repeat(*n),
                ManaSymbol::WB(n) => "{W/B}".repeat(*n),
                ManaSymbol::UB(n) => "{U/B}".repeat(*n),
                ManaSymbol::UR(n) => "{U/R}".repeat(*n),
                ManaSymbol::BR(n) => "{B/R}".repeat(*n),
                ManaSymbol::BG(n) => "{B/G}".repeat(*n),
                ManaSymbol::RG(n) => "{R/G}".repeat(*n),
                ManaSymbol::RW(n) => "{R/W}".repeat(*n),
                ManaSymbol::GW(n) => "{G/W}".repeat(*n),
                ManaSymbol::GU(n) => "{G/U}".repeat(*n),
                ManaSymbol::WC(n) => "{W/C}".repeat(*n),
                ManaSymbol::UC(n) => "{U/C}".repeat(*n),
                ManaSymbol::BC(n) => "{B/C}".repeat(*n),
                ManaSymbol::RC(n) => "{R/C}".repeat(*n),
                ManaSymbol::GC(n) => "{G/C}".repeat(*n),
            }
        )
    }
}
