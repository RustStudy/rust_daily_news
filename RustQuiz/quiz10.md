# Rust Quiz 解读：Quiz 10

### Quiz 10: 

下面这段代码输出什么？


```rust
trait Trait {
    fn f(&self);
}

impl<'a> dyn Trait + 'a {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for bool {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    Trait::f(&true);
    Trait::f(&true as &dyn Trait);
    <_ as Trait>::f(&true);
    <_ as Trait>::f(&true as &dyn Trait);
    <bool as Trait>::f(&true);
}
```

**输出结果：22222**

### 解读

考察要点：

1. trait基本用法
2. Rust 2018 edition新语法`dyn Trait`
3. 无歧义完全限定语法（Fully Qualified Syntax for Disambiguation）
4. triat 对象

Quiz代码中定义了名为Trait的trait，包含了`f`函数签名。

接下来是为此Trait内定义的`f`函数添加默认的实现，也叫固有（inherent）实现。此处使用了Rust 2018 edition的新语法：`dyn Trait`。其实`impl<'a> dyn Trait + 'a {`等价于Rust 2015中`impl<'a> Trait + 'a {`的写法。

在Rust 2015中，直接为Trait实现固有方法，和`impl Trait for SomeType`这种写法容易混淆。对于直接的`impl Trait {}`来说，其实是为`trait Object`实现固有方法。所以在Rust 2018 edition中引入了`dyn`关键字，表示`trait Object`，`impl dyn Trait {}`比起“裸写的” `impl Trait {}`可读性要高。

而且`dyn Trait`和新引入的`impl Trait语法`相对应。一个表示动态分发，一个表示静态分发。

所以，Quiz代码中，`impl<'a> dyn Trait + 'a {...}`是为了实现Trait的`trait Object`实现固有方法`f`。

接下来，`impl Trait for bool {...}`为`bool`布尔类型实现Trait，同样实现了`f`方法。

至此，`bool`类型可以调用`f`方法，而任何实现了Trait的`trait Object`也可以调用默认实现的固有`f`方法。

那么，在main函数中的五种调用方法，到底调用哪个`f`呢？


` Trait::f(&true);`，这种写法是Rust中常用的写法，通过指定`Trait::`前缀，并且传入具体类型实例的引用`&true`来调用对应的`f`方法。在Quiz代码中，明确地为`bool`类型实现了Trait，所以这里自然输出： 2。


`Trait::f(&true as &dyn Trait);` ，这种写法类似于第一种，但是通过`as`关键字将`&true`转换为了trait对象`&dyn Trait`类型。然而，对于Rust编译器来说，它已明确知道这还是`bool`类型。在更精确的`bool`和trait对象`&dyn Trait`之间，编译器肯定要选择那个更精确的类型。所以，这里也会输出： 2。

`<_ as Trait>::f(&true);` ，这种写法，叫做无歧义完全限定语法（Fully Qualified Syntax for Disambiguation），通过`<_ as Trait>`，来指定实现了Trait的类型，调用的是`Trait`中实现的方法。`_`此处泛指实现了Trait的类型，Rust会根据上下文进行自动推断。所以，必然会去使用`impl Trait for bool`中定义的`f`实现。所以，这行代码依旧输出： 2。


`<_ as Trait>::f(&true as &dyn Trait);`，同理，即便转为了trait对象，也还是去寻找为具体类型`bool`实现的`f`方法。所以，输出：2。

`<bool as Trait>::f(&true);`，这句和`<_ as Trait>::f(&true);`等价，只不过这里指定了`bool`类型。所以，继续输出： 2。

目前，Rust还不提供直接调用trait对象中定义的默认实现的语法。除非，把trait对象中实现的方法换成不同的名称。比如：

```rust
trait Trait {
    fn f(&self);
}

impl<'a> dyn Trait + 'a {
    fn ff(&self) {
        print!("1");
    }
}

impl Trait for bool {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    Trait::f(&true);
    Trait::ff(&true);
    Trait::ff(&true as &dyn Trait);
}
```

这会输出： `211`。

注意main函数中最后两行：` Trait::ff(&true);`和` Trait::ff(&true as &dyn Trait);`。这说明，没有为`bool`类型实现`ff`方法，Rust也会去trait对象的默认实现中去`查询`该方法。

但是如果使用了无歧义完全限定语法，则不会这样做。比如：

```rust
trait Trait {
    fn f(&self);
}

impl<'a> dyn Trait + 'a {
    fn ff(&self) {
        print!("1");
    }
}

impl Trait for bool {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    <_ as Trait>::ff(&true);
    <_ as Trait>::ff(&true as &dyn Trait);
    <bool as Trait>::ff(&true);
}
```

这样写会报错：

```rust
error[E0576]: cannot find method or associated constant `ff` in trait `Trait`
  --> src/main.rs:20:19
   |
20 |     <_ as Trait>::ff(&true);
   |                   ^^ did you mean `f`?

error[E0576]: cannot find method or associated constant `ff` in trait `Trait`
  --> src/main.rs:21:19
   |
21 |     <_ as Trait>::ff(&true as &dyn Trait);
   |                   ^^ did you mean `f`?

error[E0576]: cannot find method or associated constant `ff` in trait `Trait`
  --> src/main.rs:22:22
   |
22 |     <bool as Trait>::ff(&true);
   |                      ^^ did you mean `f`?

error: aborting due to 3 previous errors

```


因为，已经通过`<_ as Trait>`这样的写法，已经明确指定了调用` impl Trait for bool`中实现的`f`方法，这时候调用`ff`方法，显然，不会找到。

[点此查看 Rust Quiz 10](https://dtolnay.github.io/rust-quiz/10)