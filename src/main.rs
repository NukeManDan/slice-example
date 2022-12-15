use resize_slice::ResizeSlice;

fn sort(array: &mut [u32]) {
    // resize to only two items
    array.resize_from(1);
}

fn main() {
    let mut array = vec![3, 5, 7, 9];

    // Print the original vec.
    println!("before: {:#?}", array);

    // Sort it.
    sort(&mut array);

    // Print the new vec.
    println!("after: {:#?}", array);
}
