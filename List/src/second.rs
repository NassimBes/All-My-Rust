pub struct List{
    head: Link,
}


type Link = Option<Box<Node>>;


struct Node{
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self{
        List { head: None }
    }

    pub fn push(&mut self, elem: i32){
        let new_node = Box::new(Node{
            elem: elem,
            next : self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
