#[warn(non_snake_case)]
pub fn Optionstype() -> Option<Siblings> {
    let mut characters: Option<Siblings> = None;
    characters = Some(Siblings::Emeke);
    return characters;
}

pub enum Siblings {
    Emeke,
    Ekenem,
    Ifeanyi,
    Chidinma,
}
