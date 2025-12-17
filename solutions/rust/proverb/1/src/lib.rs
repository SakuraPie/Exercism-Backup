pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::from("");
    if list.is_empty() {
        return proverb;
    }
    for (index, _letter) in list.iter().enumerate() {
        if index + 1 < list.len() {
            proverb += format!("For want of a {} the {} was lost.\n", list[index], list[index + 1]).as_str();
        }
    }
    proverb += format!("And all for the want of a {}.", list[0]).as_str();
    proverb
}