// using for loop
pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
    let original = nums.clone();
    for (index, value) in nums.iter_mut().enumerate() {
        *value = original[original[index] as usize]
    }

    nums as Vec<i32>
}

// using map
pub fn build_array_map(mut nums: Vec<i32>) -> Vec<i32> {
    let original = nums.clone();

    nums.iter_mut()
        .enumerate()
        .map(|(index, value)| original[original[index] as usize])
        .collect()
}
