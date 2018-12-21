# Rust Quiz 解读：Quiz 16

### Quiz 16: 

下面这段代码输出什么？

```rust
fn main() {
    let mut x = 4;
    --x;
    print!("{}{}", --x, --x);
}
```

**输出结果：4**

### 解读

考察要点：

1. Rust里不存在C语言中的`++i/--i`自增/自减运算符

当看到`--x`中，可能会有所迷惑。Rust中并不存在自增/自减运算符。所以，Rust编译器怎么解析这个操作呢？答案是`-(-x)`。将`-`按负号进行解析，而非减号。

负负得正，所以结果是`4`。


[点此查看 Rust Quiz 16](https://dtolnay.github.io/rust-quiz/16)