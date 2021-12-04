#
# @lc app=leetcode id=658 lang=python3
#
# [658] Find K Closest Elements
#
# https://leetcode.com/problems/find-k-closest-elements/description/
#
# algorithms
# Medium (41.30%)
# Total Accepted:    125.3K
# Total Submissions: 302.5K
# Testcase Example:  '[1,2,3,4,5]\n4\n3'
#
# Given a sorted integer array arr, two integers k and x, return the k closest
# integers to x in the array. The result should also be sorted in ascending
# order.
# 
# An integer a is closer to x than an integer b if:
# 
# 
# |a - x| < |b - x|, or
# |a - x| == |b - x| and a < b
# 
# 
# 
# Example 1:
# Input: arr = [1,2,3,4,5], k = 4, x = 3
# Output: [1,2,3,4]
# Example 2:
# Input: arr = [1,2,3,4,5], k = 4, x = -1
# Output: [1,2,3,4]
# 
# 
# Constraints:
# 
# 
# 1 <= k <= arr.length
# 1 <= arr.length <= 10^4
# Absolute value of elements in the array and x will not exceed 10^4
# 
# 
#
class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        
