use wasm_bindgen::prelude::*;

// 告诉 wasm-pack 需要将这个函数编译成 wasm 可执行文件
// wasm-pack 将rust 代码编译成能够被js 导入的模块

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}
