//Multi Threads programming ##Example

use std::thread;
use std::time::Duration;


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}



//Asynchronouse Programming ##Example

use async_std::task;
use futures::executor::block_on;
use std::time::Duration;
use std::time::Instant;
use std::thread;


fn main() {
    let start = Instant::now();
    block_on(async_functions());
    let elapsed = start.elapsed();
    println!("Calculated time is: {:?}", elapsed);
}

async fn async_functions() {
    task::spawn(listen_songs());
    running().await;
    push_ups();

}

async fn running() {
    println!("Running start");
    task::sleep(Duration::from_millis(2000)).await;
    println!("Running end");
}

async fn listen_songs() {
    println!("Start music");
    task::sleep(Duration::from_millis(5000)).await;
}

fn push_ups() {
    println!("Push ups start");
    thread::sleep(Duration::from_millis(2000));
    println!("Push ups end");
}

