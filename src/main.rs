use std::collections::{HashMap, HashSet};

fn main() {
    let nums = vec![3, 3];
    let target = 6;
    println!(
        "result = {:?}, {:?}",
        A::two_sum(nums.clone(), target.clone()),
        B::two_sum(nums, target)
    );
}

pub trait Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

struct A {}

struct B {}

impl Solution for A {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if nums.len() <= 0 {
            return result;
        }
        for (k, &v) in nums.iter().enumerate() {
            let other = target - v;
            for (k1, &v1) in nums.iter().enumerate() {
                if k != k1 && other == v1 {
                    result.push(k as i32);
                    result.push(k1 as i32);
                    return result;
                }
            }
        }
        return result;
    }
}

impl Default for A {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Solution for B {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if nums.len() <= 0 {
            return result;
        }
        let mut val2indices: HashMap<i32, HashSet<i32>> = HashMap::new();
        for (index, &val) in nums.iter().enumerate() {
            match val2indices.get_mut(&val) {
                Some(indices) => {
                    indices.insert(index as i32);
                }
                None => {
                    let mut indices: HashSet<i32> = HashSet::new();
                    indices.insert(index as i32);
                    val2indices.insert(val, indices);
                }
            }
        }
        for (index, &val) in nums.iter().enumerate() {
            let index = index as i32;
            let other = target - val;
            if let Some(indices) = val2indices.get_mut(&other) {
                if other == val {
                    indices.remove(&index);
                }
                if let Some(&other_index) = indices.iter().next() {
                    result.push(index);
                    result.push(other_index);
                    return result;
                }
            }
        }
        return result;
    }
}

impl Default for B {
    fn default() -> Self {
        unimplemented!()
    }
}