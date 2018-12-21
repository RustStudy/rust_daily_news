# Rust Quiz 解读：Quiz 19

### Quiz 19: 

下面这段代码输出什么？

```rust
struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let s = S;
    let _ = s;
    print!("2");
}
```

**输出结果：21**

### 解读

1. 所有权和drop
2. `_`并没有所有权

我想把这个例子重新修改一下：

```rust
#[derive(Debug)]
struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let s = S;
    let _ = s;
    print!("{:?}", s)
}
```

执行该代码输出结果为`S1`。重点在main函数最后一行打印语句中，在执行了`let _ = s;`之后，还可以使用`s`。这说明，`s`的所有权并未被转移。

所以，先打印的是`S`。等main函数执行完，`S`的drop方法会被自动调用，然后打印`1`。

继续改一下代码：

```rust
fn main() {
    let s = S;
    let _a = s;
    print!("{:?}", s)
}
```

这一次只改main函数，将`_`变成`_a`，执行代码的时候出错了：`error[E0382]: use of moved value: `s``。说明`s`的所有权已经被转移了。 看来`_`和`_a`的行为是不一样的。

继续简化代码来查看生成的mir：

```rust
// mir示例1
struct S;
fn main() {
    let s = S;
    let _ = s;
}

// mir示例2
struct S;
fn main() {
    let s = S;
    let _a = s;
}
```


`mir示例1`会生成下面的mir代码：

```rust

fn main() -> (){
    let mut _0: ();                      // return place
    scope 1 {
    }
    scope 2 {
        let _1: S;                       // "s" in scope 2 at src/main.rs:4:9: 4:10
    }

    bb0: {                              
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:4:9: 4:10
        StorageDead(_1);                 // bb0[1]: scope 0 at src/main.rs:6:1: 6:2
        return;                          // bb0[2]: scope 0 at src/main.rs:6:2: 6:2
    }
}
```

`mir示例2`则生成下面的mir代码：

```rust
fn main() -> (){
    let mut _0: ();                      // return place
    scope 1 {
        scope 3 {
        }
        scope 4 {
            let _2: S;                   // "_a" in scope 4 at src/main.rs:5:9: 5:11
        }
    }
    scope 2 {
        let _1: S;                       // "s" in scope 2 at src/main.rs:4:9: 4:10
    }

    bb0: {                              
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:4:9: 4:10
        StorageLive(_2);                 // bb0[1]: scope 1 at src/main.rs:5:9: 5:11
        _2 = move _1;                    // bb0[2]: scope 1 at src/main.rs:5:14: 5:15
        StorageDead(_2);                 // bb0[3]: scope 1 at src/main.rs:6:1: 6:2
        StorageDead(_1);                 // bb0[4]: scope 0 at src/main.rs:6:1: 6:2
        return;                          // bb0[5]: scope 0 at src/main.rs:6:2: 6:2
    }
}
```


`示例1`和`示例2`的区别，正是`_`和`_a`的区别。`_a`多了一次move操作：`_2 = move _1;`。

最后再改一下Quiz代码：

```rust
struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let s = S;
    drop(s);
    print!("2");
}
```

输出结果是： `12`。

这里使用了`drop(s)`，未转移所有权，也未绑定任何变量，直接drop。所以，会在打印语句之前调用drop方法，先输出`1`。

[点此查看 Rust Quiz 19](https://dtolnay.github.io/rust-quiz/19)