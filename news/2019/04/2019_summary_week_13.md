### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust日报已经成为了Rust社区群大家每天必看的内容。

从2019年开始，日报小组成立，目前的动态由：@Chaos、 @Mike、 @Damody(台湾)轮番为大家播报。也欢迎感兴趣的朋友加入小组。

每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

独立日报订阅地址：
- [Telgram Channel](https://t.me/rust_daily_news )
- [阿里云语雀订阅](https://www.yuque.com/chaosbot/rustnews)
- [Stemmit](https://steemit.com/@blackanger)
- [GitHub](https://github.com/RustStudy/rust_daily_news)

社区学习交流平台订阅：
- [Rust.cc论坛](https://rust.cc)
- [Rust Force](https://rustforce.net/)
- [微信公众号：Rust语言学习交流](https://rust.cc/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

2019-04-28

---

# 官方新闻

### Rust核心团队发布Rust 2019 Roadmap

Rust核心团队最近发布了Rust 2019 Roadmap RFC，主要的工作分为：

- 治理。改进项目运作方式。包括整理RFC流程。
- 完成已经开始但还未完成的工作。比如异步等。
- 提高语言和工具的整体质量。包括改进编译时间和增强IDE支持等。

- [Read More](https://blog.rust-lang.org/2019/04/23/roadmap.html)
- [Rust 2019 Roadmap RFC](https://github.com/rust-lang/rfcs/blob/master/text/2657-roadmap-2019.md)

### Rust 1.34.1 发布

clippy 误报的問題已解決

- `clippy::redundant_closure`
- `clippy::missing_const_for_fn`

[Read more](https://www.reddit.com/r/rust/comments/bhap9o/announcing_rust_1341/)


### 「官方」runtime: 为更容易地创建异步应用而生的库

#runtime #async

Runtime，是由Rust异步工作组发布的一个与平台无关的库，旨在使Async Rust既灵活又简单。该库也是异步生态系统标准化的基石。它通过以下方式实现：

- 消除共享异步运行时的麻烦，包括I/O和异步执行程序
- 遵循Rust标准库主导的API约定
- 标准化运行时接口，实现应用程序和底层实现的分离，并且支持自定义运行时

```rust

#![feature(async_await, await_macro, futures_api)]

use futures::prelude::*;
use runtime::net::TcpListener;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on {}", listener.local_addr()?);

    let mut incoming = listener.incoming();
    while let Some(stream) = await!(incoming.next()) {
        runtime::spawn(async move {
            let stream = stream?;
            println!("Accepting from: {}", stream.peer_addr()?);

            let (reader, writer) = &mut stream.split();
            await!(reader.copy_into(writer))?;
            Ok::<(), std::io::Error>(())
        });
    }
    Ok(())
}
```

现在是基于宏，并且期望在未来能实现如下写法：

```rust
use std::futures::net::TcpListener;

async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on {}", listener.local_addr()?);

    #[spawn(parallel)]
    for await? stream in listener.incoming() {
        println!("Accepting from: {}", stream.peer_addr()?);
        let (reader, writer) = &mut stream.split();
        await? reader.copy_into(writer);
    }
}
```

希望成为标准库的一部分。

- [Read More](https://blog.yoshuawuyts.com/runtime/)
- [runtime](https://github.com/rustasync/runtime)

### 「官方」HashMap的SwissTable算法重新实现的PR已经被合并

将会在Rust 1.36版本中看到。

该PR作者一共发布过两个PR。第一次是将Swisstable在标准库中重新实现了一遍，第二次是将Swisstable作为独立的hashbrown crate引入标准库中。显然，这次合并的PR是第二种方式。

- [Read More](https://github.com/rust-lang/rust/pull/58623#event-2295889137)
- [hashbrown](https://crates.io/crates/hashbrown)

### 「官方」 Stabilize futures_api的PR已被合并

这意味着异步开发离稳定又近了一步。此次Futures API主要是两大模块:

`future` 模块

- std::future
- std::future::Future trait和它关联的项（Output和poll）

`task` 模块

- std::task
- std::task::Poll
- std::task::Waker
- std::task::RawWaker
- std::task::RawWakerVTable
- std::task::Context

另外还有一些api的名称按照Rust标准库中统一的命名风格被修改。

- [Read More](https://github.com/rust-lang/rust/pull/59739)
- [相关issues](https://github.com/rust-lang/rust/issues/59725#issuecomment-486082226)

### Niko：代码之外

Niko最近意识到，Rust编译器团队，需要的不仅仅是开发人员，更加需要的是编码之外的人才，包括文档作者、组织者、传道者、项目经理等角色，只有加入了这些角色，编译器团队才能正常发展。因为编译器团队不仅仅是写代码，其他任务（各种会议、管理工作组、带新人、编写文档、协助沟通问题、帮助贡献者等等）也是同样重要。

[Read More](http://smallcultfollowing.com/babysteps/blog/2019/04/15/more-than-coders/)

### 「官方演讲」Rust：未来四十年的语言

为什么是四十年？

作者拿铁路百年史做了类比

1830年5月24日，美国第一条铁路（巴尔的摩—俄亥俄，21公里）通车。
1869年5月10日，美国建成第一条横贯美洲大陆的太平洋铁路，总长2849公里。 革命性的里程碑，花费了39年。
1901年，德国西门子—哈尔斯克电机公司制造的电力机车，在柏林附近的曼菲尔德—措森线上，创造了时速162公里的世界纪录。

这个时间花了71年。

那么系统编程语言呢？

1973年，Unix正式用C语言改写。
到2015年，Rust语言发布，是站在C/Cpp语言肩膀上的革命性语言。革命性的里程碑，花费了42年。
那么未来四十年，Rust语言会取得什么辉煌成就？拭目以待。

![img](https://wx1.sinaimg.cn/mw690/71684decgy1g2e0tlqnq4j21of0u0h4g.jpg)

- [Slides](https://carols10cents.gitlab.io/rust-next-40-years/assets/player/KeynoteDHTMLPlayer.html#0)
- [gitlab仓库](https://gitlab.com/carols10cents/rust-next-40-years)

### rustup发布了1.18.0版本

[Changelog](https://github.com/rust-lang/rustup.rs/blob/master/CHANGELOG.md#1180---2019-04-22)

### Futures 0.1 兼容层

为了弥合由futures 0.1和不稳定的异步生态造成的生态鸿沟，官方异步团队在futures 0.3中引入了futures 0.1的兼容层，本文阐述了如何来使用它。

[Read More](https://rust-lang-nursery.github.io/futures-rs/blog/2019/04/18/compatibility-layer.html)

### Mozilla 将关闭Rust IRC频道

Mozilla最近宣布将关闭其IRC网络，理由是日益增加的维护和审核负担。距离最终关闭还有几个月的时间，官方建议大家转移到Rust的官方Discord服务器去交流。Discord服务器包含`#users`，`＃help`和`#beginners`频道。

如果你想继续使用IRC，那么可以去非官方的freenode频道去交流。

（IRC交流太古老了，为了扩大Rust社区的交流面，使用现代化的通信工具是必须的。）

- [Read More](https://blog.rust-lang.org/2019/04/26/Mozilla-IRC-Sunset-and-the-Rust-Channel.html)
- [更多讨论](http://exple.tive.org/blarg/2019/04/26/synchronous-text/)


---

# 社区新闻

### 首届RustConAsia大会圆满结束

非常感谢秘猿和PingCAP为社区组织了这一场盛会。

[大会的所有资料都可以在这里找到](https://shimo.im/docs/zliTJIGgNwk6QIFh/read)

### Sonic：用Rust编写的Elasticsearch的极简替代品

[Read More](https://zhuanlan.zhihu.com/p/63963140)

### Rust 生态可视化

有人将crates.io的crate生态做了可视化，可以通过这个网站看到Rust crate的生态。

- [Read More](https://rfdonnelly.github.io/crate-galaxy/)
- [源码](https://github.com/rfdonnelly/crate-galaxy-graph)
- [视频](https://www.bilibili.com/video/av49392469/?redirectFrom=h5)

### 「RustLatam 2019 系列视频」Without Boats: 零成本异步IO

Rustlatam2019大会的视频陆续放出了，此链接是without boats的演讲。大家可以在youtube列表中查看其他的。

[RustLatam 2019](https://www.youtube.com/watch?v=skos4B5x7qE)

### Rust Cheatsheet

针对Rust类型做的Cheatsheet

- [Read More](https://upsuper.github.io/rust-cheatsheet/)
- [upsuper/rust-cheatsheet](https://github.com/upsuper/rust-cheatsheet)

其他的CheatSheet汇总

- [cheats.rs](https://cheats.rs/)
- [rust_cs_canvas: 归纳了Rust语法](https://www.breakdown-notes.com/make/load/rust_cs_canvas/true)

### ripgrep 11 发布

ripgrep 是 Linux 命令行文件内容检索工具 grep 的 rust 实现版本。版本 11 修复了很多 bug，改进了性能，对二进制文件的搜索体验大幅改进。

[Read More](https://github.com/BurntSushi/ripgrep/releases/tag/11.0.0)

### Xray 死了吗？

[Xray](https://github.com/atom/xray) 是实验性的下一代基于 Electron 的编辑器的后端，用 rust 写成。但是项目进度好像遇到的问题。于是作者在 reddit 上心急如焚，发了一篇长文，细数了 Xray 的好。希望有人能接手继续开发下去。

[Read More](https://www.reddit.com/r/rust/comments/bdf3lx/we_need_to_save_xray/)

不过有人说好像有已经有人 [fork](https://github.com/fdionisi/xray/issues/1) 了。

### 「系列文章」for await loops (Part I)：无船大神对 await for 语法的思考 

> 有关async/await语法的一个悬而未决的问题是：await的最终语法。到目前为止，关于这个问题已经进行了大量的讨论;该讨论的现状和语言小组内的立场即将推出。本文无船同志这一系列文章将讨论一个影响该决定但尚未被考虑的问题：for循环流程。

目前futures-async-await库中用的语法是这样的：

```rust
#[async]
for elem in stream { ... }
```

但无船同志认为这个语法与await的作用其实是相似的，这也是JavaScript中使用`for await...of`语句来创建循环遍历异步可迭代对象的原因：

```js
for await (elem of stream) { ... }
```

对于Rust中如何设计这种语法呢？这个await是循环语法的一部分（只是for循环这么用）呢，还是单独的语法模式（更加通用）呢？无船倾向于让它成为通用的语法模式。然而目前Rust的类型系统对于后者的表达是有限制的，所以本文将讨论将await作为循环语法的一部分，而下一篇再讨论await作为单独语法模式。

所以，对于await作为for循环的一部分，这个语法设计无船倾向于使用下面这种空格分隔`prefix-await`的语法（后缀语法类似于是`elem.await`这样的表达）：

```rust
for await? elem in stream { }
```

下一篇文章将探讨更多。（看看语法设计也挺有意思）

[blog](https://boats.gitlab.io/blog/post/for-await-i/)

# Quinn 0.3 发布

使用rust实现QUIC协议

[Read more](https://github.com/djc/quinn/releases/tag/0.3.0)

###  Evolution Island：Amethys展示游戏寻找合作者

为了让Amethys成为Rust的旗舰级游戏引擎，Amethys基金会将支持一部分展示项目，可以有效地展示Amethys的关键功能。 Evolution Island是有Amethys基金会自己开发的一个展示项目。

- [evolution-island](https://github.com/khskarl/evolution-island)
- [Reddit 介绍](https://www.reddit.com/r/rust/comments/bf65l3/evolution_island_amethyst_showcase_game_looking/)
### 「Redox」重大改进

自`libstd / sys / redox`模块设计以来，Redox发生了重大变化。 `relibc`的创建导致开发了一个用于Redox的POSIX，C API，它支持libstd / sys / unix模块中绝大多数所需的功能。所以现在要考虑使用一个单独的redox target family，可能允许它属于unix target family。这一改变将大大减少将Redox OS转移到Tier 3所需的工作，完全支持Cargo，rustc和其他Rust工具。

- [Read More](https://github.com/rust-lang/rust/issues/60139)
- [Reddit上针对这一变化的讨论](https://www.reddit.com/r/rust/comments/bfedj1/should_redox_os_convert_to_the_unix_target_family/)

### pulldown-cmark 0.5 发布

- 跟进了最新的CommonMark规范 0.29版本
- 一流的性能提升。增加了simd加速。PulldownCmakr是最好的CommonMark实现。

- [Read More](https://www.reddit.com/r/rust/comments/bgx1vg/new_pulldowncmark_05_release/)
- [通用标注(CommonMark)介绍](http://www.commonmark.cn/w/)

### 让astexplorer.net支持Rust AST可视化

可以直接在浏览器里查看和解析AST了。之前我只能从命令行输出ast.json文件，然后找个在线的json可视化工具查看。现在方便多了。

通过包装Rust的syn和WASM实现了这个功能。

![img](https://wx2.sinaimg.cn/mw690/71684decly1g2er6egykkj20u00x6k24.jpg)

- [astexplorer.net](https://astexplorer.net/)
- [源码](https://github.com/fkling/astexplorer/tree/master/website/src/parsers/rust)

### Amethyst接受了Mozilla的1w美元资助

为了支持WebAssembly在浏览器中运行紫水晶游戏，该笔资金主要用于支付在以下三点有突出贡献的贡献者：

- WASM渲染器（2D、3D、UI）
- 文档改进，包括WASM
- WASM的并行性

[Read More](https://www.amethyst.rs/blog/moss-grant-announce/)

### 日本最近也出了一本Rust新书：Rust入门实践

看来大家都陆续上车了

![img](https://s2.ax1x.com/2019/04/27/EuTBlR.png)

### 量子链线上分享脑图

我（Chaos）上午看了这次分享，并且做了一份脑图。这次分享主要是介绍了椭圆曲线相关的内容，感兴趣的可以看看。

![img](https://s2.ax1x.com/2019/04/27/EuWuvT.png)

同时，量子链也在招聘Rust工程师，感兴趣的可以投简历 yangting at qtum.info

### 「Job」linkerd公司开始招Rust工程师了

- [Read More](https://www.reddit.com/r/rust/comments/bhfrcm/job_work_on_linkerd_with_you_guessed_it_rust/)
- [软件工程师职位](https://jobs.lever.co/buoyant/01011edf-e9c8-446a-9fbe-4a87865b109c)
- [系统工程师职位](https://jobs.lever.co/buoyant/7a64f7d1-6fea-40b1-ba52-5ab44802c5f6)

### 「Job」华为诺亚方舟实验室招聘 AI算法性能优化  工程师 或 实习生，职责和要求如下

该岗位也支持Rust开发

【工作职责】
1、负责AI算法中计算在华为CPU/GPU/NPU等处理器上的计算性能优化，基础库设计；
2、负责CNN/RNN/RL等模型的效果评估分析，持续优化到极致；
3、负责持续跟踪业界最优实现，超越并创新
4、负责和上下游同事对接，协同交付最优成果；

【任职要求】
1、熟悉计算机体系结构，非常了解现代处理器的特性；
2、熟练掌握C，清楚C的常见坑和编程技巧；
3、至少熟悉ARM NEON指令集、OpenCL、CUDA中一种， 有性能优化经验；
4、熟悉至少一个推理平台开源框架的实现，具有相关经验者优先；
5、熟悉机器学习算法，熟悉Caffe,Tensorflow,Pytorch,Mxnet等至少一个主流AI开源框架，具有上述开源项目经验者优先；
6、追求极致、理性的心态。

感兴趣的朋友可把简历直接发给@风辰（qq 304128534） ，或发送到 liuwenzhi4 at huawei.com

### 韩国一家金融公司已经把 Rust 用在了一个正式的盈利的**高频交易**项目上

当然，是没有开源的啦。作者只是在reddit上通报了一声，说了一下这个项目的大致情况。

这个项目在16个月前立项。上线交易近10个月，盈利9个月。交易策略重写花了7个月，上线部署花了1个月。最终的效果是Rust在各方面都打败了c++版本，虽然只是一点点性能提升。不过作者还没有提到安全性和稳定性的问题。

高频交易按道理说，应该是Rust能展示威力的地方，只是这块儿非常敏感，所以试水比较缓慢，现在终于有人出来公开宣传了。赞赞赞！

[Read More](https://www.reddit.com/r/rust/comments/bhtuah/production_deployment_of_rust_in_hft_system_at/)


---

# 学习资源

### 「系列文章」使用wasmer和Rust构建插件系统 Part 1

Wasmer是一个可以嵌入到Rust应用中的wasm解释器。

[Read More](https://wiredforge.com/blog/wasmer-plugin-pt-1)

### 「经验分享」Rust中你不应该做的三件事

该文是来自sentry公司的博客。概要

- 不要使用自引用指针。（而应该用handle来代替，也就是说，不是存储指向对象本身的指针，而是存储一些信息，以便稍后计算指针。）
- 不要陷入生命周期和借用检查地狱。（而应该使用引用计数共享所有权）
- 不要轻易使用内部可变性。（考虑建立新的状态来代替内部可变）

更多详细内容，包括代码解释请看原文。

[Read More](https://blog.sentry.io/2018/04/05/you-cant-rust-that)

### 核反应堆设计模式

作者认为，设计模式分为两种层级：低级和高级。像那种流行的设计模式，比如工厂模式之类，属于低级的设计模式。而高级的设计模式，是指整个应用程序的设计哲学。本文主要谈论后者，作者称之为「核反应堆设计模式」。作者使用该设计模式很长时间，并且它不特定于Rust，还可以应用于C/C++和Perl等其他语言。但作者认为，该模式应该在Rust中更受欢迎。

核反应堆设计模式原理

假如你的应用程序中有一个非常难以处理的难题（比如一个事件循环，或者是复杂的数据结构等等），那么你可以单独去解决这个难题，然后在这个难题的核心周围设置一堆API墙，利用这面强将难题核心和其他周围环境隔离起来。也就是说，将难题作为一个独立的整体去思考，而不能让它传播到其他程序中。就像核反应堆一样。

为什么说Rust更适合这种设计模式

- Rust语言比较吸引那些喜欢硬问题的人。
- 心理作用。比如Rust就分离了unsafe Rust。
- Rust的类型系统。可以更加方便地构建「核反应堆」。

有人说，这不就是「封装」吗？难道说，只是把「封装」换了个名字？

并不是。封装只是核反应堆的必要条件。并非每个封装的东西都是核反应堆。

[Read More](https://vorner.github.io/2019/04/21/nuclear-reactor-design-pattern.html)

### 六个你可能从未见过的有用的Rust宏

概要：

- [log-derive: 记录函数错误的宏](https://crates.io/crates/log-derive)
- [recap: 正则表达式解析库](https://crates.io/crates/recap)
- [shrinkwraprs： 将数据类型重新定义为新的不同类型](https://crates.io/crates/shrinkwraprs)
- [metered： 自动在方法上生成统计信息](https://crates.io/crates/metered)
- [derive-new： 自动实现new方法](https://github.com/nrc/derive-new)
- [snafu： 提供辅助函数来处理Rust中的错误，帮助增强代码的可读性](https://crates.io/crates/snafu)

[Read More](https://medium.com/@benmcdonald_11671/6-useful-rust-macros-that-you-might-not-have-seen-before-59d1386f7bc5)

### Rust中的新奇功能

#Rust1.34

该文作者对Rust1.34稳定版中引入的新功能`std::iter::from_fn`的探索。该功能允许通过一个函数来直接创建迭代器，以往此功能只能通过宏来辅助实现。现在看上去方便多了。

[Read More](https://weblog.latte.ca/blake/tech/rust/makingiterators.html)

学习Rust的解析器组合子

「长文预警」本文教你如何使用解析器组合子的函数式编程语言中常见的技术从头开始构建解析器。

[Read More](https://bodil.lol/parser-combinators/)

### Remap: Rust中的Webassembly地图组件

本文展示了如何使用stdweb和yew构建一个WASM的2D地图组件。

[remap](https://gitlab.com/alamminsalo/remap)

### Silver: 基于tokio实现的一个简单HTTP框架

可供学习

[Silver](https://github.com/AhmedMostafa16/Silver)

### Rocket Prometheus：给 Rocket 应用添加监控

Prometheus（普罗米修斯），在运维界几乎成了应用监控的代名词了，详细定义在[这里](https://prometheus.io/docs/introduction/overview/)。本身内容还是蛮多的，运维哥哥深有体会。

这个库应该是给rocket应用加一个metrics接口，这样就可以使用prometheus了

[Repo](https://github.com/sd2k/rocket_prometheus)

### min-sized-rust：如何缩小 Rust 二进制包大小的总结 

总结有以下方法：

1. 以 --release 编译
2. strip 掉符号
3. 把 cargo 的编译参数加个 opt-level = 'z' 以优化体积编译
4. 开启 LTO
5. 去掉 Jemalloc
6. 减少 cargo 的并行代码生成单元
7. panic 的时候直接中断掉程序 
8. 使用 xargo 优化 libstd
9. 使用 panic_immediate_abort 去掉 panic 的格式化代码
10. 不使用 libstd: #![no_std]

[Read More](https://github.com/johnthagen/min-sized-rust)

### 你应该尝试的12个杀手级库

该文总结了12个Rust生态中最常用的库。

[Read More](https://medium.com/@jondot/12-killer-rust-libraries-you-should-know-c60bab07624f)

### 「嵌入式Rust」使用DMA传输数据

DMA(Direct Memory Access)，直接存储器访问。

之前都是，CPU参与，一点点把数据，从一个地方拷贝，即像搬家一样搬到，另一个地方。很明显，此时，相对时间比较宝贵（比较值钱）的CPU，把时间，就用在（浪费在）拷贝数据了。所以才出现了DMA，专门去干拷贝数据的累活。DMA是一种无需CPU的参与就可以让外设和系统内存之间进行双向数据传输的硬件机制，它不是独立外设，而是硬件模块支持的机制。

[Read More](https://flowdsp.io/blog/stm32f3-02-dac-dma/)

### Rust如何解决依赖地狱

依赖地狱：处理应用程序依赖性版本和依赖性冲突所带来的挫败感。

Rust的解决方案：

- Cargo。允许语义版本控制兼容规则。
- Name Mangling。在Rust编译器源码的`symbol_names.rs`文件中有详细规则。

通过上面两个共同作用来解决问题。

假如你写的库被包含在某个应用程序中，而应用程序中使用了`log-0.5`，你的库中依赖的是`log-0.4`，那么Rust编译器会对应用程序内部使用log的库使用0.5版本，而你的库代码则使用0.4。

更多详细内容请看原文。

[Read More](https://stephencoakley.com/2019/04/24/how-rust-solved-dependency-hell)

### Rust AV1 视频编码器

本文讲解了一些av1编码的概念，对视频压缩感兴趣的人可以看看

[Read more](https://www.reddit.com/r/rust/comments/bh8xnl/implementing_tile_encoding_in_rav1e_a_rust_av1/)

### 如何在Rust中编写更好的编译错误信息

本文介绍了一种方法，让你在代码中编写更易于调试的编译错误：compile_error!宏，它也可以配合条件编译使用。

[Read More](https://blog.knoldus.com/how-to-write-better-compilation-error-message-in-rust/)

### 在 no_std 下，如何做序列化

这是一个不常见的问题，但是如果是针对嵌入式设备编程，或手持设备进行游戏开发。这会是一个重要问题。作者给了以下3个要求：

1. 只有 64M RAM
2. 磁盘上的空间使用应该尽可能小，比如应该用二进制存储而不是JSON
3. 能支持 no_std，也能支持其它目标

最后作者把 [quick-protobuf](https://github.com/tafia/quick-protobuf) 拿过来改了改，满足了要求。

[Read More](https://www.reddit.com/r/rust/comments/bi0xll/no_std_data_serializationdeserialization/)

---

# 项目、工具与库

### 「嵌入式Rust」atomic_bitfield: core库中原子类型的位域(bit-field)抽象

> 位域是把一个字节中的二进位划分为几个不同的区域，每个域有一个域名，并说明每个区域的位数，允许在程序中按域名进行操作。在对内存有苛刻要求的嵌入式系统中，经常会用到这个概念。

该库中没有使用任何unsafe代码，新库，具体使用的时候需要谨慎。

[atomic_bitfield](https://github.com/amiraeva/atomic_bitfield)

### libbluetooth-rs: BlueZ Linux蓝牙库的原始绑定

该库支持Unix的Bluetooth API (BlueZ)。

[libbluetooth-rs](https://github.com/Wodann/libbluetooth-rs/)

### laminar 0.2 发布 

新的版本专注于代码质量的提升。

Laminar用于多人游戏的半可靠UDP协议实现。该库在UDP的基础上实现了TCP的一些功能。它被用于Amethyst游戏引擎中。

[laminar](https://github.com/amethyst/laminar)

### nude-rs：高性能色图检测

是 [nude.js](https://github.com/pa7/nude.js) 和 [nude.py](https://github.com/hhatto/nude.py) 的移植。

看看下面的性能评测：

![pic](https://raw.githubusercontent.com/kpcyrd/nude-rs/master/docs/benchmark.png)

nude-js 社区经常引以为傲的性能优越感呢？不过 Rust 为 node 包性能的提升已经做好准备。

此库目前还处于实验阶段。

[Read More](https://github.com/kpcyrd/nude-rs)

### python-ext-wasm：用于运行 WebAssembly 二进制文件的 Python 扩展

Wasmerio 出品，之前我们报道过 [php-ext-wasm](https://github.com/wasmerio/php-ext-wasm)，这个库就是对应的 Python 版本。

主打：
- 易用
- 快速
- 安全

使用 

```
$ pip install wasmer
```

就可以安装了。

[Read More](https://github.com/wasmerio/python-ext-wasm)

### ring-channel：环形缓冲区之上的无阻塞，bounded MPMC 管道

在 [环形缓冲区 ring buffer](https://zh.wikipedia.org/zh-hans/%E7%92%B0%E5%BD%A2%E7%B7%A9%E8%A1%9D%E5%8D%80) 上建立一个 MPMC （Multiple Producer Multiple Consumer）通道。

[Read More](https://github.com/brunocodutra/ring-channel)

### org-rs：Org 模式解析器的 Rust 实现

[Org Mode](https://orgmode.org/) 是 Emacs（神的编辑器）中用于记笔记，维护待做列表，做工程规划等活儿的插件，功能强大，操作快捷，受众广泛，被超多人喜爱。但是只能在 Emacs 中，是个硬伤。所以就有人想把它独立出来用。所以就有了解析器。

现在这个解析的 Rust 版本来了。目前处于早期阶段，可以玩儿了。

[Read More](https://github.com/org-rs/org-rs)

### Gleam发布了0.1版本

Gleam是Rust实现的一门函数式编程语言，拥有一个类似于erlang和elixir的分布式并发系统。这是该语言的第一次release版本。

[Read More](https://lpil.uk/blog/hello-gleam/)

### neat-flappy-bird: NEAT算法玩像素鸟

来自社区 @planet0104 的作品，使用了quicksilver。

> Neuro Evolution Of Augmenting Topologies(拓扑扩张的神经演化), NEAT代码来自《游戏编程中的人工智能技术》一书

[neat-flappy-bird](https://github.com/planet0104/neat-flappy-bird)

### sfsdb - 高性能KV文件系统数据库

作者宣称的特点是，简单，高性能，不需要额外学习，使用它就好像使用 Rust 语言自身的结构一样。比如：

[Repo](https://github.com/AlmightyFloppyFish/sfsdb)

### lemmy - 使用actix-web和TypeScript仿reddit站点

Rust和TypeScript很配

[Repo](https://github.com/dessalines/lemmy)

### 显示树状目录文件结构有几种方式（只用Rust工具）

1. [tree-rs](https://github.com/sighol/tree-rs)
2. [treeify](https://github.com/dzamlo/treeify) 
3. [exa --tree](https://the.exa.website/features/tree-view)
4. [fd](https://github.com/sharkdp/fd) 与 treeify 配合使用

### netease-cloud-music-gtk: 基于 Rust + GTK 开发的网易云音乐播放器

国人开发，特点：

- 安全： Rust 天生的
- 极速：相比 Node/python 版，Rust 速度可谓一骑绝尘
- 稳定：除了网速或网易 API 限制，基本不会出现运行问题
- 简洁：仿 GNOME Music 风格，GTK 原生界面，纯粹得令人发指
- 简单：最小的编译与运行依赖

![pic](https://user-images.githubusercontent.com/6460323/55945759-01f55200-5c7e-11e9-9a91-606a4656555e.png)
![pic](https://user-images.githubusercontent.com/6460323/55945765-04f04280-5c7e-11e9-9f38-242524aedd66.png)

大家去感受一下。

[Repo](https://github.com/gmg137/netease-cloud-music-gtk)

### no-std-compat：一个`#![no_std]`兼容层

可以轻松地将你的crate移植到no_std。

[no-std-compat](https://gitlab.com/jD91mZM2/no-std-compat)

### wapm-cli: 命令行的WebAssembly软件包管理器

此工具允许在wapm.io注册表上安装，管理和发布wasm包。

[wapm-cli](https://github.com/wasmerio/wapm-cli)

### abstreet: Rust实现的交通模拟游戏

曾经在公交车堵车，想知道为什么有车停在路上而不是公交车道？ A/B街是一个游戏，探索城市的小变化如何影响司机，骑车人，过境用户和行人的运动。

[abstreet](https://github.com/dabreegster/abstreet)

### rust-web-boilerplate: 基于Tide和Futures0.3实现的样板项目

[rust-web-boilerplate](https://github.com/pbzweihander/rust-web-boilerplate)

### Rust 嵌入式开发的一个小示例：用一个LED灯说出 Hello world

要用一个 LED 灯说出 Hello world。当然是需要用到[莫尔斯编码](https://zh.wikipedia.org/zh-hans/%E6%91%A9%E5%B0%94%E6%96%AF%E7%94%B5%E7%A0%81)啦（就是港警匪片中经常出现的那个用手指或枪有节奏地发出声音的那个信号序列）。

就像下面这个样子，

![img](https://raw.githubusercontent.com/daogangtang/picmaterials/master/111.png)

[视频地址](https://twitter.com/i/status/1120835944003846144) 需fq。

作者参考了 [The Embedded Rust Book](https://rust-embedded.github.io/book/) 和这个[样板项目](https://github.com/rust-embedded/cortex-m-quickstart)

用了 openocd 来调试。然后向代码中灌入了：

```
.... . .-.. .-.. ---  .-- --- .-. .-.. -..
```
作为数据源，小板子就闪起来了。

结果 `...- . .-. -.-- -.-. --- --- .-.. -.-.--` (verycool)

[Read More](https://idursun.com/posts/hello_world_in_morse_code/)

### 使用 crossterm 进行跨平台终端应用开发

Rust世界中终端库很多，但是完全跨平台的终端库不多，crossterm 算一个。下面是用它开发的Pikachu.

来看看 [3D 效果](https://preview.redd.it/p3nlqakx2bt21.gif?format=mp4&s=30d2dab1679000eedc65c7cadd550f26676a704e)。

[Read More](http://www.jonathanturner.org/2019/04/porting-the-pikachu.html)  
[Repo](https://github.com/TimonPost/crossterm)

### rubot - 一个独立的可嵌入其它游戏中的游戏机器人库

非常容易使用，比如：

- [tic-tac-toe](https://github.com/lcnr/rubot/blob/master/examples/tic-tac-toe.rs)  
- [chess](https://github.com/lcnr/rubot/blob/master/examples/chess.rs)
- [oko](https://github.com/lcnr/rubot/blob/master/examples/oko.rs)

[Repo](https://github.com/lcnr/rubot)

### futures-codec: 配合 Future 0.3 进行流编解码的库

用过 tokio_codec 的都知道，我们需要用 poll 不断去检查流有没有输出结果，并对 Ready 还是 NotReady 返回分别判断处理，现在有了这个库，就可像下面这样更“符合人性”直观地写了：

```
async move {
    // let stream = ...
    let mut framed = Framed::new(stream, LinesCodec {});

    while let Some(line) = await!(framed.try_next())? {
        println!("{:?}", line);
    }
};
```

这个库只是Future 0.3 大生态的一部分。这个生态还是慢慢完善当中。

[Repo](https://github.com/matthunz/futures-codec)

### offst - 去中心化的支付系统

这个支付系统看起来很有趣。它有如下特性：

- 有效快速支付。Offst不依赖于一个区块链或者任何形式的PoW。每个交易只影响网络中的一小部分节点。支付通常在一秒内完成。
- 低支付费用。支付费用只由支付经过的路由长度决定。路径上的每个路由参与者只得到一份credit。
- 公正的资产分布。Offst系统中总的 credits 是0,并且最终会在所有成员之间公正分布。
- 如果你丢失了Key，你可以向你的朋友请求恢复你的账户。
- 拒绝审查。没有任何实体可以阻止或审核你的交易。
- 完全原始控制。Offst是可编程的，可以二次开发，方便地定制上层应用。


这套系统背后的原理请阅读下文。个人感觉很有意思。

[Read More](https://www.freedomlayer.org/offst/offst-release/)   
[Read More 2](https://www.reddit.com/r/rust/comments/bh9h71/offst_a_decentralized_credit_card/)   
[Repo](https://github.com/freedomlayer/offst)

### warmy - 通用的热加载/重载资源库

我们的在线系统中，经常会有些资源文件（如配置更改），需要重新加载。最简单的方式当然是把服务停掉，重启。但是对于高可用度在线服务来讲，这是不可取的。要么就把配置存数据库或缓存redis等。现在有了这个库，你按它指导的方式进行编程，就可以实现我们想要的目的（不需要数据库）。具体需要试用后才知道好不好用。

它还有资源发现的功能。

[Repo](https://github.com/phaazon/warmy)

### luminance-rs 准备发布1.0

Luminance是Rust实现的一个无状态类型安全的图形库。本来是Haskell实现的，后来作者使用了Rust之后就决定把Rust作为图形库开发的默认语言。可能比gfx-hal更易于使用？

- [luminance-rs](https://github.com/phaazon/luminance-rs)
- [Read More](https://phaazon.net/blog/pre-luminance-n-random-thoughts)