use crate::{card_object::CardObject, player::Player};

pub struct Game {
    pub players: Vec<Player>,
    pub stack: Vec<CardObject>,
    pub battlefield: Vec<CardObject>,
    pub exile: Vec<CardObject>,
}
