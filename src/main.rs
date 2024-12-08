use leetcode::search_insert_position;
use leetcode::find_first_one_and_last_one;
use leetcode::multiply;
use leetcode::jump_game;
use leetcode::jump_game2;


fn main() {
    // search_insert_position::solution();
    //find_first_one_and_last_one::solution();
    // let solution: String = multiply::Solution::multiply("123".to_string(), String::from("456"));
    // println!("{solution}");
    // let vector = vec![3,2,1,0,4];
    // let solution = jump_game::Solution::can_jump(vector);
    // println!("{solution}");

    let vector = vec![2,3,0,1,4];
    let result = jump_game2::Solution::jump(vector);
    println!("{result}");

}
