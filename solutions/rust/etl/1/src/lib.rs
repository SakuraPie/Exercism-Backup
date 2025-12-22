use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut transform_map: BTreeMap<char, i32> = BTreeMap::new();
    for kv in h {
        for c in kv.1 {
            transform_map.insert(c.to_ascii_lowercase(), *kv.0);
        }
    }
    transform_map
}