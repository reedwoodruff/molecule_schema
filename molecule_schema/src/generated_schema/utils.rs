use im::HashMap;

pub fn convert_to_hashmap<T: Clone + std::hash::Hash + PartialEq + Eq, U: Clone>(
    tuple_array: &[(T, U)],
) -> HashMap<T, U> {
    let hashmap = tuple_array.iter().cloned().collect();
    hashmap
}
