/*
 * @lc app=leetcode id=1239 lang=rust
 *
 * [1239] Maximum Length of a Concatenated String with Unique Characters
 *
 * https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/description/
 *
 * algorithms
 * Medium (48.36%)
 * Total Accepted:    38.6K
 * Total Submissions: 79.3K
 * Testcase Example:  '["un","iq","ue"]'
 *
 * Given an array of strings arr. String s is a concatenation of a sub-sequence
 * of arr which have unique characters.
 * 
 * Return the maximum possible length of s.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: arr = ["un","iq","ue"]
 * Output: 4
 * Explanation: All possible concatenations are "","un","iq","ue","uniq" and
 * "ique".
 * Maximum length is 4.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: arr = ["cha","r","act","ers"]
 * Output: 6
 * Explanation: Possible solutions are "chaers" and "acters".
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: arr = ["abcdefghijklmnopqrstuvwxyz"]
 * Output: 26
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= arr.length <= 16
 * 1 <= arr[i].length <= 26
 * arr[i] contains only lower case English letters.
 * 
 * 
 */
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        
    }
}
