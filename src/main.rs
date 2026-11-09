use crate::name::{Name, NameFmt};

mod name;

fn main() {
    let fmt = NameFmt::template("<h1>{name}</h1>");
    let name = Name::from("Alice");
    assert_eq!(name.show(&fmt), "<h1>Alice</h1>");
    assert_eq!(name.show(NameFmt::Raw), "Alice");
}
