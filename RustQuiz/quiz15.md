# Rust Quiz 解读：Quiz 15

### Quiz 15: 

下面这段代码输出什么？

```rust
trait Trait {
    fn f(&self);
}

impl Trait for u32 {
    fn f(&self) {
        print!("1");
    }
}

impl<'a> Trait for &'a i32 {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    let x = &0;
    x.f();
}
```

**输出结果： 1**

### 解读

考察要点：

1. trait实现
2. `T`和`&'a T`的区别
3. 类型推断
4. 自动加引用

其实Quiz 15和Quiz 5的考察点是相似的。主要是考察`T` 和`&T`有啥区别呢？虽然这里是`u32`和`&’a i32`。

- [ ] T， 是一个泛型参数，代表任何一个具体的类型。
- [ ] &T，实际上等价于`&'a T`，代表某个引用类型。注意这里的`'a`也是一个泛型参数，并不是指具体的类型。

main函数中直接使用了`let x = &0;`，`&0`实际上是一个包含了具体生命周期参数实例的具体类型`&'a 0`。所以，`&0`实际上会被推断为一个具体的`T`类型的实例`u32`。 

这和`impl Trait for ...`的定义顺序无关。不妨把`impl<'a> Trait for &'a i32 { ... }`和`impl Trait for u32 { ... }`的顺序交换，输出结果照样不变。

此时在`u32`的`f`方法实现中`self`是`&u32`。

所以，`x.f()`会输出`1`。

如果把`impl Trait for u32 {}`实现注释掉。则 `x.f()`会输出`2`。

```rust
trait Trait {
    fn f(&self);
}

impl<'a> Trait for &'a i32 {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    let x = &0;
    x.f();
}
```

在这个代码中，`f`方法中的`self`会被自动加引用为`&&0`。

[点此查看 Rust Quiz 15](https://dtolnay.github.io/rust-quiz/15)