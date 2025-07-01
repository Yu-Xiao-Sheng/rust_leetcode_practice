
type Link = Option<Box<Node>>

struct Node {
    elem: i32,
    next: Link,
}

pub struct List{
    head: Link,
}


impl List {
    pub fn new() -> Self {
        List {head:None}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node{
            elem:elem,
            // 等效 next:mem::replace(&mut self.head, None),
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        // 等效
        // match mem::replace(&mut self.head, None){
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        self.head.take().map(|node|{
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List{
    fn drop(&mut self){
        // 等效
        // let mut cur_link = mem::replace(&mut self.head, None);
        // while let Some(mut boxed_node) = cur_link{
        //     cur_link = mem::replace(&mut boxed_node.next, None);
        // }

        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link{
            cur_link = boxed_node.next.take();
        }
    }
}
