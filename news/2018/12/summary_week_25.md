前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2018-12-23

---

# 「付费阅读」系列

- [如何为Rust语言做贡献](https://zhuanlan.zhihu.com/p/51479889)
- [Rust Quiz 11](https://zhuanlan.zhihu.com/p/52032027)

---

# 官方新闻

### 通告Rust 1.31.1发布

修复了一些问题。

[Read More](https://blog.rust-lang.org/2018/12/20/Rust-1.31.1.html)

### 「官方」Rust 2018中的过程宏

官方博客介绍了Rust 2018 edition中过程宏的支持，重点介绍了过程宏基于TokenStream的工作机制，值得一阅。

[Read More](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html)

### Withoutboats眼中的Rust 2019：组织债务

[Read More](https://zhuanlan.zhihu.com/p/52568974)

### Rust最新动态摘要

#rust 

审核中的PR：

- [稳定化Pin API](https://github.com/rust-lang/rust/pull/56939)
- [重构core::iter模块（值得一看）](https://github.com/rust-lang/rust/pull/56932)
- [升级stdsimd子模块](https://github.com/rust-lang/rust/pull/56926)
- [稳定化`#[repr(packed(N))]`](https://github.com/rust-lang/rust/pull/57049)
- [rustc依赖的库：parking_lot升级到0.7版本](https://github.com/rust-lang/rust/pull/57051)
- [代码优化(没想到Rust代码中也有乱用clone的情况)](https://github.com/rust-lang/rust/pull/57027/files)
- [稳定化`Vec(Deque)::resize_with`](https://github.com/rust-lang/rust/pull/57002)
- [RalfJung改进了miri的内存分配精准度](https://github.com/rust-lang/rust/pull/56981)
- [为`Arc<T>/Rc<T>`增加`Into<NonNull<T>>`实现（引起了较多讨论）](https://github.com/rust-lang/rust/pull/56998)
- [在Release版本中提供最基本的CTFE跟踪（为了Debug？）](https://github.com/rust-lang/rust/pull/56973)

已合并的PR：

- [扩展Pin文档并且将std::pin::Pinned改成了std::marker::PhantomPinned](https://github.com/rust-lang/rust/pull/55992)
- [miri得到了更新](https://github.com/rust-lang/rust/pull/56305)
- [TokeStream得到了改进: 优化了TokenStream以及其他类型](https://github.com/rust-lang/rust/pull/56737)
- [允许胖指针实现Hash](https://github.com/rust-lang/rust/pull/56751)
- [Rust 1.31.1 Release Note已更新](https://github.com/rust-lang/rust/pull/56931)
- [修复trait对象的Bug](https://github.com/rust-lang/rust/pull/56863)
- [更新LLVM子模块](https://github.com/rust-lang/rust/pull/56948)


### 「官方」Rust 2018 edition 工具集

#devtool

本文介绍了Rust 2018 edition包含的工具集

- Rustfix
- Clippy
- Rustfmt
- IDE support

未来：

- 改进LLDB和GDB的Rust调试支持
- 完善RLS
- 让Cargo更强大
- Rustdoc改进

[Read More](https://blog.rust-lang.org/2018/12/17/Rust-2018-dev-tools.html)

---

# 社区新闻

### LeetCode现在已支持Rust

中国版和美国版都支持了，另外了解到的消息是，目前是基于Rust stable 1.31版，而且还会持续跟进Rust Stable版本。在未来会逐渐加入常用的crate，这个看使用者反馈来持续改进。

刷Leetcode Rust代码的碰到问题可以加QQ群反馈。qq群号： 950323896，进群以后可以加微信群，直接向官方反馈问题。

[Leetcode中国](https://leetcode-cn.com/explore/)

### nitric: Specs的继承者

nitric是Specs作者新写的库，那么他为什么在苦心研究两年Specs之后，要革自己的命呢？在本文中他阐述了如下理由：

- 并非是对specs的重构，也不是和specs竞争，也不会对Specs有任何影响，Specs也不会被弃用
- nitric是打算提供一个更通用的解决方案
- Specs已经有了自己的生态，所以需要独立出另外一个库继续开发
- nitric是一个通用的数据处理库，一旦完成这个库，specs将成为nitric的前端
- nitric的愿景是提供一系列的crate，这些crate是解决数据处理问题的标准方案
- 未来可以使用nitric的领域：游戏开发、游戏物理、模拟器、编译器、数据验证、图形用户界面等
- nitric的哲学是：1. 只能以合理的组合方式解决单个问题。 2. 公开一个通用、可组合和强大的API
- nitric会与其他数据结构兼容，比如ECS/CGS库等
- 也可以使用nitric来作为ECS，比如通过nitric-entity库。文章里也介绍了几个计划中的其他nitric crate。
- Amethyst将继续使用Sepcs，未来是否会迁移到nitric，需要由RFC来推动

大家也不要太亦可赛艇，他只是先分享了计划，还未动工。不过已经有了代码仓库：

- [GitHub: nitric](https://github.com/torkleyy/nitric)
- [GitLab: nitric](https://gitlab.com/nitric/nitric)

[Read More](https://users.rust-lang.org/t/announcing-nitric-the-successor-of-specs/23388)

### 「嵌入式Rust」Rust对Arduino支持进展调查

因为Arduino使用的是AVR微控制器，但Rust目前还未支持AVR。但Rust嵌入式开发组已经有计划支持AVR。

感兴趣的朋友可以关注此issues： [AVR support](https://github.com/rust-embedded/wg/issues/3)

目前也有一个avr-project GitHub项目组独立fork了Rust，提供了对AVR的支持。在官方Rust未支持AVR之前，可以使用这个，看上去还非常活跃。

[avr-rust](https://github.com/avr-rust)

### 「访谈」Josh Triplett访谈

Josh Triplett是一个开源软件贡活跃献者，他最近正和另外一个匿名资助者，准备向非盈利性开源组织Conservancy捐赠9万美元，用于继续推动开源软件的发展。这是对他的采访。

采访中，他提到Rust是他今年在开源社区看到的最激动人心的项目。他说：Rust给了我对计算未来的巨大希望。

也许Conservancy在拿到这笔捐赠后，可能帮助Rust完成组织治理的工作。（猜想）

[Read More](https://sfconservancy.org/blog/2018/dec/18/JoshT/)

### Warp 0.1.10发布

Wary是另一个Rust Web框架，由Hyper作者开发。 新版本的功能简要：

- TLS支持
- CORS
- Websocket检测助手

依然计划和tower-web进行合并，但下一步主要是完善service trait。将来可以通过Service方便地添加tower式中间件。

（那官方Tide框架的中间件协议呢？也许tower式中间件协议在官方中间件协议稳定以后也会支持吧）

[Read More](https://seanmonstar.com/post/181223452087/warp-v0110)

### Tokio 2019展望

主要是两件事：

一、 Async/Await支持

Tokio预计在Rust语言async/await稳定之后，正式支持该语法。如果你想在nightly上面使用async/await，可以使用async-await-preview。

目前tokio是以实验性功能来探索async/await的支持，比如一些特定的API会添加`_async`后缀，一旦async/await稳定了，Tokio将立马采取重大的更新并去除这些后缀。默认情况下，比如，会将`tokio::run_async`改成`tokio::run`。

那么futures-rs 0.1怎么办？然而并不能立马放弃对0.1的支持。

这是一个不断增长的生态系统，包括一些生产环境的应用，也使用了futures 0.1。Tokio会以向后兼容的方式来支持async/await，也就是说，同时支持async/await和futures 0.1。对于已经形成生态的系统而言，变革很困难，还需要和社区共同商讨过渡策略：[关注此track issues](https://github.com/tokio-rs/tokio/issues/804)

二、团队扩展

是时候扩展Tokio的开发和维护团队了，准备像Rust团队学习，分成多个工作组，不同的组负责Tokio的不同方面。当然这个过程还在讨论中。

同时也需要一些新手加入，帮助使用和发展Tokio。

[Read More](https://tokio.rs/blog/2018-12-recap-2018/)

### 「嵌入式Rust」实时消息框架RTFM发布0.4版

嵌入式工作组的老大japaric发布的库，用于构建并发的实时系统，貌似基于RTFM语言。

RTFM语言旨在促进并发编程或嵌入式实时软件的开发。

[www.rtfm-lang.org](http://www.rtfm-lang.org/)

[Read More](https://blog.japaric.io/rtfm-v4/)

### 如何成为一个超级的Rust开发者

这是一个「真香」的故事。

作者在去年看到Rust语言的时候，心想："呵呵，继续吹，继续炒。呵呵，我用C++照样可以完成同样的事，而且还拥有更多的控制权"。

然而，今天这位作者写下了这篇博文，他说：“我之前的想法真是大错特错，错到底了！，当我潜下心来研究Rust，我发现，它是一门经过深思熟虑的语言，它的工作方式有别于我所知道的一切。”

他变成了Rust传道者。

[Read More](https://hashnode.com/post/how-to-become-a-rust-super-developer-cjpv1ee7e000buhs2aqrdw2ym)


### crates.io可视化报告

截止2018年11月，已经达到2w个crates，但很多crate版本发布都小于6次release版本，生态系统依然很年轻。另外一些crate有100多个发布版本，有很多crate都是零依赖，但有3000个crate都依赖serde，所以，serde当之无愧是最流行的crate。

![img](https://wx4.sinaimg.cn/mw690/71684decly1fyghoibbpaj21k80q6goc.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1fyghoie17wj21m20q6mzg.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1fyghoijbjkj217v0u0q84.jpg)
![img](https://wx2.sinaimg.cn/mw690/71684decly1fyghoij5ujj21pi0tugr3.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1fyghoiqjljj21v40sen3f.jpg)

[Read More](https://8-p.info/visualizing-crates-io/)


---

# 学习资源

### Rust Quiz解读已更新到Quiz 21

[去专栏 Read More](https://zhuanlan.zhihu.com/time-and-spirit-hut)

### 「嵌入式Rust」Cortex-M3 入门指南（二）：寄存器与 GPIO 

[Read More](https://zhuanlan.zhihu.com/p/52855259)

### Serverless HTTP

该文作者探索aws lambda平台的无服务器HTTP应用，并编写了一个crate，叫做lando，它以http crate为核心接口，以lambda为部署目标，来部署API网关。本文就是对lando的介绍。

- [lando](https://github.com/softprops/lando)
- [Read More](https://medium.com/@softprops/serverless-http-9a58f9b2df60)

### 「Rust扩展Python案例」快速JSON解析库

基于PyO3库

[orjson](https://github.com/ijl/orjson)

### 解析工具选择之书

有人写了一本书，罗列了现在的解析工具，比如nom、combine、pest等，提供了一些文档和示例，帮助你选择适合使用场景的解析工具。目测还在完善中。

[Read More](https://freemasen.github.io/parsers_presentation/)

### 「系列文章」Rust vs Swift

作者从2015年开始写这一系列博客，直到昨天才发布了一个整理页面。感兴趣可以看看。

[Read More](https://www.chriskrycho.com/rust-and-swift.html)

### rust-derivative: 提供了替代`derive`属性的宏

- [rust-derivative](https://github.com/mcarton/rust-derivative)
- [指南](https://mcarton.github.io/rust-derivative/)

### 「Slides」魅力wasm-bindgen

[Read More](https://speakerdeck.com/tsukushi/attractions-and-interests-of-wasm-bindgen)

---

# 项目

### 下沙：Rust+WASM+WebGL实现的游戏

- [sandspiel.club](https://sandspiel.club/)
- [Code: sandspiel](https://github.com/maxbittker/sandspiel)

### 新的MQTT异步客户端

基于tokio，目前正在召人审核代码

- [rumqtt](https://github.com/AtherEnergy/rumqtt)
- [讨论贴](https://www.reddit.com/r/rust/comments/a79j5v/rumqtt_release_async_version/)

### 「嵌入式Rust」可用于串行热敏打印机的embedded-hal驱动

[thermal_printer](https://crates.io/crates/thermal_printer)

### encoding_rs: Web兼容字符编码库

这是WHATWG编码标准的高性能实现。被用于Firefox 56版本中，替代了uconv库。

- [encoding_rs](https://github.com/hsivonen/encoding_rs)
- [Read More](https://hsivonen.fi/encoding_rs/)

### 用Rust编写的roguelike游戏发布了

基于wasm和Rust，可以在线玩，也可以下载到Windows、Mac和Linux平台。
在Reddit贴中，还记录了感人的开发故事。

- [Reddit 介绍贴](https://www.reddit.com/r/rust/comments/a8be46/dose_response_roguelike_game_written_in_rust/)
- [在线玩](https://tryjumping.com/dose-response-roguelike/play/)
- [下载](https://tryjumping.itch.io/dose-response)
- [源码](https://github.com/tryjumping/dose-response)

### 「深度学习」运行时前端TVM支持（预发布）

TVM的Rust绑定。TVM介绍：

> 有了 TVM，业界与学界开发者们可以快速、轻松地在各个系统（包括手机、嵌入式设备与低功耗芯片）上部署深度学习应用程序，同时无须担心资源与速度的限制。「TVM 作为神经网络和硬件后端之间的共同层，消除了为每类设备或服务器优化各自基础架构的需要。」TVM 项目负责人陈天奇表示，「我们的框架允许开发人员快速、轻松地部署和优化大量硬件设备上的深度学习系统。」

[tvm-rust](https://github.com/ehsanmok/tvm-rust)

### p2p: 支持自定义协议的多路p2p网络库

群友漂流的作品，据说是libp2p的轻量版本。

[p2p](https://github.com/driftluo/p2p)

### ferrugo: JVM的Rust实现

[ferrugo](https://github.com/maekawatoshiki/ferrugo)


---

# 工具与库

### 使用dutree分析磁盘使用情况

该文是dutree的使用教程，dutree是Rust实现的磁盘分析工具。

- [dutree](https://github.com/nachoparker/dutree)
- [Read More](https://ownyourbits.com/2018/03/25/analyze-disk-usage-with-dutree/)

### cargo-expand：查看宏展开结果

serde作者实现的新包，包括声明宏和`#[derive]`过程宏。

```rust
$ cargo expand
```

是对rustc命令的包装：

```rust
$ cargo rustc --profile=check -- -Zunstable-options --pretty=expanded
```

[cargo-expand](https://github.com/dtolnay/cargo-expand)

### 「小工具」验证代码中内存使用

QADAPT库可以验证代码中何时分配或丢弃内存。作者写了篇文章，以构建自定义内存分配器为例来讲解如何使用QADAPT库提供的`debug_assert!`验证代码中内存分配情况。

- [qadapt](https://github.com/bspeice/qadapt)
- [Read More](https://speice.io/2018/12/allocation-safety.html)

### nymic: 一个可以打印类型名字的库

[nymic](https://github.com/myrrlyn/nymic)

### 加速并校准Rust的浮点数解析

作者实现了一个库：[rust-lexical](https://github.com/Alexhuszagh/rust-lexical)，比Rust内置的浮点数解析器快4000倍，并且更加正确。而且支持no_std。

[Read More](https://www.reddit.com/r/rust/comments/a6j5j1/making_rust_float_parsing_fast_and_correct/)

### structview-rs: 用于将二进制数据转换为更高级的数据结构

structview提供了更安全的API来转换

[structview-rs](https://gitlab.com/ra_kete/structview-rs)

###  juniper-from-schema 发布

作者用Juniper的过程中，发现需要Juniper缺少一个关键的东西：可以和客户端共享实际的Graphql架构的文件。虽然可以手工编写，但代码和scheme文件不会实时同步。作者用过程宏实现了可以自动生成scheme的库。

[juniper-from-schema](https://github.com/davidpdrsn/juniper-from-schema)
