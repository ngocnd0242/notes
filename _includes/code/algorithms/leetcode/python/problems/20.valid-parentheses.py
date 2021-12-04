#
# @lc app=leetcode id=20 lang=python3
#
# [20] Valid Parentheses
#
# https://leetcode.com/problems/valid-parentheses/description/
#
# algorithms
# Easy (39.26%)
# Total Accepted:    1.2M
# Total Submissions: 2.9M
# Testcase Example:  '"()"'
#
# <p>Given a string <code>s</code> containing just the characters
# <code>&#39;(&#39;</code>, <code>&#39;)&#39;</code>, <code>&#39;{&#39;</code>,
# <code>&#39;}&#39;</code>, <code>&#39;[&#39;</code> and
# <code>&#39;]&#39;</code>, determine if the input string is valid.</p>
# 
# <p>An input string is valid if:</p>
# 
# <ol>
# <li>Open brackets must be closed by the same type of brackets.</li>
# <li>Open brackets must be closed in the correct order.</li>
# </ol>
# 
# <p>&nbsp;</p>
# <p><strong>Example 1:</strong></p>
# 
# <pre>
# <strong>Input:</strong> s = &quot;()&quot;
# <strong>Output:</strong> true
# </pre>
# 
# <p><strong>Example 2:</strong></p>
# 
# <pre>
# <strong>Input:</strong> s = &quot;()[]{}&quot;
# <strong>Output:</strong> true
# </pre>
# 
# <p><strong>Example 3:</strong></p>
# 
# <pre>
# <strong>Input:</strong> s = &quot;(]&quot;
# <strong>Output:</strong> false
# </pre>
# 
# <p><strong>Example 4:</strong></p>
# 
# <pre>
# <strong>Input:</strong> s = &quot;([)]&quot;
# <strong>Output:</strong> false
# </pre>
# 
# <p><strong>Example 5:</strong></p>
# 
# <pre>
# <strong>Input:</strong> s = &quot;{[]}&quot;
# <strong>Output:</strong> true
# </pre>
# 
# <p>&nbsp;</p>
# <p><strong>Constraints:</strong></p>
# 
# <ul>
# <li><code>1 &lt;= s.length &lt;= 10<sup>4</sup></code></li>
# <li><code>s</code> consists of parentheses only
# <code>&#39;()[]{}&#39;</code>.</li>
# </ul>
# 
#
class Solution:
    def isValid(self, s: str) -> bool:
        
