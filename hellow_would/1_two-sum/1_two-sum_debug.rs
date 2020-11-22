
use std::collections::HashMap;                                          // include
fn main() {
  let mut num = [2,7,11,15];
  let mut result;
  result = two_sum(num.to_vec(),9);
  println!("num = [{},{}]" ,result[0],result[1]);
}


fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {           // Function 
      let mut compliments: HashMap<i32, i32> = HashMap::new();        // リテラル宣言
      for i in 0..nums.len() {                                        // ループ
          match compliments.get(&nums[i]) {                           // ハッシュ内のMach
              Some(&x) => return vec![x, i as i32],                   // マッチあり
              None => compliments.insert(target - nums[i], i as i32), // マッチなし格納
          };
      }
      return vec![-1,-1];                                             // なし
}
