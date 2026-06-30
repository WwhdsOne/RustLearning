//Clippy 工具是 lint 的集合，用于分析您的代码，以便您可以
//捕获常见错误并改进您的 Rust 代码。
//
//对于这些练习，当有 Clippy 时代码将无法编译
//警告。检查 Clippy 输出中的建议以解决练习。

use std::f32::consts::PI;

fn main() {
    // TODO: Fix the Clippy lint in this line.
    let pi = PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
