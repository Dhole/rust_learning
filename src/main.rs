
mod data_structures {
    pub struct LinkedList<T> {
        length: u32,
        list: Box<List<T>>,
    }
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil
    }
    impl<T> LinkedList<T> {
        pub fn new() -> LinkedList<T> {
            LinkedList { length: 0, list: Box::new(List::Nil) }
        }
        pub fn push(&mut self, elem: T) {
            self.length += 1;
            self.list = Box::new(List::Nil);
            //self.list = Box::new(List::Cons(elem, self.list));
        }
        /*
           fn pop(&mut self) -> T {

           }
           */
        /*
        fn print(self) {
            match self.list {
                List::Cons(v, l) => { println!("{}", v); &l.print() },
                Nil => {},
            }
        }
        */
    }

}

use data_structures::LinkedList;

fn main() {
    let mut l = LinkedList::<i32>::new();
    l.push(1);
    l.push(2);
    l.push(3);
    //l.print();
    println!("Hello, world!");
}