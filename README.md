用于学习[Rust](https://www.rust-lang.org/)的仓库，

## 文章

- [24 days from node.js to Rust](https://vino.dev/blog/node-to-rust-day-1-rustup/) | [中文](https://juejin.cn/post/7037769953440858120)

## 创建项目

```bash
$ cargo new my-app
```

## 打包&运行

```bash
$ cargo run
```

`cargo run`会调用`cargo build`来打包应用，然后运行默认（也可以指定）的二进制文件，运行完这个命令后，会生成好二进制文件，
具体的位置是`./target/debug/my-app`。如果只是想打包而不不运行，可以执行`cargo build`命令。
默认情况下会生成文件大小、性能等debug信息，当想要得到`release`版代码时，可以运行`cargo build --release`，
此时二进制文件的生成目录是`./target/release/my-app`

## 变量声明 & 可变性

`JavaScript`中的变量分为可变和不可变，分别用`let`和`const`来修饰，`rust`中也有`let`和`const`，不过这里需要先忽略`const`。在`JavaScript`中使用`const`的场景，在`rust`中需要的是`let`而非`const`；在`JavaScript`中使用`let`的场景，在`rust`中需要的是`let mut`。默认情况下在`rust`的世界里变量都是不可变的，这是个好事情。

> 在`rust`中如果变量是可变的，则更改数值的时候需要注意不要更改变量类型




## Move & Copy & Borrow

1. 一个值在同一时刻只有一个所有者。当所有者离开作用域，其拥有的值会被丢弃。赋值或者传参会导致值 Move，所有权被转移，一旦所有权转移，之前的变量就不能访问。
2. 如果值实现了 Copy trait，那么赋值或传参会使用 Copy 语义，相应的值会被按位拷贝，产生新的值。
3. 一个值可以有多个只读引用。
4. 一个值可以有唯一一个活跃的可变引用。可变引用（写）和只读引用（读）是互斥的关系，就像并发下数据的读写互斥那样。
5. 引用的生命周期不能超出值的生命周期。

![borrow copy move](https://user-images.githubusercontent.com/21967852/144003189-fd7b02b3-2214-4475-a5b7-2a329ed04c1b.jpeg)
