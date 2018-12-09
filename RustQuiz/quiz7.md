#  Rust Quiz 解读：Quiz 7

### Quiz 7: 

下面这段代码输出什么？

```rust
#[repr(u8)]
enum Enum {
    First,
    Second,
}

impl Enum {
    fn p(self) {
        match self {
            First => print!("1"),
            Second => print!("2"),
        }
    }
}

fn main() {
    Enum::p(unsafe {
        std::mem::transmute(1u8)
    });
}
```


**输出结果： 1**

### 解读

考察要点：

1. Enum内存布局
2. `std::mem::transmute`函数
3. 类型推断
4. match匹配

在上面代码中定义了Enum枚举体，使用`#[repr(u8)]`来指定其内存布局，是按8位对齐。

然后为Enum实现了方法`p`，其方法体使用`match`匹配。这里是最让人迷惑的地方了。注意match的两个匹配模式，`First`和`Second`和Enum枚举体包含值同名。正常情况下，如果要在方法p内使用枚举值，应该是`Enum::First`和`Enum::Second`，或者使用`use Enum::*;`导入。这里没有导入，也没有加`Enum::`前缀，那么实际上，Rust将`First`和`Second`当作通配符`_`来看待。

所以，在main函数中，不管给p方法传入什么类型，只能匹配到第一个，所以输出结果是： 1。

怎么理解呢？把上面的代码放到Playground执行一遍，会看到编译器有警告输出：

```rust
warning[E0170]: pattern binding `First` is named the same as one of the variants of the type `Enum`
  --> src/main.rs:10:13
   |
10 |             First => print!("1"),
   |             ^^^^^ help: to match on the variant, qualify the path: `Enum::First`

warning[E0170]: pattern binding `Second` is named the same as one of the variants of the type `Enum`
  --> src/main.rs:11:13
   |
11 |             Second => print!("2"),
   |             ^^^^^^ help: to match on the variant, qualify the path: `Enum::Second`

```

首先是这两个警告，这是提示开发者，First和Second和Enum定义的枚举值同名了，提示加上`Enum::`前缀。

```rust
warning: unreachable pattern
  --> src/main.rs:11:13
   |
10 |             First => print!("1"),
   |             ----- matches any value
11 |             Second => print!("2"),
   |             ^^^^^^ unreachable pattern
   |
   = note: #[warn(unreachable_patterns)] on by default

```

然后是上面这个警告，提示这里触发了`unreachable pattern`模式，因为Rust编译器默认是开启`#[warn(unreachable_patterns)]`警告的。

什么是Unreachable Pattern呢？p方法等价于下面代码：

```rust
impl Enum {
    fn p(self) {
        match self {
            _ => print!("1"),
            _ => print!("2"),
        }
    }
}
```

当函数被调用的时候，Rust会遍历`self`所有可能的值，但是现在这里并没有把所有可能匹配的值给列出来。所以就是`Unreachable`。在这种情况下，将会匹配任意值。这也是上面代码输出`1`的原因。

如果把p方法里的匹配模式加上`Enum::`前缀会怎么样？

```rust
#[repr(u8)]
enum Enum {
    First,
    Second,
}

impl Enum {
    fn p(self) {
        match self {
            Enum::First => print!("1"),
            Enum::Second => print!("2"),
        }
    }
}

fn main() {
    Enum::p(unsafe {
        std::mem::transmute(1u8)
    });
}

```

这次，输出结果变成了： 2。

这又是为什么呢?

事实上，Enum等价于下面代码：

```rust
enum Enum {
  First = 0u8,
  Second = 1u8,
}
```

其实，你在Playground里输出MIR代码也能看得出来：

```rust
#[repr(u8)]
enum Enum {
    First,
    Second,
}
fn main() {
    let a = Enum::First;
    let b = Enum::Second;
}
```

输出的MIR：

```rust
fn main() -> (){
    let mut _0: ();                      // return place
    scope 1 {
        scope 3 {
        }
        scope 4 {
            let _2: Enum;                // "b" in scope 4 at src/main.rs:10:9: 10:10
        }
    }
    scope 2 {
        let _1: Enum;                    // "a" in scope 2 at src/main.rs:9:9: 9:10
    }

    bb0: {                              
       // 省略其他
        discriminant(_1) = 0;            // bb0[1]: scope 0 at src/main.rs:9:13: 9:24
        discriminant(_2) = 1;            // bb0[3]: scope 1 at src/main.rs:10:13: 10:25
       // 省略其他
    }
}
```

那么main函数，其实等价于下面代码：

```rust
fn main() {
    let a = unsafe {
        std::mem::transmute(1u8)
    };
    Enum::p(a);
}
```

` std::mem::transmute()`函数本身是一个unsafe函数，它相当于直接在栈上进行拷贝，将一种类型的值，解释为另一种类型。语义上，相当于一种「按位移动」。说白了，就是一种类型转换。但不能随便使用它，必须在源值和目标值，都有效的情况下才可以转换。

但是上面代码里的`a`，是什么类型呢？`std::mem::transmute(1u8)`会把`1u8`转换为`Enum`类型。这是Rust编译器根据上下文进行类型推断得出的。你可以把`1u8`修改为`1u16`看看编译器的提示。

那么现在`a`实际上是被转换成`Enum::Second`。所以`Enum::p(a);`输出的结果是：2。

注意上面`Enum::First`对应的判别式是`0`，而`Enum::Second`对应的判别式才是`1`。

[点此查看 Rust Quiz 7](https://dtolnay.github.io/rust-quiz/7)

