#[cfg(test)]
mod tests {
    use std::thread;
    use vart::{art, FixedSizeKey};
    use vart::art::{Tree};
    use std::sync::{Arc};

    #[test]
    fn concurrent_vart() {

        thread::scope(|s| {
            let mut tree = Arc::new(Tree::<FixedSizeKey<8>, _>::new());
            // prepare
            let rk: u8 = 1;
            let key = rk.into();

            tree.insert(&key, 100, 1, 1).expect("TODO: panic message");
            let mut snapshot= Arc::new(tree.create_snapshot().unwrap());

            let t1 = tree.clone();

            // NOTE: 不好在多线程的场景下使用
            s.spawn(|| {
                let mut rk: u8 = 2;
                let mut key = rk.into();
                (&t1).insert(&key, 100, 1, 1).expect("TODO: panic message");

                let (_, val, _, _) = (&t1).get(&key, 1).unwrap();
                println!("read from tree key {} is: {}", rk, val);

                rk = 1;
                key = rk.into();
                let (_, val, _, _) = tree.get(&key, 1).unwrap();
                println!("read from tree key {} is: {}", rk, val);
            });

            s.spawn(|| {
                let mut rk: u8 = 1;
                let mut key = rk.into();
                let (val, _, _) = snapshot.get(&key).unwrap();
                println!("snapshot read 1 is: {}", val);

                rk = 2;
                key = rk.into();
                println!("snapshot read 2 should be failed: {}", snapshot.get(&key).is_ok());

            });

        });


    }
}
