# Rust Quiz 解读：Quiz 21

### Quiz  21: 

下面这段代码输出什么？

```rust
trait Trait {
    fn f(&self);
}

impl<F: FnOnce() -> bool> Trait for F {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for () {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    let x = || { (return) || true; };
    x().f();

    let x = loop { (break) || true; };
    x.f();

    let x = || { return (|| true); };
    x().f();

    let x = loop { break (|| true); };
    x.f();

    let x = || { return || true; };
    x().f();

    let x = loop { break || true; };
    x.f();
}

```

**输出结果： 221111**

### 解读

考察要点：

1. return和break的区别
2. 闭包实现和闭包调用
3. 优先级

Quiz 代码中定义了Trait和一个泛型实现，该泛型实现中包含了限定 `<F: FnOnce() -> bool>`，其意义是，只有`FnOnce() -> bool`闭包才可以调用`f`函数。

同样为单元类型`()`也实现了Trait。

接下来依次看main函数中的代码：

```rust
let x = || { (return) || true; };
x().f();
```

`x`绑定右侧，整体是一个闭包，当该闭包调用的时候，其闭包体内部的`{(return) || true; }`是一个完整的或操作表达式。不过，`(return)`首先会返回`（）`。

所以，当调用`x()`的时候，会返回`()`。这里需要注意的是，闭包调用语法。由闭包的名称紧跟括号来调用闭包。所以，`x().f()`，实际上就是调用闭包的返回值`()`的`f()`方法，输出结果是`2`。

```rust
 let x = loop { (break) || true; };
 x.f();
```

这两行代码中的x，是绑定了，从loop循环中通过`break`关键字返回的单元值`()`。因为这里是`(break)`，通过括号将`break`做为了一个独立的表达式，和上面的`(return)`类似。

```rust
let x = || { return (|| true); };
x().f();
```

这里很明显，当闭包x被调用之后，返回的是`|| true`这个闭包。这个闭包是实现了`FnOnce()->bool`的闭包，所以`x().f()`这里是调用有`<F: FnOnce()->bool>`限定的泛型实现中的`f()`方法，所以输出`1`。

```rust
let x = loop { break (|| true); };
x.f();
```

这里的break和上面的return相似，返回的依然是闭包。同理，输出`1`。

```rust
let x = || { return || true; };
x().f();
```

对于`return`关键字来说，这里不加括号和加括号的效果是一样的，所以，还是返回闭包。输出依然是`1`。

```rust
 let x = loop { break || true; };
 x.f();
```

`break`在这里和`return`相同，返回闭包，输出为`1`。

回想Quiz 20中`break`的用法，可以把最后这两行代码改一下：

```rust
// 修改1
let x = loop { break { || true }; };
x.f();

// 修改2
let x = loop { break { || true } };
x.f();

// 修改3
let x = loop { break { || true; } };
x.f();
```


对于`修改1`来说，`break`后面加了`{ ... }`块。其实在Rust中，块（block）也是一个表达式。所以，`修改1`和`修改2`其实是等价的，在块后面加不加分号类似。最终break会将闭包`|| true`返回。

所以，`修改1`和`修改2`都是输出`1`。但是`修改3`的结果就不一样了。

`修改3`中，`|| true`后面加了分号`;`，对于块表达式来说，它的求值结果必然是`()`。因为在Rust中，加分号的表达式，都会返回`()`。所以，`修改3`会输出`2`。

[点此查看 Rust Quiz 21](https://dtolnay.github.io/rust-quiz/21)