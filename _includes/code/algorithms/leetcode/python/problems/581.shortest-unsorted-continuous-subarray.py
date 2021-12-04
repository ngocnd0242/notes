#
# @lc app=leetcode id=581 lang=python3
#
# [581] Shortest Unsorted Continuous Subarray
#
# https://leetcode.com/problems/shortest-unsorted-continuous-subarray/description/
#
# algorithms
# Medium (31.25%)
# Total Accepted:    143.5K
# Total Submissions: 457.3K
# Testcase Example:  '[2,6,4,8,10,9,15]'
#
# Given an integer array nums, you need to find one continuous subarray that if
# you only sort this subarray in ascending order, then the whole array will be
# sorted in ascending order.
# 
# Return the shortest such subarray and output its length.
# 
# 
# Example 1:
# 
# 
# Input: nums = [2,6,4,8,10,9,15]
# Output: 5
# Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the
# whole array sorted in ascending order.
# 
# 
# Example 2:
# 
# 
# Input: nums = [1,2,3,4]
# Output: 0
# 
# 
# Example 3:
# 
# 
# Input: nums = [1]
# Output: 0
# 
# 
# 
# Constraints:
# 
# 
# 1 <= nums.length <= 10^4
# -10^5 <= nums[i] <= 10^5
# 
# 
#
class Solution:
    def findUnsortedSubarray(self, nums: List[int]) -> int:
        
