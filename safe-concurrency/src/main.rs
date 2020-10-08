use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    child_threads();
    single_thread_vector_update();
    multiple_threads_vector_update()
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

fn single_thread_vector_update() {
    let mut vec = vec![1, 2, 3];
    thread::spawn(move || {
        vec.push(4);
    });
    // Using 'vec' here would fail with "value borrowed here after move".
}

// The main structure for sharing data between threads is Arc, which is a
// reference counter allowing multiple threads to take ownership of some data.
// The reference counting ensures that the data gets cleaned up after all
// ‘owners’ of the data have finished executing. A Mutex is a sconstruct
// that allows performing atomic operations within threads. In order to use
// the shared data .lock() must be called on the mutex, which will
// block other threads. As long as one thread has the mutex locked, other
// threads will be blocked.
fn multiple_threads_vector_update() {
    let arc = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let cloned = arc.clone();
        thread::spawn(move || {
            cloned.lock().unwrap().push(i);
        });
    }
}
