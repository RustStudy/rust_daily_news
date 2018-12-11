# Rust Quiz 解读：Quiz 11

### Quiz 11: 

下面这段代码输出什么？

```rust
fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
    let pf = f::<'static> as fn();
    let pg = g::<'static> as fn();
    print!("{}", pf == pg);
}
```

**输出结果： 编译错误**

### 解读

考察要点：

1. 生命周期参数概念
2. 生命周期参数限定：`Early bound` vs `Late bound`
3. 生命周期子类型与协变
4. 函数指针及其比较

不得不说，此Quiz代码中涉及一个隐晦的概念：生命周期参数`Early bound` vs `Late bound`。这两个概念是官方提供的书里没有过的，也是我做这个题首次遇到的概念。通过调查Rust源码，大概得出了它们的定义：

[付费阅读•一元查看](https://zhuanlan.zhihu.com/p/52032027)