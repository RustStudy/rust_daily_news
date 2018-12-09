前言：
从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。
2018-12-09

# 「付费阅读」 如何为Rust语言做贡献

本文基于macOSX平台，通过对Rust 1.32 Nightly版本中rustdoc的一个ICE问题进行复盘，分析并记录整个Bug修复的过程。主要目的是，通过这个过程，来学习如何给Rust做贡献。

（阅读时间：17m）

1. 缘起

2. Rust本地调试环境准备

3. ICE问题分析

4. 调试代码

5. 提交PR说明

[Read More](https://zhuanlan.zhihu.com/p/51479889)

---

# 官方新闻

### Rust 2018 edition 首个语义版本 1.31.0 发布

在这个大版本里，新加入了很多内容：

- NLL（Non-lexical lifetimes）
- 新的模块系统
- 更多的生命周期省略规则
- const fn：可以在编译时把const fn定义的函数用作常量值，但目前稳定的是最小化子集。
- 新的工具：clippy、rustfmt等已稳定
- tool lint：像`#![allow(clippy::bool_comparison)]`这种属性。已经不再需要`cfg_attr`。
- 文档：改进了文档，并且重写了TRPL（The Rust Programming Language）, [TRPL 2018版](https://doc.rust-lang.org/beta/book/)
- Cargo现在已经使用HTTP/2来并行下载crate。而且也不需要在代码里`extern crate`引入crate了。

- [Read More](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
- [1.31 Release Notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1310-2018-12-06)

与此同时，Rust官网也焕然一新：[官网](https://www.rust-lang.org/)

这标志着Rust迈向了成熟。

- Why Rust？ 高性能/可靠/生产力。
- 当你构建CLI、WebAssembly、网络、嵌入式可以考虑使用Rust。
- Rust已经有了很多商业生产级应用
- 如何开始呢？除了Rust book，还有很多其他资源。

### 「官方」 Rust 2018 到底是什么？

本文中，由Rust团队的Lin Clark，用很多漫画来帮你解释Rust 2018到底是什么？

[Read More](https://hacks.mozilla.org/2018/12/rust-2018-is-here/)

### 「官方」征集Rust 2019 RoadMap意见的博客文章

Rust 2019 Roadmap开始制定了，官方现在向社区征集一些博客，希望在博文中提出经过2018年对Rust的使用感受，以及希望Rust改进的目标和方向。去年的路线图RFC是在2018年1月29开放的。今年估计也差不多。

另外一件事，是在考虑下一个大版本的目标：

- Rust 2015：稳定性
- Rust 2018：生产力
- Rust 2021：？

现在官方还不确定下一个大版本是2021，目前只是预估。

[Read More](https://blog.rust-lang.org/2018/12/06/call-for-rust-2019-roadmap-blogposts.html)

### 「官方」 反思Rust和WebAssembly在2018年的进展

官方wasm工作组成立之初提炼出一个核心价值观：Rust和WebAssembly用于扩充JavaScript，而不是替换它。这个核心价值观推动了2018年Rust在WebAssembly领域的发展。

达成的目标：

1. 零成本与JavaScript交互。
   基于wasm-bindgen来完成这一目标，现在基本已经构建出了以wasm-bindgen为核心的生态。

2. 将Rust生成的WASM库分发给NPM。
   由wasm-pack来完成这一目标。不仅仅是生成wasm，还需要分发到npm生态中。

3. 快速提示开发效率。
    对wasm-pack进行了扩展。自动管理wasm-bindgen CLI二进制文件，且自动安装浏览器的WebDriver客户端。比如：`wasm-pack test --headless --firefox`来测试。
    并且增加了一些模板项目来提升开发者效率：
    - `wasm-pack-template`用于方便创建NPM库
    - `create-wasm-app`用于创建web应用
    - `rust-webpack-template`用于使用webpack来创建应用程序
    - `rust-parcel-template`用于使用parcel来创建应用程序

4. Rust生成的wasm应该是可测试和可调试的。
   - console_error_panic_hook，可以将Rust代码产生的panic重定向到浏览器控制台
   - wasm-bindgen-test，作为基础是测试架构，结合wasm-pack来测试dom、js异步事件等。
   - [Twiggy🌱](https://github.com/rustwasm/twiggy)，可以为Wasm分析代码大小。

同样，也向社区征求Rust 2019 Rust和WebAssembly上的目标和建议。另外在感谢名单上，发现了群友：huangjj27。

[Read More](https://rustwasm.github.io/2018/12/06/reflecting-on-rust-and-wasm-in-2018.html)

### 「官方」关于RLS的状态说明

RLS（language server for Rust）马上会随着Rust 2018的发布而发布，并且会共享版本号。但其实RLS并未准备好，只是借用这次Rust 2018发版的机会强调RLS实际上是一个可用且有用的工具。

官方核心人员nrc在这篇文章里，介绍了RLS目前哪些功能已经可用，哪些还不行，以及未来的走向。关注RLS的朋友可以看一下。

[Read More](https://www.ncameron.org/blog/more-on-rls-version-numbering/)



---

# 社区新闻

### 汝为何是Romio？

#async #romio #tokio

无船同志新文：Wherefore art thou Romio?

（ "Romeo, Romeo, wherefore art thou Romeo? —— 莎士比亚《罗密欧与朱丽叶》）

本文讲述了Romio的前世今生。主要目的是为了将Tokio接口移植到Future 0.3。

- 处理显式的waker参数。 0.1和0.3的差异之一是对waker的处理。0.3中提供了一个唤醒当前任务的waker，而0.1则没有。0.1提供的是task::current这样的函数。现在统一使用LocalWaker。
- 消除代码重复。Tokio中的AtomicTask内部类型，实际上在Futures 0.3已经演变为AtomicWaker类型。通过给Tokio发PR来消除这些重复。
- tokio-io到future::io的变化。这部分是最困难的一个部分。目前tokio-io中自定义了AsyncRead和AsyncWrite，而Futures 0.3也定义了AsyncRead和AsyncWrite，这两个版本的接口有很大变化。tokio-io是在unsafe代码上构建的AsyncRead和AsyncWrite，而Futures新的AsyncRead和AsyncWrite是构建于专门的poll_vectored_read扩展。
- Pin。实际上Pin对于0.1到0.3的迁移工作影响很小。基本上就是用`＆mut self：Pin <＆mut Self>`来替换`mut self`。
- 这次迁移有利于`non-'static` Future的应用。将来使用async/await将不必担心因为不是`'static`的Future而产生任何问题。
- Romio是fork自Tokio，但不打算和Tokio竞争。Romio旨在解锁async/await，因为现在Tokio使用futures 0.1已经阻碍了想用async/await人的步伐。毕竟Tokio在生态系统中占比太高。

关于Romio的说明：

- Romio仅包含与异步网络API相关的代码 -  TCP，UDP和Unix域套接字。换句话说，Romio只是 futures + mio。
- Romio只暴露最小的API。
- Romio目前版本为0.3.0-alpha.1，配合futures 0.3来发布更新。

文章中给出了一个echo server的示例。这一切工作都是为了尽可能快速地稳定async/await。

[Read More](https://boats.gitlab.io/blog/post/romio/)

### Rustsim 月报 #2

Rustsim组织是一个GitHub组织，聚焦于提供各种数值模拟的库。包括

- alga， 抽象代数库
- nalgebra， 线性代数库
- ncollide， 2D和3D的碰撞检测库
- nphysics， 2D和3D的物理模拟库

[rustsim.org](https://rustsim.org/)

本月改进：

- 可变形体的物理模拟
- 改进ncollide
- 改进nalgebra

[Read More](https://www.rustsim.org/blog/2018/12/01/this-month-in-rustsim/)

### 「博文」教你如何用Rust写C++

长文预警!

> 背景： Firefox有一个名为encoding_rs的新字符编码转换库。它是用Rust编写的，取代了1999年初发布的名为uconv的旧C++字符编码转换库。因为调用该字符编码转换库的都是C++代码，所以新的库，尽管用Rust编写，但是该库在C++调用者看来，应该像是一个现代的C++库。也就是说，提供给C++调用者的接口使它看起来和感觉就像一个真正的C++库。

(啊哈？假装写C++，你学到了吗？)

[Read More](https://hsivonen.fi/modern-cpp-in-rust/)

### 使用Actix和Sentry构建安全web服务

本文介绍了如何使用Actix创建Web服务，并使用Sentry对其进行监控。主要基于两个sentry平台相关的crate：

- sentry
- sentry-actix

sentry是Sentry平台为Rust开发的SDK，详情：[getting-started-with-rust-error-tracking](https://blog.sentry.io/2018/10/22/getting-started-with-rust-error-tracking)

[Read More](https://blog.sentry.io/2018/12/04/safe-web-services-actix-sentry?utm_campaign=rust&utm_source=social&utm_medium=twitter&utm_content=post&utm_term=actix)

### 案例Librsvg：用Rust重构C项目值得遵循的模式

[Read More PPT](https://people.gnome.org/~federico/blog/guadec-2018-presentation.html)

[PDF](https://people.gnome.org/~federico/blog/docs/fmq-refactoring-c-to-rust.pdf)

### 超级账本新项目Ursa将使用Rust编写

Ursa主要是一个加密库，区块链开发人员可以通过简单的配置文件更改来选择和修改其加密方案。主要使用Rust语言编写，但也会包含Hyperledger中常用到的其他语言的接口。

随着Hyperledger的成熟，Hyperledger中的各个项目已经开始需要复杂的加密实现。不是让每个项目都实现自己的加密协议，而是在共享库上进行协作要更好。

[Read More](https://www.hyperledger.org/blog/2018/12/04/welcome-hyperledger-ursa)

### 在Rust中使用Passenger

Passenger是一个Web应用服务器，常用于Ruby、Node、Python等语言，现在也支持Rust了。

官方写了文章，介绍如何在Rust中使用Passenger作为应用服务器。

[Read More](https://www.phusionpassenger.com/docs/advanced_guides/gls/rust.html)

### Magic Leap用了Servo引擎技术构建浏览器

> Magic Leap One设备为早期开发人员提供浏览器预览版。 该浏览器基于Servo引擎技术构建，并通过WebRender Web渲染库展示高质量的2D图形和字体渲染，并且很快将会有更多新功能。

Magic Leap 高科技！

[Read More](https://blog.mozvr.com/a-new-browser-for-magic-leap/)

### 让Nginx运行于WebWebassmbly

wasmer是一个跨平台的WASM-JIT运行时。wasmer团队尝试将nginx编译为nginx.wasm模块，并将其运行在wasmer之上。本文介绍了他们完成这项工作的历程，以及示例代码。

- [wasmer](https://github.com/wasmerio/wasmer)
- [Read More](https://medium.com/@syrusakbary/running-nginx-with-webassembly-6353c02c08ac)

### Rocket v0.4 发布

0.4版本是Rocket迄今为止最大的更新。

[ChangeLog](https://github.com/SergioBenitez/Rocket/blob/v0.4.0/CHANGELOG.md#version-040-dec-06-2018)

并且多了一个共同维护者：@jebrosen

最关键的是，Codegen已经用Rust过程宏重写了，下一个版本Rocket将支持Rust Stable。

[Read More](https://rocket.rs/v0.4/news/2018-12-08-version-0.4/)


---

# 学习资源

### 「系列」Rust Quiz 解读

[Quiz 1 ~ 10] ： [Read More](https://zhuanlan.zhihu.com/time-and-spirit-hut)

### 「嵌入式Rust」专栏启动

> 本专栏面向于嵌入式开发的入门教程和实践，也就是说，本专栏的文章并不假定读者拥有任何嵌入式开发的知识或经验，但是要求读者有一定 Rust 语言基础，比如说熟悉借用所有权系统，懂得使用 unsafe 手动操作内存结构等等。

感兴趣的可以关注

[Read More](https://zhuanlan.zhihu.com/embedded-rust)

### 使用Rust编写AWS Lambda

[Read More](https://medium.com/@kkostov/rust-aws-lambda-30a1b92d4009)

### Rust类型级（Type Level）的生命游戏

又一次证明了Rust的类型系统是图灵完备的

[primitive-recursive-functions](https://github.com/gtestault/primitive-recursive-functions)

### wasm-bindgen的工作原理

文章里包含了一个PPT

[Read More](http://fitzgeraldnick.com/2018/12/02/wasm-bindgen-how-does-it-work.html)

[ppt](https://fitzgen.github.io/wasm-cg-wasm-bindgen/#1)

### 各种 AOC 2018 解题

另外两个GitHub仓库

- [aoc2018](https://github.com/bpicolo/aoc2018)
- [adventurous](https://github.com/maxdeviant/adventurous)


### 一个简单的光线跟踪实现

[lucis](https://github.com/shaunbennett/lucis)

### 「视频」2018-12-04 RustAKL: ECS编程范例

- [视频](https://www.youtube.com/watch?v=Qc8a2hmpHCA&feature=youtu.be)
- [Code](https://github.com/azriel91/ecs_paradigm)
- [Slides](https://gitpitch.com/azriel91/ecs_paradigm/master?grs=github&t=sky#/)

### HashMap界的瑞士军刀——深入研究hashbrown

hashbrown之前介绍过，是对Google的SwissTable算法实现。目前作者正在尝试将其整合到Rust标准库中（RustFest Roma 2018演讲）。

本文是对hashbrown工作机制的研究报告。值得仔细阅读一下。

[Read More](https://blog.waffles.space/2018/12/07/deep-dive-into-hashbrown/)

---

# 项目

### 一个新的2D游戏引擎

[tetra](https://github.com/17cupsofcoffee/tetra)

### 另一个轻量级的序列化框架：sval

[sval](https://github.com/KodrAus/sval)

[Read More](https://www.reddit.com/r/rust/comments/a2kn7y/sval_a_prototype_nostd_objectsafe/)

### CFG Game: 利用你的计算机技能来制作汉堡包

使用Rust编译为Wasm的在线小游戏。游戏的核心是一个非常标准的LL（1）解析器，其语法是在玩家的游戏中动态定义的。

[Read More](http://rickyhan.com/jekyll/update/2018/12/03/make-burgers-context-free-grammar.html)

源码：[dyn-grammar](https://github.com/rickyhan/dyn-grammar)


---

# 工具与库

### 「工具」更智能展示你shell历史的工具: mcfly

这个工具不得了，自带小型神经网络（mall neural network），可以替换`默认ctrl-r Bash历史搜索`了，更加智能。

[mcfly](https://github.com/cantino/mcfly)

### Tui-rs: 用于构建丰富的命令行界面和DashBoard

[tui-rs](https://github.com/fdehau/tui-rs)

![img](https://wx1.sinaimg.cn/mw690/71684decly1fxtehwnfi7g20k00bkb29.gif)

### 从HTML反序列化为Rust类型

[unhtml.rs](https://github.com/Hexilee/unhtml.rs)

### prettyprint: 让命令行拥有漂亮的输出

[prettyprint](https://github.com/mre/prettyprint)

### cargo-inspect: 探索Rust内部的工具

之前介绍过这个库，今天看到该库作者写了篇文章，更详细介绍它的用法。

[Read More](https://matthias-endler.de/2018/cargo-inspect/)

[cargo-inspect](https://github.com/mre/cargo-inspect)

### cargo-call-stack: 分析程序中的静态调用栈

官方嵌入式组老大japaric写的库。可以对整个程序做静态调用栈的分析，可以最终生成svg图片。调用栈分析结果还包含了栈的具体使用情况（以字节为单位），以及包括Max最大值。

在写一些对栈内存要求苛刻的程序，比如嵌入式，比较有用。

[cargo-call-stack](https://github.com/japaric/cargo-call-stack#cargo-call-stack)

![img](https://wx1.sinaimg.cn/mw690/71684decly1fxuh40azmhj218g0h1jsy.jpg)

### 「CLI」indicatif: 命令行进度条工具

可生成各样且彩色的进度条，非常酷。就不上图了。

[indicatif](https://github.com/mitsuhiko/indicatif)

### JQL: 命令行JSON查询工具

[jql](https://github.com/yamafaktory/jql)
