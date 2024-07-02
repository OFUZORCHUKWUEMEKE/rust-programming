use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::{time::Duration, vec};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread_example();
    spawn_thread();
}

fn thread_example() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Heres a vector : {:?}", v);
    });

    handle.join().unwrap();
}

fn spawn_thread() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals =vec![
            String::from("hi"),
            String::from("My Name "),
            String::from("is"),
            String::from("Ofuzor Chukwuemeke"),
        ];
      for val in vals{
         tx.send(val).unwrap();
         thread::sleep(Duration::from_secs(1));
      }
    });

    for recieved in rx {
        println!("Got :{}",recieved);
    }
}
