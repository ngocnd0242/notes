/*
* @lc app=leetcode id=240 lang=rust
*
* [240] Search a 2D Matrix II
*
* https://leetcode.com/problems/search-a-2d-matrix-ii/description/
*
* algorithms
* Medium (43.47%)
* Total Accepted:    369.2K
* Total Submissions: 847.8K
* Testcase Example:  '[[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]]\n' +
 '5'
*
* Write an efficient algorithm that searches for a value in an m x n matrix.
* This matrix has the following properties:
*
*
* Integers in each row are sorted in ascending from left to right.
* Integers in each column are sorted in ascending from top to bottom.
*
*
* Example:
*
* Consider the following matrix:
*
*
* [
* ⁠ [1,   4,  7, 11, 15],
* ⁠ [2,   5,  8, 12, 19],
* ⁠ [3,   6,  9, 16, 22],
* ⁠ [10, 13, 14, 17, 24],
* ⁠ [18, 21, 23, 26, 30]
* ]
*
*
* Given target = 5, return true.
*
* Given target = 20, return false.
*
*/
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut v = vec![];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                v.push(matrix[i][j]);
            }
        }
        v.binary_search(&target).is_ok()
    }
}

#[test]
fn valid() {
    assert_eq!(
        Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ),
        true
    );
}
