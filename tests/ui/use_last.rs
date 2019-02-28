#![warn(clippy::use_last)]

fn dont_use_last() -> Option<i32> {
    let x = vec![2, 3, 5];
    let last_element = x.get(x.len() - 1); // ~ERROR Use x.last()
    last_element.map(|val| val + 1) // To avoid warnings
}

fn indexing_two_from_end() -> Option<i32> {
    let x = vec![2, 3, 5];
    let last_element = x.get(x.len() - 2);
    last_element.map(|val| val + 3) // To avoid warnings
}

fn index_into_last() -> i32 {
    let x = vec![2, 3, 5];
    let last_element = x[x.len() - 1];
    last_element + 1 // To avoid warnings
}

// False positive test (currently failing)
// fn use_last_with_different_vec_length() -> Option<i32> {
//     let x = vec![2, 3, 5];
//     let y = vec!['a', 'b', 'c'];
//     let last_element = x.get(y.len() - 1);
//     last_element.map(|val| val + 1)
// }

fn main() {
    let expected_value: i32 = 6;
    println!("Working...");
    assert_eq!(dont_use_last(), Some(expected_value));
    assert_eq!(indexing_two_from_end(), Some(expected_value));
    assert_eq!(index_into_last(), expected_value);
    // assert_eq!(use_last_with_different_vec_length(), Some(expected_value));
}
