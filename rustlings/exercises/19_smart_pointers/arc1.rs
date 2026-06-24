//在本练习中，我们得到一个名为“numbers”的“u32”的“Vec”，其中包含值
//范围从0到99。我们想使用8以内的这组数字
//不同线程同时进行。每个线程都会得到总和
//每八个值都有一个偏移量。
//
//第一个线程（偏移量 0）将对 0, 8, 16, … 求和
//第二个线程（偏移量 1）将对 1, 9, 17, … 求和
//第三个线程（偏移量 2）将对 2, 10, 18, … 求和
//…
//第八个线程（偏移量 7）将对 7, 15, 23, … 求和
//
//每个线程应该拥有一个指向向量的引用计数指针
//数字。但 `Rc` 不是线程安全的。因此，我们需要使用“Arc”。
//
//不要因为线程的生成和连接方式而分心。我们将练习
//稍后在有关线程的练习中。

//不要更改下面的行。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Define `shared_numbers` by using `Arc`.
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
