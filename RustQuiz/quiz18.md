# Rust Quiz 解读：Quiz 18

### Quiz 18: 

下面这段代码输出什么？

```rust
struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("1");
    }
}

fn main() {
    let print2 = || print!("2");
    S { f: print2 }.f();
}
```

**输出结果： 1**

### 解读

考察要点：

1. 闭包调用
2. 结构体字段和函数调用的优先级

Quiz代码中实现了结构体S，包含一个类型为`fn()`的函数指针字段。然后为S实现了同样名为`f`的方法。

在main函数中，定义了闭包print2。然后main中最后一行代码就开始迷惑人了。

`S {f: print2 }.f();`

如果你没有关注结构体字段名也是f，并且它是个函数指针类型，也罢。你就不会想太多。一眼看上去就是调用`S`的结构体实例的`f`方法。输出结果是`1`。那么正好答对了。

但是，如果你如果注意到`print2`闭包也可以作为一个函数指针。那么为什么不是`(S {f: print2 }.f)()`呢？

不好意思，如果你想调用闭包，那么最好显式地加上括号告诉编译器你是想调用闭包。就像这样：

```rust
struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("1");
    }
}

fn main() {
    let print2 = || print!("2");
    (S { f: print2 }.f)();
}
```

这样输出结果就是`2`。

否则，编译器在这里，只认函数调用。如果不信的话，你把`impl S { ... }`代码删除以后看看：

```rust
struct S {
    f: fn(),
}

fn main() {
    let print2 = || print!("2");
    S { f: print2 }.f();
}
```

编译器会报错：`error[E0599]: no method named `f` found for type `S` in the current scope`。


[点此查看 Rust Quiz 18](https://dtolnay.github.io/rust-quiz/18)