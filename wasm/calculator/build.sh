#!/bin/bash

# 构建Rust WebAssembly模块
wasm-pack build --target web

# 复制生成的文件到www目录
cp -r pkg www

# 复制HTML文件到www目录
cp index.html www