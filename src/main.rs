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
