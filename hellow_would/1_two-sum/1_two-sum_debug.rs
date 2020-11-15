/**
 Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

 

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Output: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
 

Constraints:

2 <= nums.length <= 105
-109 <= nums[i] <= 109
-109 <= target <= 109
Only one valid answer exists.
 */


use std::collections::HashMap;                                          // include


fn main() {
  let mut num = [2,7,11,15];
  let mut result;
  result = two_sum(num.to_vec(),9);
  println!("num = [{},{}]" ,result[0],result[1]);
}


fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  for i in 0..nums.len() {
      for j in i + 1..nums.len() {
          if nums[i] + nums[j] == target {
              return vec![i as i32, j as i32];
          }
      }
  }
//  unreachable!()
  return vec![-1,-1]  
}


// impl Solution {                                                         // class宣言
//   pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {           // Function 
//       let mut compliments: HashMap<i32, i32> = HashMap::new();        // リテラル宣言
//       for i in 0..nums.len() {                                        // ループ
//           match compliments.get(&nums[i]) {                           // ハッシュ内のMach
//               Some(&x) => return vec![x, i as i32],                   // マッチあり
//               None => compliments.insert(target - nums[i], i as i32), // マッチなし格納
//           };
//       }
//       return vec![-1,-1];                                             // なし
//   }