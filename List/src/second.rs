// Aliases
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T>{
    head: Link<T>,
}

struct Node<T>{
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self{
        List { head: None }
    }

    pub fn push(&mut self, elem: T){
        let new_node = Box::new(Node{
            elem: elem,
            next : self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }


    pub fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    

    pub fn peek_mut(&mut self) -> Option<&mut T>{
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T>{
        Iter { 
            next: self.head.as_deref()
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_,T>{
        IterMut { next: self.head.as_deref_mut() }
    }

}

//DROP
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

// ITERATORS
// INTO ITER
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

//ITER
pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>,
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next<'b>(&'b mut self) -> Option<&'a T> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// IterMut
pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>
}

impl<'a,T> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;

    fn next<'b>(&'b mut self) -> Option<&'a mut T> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


// TESTS
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

    #[test]
    fn peek_test(){
        let mut lst = List::new();

        

        assert_eq!(lst.peek(),None);
        assert_eq!(lst.peek_mut(),None);
        lst.push(1); lst.push(2); lst.push(3); 

        
        assert_eq!(lst.peek(),Some(&3));
        assert_eq!(lst.peek_mut(),Some(&mut 3));

        lst.peek_mut().map(|value|{
            *value = 32
        });
        assert_eq!(lst.peek(),Some(&32));
        assert_eq!(lst.pop(),Some(32));
    }


    #[test]
    fn into_iter(){
        let mut list = List::new();
        list.push(1);list.push(2);list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(),Some(3));
        assert_eq!(iter.next(),Some(2));
        assert_eq!(iter.next(),Some(1));
        assert_eq!(iter.next(),None);
    }

    #[test]
    fn iter(){
        let mut list = List::new();
        list.push("A");list.push("B");list.push("C");
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&"C"));
        assert_eq!(iter.next(), Some(&"B"));
        assert_eq!(iter.next(), Some(&"A"));
    }

    #[test]
    fn iter_mut(){
        let mut list = List::new();
        list.push("A");list.push("B");list.push("C");
        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut "C"));
        assert_eq!(iter.next(), Some(&mut "B"));
        assert_eq!(iter.next(), Some(&mut "A"));
    }
}
