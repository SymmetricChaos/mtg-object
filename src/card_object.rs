use mtg_color::ColorSet;

use crate::{mana_cost::ManaCost, number::Number};

pub struct MtgObject {
    pub name: Vec<String>,        // a card can have zero or more names
    pub mana_cost: Vec<ManaCost>, // a card may have zero or more mana costs
    pub color: ColorSet,
    pub color_identity: ColorSet,
    pub power: Option<Number>,
    pub pt: Option<(Number, Number)>,
    pub loyalty: Option<Number>,
    pub defense: Option<Number>,
    pub type_line: Vec<String>,
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
    pub fn total_mana_value(&self) -> i32 {
        let mut mv = 0;
        for cost in self.mana_cost.iter() {
            mv += cost.mana_value()
        }
        mv
    }

    /// The sum of all mana values of all mana costs.
    pub fn mana_values(&self) -> Vec<i32> {
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
