use std::collections::HashMap;
use crate::utils::to_vec;


pub fn compile_utf_map(static_map: &[(&str, &str)]) -> HashMap<Vec<char>, Vec<char>> {
    let mut compiled_map = HashMap::with_capacity(static_map.len());
    for (pattern, replace) in static_map {
        compiled_map.insert(to_vec(pattern), to_vec(replace));
    }
    compiled_map
}
