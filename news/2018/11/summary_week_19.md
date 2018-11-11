前言：
从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。
2018-11-11

# 官方新闻

### Rust 1.30.1 发布

修补了一些bug

[Read More](https://blog.rust-lang.org/2018/11/08/Rust-1.30.1.html)

### Pin API即将在1.32版本中稳定

[issues-55766](https://github.com/rust-lang/rust/issues/55766)


### 如何在2018年加速Rust编译器：NLL edition

本篇文章是对Niko之前博文的增补。其中按时间线罗列了针对NLL性能提升的各个PR。

这些PR主要是提升了编译器基于NLL做静态借用检查的性能。

[Read More](https://blog.mozilla.org/nnethercote/2018/11/06/how-to-speed-up-the-rust-compiler-in-2018-nll-edition/)

---

# 社区新闻

### 「有趣的数据」Firefox中近一年内Rust使用数据统计

从2017年11月1日 ~ 2018年11月1日

- Rust代码行数从3.7%增加到6.24%，反过来C++代码的比例就降低了
- 一共有405,379行Rust代码，6,091,861行C++代码
- 更有趣的是，Rust代码增加了172,839行，C++代码减少了38,241行

![img](https://wx2.sinaimg.cn/mw690/71684decly1fx3yy1nmg4j20y40n27ac.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1fx3yy4c2sdj20x00lagtd.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1fx3yy6u5bqj20yi0p0dov.jpg)

[Read More推文](https://mobile.twitter.com/eroc/status/1061049330574884864)

### 前EA首席设计官成立的新独立游戏工作室Embark宣布使用Rust为主要语言

作为EA的CDO（首席设计官，Chief design officer）在DICE/EA为游戏奉献了十八个年头，现在创建了新公司Embark，打算尝试点新东西。Embark宣布将用Rust构建他们的技术。

[相关新闻](https://www.theverge.com/2018/11/8/18073992/patrick-soderlund-new-studio-embark-nexon)

[推文](https://twitter.com/repi/status/1060469377500274689)

![img](https://wx2.sinaimg.cn/mw690/71684decly1fx2w65gt13j20z20swwmp.jpg)

### 官方异步Web开发框架Tide中间件介绍

[Read More](https://zhuanlan.zhihu.com/p/49293350)

###「录播视频」11-07晚上杭州秘猿Rust开发分享会

秘猿计划每个月的Dev Meetup第一场都会开设Rust专题

B站视频前面几分钟无声音，后面有了

[ B站 ](https://www.bilibili.com/video/av35546990/)

[ 油管视频声音比较完整 ](https://www.youtube.com/watch?reload=9&v=s7gFNTb3rWU&feature=youtu.be)

### Suricata 4.1 发布

Suricata是一个网络实时入侵检测(IDS)、嵌入式入侵防御(IPS)和网络安全监控(NSM)的引擎。本来是C语言实现的，现在4.1版本中引入的新协议支持是基于Rust构建。

也就是说:
> Suricata 4.1 is not really 4.1 if you don’t have Rust.

[Read More](https://suricata-ids.org/2018/11/06/suricata-4-1-released/)

[suricata](https://github.com/OISF/suricata)

### Parity与Zcash 基金会合作

新的Zcash节点将使用Rust编写，并成为Zcash公司自zcashed之后的第一个可选客户端。zcashed是Zcash唯一可用的完整节点。

[Read More](https://www.parity.io/parity-teams-up-with-zcash-foundation-for-parity-zcash-client/)


### lazy_static发布1.2.0版本

该版本主要是让lazy_static可以在no_std下使用。

[Read More](https://www.reddit.com/r/rust/comments/9u8u6h/psa_lazy_static_120_requires_rustc_1241_or_higher/)


### 探讨所有权和借用语义对API接口设计的影响

[Read More](https://zhuanlan.zhihu.com/p/48585534)

### xray有什么进展？

xray是Atom团队使用Rust和Electron实现的下一代文本编辑器。今天看了下其代码仓库，发现还是在持续地更新。在2018第三个季度，团队主要聚焦于memo子项目。memo是xray的基础，它是一个版本控制系统，也可以作为独立库使用。

memo是想作为Git的扩展，使用无冲突的复制数据类型（CRDT）记录工作副本，用于增强Git的实时协作功能。其技术栈也用到了WebAssembly，用于公开虚拟文件系统的API。

在memo稳定以后，会将其从xray独立出来，然后会继续进行xray的开发。将xray作为一个以memo为基础的一流的可实时协作的编辑器。

牛逼了

[memo](https://github.com/atom/xray/tree/master/memo_core)

[xray](https://github.com/atom/xray)

### 「演讲」使用Rust构建下一代网络基础设施

Buoyant的软件工程师Carl分享了在Conduit开发过程中利用Rust零成本抽象开发下一代网络基础平台。

[infoq 视频](https://www.infoq.com/presentations/rust-infrastructure)

---

# 学习资源

### Monadic Rust: Part I 

[ Read More ](https://zhuanlan.zhihu.com/p/49276748)

### NLL之后：移动借用数据和Sentinel模式

[Read More](https://zhuanlan.zhihu.com/p/49290855)

### 用Rust编写网络驱动程序

[PDF](https://www.net.in.tum.de/fileadmin/bibtex/publications/theses/2018-ixy-rust.pdf)

### 利用Rust所有权语义来构造有约束的API

该文作者在2017年的Rust Fest大会做了分享，直到昨天，他才把演讲内容更新成了文稿。

[演讲视频](https://www.youtube.com/watch?v=3Q2hQfYW-XM&index=9&list=PL85XCvVPmGQj9mqbJizw-zi-EhcpS5jTP)

（ 我在《Rust编程之道》一书的设计模式 - RAII模式 里也借鉴了他的这次演讲内容 ）

[Read More](https://blog.systems.ethz.ch/blog/2018/a-hammer-you-can-only-hold-by-the-handle.html)

### 「语言特性」重温proc_macro_attribute

#proc_macro #attribute #flame #flamegraph

随着proc-macro功能的逐渐稳定，proc_macro_attribute现在也不需要再加特定的features，比如registrar 和 rustc_private。该文作者重构了他的项目flamer(一个编译器插件)，包含了最新的proc_macro_attribute用法，当然还是需要nightly版本。

[flamer](https://github.com/llogiq/flamer)

[Read More](https://llogiq.github.io/2018/11/10/proc-macro.html)


### Rust实现可选参数

本文记录了作者使用Rust实现可选参数的一步步思路，也是对Rust设计API的思考。

P.S 这还是一个PingCAP隐藏的招聘贴呢。

[Read More](https://hoverbear.org/2018/11/04/optional-arguments/)

### Rust谜题：让内嵌于迭代中的Result扁平化

ndarray-csv的作者第三次重构该库的时候碰到一个问题。

他想实现一个函数：

```rust
pub fn flatten_nested_results<T, E, II, IO>(iter_outer: IO) -> impl Iterator<Item = Result<T, E>>
where
    II: Iterator<Item = Result<T, E>>,
    IO: Iterator<Item = Result<II, E>>,
{
    /// Fill me in!
}
```

然后用于处理像下面这种迭代器中的Result

```rust
fn vec_to_nested_iter(
    vec_outer: Vec<Result<Vec<Result<i8, f32>>, f32>>,
) -> impl Iterator<Item = Result<impl Iterator<Item = Result<i8, f32>>, f32>> {
    vec_outer
        .into_iter()
        .map(|vec_inner| vec_inner.map(Vec::into_iter))
}

/// Without any Errs, we should return the whole sequence
#[test]
fn test_all_ok() {
    let iter_outer = vec_to_nested_iter(vec![Ok(vec![Ok(1), Ok(2)]), Ok(vec![Ok(3)])]);
    let expected: Result<Vec<i8>, f32> = Ok(vec![1, 2, 3]);
    assert_eq!(expected, flatten_nested_results(iter_outer).collect())
}
```

作者经过三次失败的flatten_nested_results函数实现，首先找出的解决方案是使用trait object。

```rust
pub fn flatten_nested_results<T, E, II, IO>(iter_outer: IO) -> impl Iterator<Item = Result<T, E>>
where
    T: 'static,
    E: 'static,
    II: 'static + Iterator<Item = Result<T, E>>,
    IO: 'static + Iterator<Item = Result<II, E>>,
{
    iter_outer.flat_map(|iter_inner_result| match iter_inner_result {
        Ok(iter_inner) => Box::new(iter_inner) as Box<Iterator<Item = Result<T, E>>>,
        Err(err) => Box::new(once(Err(err))) as Box<Iterator<Item = Result<T, E>>>,
    })
}
```

但他还不喜欢这个方案，最终他使用了either库来解决此问题

```rust
extern crate either;

use either::Either;
use std::iter::once;

pub fn flatten_nested_results<T, E, II, IO>(iter_outer: IO) -> impl Iterator<Item = Result<T, E>>
where
    II: Iterator<Item = Result<T, E>>,
    IO: Iterator<Item = Result<II, E>>,
{
    iter_outer.flat_map(|iter_inner_result| match iter_inner_result {
        Ok(iter_inner) => Either::Right(iter_inner),
        Err(err) => Either::Left(once(Err(err))),
    })
}
```

[Read More](https://paulkernfeld.com/2018/11/03/flatten-nested-iterator-result.html)

---

# 项目

### swc：加速web开发

swc是babel 和 closure compiler二合一的Rust实现。潜力不错，但目前缺乏文档。

[swc](https://github.com/swc-project/swc)

### rust-analyzer：给IDE使用的Rust实验性模块化编译器前端

旨在为优秀的IDE支持奠定基础

[rust-analyzer](https://github.com/rust-analyzer/rust-analyzer)

### martin： PostGIS矢量切片（ vector tiles）服务器

martin是基于rust和actix-web实现的PostGIS矢量切片服务器。主要是基于mapbox格式做矢量切片，一般用于地图服务。

PostGIS 是 PostgreSQL 的空间数据库扩展。

> 考察了一下该库背后的公司Urbica，是一家来自莫斯科的数据分析及可视化公司。他们最新开发的产品AReal，是一款 iOS 平台上的增强现实 App，通过 3D 的方式展示了圣彼得堡的地图和地标建筑，包括了七座杰出建筑在地图上用星标标记。但是展示方式可能与大家想象的有所不同——不再是低下头在屏幕上看，而是举起手机把建筑“搭建”到你眼前！只要点击任何一座建筑，你就可以通过 AR 模块将建筑融入你所处的环境，并通过缩放、旋转等操作来探索。

一句话描述： Urbica公司是做AR地图的。据说苹果公司也正在筹备做AR地图产品。

[Urbica介绍来源](https://zhuanlan.zhihu.com/p/43656372)

[martin](https://github.com/urbica/martin)

### 「小游戏」Rust和WASM实现的贪食蛇

[rust-snake-wasm](https://github.com/yiransheng/rust-snake-wasm)

### 「嵌入式」Monotron在Rust Belt上的展示

Monotron是基于TM4C123单片机和ARM Cortex-M4内核的一款DIY 8位家用小电脑，作者在RustBelt大会上展示了让它来跑snake小游戏。 并且可以使用C和Rust来为其编写应用。

[Monotron](https://github.com/thejpster/monotron)

[Read More](https://railwayelectronics.blogspot.com/2018/11/monotron-at-rust-belt-rust.html)

### Plume:又一个基于ActivityPub协议开发的社区联盟博客系统

plume是基于rocket和diesel实现。

[Plume](https://github.com/Plume-org/Plume)

[Demo](https://baptiste.gelez.xyz/)

相似的项目有: [Aardwolf](https://github.com/Aardwolf-Social/aardwolf)

### Rust + SDL实现的rougelike沙盒游戏 

> Roguelike是电子角色扮演游戏（RPG游戏）的一个子类。标志性特征有：在随机生成的地牢中探索、回合制、基于图块的图像（tile-based graphics）以及角色的永久死亡。

[rusted-ruins](https://github.com/garkimasera/rusted-ruins)

---

# 工具与库

### Mundane : 基于BoringSSL的Rust加密库

BoringSSL 是由谷歌从 OpenSSL 中抽出来后独立发展的作品。

[mundane](https://github.com/google/mundane)

### JSON5的序列化和反序列化工具

基于pest和serde

[json5-rs](https://github.com/callum-oakley/json5-rs)

### s3-concat: 使用Rust快速合并S3文件

[s3-concat](https://github.com/whitfin/s3-concat)

[Read More](https://whitfin.io/quickly-concatenating-files-in-amazon-s3/)

### 「机器学习」使用Rust在命令行构建训练数据

ttv是一个命令行工具，用于将大型文件拆分为适合于机器学习的训练/测试/验证拆分的块。基于Rust 2018版本。

[ttv](https://github.com/sd2k/ttv)

### vulkano 0.11 发布

vulkano是对Vulkan API的安全包装

[vulkano](https://github.com/vulkano-rs/vulkano)

[Read More](https://www.patreon.com/posts/22587417)

### 「库」imag 0.9发布

私人信息管理套件工具imag发布了新版本：

- 本次版本修改影响比较广
- 增加了读取大文件的支持，不会遇到文件描述符限制
- 加速图片操作
- 使用统一的IO系统
- 使用failure库进行错误处理
- imag-diagnostics增加进度条
- 大量bug修复

[imag](https://imag-pim.org/blog/2018/11/10/imag-0-9-0/)

### instru: 监控Rust代码性能

instru通过在编译阶段（编译器插件）将代码注入到AST来执行对整个程序的跟踪。需要使用nightly版本。

该工具和flamer库有点相似，flamer库可以生成火焰图，用来帮助跟踪代码性能。

[instru](https://github.com/da-x/instru)

[flamer](https://github.com/llogiq/flamer)

### 「小工具」清理cargo或rustc产生的构建垃圾 

[cargo-sweep](https://github.com/holmgr/cargo-sweep)

### hexyl 命令行16进制查看器

彩色输出不同类别的字节

[hexyl](https://github.com/sharkdp/hexyl)

### no-panic ：通过一个属性宏让编译器检验函数不会发生panic

serde/syn 作者dtolnay的新库：no-panic。 提供了一个属性宏，通过编译器来保证函数不会发生panic。

比如：

```rust
extern crate no_panic;
use no_panic::no_panic;

#[no_panic]
fn demo(s: &str) -> &str {
    &s[1..]
}

fn main() {
    println!("{}", demo("input string"));
}
```

如果该demo函数发生panic（或编译器无法检验函数不能panic），则该函数会编译失败，并且错误信息中会携带包含该函数名称标识的链接器错误信息。

注意：通过看该库的源码发现：

- Rust的proc_macro_attribute功能也移出了`#![feature(custom_attribute)]`特性，意味着，也要准备稳定了
- 现在稳定的proc_macro功能，不能用作表达式，除非使用`#![feature(proc_macro_hygiene)]`特性。[相关issues ](https://github.com/rust-lang/blog.rust-lang.org/issues/285)

[no-panic](https://github.com/dtolnay/no-panic)

---

# 招聘

### 「远程Job」来自丹麦的召唤：Rust全栈工程师

丹麦的一家正规公司：impero，目前正在招募Rust全栈工程师。可远程。

[Read More](https://www.reddit.com/r/rust/comments/9v7qpx/rust_fullstack_developer_denmark_or_remote/)