use std::collections::HashMap;


pub fn compile_utf_map(static_map: &[(&str, &str)]) -> HashMap<Vec<char>, Vec<char>> {
    let mut compiled_map = HashMap::with_capacity(static_map.len());
    for (pattern, replace) in static_map {
        compiled_map.insert(
            pattern.chars().collect(),
            replace.chars().collect()
        );
    }
    compiled_map
}
