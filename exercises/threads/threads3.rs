// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
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

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // 克隆发送端，每个线程一个
    let tx1 = tx.clone();
    let tx2 = tx;

    // 线程 1：发送 first_half
    thread::spawn(move || {
        for val in q.first_half {
            println!("sending {:?}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 线程 2：发送 second_half
    thread::spawn(move || {
        for val in q.second_half {
            println!("sending {:?}", val);
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break; // 所有数据接收完毕，退出循环
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}