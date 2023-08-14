pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, value) in nums.iter().enumerate() {
        for (index2, value2) in nums.iter().enumerate() {
            if index != index2 && value + value2 == target {
                return [index as i32, index2 as i32].to_vec();
            }
        }
    }

    return vec![];
}
