前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-01-06

---

### 「通告」

微店《Rust编程之道》签名版已发货，请注意查收。没有发货的，等待下一批吧，就这一两天内发。

---

# 官方新闻

### Mozilla招人了

很多岗位，包括Rust实习生。在美国的朋友有福了。

[Read More](https://careers.mozilla.org/position/gh/1480831/)

---

# 社区新闻

### 「社区」关注Async动态

[are we async yet?](https://areweasyncyet.rs/)


### 「区块链」 Holochain: 进展

 > Holochain是一个采用DHT(分布式哈希表)技术的创新项目，借助BitTorrent 的并行性，为分布式应用（DAPP）提供动力，它是分布式应用程序的数据完整性引擎。它可以将具有用户自主权的分布式 Web 直接构建到其架构和协议中。Holochain和区块链是针对截然不同的应用场景而构建的。对于维持绝对的全球共识的系统来说，区块链相对有优势。在需要弱共识（大多数情况下）的情况下，Holochain 比区块链要好得多：它更快，更高效，更具可扩展性，适应性强和可扩展性。分布式账本技术（DLT）可互换的分支分别是有向无环图（DAG）、Swirlds的哈希图Hashgraph、Holoochain 的分布式散列表（DHT）、区块链，四者是并列关系。

 Holochain用Rust重新实现了之前Go写的核心框架。目前正在准备完善的P2P网络。

- [Holochain深度介绍](http://www.genesisfor.com/life1/show/338.html)
- [holochain-rust](https://github.com/holochain/holochain-rust)
- [Read More](https://medium.com/holochain/establishing-tech-stack-foundation-and-preparing-for-full-p2p-networking-46ababdb6e44)

### 「博文」GitHub Actions第一印象

作者（mgattozzi）最近尝试了GitHub Action的测试版。本文实现了一个类似于Rust项目机器人（比如Rust源码仓库中的机器人bors）的容器来介绍GitHub Action的用法。

- [github-rs](https://github.com/mgattozzi/github-rs)
- [Read More](https://mgattozzi.com/github-actions-an-introductory-look-and-first-impressions/)

### 「博文」Rust重写Python项目心得

作者用Rust重写了一个Python项目，性能提升了9倍，内存占用少了一半。本文记录了他重写过程中的心得。

[Read More](https://alantrick.ca/writings/programming/python_to_rust/)

### 「博文」Cpp、Rust和D分别实现Pythagorean三元组的性能比较

[Read More](https://atilanevesoncode.wordpress.com/2018/12/31/comparing-pythagorean-triples-in-c-d-and-rust/)

### 经验报告：从Rustacean的角度来看Swift

同时喜欢Swift的可以关注下

[Read More](https://blog.waffles.space/2018/12/31/swift-experience-report/)

### 2018俄罗斯AI杯挑战赛

俄罗斯AI杯 - 由Mail.Ru Group和Codeforces组织的年度IT导向竞赛计划。今年的比赛叫做CodeBall。参与者编写代码使他们的机器人将球击入对手的网中。用Rust和WASM编写AI策略。

- [Read More](https://blog.kuviman.com/2019/01/01/russian-ai-cup-2018.html)
- [Quick Start](https://russianaicup.ru/p/quick)
- [在线试玩](https://russianaicup.ru/play)

### 「博文」2019期望：使用Rust制作和使用C兼容的库

目前构建和安装共享库比较困难，尤其是在多个平台上。Cargo的能力目前还待扩展，Rust的ABI也未稳定，所以需要使用兼容C-ABI来和其他语言沟通。但是现在如果你的库想提供一个C绑定接口，目前还没有一个完美的解决方案。作者罗列了现存的问题，并且他计划在2019年尝试提供一些解决方案。

[Read More](https://blogs.gentoo.org/lu_zero/2018/12/30/making-and-using-c-compatible-libraries-in-rust-present-and-future/)

### 「学术论文」利用Rust类型进行模块化规范和验证

论文的四个作者均来自于瑞士苏黎世联邦理工学院计算机科学系。文中声称利用了一种新的验证技术，利用Rust类型系统来简化Rust程序的规范和验证。

[Read More](https://www.research-collection.ethz.ch/handle/20.500.11850/311092)

### Google出品的最节能的语言数据统计

[Read More](https://sites.google.com/view/energy-efficiency-languages/results)


---

# 学习资源

### 趣图分享

我在刚学Rust的时候，这俩类型搞的我很懵。今天做这俩图，方便帮助初学者理解。

![img](https://wx3.sinaimg.cn/mw690/71684decly1fyt9hjn4nnj217b0u0npd.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1fyt9hmyc71j21in0u0e82.jpg)

### 「视频」使用Rust构建高性能并发数据库

- [Read More](https://www.reddit.com/r/rust/comments/acucrs/rust_at_speed_building_a_fast_concurrent_database/)
- [Reddit 讨论贴](https://www.reddit.com/r/rust/comments/acucrs/rust_at_speed_building_a_fast_concurrent_database/)

### 「嵌入式Rust」树莓派3裸机编程指南

> 目标受众是对此硬件不熟悉的业余操作系统开发人员。它将为您提供有关如何执行常见操作系统任务的示例，例如写入串行控制台，从中读取击键或使用各种外围设备（如硬件支持的随机数生成器）。但是，它不是如何编写完整操作系统的教程。我不会涉及高级内存管理和虚拟文件系统等主题，或者如何实现多任务处理。

[Read More](https://github.com/rust-embedded/rust-raspi3-tutorial)

### 「嵌入式Rust」设置Arduino UNO版和Mac的开发环境

[Read More](https://treesandrobots.com/2018/12/rustduino-pt-1-setting-up-development-environment.html)

### 「博文」使用过程宏派生trait

作者以扩展askama模板为例，介绍了过程宏的使用。

[Read More](https://naftuli.wtf/2019/01/02/rust-derive-macros/)

### 「博文」对Rust中错误传播的思考

[Read More](https://people.gnome.org/~federico/blog/propagating-errors.html)


### 「流媒体技术」构建MPD索引

MPD(Media Present Description)可以非常简洁——只需要短短的几行表述就可以构建一个很大的媒体列表。

作者一般从SoundCloud下载音乐进行播放，但是过程很麻烦：必须浏览SoundCloud，下载一个看起来很有趣的曲目，将其上传到音乐服务器，让MPD将其编入索引，然后播放。所以作者用Rust制作了SoundClound音乐索引，文章记录了他的做法。

- [Read More](https://polyfloyd.net/post/soundcloud-fuse-mpd/)
- [源码：soundcloud-fs](https://github.com/polyfloyd/soundcloud-fs)

### hackerman: 用Rust编写的16位Hack机器语言的汇编程序

基于nom实现，可当学习之用。

[hackerman](https://github.com/onatm/hackerman)

---

# 项目、框架、工具与库

### Reducer 1.0发布

号称用Rust实现了Flux模式的可预测性(Predictable)响应式框架。受js的Redux库影响很大。可用于管理任何类型的应用程序的状态。 尤其是GUI。

[reducer](https://github.com/brunocodutra/reducer)

### 「小项目」[PoC]使用WASM、async/await和Futures0.3构建项目模板

这是对rust-webpack-template库的改装，支持了wasm、async/await和futures 0.3。

[Rust-WASM-Async-Example](https://github.com/SillyFreak/Rust-WASM-Async-Example)

### Piet：2D图形抽象

包含了一些用于2D图形绘制的API。

[piet](https://github.com/linebender/piet)

### r64emu： Rust实现的任天堂N64模拟器

目前完成度很低，还不能玩任何游戏。

[r64emu](https://github.com/rasky/r64emu)

### rust-rocksdb: rocksdb的Rust绑定

[rust-rocksdb](https://github.com/rust-rocksdb/rust-rocksdb)

### console_log: 支持在浏览器控制台中打印Rust的日志信息

基于wasm-bindgen实现

[console_log](https://github.com/iamcodemaker/console_log)

### 「图形处理」meshlite: 3D网格生成和处理

状态：WIP

[meshlite](https://github.com/huxingyi/meshlite)

### vec_tree: 提供了安全的树结构

基于generational-arena，避免了ABA问题。

- [vec_tree](https://docs.rs/vec-tree/0.1.0/vec_tree/)
- [generational-arena](https://github.com/fitzgen/generational-arena)

### pyoxidizer: Rust编写的Python应用分发工具

[pyoxidizer](https://github.com/indygreg/pyoxidizer)

### Ropey发布1.0版

Ropey是一个文本rope数据结构的实现，旨在成为文本编辑器等应用程序的后备文本缓冲区。

Rope一个二叉查找树。优势：

- Rope不需要连续的内存空间，不像数组
- Rope在字符串中进行插入和删除更快O（logn）。而在字符串数组中需要O（n）。
- 在进行数据copy时。Rope不需要额外的O（n）内存空间。

缺点

- 需要更多的内存空间，需要维护父节点。
- 增加代码的复杂度。

[ropey](https://github.com/cessen/ropey)

### regex-automata: 使用DFA的正则表达式库

支持no_std环境。与regex库的区别是：

- regex是通用的正则表达式引擎，对编译时间、搜索和内存使用有一个均匀的平衡，还提供了方便的API。
- regex-automata，则提供了更加底层的接口，可能对于用户来说不太方便，但它提供对内存使用和搜索时间更明确的控制。

[regex-automata](https://github.com/BurntSushi/regex-automata)

### strength_reduce: 快速整数除法和模数运算

支持no_std环境。

[strength_reduce](https://github.com/ejmahler/strength_reduce)

### umeboshi: Rust实现的交互式shell

[umeboshi](https://github.com/masahiko-ofgp/umeboshi)

### rust-unic: Unicode和I18n库发布0.8版

[rust-unic/](https://github.com/open-i18n/rust-unic/)

### kurbo：处理曲线的库

包含曲线和矢量路径的数据结构和算法。 它可能最适合创作工具，但它足够通用，可能对其他应用程序有用。

[kurbo](https://github.com/linebender/kurbo)