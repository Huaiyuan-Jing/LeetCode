// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check_sub_path(
            head: Option<&Box<ListNode>>,
            cur: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if head.is_none() {
                return true;
            }
            if cur.is_none() {
                return false;
            }
            let head = head.unwrap();
            let cur = cur.unwrap();
            let cur = cur.borrow();
            if cur.val == head.val {
                if check_sub_path(head.next.as_ref(), cur.left.clone()) {
                    return true;
                }
                if check_sub_path(head.next.as_ref(), cur.right.clone()) {
                    return true;
                }
            }
            false
        }
        fn dfs(cur: Option<Rc<RefCell<TreeNode>>>, head: &Option<Box<ListNode>>) -> bool {
            if cur.is_none() {
                return false;
            }
            if check_sub_path(head.as_ref(), cur.clone()) {
                return true;
            }
            let cur = cur.unwrap();
            let cur = cur.borrow();
            dfs(cur.left.clone(), head) || dfs(cur.right.clone(), head)
        }
        dfs(root, &head)
    }
}
struct Solution;
fn main() {}
