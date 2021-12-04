#
# @lc app=leetcode id=653 lang=python3
#
# [653] Two Sum IV - Input is a BST
#
# https://leetcode.com/problems/two-sum-iv-input-is-a-bst/description/
#
# algorithms
# Easy (55.83%)
# Total Accepted:    167.8K
# Total Submissions: 300.1K
# Testcase Example:  '[5,3,6,2,4,null,7]\n9'
#
# Given the root of a Binary Search Tree and a target number k, return true if
# there exist two elements in the BST such that their sum is equal to the given
# target.
# 
# 
# Example 1:
# 
# 
# Input: root = [5,3,6,2,4,null,7], k = 9
# Output: true
# 
# 
# Example 2:
# 
# 
# Input: root = [5,3,6,2,4,null,7], k = 28
# Output: false
# 
# 
# Example 3:
# 
# 
# Input: root = [2,1,3], k = 4
# Output: true
# 
# 
# Example 4:
# 
# 
# Input: root = [2,1,3], k = 1
# Output: false
# 
# 
# Example 5:
# 
# 
# Input: root = [2,1,3], k = 3
# Output: true
# 
# 
# 
# Constraints:
# 
# 
# The number of nodes in the tree is in the range [1, 10^4].
# -10^4 <= Node.val <= 10^4
# root is guaranteed to be a valid binary search tree.
# -10^5 <= k <= 10^5
# 
# 
#
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def findTarget(self, root: TreeNode, k: int) -> bool:
        
