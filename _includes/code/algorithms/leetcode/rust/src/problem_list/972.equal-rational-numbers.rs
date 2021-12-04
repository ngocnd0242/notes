/*
 * @lc app=leetcode id=972 lang=rust
 *
 * [972] Equal Rational Numbers
 *
 * https://leetcode.com/problems/equal-rational-numbers/description/
 *
 * algorithms
 * Hard (41.73%)
 * Total Accepted:    4.3K
 * Total Submissions: 10.2K
 * Testcase Example:  '"0.(52)"\n"0.5(25)"'
 *
 * Given two strings S and T, each of which represents a non-negative rational
 * number, return True if and only if they represent the same number. The
 * strings may use parentheses to denote the repeating part of the rational
 * number.
 * 
 * In general a rational number can be represented using up to three parts: an
 * integer part, a non-repeating part, and a repeating part. The number will be
 * represented in one of the following three ways:
 * 
 * 
 * <IntegerPart> (e.g. 0, 12, 123)
 * <IntegerPart><.><NonRepeatingPart>  (e.g. 0.5, 1., 2.12, 2.0001)
 * <IntegerPart><.><NonRepeatingPart><(><RepeatingPart><)> (e.g. 0.1(6),
 * 0.9(9), 0.00(1212))
 * 
 * 
 * The repeating portion of a decimal expansion is conventionally denoted
 * within a pair of round brackets.  For example:
 * 
 * 1 / 6 = 0.16666666... = 0.1(6) = 0.1666(6) = 0.166(66)
 * 
 * Both 0.1(6) or 0.1666(6) or 0.166(66) are correct representations of 1 /
 * 6.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: S = "0.(52)", T = "0.5(25)"
 * Output: true
 * Explanation:
 * Because "0.(52)" represents 0.52525252..., and "0.5(25)" represents
 * 0.52525252525..... , the strings represent the same number.
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: S = "0.1666(6)", T = "0.166(66)"
 * Output: true
 * 
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: S = "0.9(9)", T = "1."
 * Output: true
 * Explanation: 
 * "0.9(9)" represents 0.999999999... repeated forever, which equals 1.  [See
 * this link for an explanation.]
 * "1." represents the number 1, which is formed correctly: (IntegerPart) = "1"
 * and (NonRepeatingPart) = "".
 * 
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * Each part consists only of digits.
 * The <IntegerPart> will not begin with 2 or more zeros.  (There is no other
 * restriction on the digits of each part.)
 * 1 <= <IntegerPart>.length <= 4 
 * 0 <= <NonRepeatingPart>.length <= 4 
 * 1 <= <RepeatingPart>.length <= 4
 * 
 * 
 */
impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        
    }
}
