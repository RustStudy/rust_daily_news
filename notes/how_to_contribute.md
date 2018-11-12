# 如何给Rust做贡献

### 下载Rust源码

```rust
$ git clone https://github.com/rust-lang/rust.git 
```

### 在本地编译Rust

```rust
$ cd rust
$ mv config.toml.example config.toml
```
在构建系统之前，根据需要设置好配置。

```rust
# Indicate whether the compiler should be documented in addition to the standard
# library and facade crates.
compiler-docs = false
# Indicate whether submodules are managed and updated automatically.
submodules = false
# 使用nightly
channel = "nightly"
```
推荐修改上面三个配置，以及debug相关配置，都设置为true。

然后执行:

```
$ ./configuration
```

开始构建：

```rust
# build the entire compiler
python x.py build
```

### 构建过程

要经历三个阶段：

- stage 0
- stage 1
- stage 2

推荐步骤：

- `./x.py check`
- `--keep-stage`

```rust
$ ./x.py build -i --stage 1 src/libstd
$ ./x.py build -i --stage 1 src/libstd --keep-stage 1
```

构建如果有问题，别忘记检查下是否缺乏依赖：

```rust
$ git submodule update --init
```

[构建参考](https://rust-lang-nursery.github.io/rustc-guide/how-to-build-and-run.html#workflow)

使用rustup将本地编译版本的Rust加到toolchain中：

```rust
$ rustup toolchain link local_rust build/x86_64-apple-darwin/stage0
$ rustup default local_rust
```

语法为：`rustup toolchain link <name> build/<host-triple>/stage0 `

其中，`<host-triple>`就是上面示例中的`x86_64-apple-darwin`

### 在本地测试

```rust
$  python x.py test --stage 0
```

### Debug 编译器

[参考](https://rust-lang-nursery.github.io/rustc-guide/compiler-debugging.html)


### 提交PR

需要使用`r?`来让机器人@rust-highfive来指定review的人。比如：

```rust
r?@steveklabnik
```

@rust-highfive就会自动指定steveklabnik来review这个PR。

### 参考资料

- [Compiler Guide](https://rust-lang-nursery.github.io/rustc-guide/how-to-build-and-run.html)
- [CONTRIBUTING.md](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md)
- [slides](http://rust-meetup-paris.github.io/Talks/how_to_contribute/index.html)