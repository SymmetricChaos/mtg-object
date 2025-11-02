use mtg_color::ColorSet;

/// Mana symbols other than the generic mana symbol.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ManaSymbol {
    W, // white
    U, // blue
    B, // black
    R, // red
    G, // green
    Colorless,
    X,
    Y,
    Snow,
    W2,
    U2,
    B2,
    R2,
    G2,
}

impl ManaSymbol {
    /// The text form of the symbol.
    pub fn symbol(&self) -> &'static str {
        match self {
            ManaSymbol::W => "{W}",
            ManaSymbol::U => "{U}",
            ManaSymbol::B => "{B}",
            ManaSymbol::R => "{R}",
            ManaSymbol::G => "{G}",
            ManaSymbol::Colorless => "{C}",
            ManaSymbol::X => "{X}",
            ManaSymbol::Y => "{Y}",
            ManaSymbol::Snow => "{S}",
            ManaSymbol::W2 => "{W/2}",
            ManaSymbol::U2 => "{U/2}",
            ManaSymbol::B2 => "{B/2}",
            ManaSymbol::R2 => "{R/2}",
            ManaSymbol::G2 => "{G/2}",
        }
    }
}

/// A mana cost.
pub struct ManaCost {
    generic: Option<usize>, // None represents no generic portion at all.
    symbols: Vec<ManaSymbol>,
}

impl ManaCost {
    pub fn print_symbols(&self) -> String {
        let mut symbols = match self.generic {
            Some(n) => format!("{{{n}}}"),
            None => String::new(),
        };

        for s in self.symbols.iter() {
            symbols.push_str(s.symbol());
        }

        symbols
    }

    /// The mana value associated with a cost when X and Y are both set to zero.
    pub fn mana_value(&self) -> usize {
        let mut mv = match self.generic {
            Some(n) => n,
            None => 0,
        };
        for symbol in self.symbols.iter() {
            match symbol {
                ManaSymbol::X | ManaSymbol::Y => (), // X and Y are zero except on the stack
                _ => mv += 1,
            }
        }
        mv
    }
    pub fn mana_value_on_stack(&self) {
        todo!()
    }
}

pub struct MtgObject {
    pub name: Vec<String>,        // a card can have zero or more names
    pub mana_cost: Vec<ManaCost>, // a card may have zero or more mana costs
    pub color: ColorSet,
    pub color_identity: ColorSet,
}

impl MtgObject {
    /// Print all the names of the object. For objects with multiple names each is separated by ` // `.
    pub fn print_name(&self) -> String {
        if self.name.is_empty() {
            String::new()
        } else if self.name.len() == 1 {
            self.name[0].clone()
        } else {
            self.name.join(" // ")
        }
    }

    /// The sum of all mana values of all mana costs.
    pub fn total_mana_value(&self) -> usize {
        let mut mv = 0;
        for cost in self.mana_cost.iter() {
            mv += cost.mana_value()
        }
        mv
    }

    /// The sum of all mana values of all mana costs.
    pub fn mana_values(&self) -> Vec<usize> {
        self.mana_cost
            .iter()
            .map(|cost| cost.mana_value())
            .collect()
    }

    pub fn color(&self) -> &str {
        self.color.symbols()
    }

    pub fn color_identity(&self) -> &str {
        self.color_identity.symbols()
    }
}

fn main() {}
