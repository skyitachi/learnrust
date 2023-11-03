#[macro_use]
extern crate tantivy;
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

    let df2 = ctx.read_csv("tests/example.csv", CsvReadOptions::new()).await?;

    df2.filter(col("a").lt_eq(col("b")))?
        .aggregate(vec![col("a")], vec![min(col("b"))])?
        .limit(Some(0), Some(100))?;

    df2.show().await?;

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

// #[cfg(test)]
// mod test_lance {
//     use std::env::temp_dir;
//     use datafusion::arrow::record_batch::RecordBatchReader;
//     use ::lance::dataset::Dataset;
//     use ::lance::dataset::WriteParams;
//     use std::sync::Arc;
//     use arrow::array::{FixedSizeListArray, Float32Array, Array, ArrayDataBuilder};
//     use arrow::datatypes::{DataType, Field, Schema as ArrowSchema};
//     use std::iter::repeat_with;
//     use arrow::datatypes::DataType::FixedSizeList;
//     use arrow::record_batch::RecordBatch;
//     use lance::arrow::RecordBatchBuffer;
//     use rand::prelude::*;

//     pub fn generate_random_array(n: usize) -> Float32Array {
//         let mut rng = rand::thread_rng();
//         Float32Array::from(
//             repeat_with(|| rng.gen::<f32>())
//                 .take(n)
//                 .collect::<Vec<f32>>(),
//         )
//     }

//     #[derive(Debug)]
//     pub enum Error {
//         Arrow(String),
//         Schema(String),
//         IO(String),
//         Index(String),
//         /// Stream early stop
//         Stop(),
//     }

//     pub type Result<T> = std::result::Result<T, Error>;

//     pub trait FixedSizeListArrayExt {
//         /// Create an [`FixedSizeListArray`] from values and list size.
//         ///
//         /// ```
//         /// use arrow_array::{Int64Array, FixedSizeListArray};
//         /// use arrow_array::types::Int64Type;
//         /// use lance::arrow::FixedSizeListArrayExt;
//         ///
//         /// let int_values = Int64Array::from_iter(0..10);
//         /// let fixed_size_list_arr = FixedSizeListArray::try_new(int_values, 2).unwrap();
//         /// assert_eq!(fixed_size_list_arr,
//         ///     FixedSizeListArray::from_iter_primitive::<Int64Type, _, _>(vec![
//         ///         Some(vec![Some(0), Some(1)]),
//         ///         Some(vec![Some(2), Some(3)]),
//         ///         Some(vec![Some(4), Some(5)]),
//         ///         Some(vec![Some(6), Some(7)]),
//         ///         Some(vec![Some(8), Some(9)])
//         /// ], 2))
//         /// ```
//         fn try_new<T: Array>(values: T, list_size: i32) -> Result<FixedSizeListArray>;
//     }

//     impl FixedSizeListArrayExt for FixedSizeListArray {
//         fn try_new<T: Array>(values: T, list_size: i32) -> Result<Self> {
//             let list_type = DataType::FixedSizeList(
//                 Arc::new(Field::new("item", values.data_type().clone(), true)),
//                 list_size,
//             );
//             let data = ArrayDataBuilder::new(list_type)
//                 .len(values.len() / list_size as usize)
//                 .add_child_data(values.into_data())
//                 .build()?;

//             Ok(Self::from(data))
//         }
//     }

//     #[tokio::test]
//     async fn test_lance() {
//         let test_dir = temp_dir();

//         let mut write_params = WriteParams::default();

//         let dimension = 16;

//         let schema = Arc::new(ArrowSchema::new(vec![Field::new(
//             "embeddings",
//             DataType::FixedSizeList(
//                 Arc::new(Field::new("item", DataType::Float32, true)),
//                 dimension,
//             ),
//             false,
//         )]));

//         let float_arr = generate_random_array(512 * dimension as usize);
//         let vectors = Arc::new(FixedSizeListArray::try_new(
//             float_arr, dimension).unwrap());

//         let batches = RecordBatchBuffer::new(vec![RecordBatch::try_new(
//             schema.clone(),
//             vec![vectors.clone()],
//         ).unwrap()]);

//         let mut reader: Box<dyn RecordBatchReader<Item=()>> = Box::new(batches);


