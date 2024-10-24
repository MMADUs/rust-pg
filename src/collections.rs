use std::collections::*;

// cargo test collections::try_vector -- --nocapture
#[test]
fn try_vector() {
    // hardcoded vector
    let mut vector = vec![1, 2, 3];
    // insert keyword
    vector.insert(1, 4);
    assert_eq!(vector, [1, 4, 2, 3]);

    vector.insert(4, 5);
    assert_eq!(vector, [1, 4, 2, 3, 5]);

    // new empty vector
    let mut vector = Vec::new();
    // push keyword
    vector.push(1);
    vector.push(2);
    assert_eq!(vector[0], 1);
    assert_eq!(vector[1], 2);

    // empty vector with capacity
    // when it reach the capacity, it will realloc
    let mut vector = Vec::with_capacity(2);
    vector.push(10);
    vector.push(20);
    vector.push(30);
    assert_eq!(vector[0], 10);
    assert_eq!(vector[1], 20);
    assert_eq!(vector[2], 30);
    // get vector length
    let vec_length = vector.len();
    println!("vector length: {}", vec_length);
    assert_eq!(vec_length, 3);

    // vec with string
    let mut vector = vec!["foo", "bar", "baz", "qux"];
    // remove a value by index and return the removed value
    // the removed element is replaced by the last element of the vector
    let removed = vector.swap_remove(1);
    assert_eq!(removed, "bar");
    assert_eq!(vector, ["foo", "qux", "baz"]);

    let removed = vector.swap_remove(0);
    assert_eq!(removed, "foo");
    assert_eq!(vector, ["baz", "qux"]);

    // another vec with string
     let mut vector = vec!["foo", "bar", "baz", "qux"];
    // remove element by index and return the removed element
    let removed = vector.remove(1);
    assert_eq!(removed, "bar");
    assert_eq!(vector, ["foo", "baz", "qux"]);
    let removed = vector.remove(1);
    assert_eq!(removed, "baz");
    assert_eq!(vector, ["foo", "qux"]);
    
    // another vec with string
    let mut vector = vec!["foo", "bar", "baz", "qux"];
    // pop means remove the last element and return Option element
    let popped = vector.pop();
    assert_eq!(popped, Some("qux"));
    assert_eq!(vector, ["foo", "bar", "baz"]);

    let popped = vector.pop();
    assert_eq!(popped, Some("baz"));
    assert_eq!(vector, ["foo", "bar"]);

    // append vector, kinda merge it
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);

    // clear vector
    let mut v = vec![1, 2, 3];
    v.clear();
    let is_empty = v.is_empty();
    assert_eq!(is_empty, true); // check if empty
    
    // remove duplicated elements
    let mut vec = vec![1, 2, 2, 3, 2];
    vec.dedup();
    assert_eq!(vec, [1, 2, 3, 2]);
    
    // split vec into a new vec
    let mut vec = vec![1, 2, 3];
    let vec2 = vec.split_off(1);
    assert_eq!(vec, [1]);
    assert_eq!(vec2, [2, 3]);
}
