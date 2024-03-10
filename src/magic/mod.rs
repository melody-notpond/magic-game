pub mod components;
pub mod spells;

#[derive(Clone, Copy)]
pub enum ManaColor {
    Black,
    Red,
    Yellow,
    Blue,
}

#[derive(Clone, Copy)]
pub enum MagicElement {
    // black
    NonElemental,

    // red
    Earth,

    // yellow
    Electricity,

    // blue
    Water,

    // ry
    Magnetism,

    // ru
    Ice,

    // yr
    Metal,

    // yu
    Plant,

    // ur
    Lava,

    // uy
    Fire,
}
