#  Rust Quiz 解读：Quiz 6

### Quiz 6: 

下面这段代码输出什么？

```rust
use std::mem;

fn main() {
    let a;
    let a =  a = true;
    print!("{}", mem::size_of_val(&a));
}
```

**输出结果：0**

### 解读

考察要点：

1. `std::mem::size_of_val()`函数的用法
2. `let`变量屏蔽
3. 变量声明和赋值
4. ZST（零大小类型）

main函数中，首先声明了`a`，但并未赋值，也没有指定具体的类型。

`let a = a = true;`，等价于`let a = (a = true);`，Rust中可以说是「一切皆表达式」。对于`(a=true)`来说，是一个赋值表达式，该表达式的值永远返回单元类型`()`，同时它的值也是`()`。那么`let a = (a = true)`;最终等价于`let a = ();`。 

所以，在最后的print!打印语句里，使用`mem::size_of_val(&a)`函数来计算`a`的内存大小，此时等价于`mem::size_of_val(&())`，对于单元类型来说，它并不占任何内存空间。在Rust中，这种类型叫做ZST（Zero Size Type）。它们不占空间，最终会被LLVM优化掉。

那么，上面代码发散一下，能否这么写呢?

```rust
fn main() {
    let a;
    let a =  a =  a = true;
    print!("{}", std::mem::size_of_val(&a));
}
```

注意这里的代码变化，`let a =  a =  a = true;`，等价于`let a =  ( a =  (a = true) );`。但仔细想想，这样成立吗？

执行以后会报错：

```rust
error[E0308]: mismatched types
 --> src/main.rs:3:17
  |
3 |     let a = a = (a = true);
  |                 ^^^^^^^^^^ expected bool, found ()
  |
  = note: expected type `bool`
             found type `()`

error: aborting due to previous error
```

这是为什么呢？

首先，`(a = true)`，已经让Rust编译器推断`a`的类型为`bool`了。然后，`(a = (a = true))`，等价于，`(a = ());`，类型不匹配，自然会报错。

但是为什么`let a = (a = true)`，没有报相似的错呢？ 这是因为，当前执行的是`let a = ();`，和`a = ();`比较一下，区别在哪里？ 

区别正是因为那个`let`，因为有`let`，所以这里是一个`变量屏蔽`，也就是重新定义了一个`a`，它的类型自然不受之前类型推断结果的影响。


[点此查看 Rust Quiz 6](https://dtolnay.github.io/rust-quiz/6)