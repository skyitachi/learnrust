use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // register the table
    let ctx = SessionContext::new();
    ctx.register_csv("example", "tests/example.csv", CsvReadOptions::new()).await?;

    // create a plan to run a SQL query
    let df = ctx.sql("SELECT a, MIN(b) FROM example GROUP BY a LIMIT 100").await?;

    // execute and print results
    df.show().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::test::List::{Cons, Nil};

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    #[test]
    fn smart_pointer_demo() {
        let b = Box::new(10);
        println!("b = {}", b);
        let list = Cons(1, Box::new(Nil));
    }

    fn hello(name: &str) {
        println!("Hello {}", name);
    }

    #[test]
    fn demo_15_2() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);

        let m = Box::new(String::from("rust"));
        hello(&m);
    }


    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        // fn deref(&self) -> &Self::Target {
        //     &self.0
        // }
        fn deref(&self) -> &T {
            &self.0
        }
    }

    #[test]
    fn test_custom_smart_pointer() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        let m = MyBox::new(String::from("rust"));
        hello(&m);
    }
}

#[cfg(test)]
mod test2 {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::test2::List::{Cons, Nil};

    enum List {
        Cons(i32, Rc<List>),
        Nil
    }

    #[test]
    fn test_rc_type() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        {
            let b = Cons(3, Rc::clone(&a));
            println!("count after creating b = {}", Rc::strong_count(&a));
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after leave scope = {}", Rc::strong_count(&a));
    }

    // RefCell
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
        where
            T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }


}

#[cfg(test)]
mod test3 {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::test3::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    #[test]
    fn test_multi_owner_list() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    use std::rc::Weak;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[test]
    fn test_weak_pointer() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);


        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let a = Rc::new(10);
        let w = RefCell::new(Weak::new());
        *w.borrow_mut() = Rc::downgrade(&a);
    }
}


#[cfg(test)]
mod test_concurrent {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_use_spawn() {
        let handle = thread::spawn(|| {
           for i in 1..10 {
               println!("run in thread {:?} with number {}", thread::current().id(), i);
               thread::sleep(Duration::from_millis(i));
           }
        });

        for i in 1..5 {
            println!("run in thread {:?} with number {}", thread::current().id(), i);
            thread::sleep(Duration::from_millis(i));
        }

        handle.join().unwrap();
    }

    #[test]
    fn test_mpsc() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
           let val = String::from("hello");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
}
// fn main() {
//
//     let v = vec![1, 2, 3];
//     let handle = thread::spawn(move || {
//         println!("here's is a vector: {:?}", v);
//     });
//     handle.join().unwrap();
//
//     let listener = TcpListener::bind("localhost:10001").unwrap();
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         handle_connection(stream)
//     }
// }

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap()
}
