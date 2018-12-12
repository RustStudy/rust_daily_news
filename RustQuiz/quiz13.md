# Rust Quiz 解读：Quiz 13

### Quiz 13: 

下面这段代码输出什么？

```rust
struct S;

fn main() {
    let [x, y] = &mut [S, S];
    let eq = x as *mut S == y as *mut S;
    print!("{}", eq as u8);
}
```

**输出结果：1**

### 解读

考察要点：

1. 单元结构体
2. let绑定模式匹配
3. 数组布局
4. 操作符优先级
5. 引用和原生指针的转换
6. bool类型转换为数字

Quiz代码中定义了单元结构体`S`。在main函数中，使用let绑定模式匹解构`&mut [S, S]`数组，定义了`x`和`y`两个变量。

此时`x`和`y`的值分别是`&mut S`和`&mut S`。 数组本身是可变借用，那么其元素自然也是借用，不可能是拥有其所有权。

这里可能有人要怀疑`x`和`y`同时对`S`进行可变借用，合法吗？答案是肯定的。因为此时`x`和`y`借用的单元结构体`S`可看作是两个独立的结构体实例。

`let eq = x as *mut S == y as *mut S;`这行代码等价于`let eq = ( (x as *mut S) == (y as *mut S) );`。优先级`as` > `==` > `=`。

`x`和`y`本身是可变借用，然后通过`as`转换为原生可变指针类型（*mut S），然后对它们进行比较，是看它们的地址是否相同。然后将最后的bool类型结果赋值给`eq`。最后通过`as`将`bool`类型的值转换为`u8`。可以预测结果，不是`0`就是`1`，其中`0`对应于`false`， `1`对应于`true`。

虽然分析出了整个过程，但是我无法确定这里到底是输出1还是0。因为我无法确定这里Rust的行为，这两个原生指针地址是否相同。

想知道我为啥有这个疑问吗？可以看看下面的代码：

```rust
struct Empty;
fn main() {
    let x = &mut Empty;
    println!("x {:p}", x);
    let y = &mut Empty;
    println!("y {:p}", y);
}
```

此代码在Debug模式下编译的结果是：

```
x 0x7fff5b8c2058
y 0x7fff5b8c20c0
```

在Release模式下编译的结果是：

```
x 0x7ffe9ae101d8
y 0x7ffe9ae101d8
```

所以，你明白了吗？ 在Debug模式下编译，不同的单元结构体实例，地址是不同的，但是在Release模式下，不同的单元结构体实例会被优化成同一个地址。

这就是我为什么不敢确定Quiz代码输出结果的原因。没办法，只能把Quiz代码实际执行一下看看输出结果了。然后发现，不管是Debug模式还是Release模式，Quiz代码的输出结果都是`1`。也就是说，`x`和`y`的地址是一样的。

这是怎么回事呢？

我注意到Quiz代码和我上面编写代码中定义变量的区别：

```rust
// Quiz 代码中
let [x, y] = &mut [S, S];

// 我自定义代码中
let x = &mut Empty;
let y = &mut Empty;
```

我有所悟： 在Quiz代码中的两个单元结构体实例是放到一个`[S; 2]`类型的数组中的，同一个数组的起始地址肯定是一样的。难道Rust把数组的起始地址作为`x`和`y`这两个实例的地址？

来看下面代码：

```rust
struct S;
struct E;

struct A {
    s: S,
    e: E,
}

fn main() {
    let (x, y) = &mut (S, E);
    println!("{:p}", x as *mut S);
    println!("{:p}", y as *mut E);
    
    let A{s, e} = &mut A{s: S, e: E};
    println!("{:p}", s as *mut S);
    println!("{:p}", e as *mut E);
}
```


在Debug模式下编译：

```
0x7ffe370bd7b8
0x7ffe370bd7b8
0x7ffe370bd890
0x7ffe370bd890
```

在Release模式下编译：

```
0x7ffcde64f608
0x7ffcde64f608
0x7ffcde64f608
0x7ffcde64f608
```

所以，你明白了吗？

[点此查看 Rust Quiz 13](https://dtolnay.github.io/rust-quiz/13)