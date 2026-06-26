//该程序生成多个线程，每个线程运行至少 250 毫秒，并且
//每个线程返回完成所需的时间。该程序应该
//等待所有生成的线程完成并收集它们的线程
//将值返回到向量中。

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        results.push(handle.join().unwrap());
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
