use crate::card_object::CardObject;

pub struct Player {
    pub id: usize, // must be unique per game
    pub life: i32,
    pub storm_count: i32,
    // floating mana
    pub generic_mana: i32,
    pub white_mana: i32,
    pub blue_mana: i32,
    pub black_mana: i32,
    pub red_mana: i32,
    pub green_mana: i32,
    // zones unique to a player
    pub library: Vec<CardObject>,
    pub graveyard: Vec<CardObject>,
    pub command: Vec<CardObject>,
}
