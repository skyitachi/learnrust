use std::thread;
use std::time::Duration;
use std::sync::mpsc;
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn spawn() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("number {} spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..10 {
            println!("number {} from main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn spawn_wait() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("number {} spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..10 {
            println!("number {} from main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn move_demo() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("display vector in child thread: {:?}", v);
        });

        handle.join().unwrap();
    }

    #[test]
    fn mpsc_demo() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(1).unwrap();
        });

        println!("receive: {}", rx.recv().unwrap());
    }

    #[test]
    fn mpsc_demo2() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            tx.send(String::from("hi from raw tx")).unwrap();
        });

        thread::spawn(move || {
            tx1.send(String::from("hi from cloned tx")).unwrap();
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn future_demo() {
        let pool = ThreadPool::new().expect("Failed to build threadpool");
        let (tx, rx) = mpsc::unbounded::<i32>();

        let fut_values = async {
            let fut_tx_result = async move {
                (0..100).for_each(|v| {
                    tx.unbound_send(v).expect("Failed to send");
                })
            };
            pool.spawn_ok(fut_tx_result);

            let fut_values = rx
                .map(|v| v * 2)
                .collect();

            fut_values.await
        };

        let values: Vec<i32> = executor::block_on(fut_values);

        println!("Values: {:?}", values);
    }
}