# Concept of bin, lib, rlib, a and so in Rust

From https://rust.cc/article?id=98b96e69-7a5f-4bba-a38e-35bdd7a0a7dd

写了这么久的 Rust 代码了，可能很多人还对 Rust 的编译后的文件格式不是很清晰。本篇我们就来理一下，Rust 中的 bin, lib, rlib, a, so 是什么，如何生成，以及其它一些细节。

## 从 cargo new 说起

我们创建一个新工程，通常从下面两句入手：

```bash
cargo new foobar
```

或

```bash
cargo new --lib foobar
```

前者创建一个可执行工程，而后者创建一个库工程。

实际上，你去探索上述命令行生成的文件，发现它们的 `Cargo.toml` 完全一样，区别仅在于 src 目录下，可执行工程是一个 `main.rs`，而库工程是一个 `lib.rs`。

这是因为 `main.rs` 和 `lib.rs` 对于一个 crate 来讲，是两个特殊的文件名。rustc 内置了对这两个特殊文件名的处理（当然也可以通过 `Cargo.toml` 进行配置，不详谈），我们可以认为它们就是一个 crate 的入口。

可执行 crate 和库 crate 是两种不同的 crate。下面我们就来一并说一下它们的兄弟姐妹及其之间的异同。

## crate type

执行

```bash
rustc --help|grep crate-type
```

可得到如下输出

```bash
       --crate-type [bin|lib|rlib|dylib|cdylib|staticlib|proc-macro]
```

才发现，原来有这么多种 crate type。下面挨个看一下。

## bin

二进制可执行 crate，编译出的文件为二进制可执行文件。必须要有 main 函数作为入口。这种 crate 不需要在 Cargo.toml 中或 --crate-type 命令行参数中指定，会自动识别。

## lib

库 crate。它其实并不是一种具体的库，它指代后面各种库 crate 中的一种，可以认为是一个代理名称（alias）。

通常来讲，如果什么都不配置，默认指的是 rlib, 会生成 .rlib 的文件。

## rlib

rlib 是 Rust Library 特定静态中间库格式。如果只是纯 Rust 代码项目之间的依赖和调用，那么，用 rlib 就能完全满足使用需求。

rlib 实现为一个 ar 归档文件。

```bash
> file target/debug/libfoobar.rlib
target/debug/libfoobar.rlib: current ar archive
```

rlib 中包含很多 metadata 信息（比如可能的上游依赖信息），用来做后面的 linkage。

在 `Cargo.toml` 中配置：

```toml
[lib]
name = "foobar"
crate-type = ["rlib"]
```

可以指定生成 rlib，但是一般没必要设置，因为默认 lib 就是 rlib。

rlib 是平台（Linux, MacOS, Windows ...）无关的。

## dylib

动态库。

在 `Cargo.toml` 中配置：

```toml
[lib]
name = "foobar"
crate-type = ["dylib"]
```

会在编译的时候，生成动态库（Linux 上为 .so, MacOS 上为 .dylib, Windows 上为 .dll）。

动态库是平台相关的库。动态库在被依赖并链接时，不会被链接到目标文件中。这种动态库只能被 Rust 写的程序(或遵循 Rust 内部不稳定的规范的程序)调用。这个动态库可能依赖于其它动态库（比如，Linux 下用 C 语言写的 PostgreSQL 的 libpq.so，或者另一个编译成 "dylib" 的 Rust 动态库）。

## cdylib

C规范动态库。

在 `Cargo.toml` 中配置：

```toml
[lib]
name = "foobar"
crate-type = ["cdylib"]
```

与 dylib 类似，也会生成 .so, .dylib 或 .dll 文件。但是这种动态库可以被其它语言调用（因为几乎所有语言都有遵循 C 规范的 FFI 实现），也就是跨语言 FFI 使用。这个动态库可能依赖于其它动态库（比如，Linux 下用 C 语言写的 PostgreSQL 的 libpq.so）。

## staticlib

静态库。

在 `Cargo.toml` 中配置：

```toml
[lib]
name = "foobar"
crate-type = ["staticlib"]
```

编译会生成 .a 文件（在 Linux 和 MacOS 上），或 .lib 文件（在 Windows 上）。

编译器会把所有实现的 Rust 库代码以及依赖的库代码全部编译到一个静态库文件中，也就是对外界不产生任何依赖了。这特别适合将 Rust 实现的功能封装好给第三方应用使用。

## proc-macro

过程宏 crate.

在 `Cargo.toml` 中配置：

```toml
[lib]
name = "foobar"
crate-type = ["proc-macro"]
```

这种 crate 里面只能导出过程宏，被导出的过程宏可以被其它 crate 引用。

Crate type 以及它们之间的区别就介绍到这里了，有些细节还是需要仔细理解的。本篇意在阐述一些基础知识，而不打算成为一篇完整的参考文件，如要查看 Rust Linkage 的详细内容，直接访问 Rust Reference。

https://doc.rust-lang.org/reference/linkage.html

这一篇帖子非常有用：

https://users.rust-lang.org/t/what-is-the-difference-between-dylib-and-cdylib/28847

