# yew-actix-graphql-mongodb-boilerplate
boilerplate project using yew + actix-web + graphql + mongodb


## 技术栈选型
### 前端
* [yew](https://yew.rs/) - Yew 是一个设计先进的 Rust 框架，目的是使用 WebAssembly 来创建多线程的前端 web 应用。
* [tokio](https://tokio.rs/) - Tokio 是 Rust 中的异步编程框架，它将复杂的异步编程抽象为 Futures、Tasks 和 Executor，并提供了 Timer 等基础设施
* [wasm-pack](https://github.com/rustwasm/wasm-pack) - Wasm-pack 是 Rust-Wasm 官方工作组开发，用于构建wasm应用程序的工具。

### 后端
* [actix-web](https://actix.rs/) - actix-web 是采用 Rust 开发的一个 Web 框架。它强大快速切于实际，是采用 Rust 进行 Web 开发的最佳选择。
* [hyper](https://docs.rs/hyper) - hyper是一个偏底层的http库，支持HTTP/1和HTTP/2，支持异步Rust，提供了服务端和客户端的API支持。
* [async-graphql](https://docs.rs/async-graphql) - Async-graphql是用Rust语言实现的GraphQL服务端库。
* [tracing](https://docs.rs/tracing/) - tracing 是用于检测 Rust 程序以收集结构化的、基于事件的诊断信息的框架。
* [tracing-subscriber](https://docs.rs/tracing-subscriber) - tracing-subscriber 能够使用log库和模块发出的消息。

### 数据库
* [MongoDB](https://www.mongodb.org.cn/) - MongoDB是一个介于关系数据库和非关系数据库(nosql)之间的产品，是非关系数据库当中功能最丰富，最像关系数据库的。



# How to
### How to build
```
$ cargo build
```

### How to run server
```
$ cargo run --bin server
```
visit http://localhost:8080/

### How to run client

```
cargo install trunk

cd client
trunk serve
```
visit http://localhost:3000/

