# Rust Quiz 解读：Quiz 14

### Quiz 14: 

下面这段代码输出什么？

```rust
trait Trait: Sized {
    fn is_reference(self) -> bool;
}

impl<'a, T> Trait for &'a T {
    fn is_reference(self) -> bool {
        true
    }
}

fn main() {
    match 0.is_reference() {
        true => print!("1"),
        false => print!("0"),
    }

    match '?'.is_reference() {
        true => print!("1"),
        false => {
            impl Trait for char {
                fn is_reference(self) -> bool {
                    false
                }
            }
            print!("0")
        }
    }
}

```

**输出结果：10**

### 解读

考察要点：

1. 自动引用（autoref）
2. impl trait 的可见性

Quiz代码中定义了Trait，并限定了Sized trait。这意味着该Trait无法当作trait对象来使用。但是对于Quiz代码来说没有啥特殊意义。

然后为`&'a T`实现了Trait：`impl<'a, T> Trait for &'a T { ... }`。`&'a T`和`T`有什么区别呢？在之前的Quiz解读中已经讲过了。

在main函数中， `match 0.is_reference(){ ... }`，当调用数字`0`的`is_reference`方法时，你可能会想，数字类型有没有被实现`Trait`呢？在为`&'a T`的实现中，`is_reference`的参数是`self`。到底编译会不会成功呢？

事实上，这里涉及一个规则：自动引用。

当调用`0.is_reference()`的时候，会为`0`自动添加引用，等价于`(&0).is_reference()`。 这是因为上下文中只有为`&'a T`实现了`Trait`。此时`is_reference`方法中的`self`等价于`&'a i32`。

如果为`T`实现`Trait`，就不会存在自动引用了。

注意：在Rust中与自动引用对应的规则，还有一个`自动解引用(auto deref)`。

所以，这里`0.is_reference()`调用，会返回true。输出结果当然是： `1`。

而`match '?'.is_reference()`，是调用字符`?`的`is_reference()`方法。这里迷惑人的地方就是`impl Trait for char { ... }`实现是放到了`false`匹配分支中。但实际上`impl Trait for char {...}`是对整个Quiz代码可见的。所以，在`'?'.is_reference()`调用的时候，它会输出`false`，最终match匹配结果自然是： `0`。

如果你将`impl Trait for char {...}`实现代码去掉的话：

```rust
match '?'.is_reference() {
    true => print!("1"),
    false => print!("0")
}
```

输出结果将会是：`11`。


[点此查看 Rust Quiz 14](https://dtolnay.github.io/rust-quiz/14)
