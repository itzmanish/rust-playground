// concurrency, threads, channels and Mutex<T>/Arc<T>
use std::sync::{Arc, Mutex};

fn main() {
    // let (tx, rx) = mpsc::channel();
    // std::thread::spawn(move || {
    //     let v = String::from("hi");
    //     tx.send(v).unwrap();
    // });

    // let recv = rx.recv().unwrap();
    // println!("Got value from thread: {}", recv);

    // let (tx, rx) = mpsc::channel();

    // for value in 0..10 {
    //     let tx = tx.clone();
    //     std::thread::spawn(move || {
    //         tx.send(value).unwrap();
    //     });
    // }

    // for reciever in rx {
    //     println!("got value from thread: {}", reciever);
    // }

    // let m = Mutex::new(0);
    // {
    //     let mut value = m.lock().unwrap();
    //     *value = 2;
    // }
    // println!("m = {:?}", m)

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
