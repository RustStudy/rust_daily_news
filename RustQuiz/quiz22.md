# Rust Quiz 解读：Quiz 22

### Quiz  22: 

下面这段代码输出什么？

```rust
macro_rules! m {
    ($a:tt) => { print!("1") };
    ($a:tt $b:tt) => { print!("2") };
    ($a:tt $b:tt $c:tt) => { print!("3") };
    ($a:tt $b:tt $c:tt $d:tt) => { print!("4") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt) => { print!("5") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt $f:tt) => { print!("6") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt) => { print!("7") };
}

fn main() {
    m!(-1);
    m!(-1.);
    m!(-1.0);
    m!(-1.0e1);
    m!(-1.0e-1);
}
```

**输出结果： 22222**

### 考察要点：

1. 声明宏模式分支匹配规则
2. Rust如何对表达式进行词法分析

Quiz代码中定义了m!宏，通过该宏可以判断Rust如何对表达式进行解析。

在main函数中：

`m!(-1)`，其中`-1`，会被Rust分词为两个独立词条（Token）：`-`和`1`。所以，宏调用会匹配到第二个模式条件分支，最终输出：`2`。

Rust对于数字`1`、`1.`、`1.0`、`1.0e1`和`1.0e-1`均会识别为一个词条。所以，剩余的4个宏调用，也同样会输出： `2`。

`-`永远都会被识别为独立的词条，比如下面这段代码：

```rust
fn main() {
    let n = -3i32.pow(4);
    println!("{}", n);
}
```

输出结果是：`-81`。

[点此查看 Rust Quiz 22](https://dtolnay.github.io/rust-quiz/22)