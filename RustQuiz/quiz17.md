# Rust Quiz 解读：Quiz 17

### Quiz 17: 

下面这段代码输出什么？

```rust
fn main() {
    let mut a = 5;
    let mut b = 3;
    print!("{}", a-- - --b);
}
```

**输出结果: 2**

### 解读

考察要点：

1. 和Quiz16代码一样，Rust中不存在自增或自减运算符

`a-- - --b`等价于`a - (-(-(-(-b))))` ，`5 - 3 = 2`。

那为什么Rust中没有自增/自减运算符呢？

在Rust官方的FAQ中有描述：

> 先增和后增（以及相对应的减法）虽然方便，但也相当复杂。它们都需要求值顺序的知识，而且经常导致 C 和 C++ 中的细节错误和未定义的行为。 x = x + 1 或 x += 1 只是略长一点，但更加明确。  

Rust新官网已经没有了FAQ页面，查看需要从旧官网入口进入：[常见问题解答 · Rust 程序设计语言](https://prev.rust-lang.org/zh-CN/faq.html#why-doesnt-rust-have-increment-and-decrement-operators)

[点此查看 Rust Quiz 17](https://dtolnay.github.io/rust-quiz/17)