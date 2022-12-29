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
        println!("Hello {name}");
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
