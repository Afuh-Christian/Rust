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





pub fn merge_two_lists(
    list1: Option<Box<ListNode>>, 
    list2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    // Create a dummy node to simplify list construction
    let mut dummy = ListNode::new(0);
    let mut tail = &mut dummy;

    // Work with mutable references to the lists
    let mut l1 = list1;
    let mut l2 = list2;

    while l1.is_some() && l2.is_some() {
        // unwrap is safe here because we checked is_some
        let l1_val = l1.as_ref().unwrap().val;
        let l2_val = l2.as_ref().unwrap().val;

        if l1_val < l2_val {
            let next = l1.as_mut().unwrap().next.take();
            tail.next = l1;
            l1 = next;
        } else {
            let next = l2.as_mut().unwrap().next.take();
            tail.next = l2;
            l2 = next;
        }
        // Move tail to next
        tail = tail.next.as_mut().unwrap();
    }

    // Attach the remaining list (one of l1 or l2 is Some)
    tail.next = if l1.is_some() { l1 } else { l2 };

    dummy.next
}