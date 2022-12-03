mod readfile;
use std::collections::HashMap;

static ALPH: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut total_priorities = 0;
    let mut lookup_table = HashMap::new();
    while let (Some(elf1),Some(elf2),Some(elf3)) = (lines.next(),lines.next(),lines.next()) {
        lookup_table.clear();
        for c in elf1.chars() {
            lookup_table.insert(c, 1);
        }
        for c in elf2.chars() {
            if lookup_table.contains_key(&c) {
                lookup_table.insert(c, 2);
            }
        }
        for c in elf3.chars() {
            if lookup_table.contains_key(&c) && lookup_table[&c] == 2 {
                for elem in ALPH.chars().enumerate() {
                    if elem.1 == c { total_priorities += elem.0 + 1; }
                }
                break;
            }
        }
    }
    println!("The total priority is: {}", total_priorities);
}
