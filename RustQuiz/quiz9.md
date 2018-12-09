# Rust Quiz 解读：Quiz 9

### Quiz 9: 

下面这段代码输出什么？

```rust
macro_rules! m {
    (1) => { print!("1") };
    ($tt:tt) => { print!("2") };
}

macro_rules! e {
    ($e:expr) => { m!($e) };
}

macro_rules! t {
    ($tt:tt) => { e!($tt); m!($tt); };
}

fn main() {
    t!(1);
}
```

**输出结果： 21**

### 解读

考察要点：

1. 声明宏基本匹配规则
2. 声明宏元变量匹配的可传导性

Quiz代码中依次定义了三个声明宏，`m!`、`e!`和`t!`，并且依次是包含关系。

在main函数中，调用`t!(1)`时，根据宏定义，其参数`1`经过词法分析得到词条树，也就是宏元变量`$tt:tt`的类型`tt`所指示。然后由`e!($tt);`和`m!($tt);`继续匹配。

`e!($tt)`在匹配过程中，因为`e!`定义左边元变量`$e:expr`，表明是一个表达式类型。对于Rust编译器来说，经过`e!`宏处理的词条，将会变成一个不透明的词条（opaque token tree）。也就是说，后续的宏都会认为它是一个表达式token，而不是其他。所以，在`e!`宏内匹配右侧`m!($e);`的时候，只能匹配`m!`宏的第二条分支`($tt:tt)`，所以输出： 2。

如果试着在`m!`中加一条匹配分支：

```rust
macro_rules! m {
    (1) => { print!("1") };
    ($ee;expr) => { print!("3")};
    ($tt:tt) => { print!("2") };
}
```

输出结果就会变成： `31`。

对于`m!(1)`来说，它一直以`tt`词条树类型来传播，到最后会尝试匹配到`(1)`这个token字面量，然后匹配成功，最后输出： `1`。

这就是为什么输出结果是`21`的原因。

同理，如果把`e!`的匹配模式修改为：

```rust
macro_rules! e {
    ($e:tt) => { m!($e) };
}
```

则输出结果会是：`11`。

所谓不透明词条树（Opaque Token Tree），就是指，不能和token字面量相匹配的词条树。

尝试把Quiz代码中`m!`定义中的匹配分支修改如下：

```rust
macro_rules! m {
    (1) => { print!("1") };
}
```

再去执行Quiz代码，编译器会报错：

```rust
error: no rules expected the token `1`
  --> src/main.rs:6:23
   |
6  |     ($e:expr) => { m!($e) };
   |                       ^^
...
14 |     t!(1);
   |     ------ in this macro invocation
```

编译器提示：no rules expected the token `1`。这意味着，`expr`不能与`1`这个token字面量相匹配。这就是所谓的「不透明词条」。

在Rust中，不透明词条类型除了`expr`，还有其他类型，罗列如下：

```rust
$:block
$:expr
$:item
$:literal
$:meta
$:pat
$:path
$:stmt
$:ty
```

而透明的词条如下：

```rust
$:ident
$:lifetime
$:tt
```


[点此查看 Rust Quiz 9](https://dtolnay.github.io/rust-quiz/9)