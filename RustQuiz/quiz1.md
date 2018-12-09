# Rust Quiz 解读： Quiz 1

### Quiz 1: 

下面这段代码输出什么？

```rust
macro_rules! m {
    ($($s:stmt)*) => {
        $(
            { stringify!($s); 1 }
        )<<*
    };
}

fn main() {
    print!(
        "{}{}{}",
        m! { return || true },
        m! { (return) || true },
        m! { {return} || true },
    );
}
```

**输出结果： 112**

### 解读：

考察要点： 

1. 声明宏的语法
2.  Rust中语句（statements）的区分
3. 操作符优先级
4. return返回的是Never Type类型(!)

上面代码中，宏声明代码中火箭符（=>）左边的是匹配表达式，其中`$s:stmt`  代表匹配语句（statements），而`$( )*`这样的语法，类似于正则表达式，代表匹配零个或多个。那么`$($S: stmt)*`就代表匹配零个或多个语句tokens。

在火箭符的右侧，`{stringify!($s); 1}`  其中的stringily!宏是将匹配的语句$s转换为字符串，  `$( )<<*`  对应于左边匹配表达式，生成零个或多个形如` ... << ... << `这样的语句。

再来看main函数中：

`m!{ return || true}`  ，等价于`return (|| true )` ，返回一个闭包。这属于一条独立语句。所以 最终宏生成的代码是 `{"return || true"; 1}`，最终在print!语句中执行结果是返回块代码中最后一个表达式的值：1.

`m!{(return) || true}`，这里面return被括号隔离，那么整个语句是或操作。这里return 是一个表达式，它产生的值是NeverType（!）。右边表达式是true，所以是一个独立的语句。最终宏生成的代码是 `{"(return) || true"; 1}`，最终的值是：1。

`m!{ {return} || true }`，这里rutrn用花括号隔开，代表它是一个独立的语句，后面的`|| true`就会被看做一个独立的闭包。那么这里存在两条语句，最终宏生成的代码是` {"{true}"; 1} << {"|| true"; 1}`，那么最终的值就是`1<<1`，也就是：2。

所以代码的执行结果就是112。

点此查看原题，及更多解释：[Rust Quiz 1 ](https://dtolnay.github.io/rust-quiz/1) 
