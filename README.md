# yew-actix-graphql-mongodb-boilerplate
boilerplate project using yew + actix-web + graphql + mongodb


## 技术栈选型
### 前端
* [yew](https://yew.rs/) - yew 是一个设计先进的 Rust 框架，目的是使用 WebAssembly 来创建多线程的前端 web 应用。
* [tokio](https://tokio.rs/) - tokio 是 Rust 中的异步编程框架，它将复杂的异步编程抽象为 Futures、Tasks 和 Executor，并提供了 Timer 等基础设施。
* [wasm-pack](https://github.com/rustwasm/wasm-pack) - wasm-pack 是 Rust-Wasm 官方工作组开发，用于构建wasm应用程序的工具。
* [reqwest](https://docs.rs/reqwest/) - reqwest 是一个简单的 Rust HTTP 客户端，reqwest的async使用的是Tokio的，所以要同时加入Tokio的依赖。

### 后端
* [actix-web](https://actix.rs/) - actix-web 是采用 Rust 开发的一个 Web 框架。它强大快速切于实际，是采用 Rust 进行 Web 开发的最佳选择。
* [hyper](https://docs.rs/hyper) - hyper是一个偏底层的http库，支持HTTP/1和HTTP/2，支持异步Rust，提供了服务端和客户端的API支持。
* [async-graphql](https://docs.rs/async-graphql) - async-graphql是用Rust语言实现的GraphQL服务端库。
* [tracing](https://docs.rs/tracing/) - tracing 是用于检测 Rust 程序以收集结构化的、基于事件的诊断信息的框架。
* [tracing-subscriber](https://docs.rs/tracing-subscriber) - tracing-subscriber 能够使用log库和模块发出的消息。
* [serde](https://serde.rs/) - serde 是rust语言用来序列化和反序列化数据的一个非常高效的解决方案。
* [mongodb](https://docs.rs/mongodb) - mongodb 是一个介于关系数据库和非关系数据库(nosql)之间的产品，是非关系数据库当中功能最丰富，最像关系数据库的。

### 数据库
* [MongoDB](https://github.com/mongodb/mongo-rust-driver) - 来自 MongoDB 官方支持的 MongoDB Rust 驱动程序，该客户端库可用于与 Rust 应用程序中的 MongoDB 部署进行交互。


# How to

### How to install MongoDB on Mac
```
$ brew install mongodb-community@4.0
$ mongod --dbpath .
```
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

cd client && trunk serve
```
visit http://localhost:3000/


## Referrence Links
### Yew
* https://yew.rs/zh-CN/docs/getting-started/build-a-sample-app

### Actix-web
* https://actix.rs/docs/getting-started/

### Actix-web + MongoDB
* https://github.com/actix/examples/tree/master/databases/mongodb

### 如何使用Actix-web和MongoDB构建简单博客网站
* https://github.com/nintha/demo-myblog

### Actix-web + GraphQL + MongoDB
* https://github.com/liamdebellada/Rust-Actix-GraphQL-MongoDB
* https://github.com/shareeff/rust_graphql_mongodb

### Yew + Todos
* https://github.com/yewstack/yew/tree/master/examples/todomvc

### Yew + Reqwest
* https://github.com/jetli/rust-yew-realworld-example-app
* Blog Demo 
  -  https://demo.realworld.io/

### Awesome Yew
* https://project-awesome.org/jetli/awesome-yew
