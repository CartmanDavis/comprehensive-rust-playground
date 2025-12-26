use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::sync_channel(3);

    tx.send("Message 1");
    tx.send("Message 2");
    tx.send("Message 3");
    // tx.send("Message 4");

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}
