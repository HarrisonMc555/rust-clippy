#![warn(clippy::all)]
// #![warn(clippy::use_last)]

fn dont_use_last() -> Option<i32> {
    let x = vec![2, 3, 5];
    let last_element = x.get(x.len() - 1); // ~ERROR Use _.last()
    last_element.map(|val| val + 1) // To avoid warnings
}

fn index_into_last() -> i32 {
    let x = vec![2, 3, 5];
    let last_element = x[x.len() - 1];
    last_element + 1 // To avoid warnings
}

fn main() {
    let expected_value: i32 = 5;
    assert_eq!(dont_use_last(), Some(expected_value));
    assert_eq!(index_into_last(), 5);
}
