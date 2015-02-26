mod data_structures {
    use std::mem;

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
            //self.list = Box::new(List::Nil);
            /*
            let tmp_list = mem::replace(&mut self.list, Box::new(List::Nil));
            self.list = Box::new(List::Cons(elem, tmp_list));
            */
            let tmp_list = mem::replace(&mut *self.list, List::Nil);
            *self.list = List::Cons(elem, Box::new(tmp_list));
        }
        pub fn pop(&mut self) -> Option<T> {
            let tmp_list = mem::replace(&mut *self.list, List::Nil);
            match tmp_list {
                List::Cons(v, l) => {
                    self.length -= 1;
                    *self.list = *l; 
                    Some(v) },
                List::Nil => None,
            }
        }
        pub fn len(&self) -> u32 {
            self.length
        }

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
use std::rand;
use std::rand::Rng;

fn main() {
    let mut l = LinkedList::<i32>::new();
    let mut rng = rand::thread_rng();
    for x in 0..100000 {
        l.push(rng.gen());
        //l.pop();
    }
    l.push(1);
    l.push(2);
    l.push(3);
    //l.print();
    println!("{} {} {}", l.pop().unwrap(), l.pop().unwrap(), 
             l.pop().unwrap());
    println!("Final lenght: {}", l.len());
    println!("Hello, world!");
}
