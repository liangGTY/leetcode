//给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。 
//
// 请你将两个数相加，并以相同形式返回一个表示和的链表。 
//
// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。 
//
// 
//
// 示例 1： 
// 
// 
//输入：l1 = [2,4,3], l2 = [5,6,4]
//输出：[7,0,8]
//解释：342 + 465 = 807.
// 
//
// 示例 2： 
//
// 
//输入：l1 = [0], l2 = [0]
//输出：[0]
// 
//
// 示例 3： 
//
// 
//输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//输出：[8,9,9,9,0,0,0,1]
// 
//
// 
//
// 提示： 
//
// 
// 每个链表中的节点数在范围 [1, 100] 内 
// 0 <= Node.val <= 9 
// 题目数据保证列表表示的数字不含前导零 
// 
//
// Related Topics 递归 链表 数学 👍 10381 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode {
//             next: None,
//             val,
//         }
//     }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Solution::add_two_numbers_iter(l1, l2, 0);
    }

    pub fn add_two_numbers_iter(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if l1 == None && l2 == None {
            if x == 0 {
                return None;
            }
            return Some(Box::new(ListNode::new(x)));
        }

        let (l1_val, l1_next) = l1.map_or((0, None), |v| {
            ((*v).val, (*v).next)
        });
        let (l2_val, l2_next) = l2.map_or((0, None), |v| {
            ((*v).val, (*v).next)
        });

        let y = l1_val + l2_val + x;

        let a;
        let mut current_node;
        if y < 10 {
            a = 0;
            current_node = Box::new(ListNode::new(y));
        } else {
            a = 1;
            current_node = Box::new(ListNode::new(y - 10));
        }

        let iter = Solution::add_two_numbers_iter(l1_next, l2_next, a);

        (*current_node).next = iter;

        return Some(current_node);
    }
}
//leetcode submit region end(Prohibit modification and deletion)
