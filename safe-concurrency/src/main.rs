use std::{thread, time};

fn main() {
    child_threads();
    data_sharing();
}

fn child_threads() {
    let duration = time::Duration::from_millis(100);

    println!("Parent thread");

    let handle = thread::spawn(move || {
        println!("Inner thread 1");

        let handle2 = thread::spawn(move || {
            println!("Inner thread 2");
            thread::sleep(duration);
        });

        handle2.join().unwrap();
        thread::sleep(duration);
    });

    handle.join().unwrap();

    thread::sleep(duration);
}

fn data_sharing() {
    let mut vec = vec![1, 2, 3];
    thread::spawn(move || {
        vec.push(4);
    });
}