//     }

// }


#[cfg(test)]
mod test_arrow {

    use arrow::array::{Int32Array, StringArray};
    use arrow::datatypes::{Field, Schema};
    use arrow::ipc::writer::FileWriter;
    use arrow::ipc::reader::FileReader;
    use arrow::record_batch::RecordBatch;
    use arrow::util::pretty;
    use std::fs::File;
    use std::sync::Arc;

    #[test]
    fn basic_arrow_demo() {
        let schema = Schema::new(vec![
            Field::new("name", arrow::datatypes::DataType::Utf8, false),
            Field::new("age", arrow::datatypes::DataType::Int32, false),
        ]);

        let name = StringArray::from(vec!["Alice", "Bob", "Charlie"]);

        let age = Int32Array::from(vec![25, 30, 35]);

        let batch = RecordBatch::try_new(Arc::new(schema), vec![
            Arc::new(name) as Arc<dyn arrow::array::Array>,
            Arc::new(age) as Arc<dyn arrow::array::Array>,
        ])
        .unwrap();
        
        let path = "example.arrow";

        let file = File::create(path).unwrap();
        
        let mut writer = FileWriter::try_new(file, &batch.schema()).unwrap();

        writer.write(&batch).unwrap();

        writer.finish().unwrap();

        let file = File::open(path).unwrap();
        let reader = FileReader::try_new(file, Some(vec![0, 1])).unwrap();

        let mut batches = Vec::new();

        for batch in reader {
            let batch = batch.unwrap();
            batches.push(batch);
        }
        
        // for batch in batches {
        //     pretty::print_batches(&[batch]).unwrap();
        // }

        pretty::print_batches(batches.as_slice()).unwrap();
    }

}


#[cfg(test)]
mod test_tantivy {


    use tantivy::collector::TopDocs;
    use tantivy::Index;
    use tantivy::query::QueryParser;
    use tantivy::schema::*;
    use tantivy::schema::Type::I64;
    use tempfile::TempDir;
    use tantivy::ReloadPolicy;


    #[test]
    fn tantivy_demo() -> tantivy::Result<()> {
        let index_path = TempDir::new()?;

        let mut schema_builder = Schema::builder();

        schema_builder.add_i64_field("id", NumericOptions::default().set_indexed().set_stored());
        schema_builder.add_facet_field("catid", FacetOptions::default().set_stored());

        schema_builder.add_text_field("name", TextOptions::default().set_stored());
        schema_builder.add_text_field("pname", TextOptions::default().set_stored());

        let schema = schema_builder.build();

        let index = Index::create_in_dir(&index_path, schema.clone()).unwrap();

        let mut index_writer = index.writer(50_000_000).unwrap();

        let id = schema.get_field("id").unwrap();
        let catid = schema.get_field("catid").unwrap();
        let name = schema.get_field("name").unwrap();
        let pname = schema.get_field("pname").unwrap();

        let mut doc = Document::default();

        doc.add_i64(id, 1000);
        doc.add_i64(id, 10001);

        index_writer.add_document(doc)?;

        index_writer.commit()?;
        // doc.add_facet(catid, );

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![id]);

        let query = query_parser.parse_query("10001")?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(2)).unwrap();

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("{}", schema.to_json(&retrieved_doc));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_demo_snippets {

    #[test]
    fn run_enum_demo() {
        enum Book {
            Papery {
                name: String,
                price: u32,
                author: String,
            },
            Electronic {
                name: String,
                url: String,
                author: String,
                filetype: String,
            },
        }
        let book = Book::Papery { name: String::from("rust"), price: 20, author: String::from("abc") };
        let ebok: Book = Book::Electronic {
            name:String::from("elec"), 
            url: String::from("abc.com"), 
            author: String::from("author abc"), 
            filetype: String::from("a"),
        };
        match book {
            Book::Papery { name, price, author } => {
                println!(
                    "Papery book name: {}, price:{} author: {}",
                    name, price, author
                );
            }
            Book::Electronic { name, url, author, filetype } => {
                println!(
                    "E-book name: {} url:{} author: {} filetype:{}",
                    name, url, author, filetype
                );
            }
        }
        
    }
}