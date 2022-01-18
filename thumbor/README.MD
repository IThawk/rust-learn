# 安装proto
## 一、首先下载准备
https://github.com/protocolbuffers/protobuf/releases

## 二、配置环境变量
解压后复制路径

![](images\protoc.jpg)



![](images\protoc1.jpg)

## 三、执行编译
```toml
[dependencies]
bytes = "1" # 处理字节流
prost = "0.8" # protobuf 处理

[build-dependencies]
prost-build ={ version = "0.8" }# 编译 protobuf
```

添加build.rs 到根目录
```rust
use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("src/pd")
        .compile_protos(&["src/abi.proto"], &["src"])?;
    Ok(())
}

```
```
cargo build
   Compiling thumbor v0.1.0 (D:\workspace\language\github\rust-learn\thumbor)
    Finished dev [unoptimized + debuginfo] target(s) in 1.58s
```