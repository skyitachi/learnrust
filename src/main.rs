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
mod polars_test {
    use polars::prelude::*;
    use polars::frame::*;

    #[test]
    fn run_series_example() {
        let s: Series = (0..10).map(Some).collect();
        println!("series s1 = {}", s);
        let s = Series::new("foo", &[1, 2, 3]);
        println!("series s2 = {}", s);

        let ca = UInt32Chunked::new("foo", &[Some(1), None, Some(3)]);
        let s = ca.into_series();

        println!("series s3 = {}", s);
    }

    #[test]
    fn run_dataframe_example() {
        let df = DataFrame::default();
        assert!(df.is_empty());

        {
            let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
            let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

            let df: PolarsResult<DataFrame> = DataFrame::new(vec![s1, s2]);
        }

    }
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
    use std::sync::{Arc, mpsc};
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
            // let val = String::from("hello");
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            // tx.send(val).unwrap();

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        });

        for received in rx {
            println!("Got {}", received)
        }
    }

    #[test]
    fn test_multi_producer() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }

    }

    use std::sync::Mutex;

    #[test]
    fn test_shared_memory_concurrency() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let counter = Arc::clone(&counter);

            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
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

#[cfg(test)]
mod test_openal {
}


#[cfg(test)]
mod test_bitpacking {

    use bitpacking::{BitPacker4x, BitPacker, BitPacker1x};
    use datafusion::parquet::data_type::AsBytes;
    use polars::export::arrow::compute::arithmetics::mul;

    #[test]
    fn bitpacking_demo() {
        let bitpacker = BitPacker4x::new();
        let mut my_data = [0u32; 128];
        for i in 0..128 {
            my_data[i] = i as u32;
        }
        let num_bits: u8 = bitpacker.num_bits(&my_data);
        let mut compressed = vec![0u8; 4 * BitPacker4x::BLOCK_LEN];
        let compressed_len =
            bitpacker.compress(&my_data, &mut compressed[..], num_bits);

        println!("compress len: {}", compressed_len);
    }
}

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
