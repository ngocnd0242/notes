/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
 *
 * https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/
 *
 * algorithms
 * Medium (35.35%)
 * Total Accepted:    714.4K
 * Total Submissions: 2M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * <p>Given the <code>head</code> of a linked list, remove the
 * <code>n<sup>th</sup></code> node from the end of the list and return its
 * head.</p>
 * 
 * <p><strong>Follow up:</strong>&nbsp;Could you do this in one pass?</p>
 * 
 * <p>&nbsp;</p>
 * <p><strong>Example 1:</strong></p>
 * <img alt=""
 * src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg"
 * style="width: 542px; height: 222px;" />
 * <pre>
 * <strong>Input:</strong> head = [1,2,3,4,5], n = 2
 * <strong>Output:</strong> [1,2,3,5]
 * </pre>
 * 
 * <p><strong>Example 2:</strong></p>
 * 
 * <pre>
 * <strong>Input:</strong> head = [1], n = 1
 * <strong>Output:</strong> []
 * </pre>
 * 
 * <p><strong>Example 3:</strong></p>
 * 
 * <pre>
 * <strong>Input:</strong> head = [1,2], n = 1
 * <strong>Output:</strong> [1]
 * </pre>
 * 
 * <p>&nbsp;</p>
 * <p><strong>Constraints:</strong></p>
 * 
 * <ul>
 * <li>The number of nodes in the list is <code>sz</code>.</li>
 * <li><code>1 &lt;= sz &lt;= 30</code></li>
 * <li><code>0 &lt;= Node.val &lt;= 100</code></li>
 * <li><code>1 &lt;= n &lt;= sz</code></li>
 * </ul>
 * 
 */
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        
    }
}
