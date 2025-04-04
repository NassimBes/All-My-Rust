use std::mem;

pub struct List{
    head: Link,
}


enum Link {
    Empty,
    More(Box<Node>)
}

struct Node{
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self{
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32){
        let new_node = Box::new(Node{
            elem: elem,
            next : mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn basic(){
        let mut lst = List::new();

        assert_eq!(lst.pop(),None);

        lst.push(1);
        lst.push(2);
        assert_eq!(lst.pop(),Some(2));

        lst.push(3);
        lst.push(4);
    }
}
