use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use threadpool::ThreadPool;

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
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
    fn concurrent_counter() {
        let shared_counter = &AtomicUsize::new(0);


        thread::scope(|s| {
            let j1 = s.spawn(|| {
                for i in 0..1000000 {
                    shared_counter.fetch_add(1, Ordering::Acquire);

                }

            });

            let j2 = s.spawn(|| {
                for i in 0..1000000 {
                    shared_counter.fetch_add(1, Ordering::Acquire);

                }
            });
            j1.join().expect("TODO: panic message");
            j2.join().expect("TODO: panic message");
        });
        println!("counter: {}", shared_counter.load(Ordering::Relaxed));
    }

    #[test]
    fn atomic_demo() {
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::AtomicU64;
        use std::sync::atomic::Ordering::Relaxed;
        use std::thread;
        use std::time::Duration;
        use std::time::Instant;
        let num_done = &AtomicUsize::new(0);
        let total_time = &AtomicU64::new(0);
        let max_time = &AtomicU64::new(0);

        fn process_item(_: usize) {
            thread::sleep(Duration::from_millis(123));
        }

        thread::scope(|s| {
            // Four background threads to process all 100 items, 25 each.
            for t in 0..4 {
                s.spawn(move || {
                    for i in 0..25 {
                        let start = Instant::now();
                        process_item(t * 25 + i); // Assuming this takes some time.
                        let time_taken = start.elapsed().as_micros() as u64;
                        num_done.fetch_add(1, Relaxed);
                        total_time.fetch_add(time_taken, Relaxed);
                        max_time.fetch_max(time_taken, Relaxed);
                    }
                });
            }

            // The main thread shows status updates, every second.
            loop {
                let total_time = Duration::from_micros(total_time.load(Relaxed));
                let max_time = Duration::from_micros(max_time.load(Relaxed));
                let n = num_done.load(Relaxed);
                if n == 100 { break; }
                if n == 0 {
                    println!("Working.. nothing done yet.");
                } else {
                    println!(
                        "Working.. {n}/100 done, {:?} average, {:?} peak",
                        total_time / n as u32,
                        max_time,
                    );
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Done!");
    }
}