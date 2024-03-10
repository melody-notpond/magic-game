use crate::*;
use magic::{ManaColor, MagicElement};

#[derive(Component)]
pub struct MagicCaster {
    pub source_color_a: ManaColor,
    pub mana_a: u32,
    pub max_mana_a: u32,

    pub source_color_b: ManaColor,
    pub mana_b: u32,
    pub max_mana_b: u32,

    pub primary: MagicElement,
}

#[derive(Component)]
pub struct Health {
    pub health: u32,
    pub max_health: u32,
    pub typed: MagicElement,
}
