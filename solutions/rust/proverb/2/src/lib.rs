use std::fmt::Write;
pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.is_empty() {
        return proverb;
    }
    for pair in list.windows(2) {
        write!(proverb, "For want of a {} the {} was lost.\n", pair[0], pair[1]).unwrap();
    }
    write!(proverb, "And all for the want of a {}.", list[0]).unwrap();
    proverb
}