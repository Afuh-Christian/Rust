// use std::rc::{Rc, Weak};
// use std::cell::{Ref, RefCell};

// #[derive(Debug )]
// struct Node {
//     value: i32,
//     next: Option<Rc<RefCell<Node>>>,
//     prev: Option<Weak<RefCell<Node>>>,
// }


// #[derive(Debug )]
// struct LinkedList {
//     head: Option<Rc<RefCell<Node>>>,
//     tail: Option<Rc<RefCell<Node>>>,
// }




// impl LinkedList {
//     fn new() -> Self {
//         LinkedList { head: None, tail: None }
//     }

//     fn push_front(&mut self, value: i32) {

//     let new_node = Rc::new(RefCell::new(Node {
//         value,
//         next: None,
//         prev: None,
//     }));

//     match self.head.take() {
//         Some(old_head) => {
//             // TODO

//             // insert at start .. 
//             new_node.borrow_mut().next = Some(Rc::clone(&old_head));
//             self.head = Some(Rc::clone(&new_node));

//             old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
      
//         }
//         None => {
//             // empty list
//             self.head = Some(Rc::clone(&new_node));
//             self.tail = Some(Rc::clone(&new_node));
//         }
//     }
// }


// fn push_back(&mut self, value: i32) {

//     let new_node = Rc::new(RefCell::new(Node {
//         value,
//         next: None,
//         prev: None,
//     }));

//     match self.tail.take() {
//         Some(old_tail) => {
//                     // insert at end .. 
//             old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
//             self.tail = Some(Rc::clone(&new_node));

//             new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
//         }
//         None => {
//               // empty list
//             self.head = Some(Rc::clone(&new_node));
//             self.tail = Some(Rc::clone(&new_node));
//         }
//     }
// }



// fn pop_front(&mut self) -> Option<i32> {

//     self.head.take().map(|old_head| {
//         // TODO
//          let new_current_node = old_head.borrow().next.clone(); 
//          match new_current_node {
//              None => {}, 
//              Some(new_node) => {
//                 new_node.borrow_mut().prev = None ; 
//                  self.head = Some(new_node); 
//              } 
//          }
//         old_head.borrow().value 
//     })
// }



// fn pop_back(&mut self) -> Option<i32> {

//     self.tail.take().map(|old_tail| {

//         let new_current_node = old_tail.borrow().prev.clone(); 

//         new_current_node.map(|new_node|{

//             new_node.upgrade().map(|new_node_|{
//                 new_node_.borrow_mut().next = None; 
//                 self.tail = Some(new_node_) ; 
//             });
//         }); 

//         // TODO

//         old_tail.borrow().value
//     })
// }


// fn print_forward(&self){

//     let current = Rc::clone(self.head) ; 

//     loop {
            
//     }

// }



//     // fn push_front(&mut self , value: i32){

//     //     let node = Rc::new(RefCell::new(Node{value, next:None  , prev:None}));

//     //     match &self.head{
//     //         None => {
//     //             self.head = Some(Rc::clone(&node));
//     //             assign_tail(&mut self.tail ,&node);
//     //         },
//     //         Some(head) => {
//     //             let mut current = Rc::clone(head); 

//     //             loop {
//     //              let next = current.borrow().next.clone(); 

//     //              match next {
//     //                  None => {
//     //                     break;
//     //                     //  assign_tail(&mut self.tail ,&node);
//     //                  },
//     //                  Some(next_) => {
//     //                     current = next_ ; 
//     //                  },
//     //              }

//     //             }

//     //              node.borrow_mut().prev = Some(Rc::downgrade(&current)); 
//     //              current.borrow_mut().next = Some(Rc::clone(&node)); 

//     //              assign_tail(&mut self.tail ,&node);

//     //         }
//     //     }

//     //     print!("item added"); 
        
//     // }






// }




// fn main() {
//     let mut list  = LinkedList::new() ; 

//     list.push_back(1);
//     list.push_back(2);
//     list.push_back(3);
//     list.push_back(4);

//     list.pop_back(); 

//  println!("Print out {:#?}" , list); 

// }









// // fn main() {
// //     let mut list  = LinkedList::new() ; 

// //     for i in 0..6 {
// //         let node = Rc::new(RefCell::new(Node{value: i , next : None , prev: None}));

// //         match &list.head{
// //             None => {
// //                 list.head = Some(Rc::clone(&node)); 
// //                 assign_tail(&mut list.tail, &node);
// //             }, 
// //             Some(head) => {
// //                 let mut current = Rc::clone(head); 

// //                 while let Some(next) = { current.borrow().next.clone() } {
// //                     current = next ; 
// //                 }

// //                 // 
// //                 node.borrow_mut().prev = Some(Rc::downgrade(&current)); 
// //                 current.borrow_mut().next = Some(Rc::clone(&node)); 
// //                 assign_tail(&mut list.tail , &node)
// //             }
// //         }

// //     }

// //      println!("Print out {:#?}" , list); 

// // }


// fn assign_tail(tail : & mut Option<Rc<RefCell<Node>>> , node: & Rc<RefCell<Node>>) {
//     match tail {
//         None => {*tail = Some(Rc::clone(node))}
//         Some(_) => {*tail = Some(Rc::clone(node))}
//     }
// }