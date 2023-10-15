// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
//     let qc = Arc::new(q);
//     let qc1 = Arc::clone(&qc);
//     let qc2 = Arc::clone(&qc);

//     thread::spawn(move || {
//         for val in &qc1.first_half {
//             println!("sending {:?}", val);
//             tx.send(*val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         for val in &qc2.second_half {
//             println!("sending {:?}", val);
//             tx.send(*val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let queue = Queue::new();
//     let queue_length = queue.length;

//     send_tx(queue, tx);

//     let mut total_received: u32 = 0;
//     for received in rx {
//         println!("Got: {}", received);
//         total_received += 1;
//     }

//     println!("total numbers received: {}", total_received);
//     assert_eq!(total_received, queue_length)
// }
fn send_tx(q: Arc<Mutex<Queue>>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let q1 = Arc::clone(&q);
    let q2 = Arc::clone(&q);
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    thread::spawn(move || {
        let q1 = q1.lock().unwrap();
        let tx1 = tx1.lock().unwrap();
        for val in &q1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let q2 = q2.lock().unwrap();
        let tx2 = tx2.lock().unwrap();
        for val in &q2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let sender = Arc::new(Mutex::new(tx));
    let queue_length = {
        let q = queue.lock().unwrap();
        q.length
    };

    send_tx(queue.clone(), sender.clone());

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
