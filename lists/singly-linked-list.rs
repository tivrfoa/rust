// https://leetcode.com/problems/reverse-linked-list-ii

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if head == None { return None; }
        if m == n { return head; }
        let m = m - 1;
        let n = n - 1;
        
        let mut new_list: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        
        let mut idx = 0;
        
        let mut current: Option<Box<ListNode>> = head;
        let mut curr_new = new_list.as_mut().unwrap();
        while idx < m {
            let c = current.unwrap();
            let tmp = Box::new(ListNode::new(c.val));
            curr_new.next = Some(tmp);
            curr_new = curr_new.next.as_mut().unwrap();
            idx += 1;
            current = c.next;
        }
        
        let mut prev: Option<Box<ListNode>> = None;
        while idx <= n {
            let mut c = current.unwrap();
            let next_node = c.next.take();
            c.next = prev.take();
            prev = Some(c);
            current = next_node;            
            idx += 1;
        }

        curr_new.next = prev.take();
        
        while curr_new.next != None {
            curr_new = curr_new.next.as_mut().unwrap();
        }
        
        curr_new.next = current;
        
        new_list.unwrap().next
    }
}

fn main() {
	let list = get_list(vec![1,2,3,4,5]);
	println!("Before: {:?}", list);
	println!("After.: {:?}", Solution::reverse_between(list, 2, 4));
}


fn get_list(v: Vec<i32>) -> Option<Box<ListNode>> {
	let mut new_list: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut prev = new_list.as_mut().unwrap();
	for n in v {
		prev.next = Some(Box::new(ListNode::new(n))); 
		prev = prev.next.as_mut().unwrap();
	}
	
	new_list.unwrap().next
}
