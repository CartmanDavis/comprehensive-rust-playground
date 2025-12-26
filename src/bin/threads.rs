use std::thread;
use std::time::Duration;

fn main() {
    let builder = thread::Builder::new();

    let handler: thread::JoinHandle<_> = builder.spawn(|| {
        for i in 0..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    }).unwrap();


    handler.join().expect("couldn't join");

    for i in 0..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}
