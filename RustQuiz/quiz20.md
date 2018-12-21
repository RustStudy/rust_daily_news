#  Rust Quiz 解读：Quiz 20

### Quiz  20: 

下面这段代码输出什么？

```rust
fn return1() {
    if (return { print!("1") }) {
    }
}

fn return2() {
    if return { print!("2") } {
    }
}

fn break1() {
    loop {
        if (break { print!("1") }) {
        }
    }
}

fn break2() {
    loop {
        if break { print!("2") } {
        }
    }
}

fn main() {
    return1();
    return2();
    break1();
    break2();
}
```

**输出结果： 121**

### 解读

考察要点：

1. return和break的一点区别

其实在Rust 1.19版本之前，上面代码的输出结果是`1212`。但是在1.19之后，加入了`break value in loop`的功能，也就是说，可以从loop循环中使用break返回一个值。

对于`break1`函数来说：

`loop {  if (break { print!("1") }) {}}`，会先计算`break {print!("1")}`的值。

而`break2`函数，因为没有使用括号来表达优先级，所以会解析为以下等价的代码：

```rust
fn break2() {
    loop {
        if (break) { 
            print!("2") 
        }
        { }
    }
}
```

所以，在打印之前就已经`break`出了循环体，所以什么都没打印。而return就不会产生这种解释歧义，等以后Rust可能会统一break和return的行为。

[点此查看 Rust Quiz 20](https://dtolnay.github.io/rust-quiz/20)