```bash
$ cargo run -- -f 4
```

![img](https://cdn.jsdelivr.net/gh/MrSeaWave/figure-bed-profile@main/uPic/2021/y4yIBS_qeplNf.png)

## 打包

在 Mac OS 平台下打可执行程序：

```
$ cargo build --release
```

如果要交叉编译参考[rust-cross](https://github.com/japaric/rust-cross#cross-compiling-with-cargo)

or [超方便的 rust 交叉编译](https://moevis.github.io/cheatsheet/2018/08/18/%E8%B6%85%E6%96%B9%E4%BE%BF%E7%9A%84-Rust-%E4%BA%A4%E5%8F%89%E7%BC%96%E8%AF%91.html)

or [RUSTLANG OSX 下交叉编译LINUX](https://erasin.wang/rust-cross-build/)


## 安装链接器

`musl-cross` 似乎是在链接时候起作用的。

首先通过 `brew` 安装 `musl-cross` 工具，其中 `musl-cross` 是用来专门编译到 linux 的工具链，而 `mingw-w64` 是用来编译到 windows 的工具链。根据你的目标平台自己装一个就好。

```
$ brew install FiloSottile/musl-cross/musl-cross  
$ brew install mingw-w64  
```

我装的是 mingw-w64，装好后到 `~/.cargo/config.toml` or  `~/.cargo/config` ([Configuration](https://doc.rust-lang.org/cargo/reference/config.html))文件中添加上：

```
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

注意一下等于号后面跟的是 `x86_64-linux-musl-gcc`。



## rustup 配置
rustup 是 Rust 的包管理工具，用它来配置工具链可以很省心。

运行 `rustup target list` 你可以看到很多的目标平台。

```
$ rustup target list
aarch64-apple-darwin
aarch64-apple-ios
aarch64-apple-ios-sim
aarch64-fuchsia
aarch64-linux-android
aarch64-pc-windows-msvc
aarch64-unknown-linux-gnu
aarch64-unknown-linux-musl
aarch64-unknown-none
aarch64-unknown-none-softfloat
arm-linux-androideabi
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
armebv7r-none-eabi
armebv7r-none-eabihf
armv5te-unknown-linux-gnueabi
armv5te-unknown-linux-musleabi
armv7-linux-androideabi
armv7-unknown-linux-gnueabi
armv7-unknown-linux-gnueabihf
armv7-unknown-linux-musleabi
armv7-unknown-linux-musleabihf
armv7a-none-eabi
armv7r-none-eabi
armv7r-none-eabihf
asmjs-unknown-emscripten
i586-pc-windows-msvc
i586-unknown-linux-gnu
i586-unknown-linux-musl
i686-linux-android
i686-pc-windows-gnu
i686-pc-windows-msvc
i686-unknown-freebsd
i686-unknown-linux-gnu
i686-unknown-linux-musl
mips-unknown-linux-gnu
mips-unknown-linux-musl
mips64-unknown-linux-gnuabi64
mips64-unknown-linux-muslabi64
mips64el-unknown-linux-gnuabi64
mips64el-unknown-linux-muslabi64
mipsel-unknown-linux-gnu
mipsel-unknown-linux-musl
nvptx64-nvidia-cuda
powerpc-unknown-linux-gnu
powerpc64-unknown-linux-gnu
powerpc64le-unknown-linux-gnu
riscv32i-unknown-none-elf
riscv32imac-unknown-none-elf
riscv32imc-unknown-none-elf
riscv64gc-unknown-linux-gnu
riscv64gc-unknown-none-elf
riscv64imac-unknown-none-elf
s390x-unknown-linux-gnu
sparc64-unknown-linux-gnu
sparcv9-sun-solaris
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv7neon-linux-androideabi
thumbv7neon-unknown-linux-gnueabihf
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
wasm32-unknown-emscripten
wasm32-unknown-unknown
wasm32-wasi
x86_64-apple-darwin (installed)
x86_64-apple-ios
x86_64-fortanix-unknown-sgx
x86_64-fuchsia
x86_64-linux-android
x86_64-pc-solaris
x86_64-pc-windows-gnu (installed)
x86_64-pc-windows-msvc
x86_64-sun-solaris
x86_64-unknown-freebsd
x86_64-unknown-illumos
x86_64-unknown-linux-gnu
x86_64-unknown-linux-gnux32
x86_64-unknown-linux-musl
x86_64-unknown-netbsd
x86_64-unknown-redox
```

安装装对应的这个目标平台相关工具：
```
$ rustup target add x86_64-unknown-linux-musl
```
这一步如果很慢可以在你的 `.bashrc` 或者 `.zshrc` 中添加这一行，中科大的源加速很靠谱：

```
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
```

## 编译
接着只要在运行 build 命令时指定 target 就可以了。

```
$ cargo build --target x86_64-pc-windows-gnu
```


