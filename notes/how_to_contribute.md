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
```
推荐修改上面两个配置，以及debug相关配置，都设置为true。

开始构建：

```rust
# build the entire compiler
python x.py build

# build all documentation
python x.py doc

# run all test suites
python x.py test

# build only the standard library
python x.py build src/libstd

# test only one particular test suite
python x.py test src/test/rustdoc

# build only the stage0 libcore library
python x.py build src/libcore --stage 0
```



### 参考资料

- [CONTRIBUTING.md](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md)
- [slides](http://rust-meetup-paris.github.io/Talks/how_to_contribute/index.html)