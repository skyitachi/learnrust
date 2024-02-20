#[cfg(test)]
mod tests {
    use std::thread;
    use vart::{art, FixedSizeKey};
    use vart::art::{Tree};
    use std::sync::{Arc};

    #[test]
    fn concurrent_vart() {

        thread::scope(|s| {
            // let mut tree = Arc::new(Tree::<FixedSizeKey<8>, _>::new());
            // // prepare
            // let rk: u8 = 1;
            // let key = rk.into();

            // tree.insert(&key, 100, 1, 1).expect("TODO: panic message");
            // let mut snapshot= Arc::new(tree.create_snapshot().unwrap());
            //
            // let t1 = tree.clone();
            //
            // // NOTE: 不好在多线程的场景下使用
            // s.spawn(|| {
            //     let mut rk: u8 = 2;
            //     let mut key = rk.into();
            //     // (&t1).insert(&key, 100, 1, 1).expect("TODO: panic message");
            //
            //     let (_, val, _, _) = (&t1).get(&key, 1).unwrap();
            //     println!("read from tree key {} is: {}", rk, val);
            //
            //     rk = 1;
            //     key = rk.into();
            //     let (_, val, _, _) = tree.get(&key, 1).unwrap();
            //     println!("read from tree key {} is: {}", rk, val);
            // });
            //
            // s.spawn(|| {
            //     let mut rk: u8 = 1;
            //     let mut key = rk.into();
            //     let (val, _, _) = snapshot.get(&key).unwrap();
            //     println!("snapshot read 1 is: {}", val);
            //
            //     rk = 2;
            //     key = rk.into();
            //     println!("snapshot read 2 should be failed: {}", snapshot.get(&key).is_ok());
            //
            // });

        });


    }

    fn foo(x: &str) -> String {
        let a = "hello, ".to_string() + x;
        a
    }

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    trait Draw {
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("button id {}", self.id);
        }
    }

    struct Select {
        id: u32
    }

    impl Draw for Select {
        fn draw(&self) {
            println!("select id {}", self.id);
        }
    }

    fn gen_static_str() -> &'static str {
        let mut s = String::new();
        s.push_str("hello world");
        Box::leak(s.into_boxed_str())
    }

    #[test]
    fn box_test() {
        let b = foo("world");
        println!("{}", b);

        let a = Box::new(3);
        let arr = [0;1000];
        let arr1 = arr;

        println!("{:?}", arr.len());
        println!("{:?}", arr1.len());

        let arr2 = Box::new([0;1000]);
        let arr3 = arr2;
        println!("{:?}", arr3.len());
        // println!("{:?}", arr2.len());

        let first = Box::new(List::Nil);
        let second = Box::new(List::Cons(1, first));

        let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button{id: 1}), Box::new(Select{id: 2})];
        for e in elems {
            e.draw()
        }

        let ss = gen_static_str();
        println!("static string: {}", ss);
    }


    use std::rc::Rc;

    #[test]
    fn rc_test() {
        let a = Rc::new(String::from("hello, world"));
        println!("count after creating a = {}", Rc::strong_count(&a));

        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a));

        assert_eq!(2, Rc::strong_count(&a));
        assert_eq!(2, Rc::strong_count(&b));
        assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

        {
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    #[test]
    fn arc_test() {
        let s = String::from("hello world");
        let a = Box::new(s);
        // let b = Box::new(s);
    }
}
