用于学习[Rust](https://www.rust-lang.org/)的仓库，

## 文章

- [24 days from node.js to Rust](https://vino.dev/blog/node-to-rust-day-1-rustup/) | [中文](https://juejin.cn/post/7037769953440858120)



## Cargo

- 使用 `cargo new` 命令创建新的项目模板。
- 使用 `cargo build` 编译项目。
- 使用 `cargo run` 命令编译并运行项目。
- 使用 `cargo test` 命令测试项目。
- 使用 `cargo check` 命令检查项目类型。
- 使用 `cargo doc` 命令编译项目的文档。
- 使用 `cargo publish` 命令将库发布到 crates.io。
- 使用`cargo install`全局安装包
- 通过包名添加到 Cargo.toml 文件来将依赖添加到项目。

### 创建项目

```bash
$ cargo new my-app
```

### 打包&运行

```bash
$ cargo run
```

`cargo run`会调用`cargo build`来打包应用，然后运行默认（也可以指定）的二进制文件，运行完这个命令后，会生成好二进制文件，
具体的位置是`./target/debug/my-app`。如果只是想打包而不不运行，可以执行`cargo build`命令。
默认情况下会生成文件大小、性能等debug信息，当想要得到`release`版代码时，可以运行`cargo build --release`，
此时二进制文件的生成目录是`./target/release/my-app`

### 安装依赖

在`Node.js`中我们使用`npm install [dep]`命令来安装依赖的模块，如果安装了 [cargo-edit](https://github.com/killercup/cargo-edit) 那么在`rust`中我们可以使用`cargo add [dep]`来增添依赖，安装方法：

```bash
$ cargo install cargo-edit
```

安装完毕后，会增加4个子命令：`add`、`rm`、`upgrade`、`set-version`

### 全局安装

类比`Node.js`中的`npm install --global`，在`rust`中我们使用`cargo install`

### 运行测试

类比`Node.js`中的`npm test`，在`rust`中我们使用`cargo test`

### 发布模块

类比`Node.js`中的`npm publish`，在`rust`中我们使用`cargo publish`

### 运行任务

在`Node.js`中我们使用`npm run xxx`来运行任务，然而在`rust`中除了几个常见的命令外，其余的都是取决于用户自己。例如我们可以通过`cargo run`来运行一个代码，或者用`cargo bench`分析一段代码的性能，或者是用`cargo build`来做打包，或者是用`cargo clean`来清空打包目录（默认是`target`），或者是用`cargo doc`生成文档。`cargo`还支持 [Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html#build-scripts) 机制确保可以打包之前运行指定的程序。

在`JavaScript`中已经不需要`Makefile`，但在`Rust`中就没那么幸运了，`Makefile`还是很常见。不过 [just](https://github.com/casey/just) 正在被广泛接受，它弥补了`Makefile`的一些弱点，语法上也很相似，可以如下安装：

```bash
$ cargo install just
```

`cargo-make`和`cargo-cmd`也是很好的替代方案

### workspace与monorepo

包管理器在处理大项目中的小模块时一般都会用到`workspace`概念，在`Rust`中你可以在根目录下创建一个`Cargo.toml`文件作为`workspace`的入口，描述清楚`workspace`中所包含的内容，大体上类似如下：

```toml
[workspace]
members = [
  "crates/*"
]
```

`workspace`中相互引用的各个模块可以指向本地的文件夹作为依赖项：

```toml
[dependencies]
other-project = { path = "../other-project" }
```

## 变量声明 & 可变性

`JavaScript`中的变量分为可变和不可变，分别用`let`和`const`来修饰，`rust`中也有`let`和`const`，不过这里需要先忽略`const`。在`JavaScript`中使用`const`的场景，在`rust`中需要的是`let`而非`const`；在`JavaScript`中使用`let`的场景，在`rust`中需要的是`let mut`。默认情况下在`rust`的世界里变量都是不可变的，这是个好事情。

> 在`rust`中如果变量是可变的，则更改数值的时候需要注意不要更改变量类型

## 字符串

### &str

字符串字面量本质是一个字符串切片的引用，这意味着实际上它们是字符串数据的子字符串指针。`rust`编译器会将我们所有的字符串字面量存放在某个地方，然后将它们的值替换为指针，这就让`rust`优化掉了重复字符串的问题。你可以通过代码来验证这种优化现象，将下面代码中的字符串复制`n`次，然后查看下编译打包出来的可执行文件体积：

```rust
fn main() {
  print("TESTING:12345678901234567890123456789012345678901234567890");
}

fn print(msg: &str) {
  println!("{}", msg);
}

```

我们可以对生成的二进制文件用`strings`命令（这不是`rust`提供的功能）进行探查：

```bash
$ strings target/release/200-prints | grep TESTING
TESTING:12345678901234567890123456789012345678901234567890
```

### String

`String`可以被改变，支持截取、缩小、增长等操作，当然这也会带来额外的成本

### &str转化成String

简单来讲，用`.to_owned()`方法可以将`&str`（一个字符串切片的引用）变为有所有权的`String`，例如：

```rust
let my_real_string = "string literal!".to_owned();
```

底层实际上是这样的：

```rust
String::from_utf8_unchecked(self.as_bytes().to_owned())
```

> 注意：`self`就是`rust`中的`this`

> 字符串字面量只是一个引用，没有所有权，想变成有所有权的`string`，需要进行格式的转化。是不是这意味着我们要每次都使用`.to_owned()`呢？既是也不是

## Some()、None、Option

`some()`实际上是 [`Option` enum](https://doc.rust-lang.org/std/option/enum.Option.html) 的一个变体，`Option`是另一种表达“无”的方式


> `rust`中没有`undefined`，也没有`null`，取而代之的是`Option`，它有两个变体：`Some()`和`None`
>
> 注意：我们可以用`.is_some()`或`.is_none()`来检测一个`Option`；可以通过`.unwrap()`来获取`Some`值，不过如果值是`None`的话代码会出问题，所以我们用`.unwrap_or(default_value)`代替。更多信息可见 [Option](https://doc.rust-lang.org/std/option/enum.Option.html)
>
> 

## HashMap

`HashMap`是存储和使用键值对的主要工具，在 [这里](https://doc.rust-lang.org/beta/std/collections/struct.HashMap.html) 可以阅读到更多有关信息

## Struct

`struct`使我们可以在`rust`中获得类似`JavaScript`中`class`行为的能力，这需要点时间来适应

```rust
struct TrafficLight {
  color: String,
}

impl TrafficLight {
  pub fn new() -> Self {
    Self {
      color: "red".to_owned(),
    }
  }
}

fn main() {
  let light = TrafficLight::new();
}  
```

### 添加方法

在`TypeScript`中，默认情况下添加到`class`中的方法是`public`的并且是添加到原型链上，所有实例都可以访问到。但在`rust`中，任何`impl`中的函数默认都是`private`的。为了创建一个方法，你需要指定第一个参数是`self`。使用`self`时不必每次都指定它的类型，如果你写`&self`，那么类型默认就是`Self`的引用，当你写`self`时，类型就是`Self`。在一些公共库里你会见到更奇怪的`self`类型，不过这是后话。

> 注意：你或许会注意到有时候你拿到了某个`struct`的实例而且你也知道它具备方法，但是你就是无法访问到，通常有两种原因：一种是你想访问的是一个`trait`方法，但是没有导入，你需要导入这个`trait`（例如`use [...]::[...]::Trait;`）；第二种原因是你的实例需要用指定类型来包裹，如果你看到了类似这样的一个函数：`n work(self: Arc<Self>)`，那么你可以用`Arc`包裹该实例然后访问`.work()`




`TypeScript`将`getState`方法添加到`class`上:

```ts
class TrafficLight {
  color: string;
  constructor() {
    this.color = "red";
  }
  getState(): string {
    return this.color;
  }
}

const light = new TrafficLight();
console.log(light.getState());

```

`rust`:

```rust
fn main() {
  let mut light = TrafficLight::new();
  println!("{:?}", light);
}

impl std::fmt::Display for TrafficLight {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Traffic light is {}", self.color)
  }
}

#[derive(Debug)]
struct TrafficLight {
  color: String,
}

impl TrafficLight {
  pub fn new() -> Self {
    Self {
      color: "red".to_owned(),
    }
  }

  // 添加方法
  pub fn get_state(&self) -> &str {
    &self.color
  }
}
```

### &self vs self

`struct`的方法第一个参数可以是引用的`self`，也可以是有所有权的`self`，但是有所有权就意味着在调用方法时调用方必然会失去所有权，也就是丢失了实例，我们可以试试将`&self`改为`self`：

```rust
ub fn get_state(self) -> String {
  self.color
}

```

然后调用两次：

```rust
fn main() {
  let light = TrafficLight::new();
  light.get_state();
  light.get_state();
}

```

编译的时候会发现无法通过编译，`rust`会告诉你你使用了一个被移除的内容：

```
error[E0382]: use of moved value: `light`
  --> crates/day-9/structs/src/main.rs:4:18
   |
2  |   let light = TrafficLight::new();
   |       ----- move occurs because `light` has type `TrafficLight`, which does not implement the `Copy` trait
3  |   println!("{}", light.get_state());
   |                        ----------- `light` moved due to this method call
4  |   println!("{}", light.get_state());
   |                  ^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `light`
  --> crates/day-9/structs/src/main.rs:25:20
   |
25 |   pub fn get_state(self) -> String {
   |                    ^^^^

For more information about this error, try `rustc --explain E0382`.

```

第一次调用的时候就丢失了所有权，可以从以下几个方面谈谈：

- 当你创建了一些数据并将他们进行转化，确实是将所有权转移给了新的数据
- 对象的销毁通常是与实例的创建一起完成
- 在`builder patterns`或链式的`API`，你可以使用一个有所有权的`mute`模式的`self`，并且返回它，以便其他方法可以链式使用

### Mutating state

目前我们创建的`TrafficLight`可用性还不是很强，比如不能改变`color`。在`rust`中所有的变量默认都是不能修改的，也包括`self`，我们需要将这个方法标记成需要一个可修改的`self`

```rust
pub fn turn_green(&mut self) {
  self.color = "green".to_owned()
}

```

同时我们还需要将`TrafficLight`的实例改为可变的，不然`rust`会继续报错，在`main()`中我们这样修改：

```rust
let mut light = TrafficLight::new();
```

完整代码：

```rust
use std::fmt::Formatter;

fn main() {
  let mut light = TrafficLight::new();
  println!("{:?}", light);
  light.turn_green();
  println!("{:?}", light)
}

impl std::fmt::Display for TrafficLight {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Traffic light is {}", self.color)
  }
}

#[derive(Debug)]
struct TrafficLight {
  color: String,
}

impl TrafficLight {
  pub fn new() -> Self {
    Self {
      color: "red".to_owned(),
    }
  }

  pub fn get_state(&self) -> &str {
    &self.color
  }
  pub fn turn_green(&mut self) {
    self.color = "green".to_owned()
  }
}

```

## 枚举（Enums）

在ts中：

```ts
class TrafficLight {
  color: TrafficLightColor;

  constructor() {
    this.color = TrafficLightColor.Red;
  }

  getState(): TrafficLightColor {
    return this.color;
  }

  turnGreen() {
    this.color = TrafficLightColor.Green;
  }
}

enum TrafficLightColor {
  Red,
  Yellow,
  Green,
}

const light = new TrafficLight();
console.log(light.getState());
light.turnGreen();
console.log(light.getState());

```

rust中：

```rust
use std::fmt::Display;

fn main() {
  let mut light = TrafficLight::new();
  println!("{}", light);
  println!("{:?}", light);
  light.turn_green();
  println!("{:?}", light);
}

impl std::fmt::Display for TrafficLight {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Traffic light is {}", self.color)
  }
}

#[derive(Debug)]
struct TrafficLight {
  color: TrafficLightColor,
}

impl TrafficLight {
  pub fn new() -> Self {
    Self {
      color: TrafficLightColor::Red,
    }
  }

  pub fn get_state(&self) -> &TrafficLightColor {
    &self.color
  }

  pub fn turn_green(&mut self) {
    self.color = TrafficLightColor::Green
  }
}

#[derive(Debug)]
enum TrafficLightColor {
  Red,
  Yellow,
  Green,
}

impl Display for TrafficLightColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let color_string = match self {
      TrafficLightColor::Green => "green",
      TrafficLightColor::Red => "red",
      TrafficLightColor::Yellow => "yellow",
    };
    write!(f, "{}", color_string)
  }
}

```



`trait`是强大的，是`struct`的支柱。数据和行为的分离很重要，但需要一些实践来适应

## Move & Copy & Borrow

1. 一个值在同一时刻只有一个所有者。当所有者离开作用域，其拥有的值会被丢弃。赋值或者传参会导致值 Move，所有权被转移，一旦所有权转移，之前的变量就不能访问。
2. 如果值实现了 Copy trait，那么赋值或传参会使用 Copy 语义，相应的值会被按位拷贝，产生新的值。
3. 一个值可以有多个只读引用。
4. 一个值可以有唯一一个活跃的可变引用。可变引用（写）和只读引用（读）是互斥的关系，就像并发下数据的读写互斥那样。
5. 引用的生命周期不能超出值的生命周期。

![borrow copy move](https://user-images.githubusercontent.com/21967852/144003189-fd7b02b3-2214-4475-a5b7-2a329ed04c1b.jpeg)
