前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-01-20

---

# 官方新闻

### Rust 1.32 稳定版发布

- 引入了dbg!宏，方便调试代码。（ 有问题，找大(d)表(b)哥(g)看一看 ）
- 默认移除jemalloc。（从1.28开始，从jemalloc切换到全局分配器，直到1.32这项工作才做完）
- 模块系统改进，支持uniform path
- 宏系统改进，支持识别字面量，$lt:literal
- 库稳定。包括标准库里有19个函数使用了const fn等。
- cargo check，增加了cargo c缩写

[详情](https://blog.rust-lang.org/2019/01/17/Rust-1.32.0.html)

### Niko: Rust 2019 专注于可持续发展

[Read More](https://zhuanlan.zhihu.com/p/54349606)

### 「讨论」官方专门独立开启了一个issue来讨论await语法

可以关注下

[Resolve `await` syntax](https://github.com/rust-lang/rust/issues/57640)

### Tide框架发布了0.0.1版本

是Tide框架的首个语义化版本，但还是WIP状态，仅供参观。

[tide](https://crates.io/crates/tide)

---

# 社区新闻

### Rust比Haskell更加安全

这篇文章阐述了Rust比Haskell更加安全的利用。

- [Read More](https://www.fpcomplete.com/blog/when-rust-is-safer-than-haskell)
- [Reddit讨论](https://www.reddit.com/r/rust/comments/ah85lu/when_rust_is_safer_than_haskell_fp_complete/)

### Rust上升GitHub语言排行榜第13位

GitHub最近30天按PR排序，Rust排名第13 ​​​​

![img](https://wx4.sinaimg.cn/mw690/71684decly1fyxu1f07obj20wm0u0wq9.jpg)

### 来用Rust实现星际争霸的AI吧

[Read More](https://habr.com/en/post/436254/)

### rustsim最新月报

Rustsim组织是一个GitHub组织，聚焦于提供各种数值模拟的库。包括

- alga， 抽象代数库
- nalgebra， 线性代数库
- ncollide， 2D和3D的碰撞检测库
- nphysics， 2D和3D的物理模拟库

[rustsim.org](https://rustsim.org/)

版本升级：

- nphysics2d and nphysics3d v0.19
- ncollide2d and ncollide3d v0.18:
- nalgebra v0.17

本次月报里还阐述了他们的2019目标。总结来说：

[Read More](https://www.rustsim.org/blog/2019/01/01/this-month-in-rustsim/)

### Rust devroom ： Rust生态专题

本专题集合了Rust生态中令人兴奋（据专题作者描述）的工具和动态。

[Read More](https://fosdem.org/2019/schedule/track/rust/)

### Rust在Hyperledger Sawtooth团队中的应用

区块链公司hyperledger，使用Rust实现了两个新的共识引擎：Sawtooth PBFT和RAFT（基于PingCAP的raft-rs）

[Read More](https://www.hyperledger.org/blog/2019/01/18/safety-performance-and-innovation-rust-in-hyperledger-sawtooth)

### 「演讲」是时候用Rust重写OS了吗？

来自QCon的演讲，Bryan从操作系统的历史，到Rust语言的实现机制，甚至还探讨了Rust标准库中为什么用B树，而不是AVL树。最终的结论是，迫不及待用Rust了。

> Bryan Cantrill是Joyent的首席技术官，负责监督SmartOS和SmartDataCenter平台的全球开发。他很惊叹为什么Rust语言可以在性能上快于C。

[Read More（内含视频及演讲稿）](https://www.infoq.com/presentations/os-rust)

### 「安全」Rust 2019以Security作为目标

Rust社区的民间组织「Rust安全代码工作组」发起者发布的文章，阐述了2019年安全组的工作目标。该民间工作组致力于使用Rust编写更安全的代码。这里的安全主要是指网络安全/计算机安全/代码安全验证/代码审计等。

感谢有这样的组织为Rust保驾护航，现在他们也在寻求社区的力量。

- [Read More](https://medium.com/@shnatsel/security-as-rust-2019-goal-6a060116ba39)
- [rust-secure-code/wg](https://github.com/rust-secure-code/wg)

### Librsvg目前几乎是100%Rust代码了

（追求100%Rust代码，是一种拥抱新世界的决心）

[Read More](https://people.gnome.org/~federico/blog/librsvg-is-almost-rustified.html)

### Timely Dataflow 将其Git仓库转移到了GitHub了

- timely-dataflow, 用於管理和執行數據並行數據流計算的框架。
- differential-dataflow，對一個圖(graph)，不是圖片是圖論的圖去做差分運算，也能統計圖跟圖之前的變化，
用於輔助timely-dataflow的計算。
- abomonation， 一個追求簡單快速但unsafe的序列化程式庫
- Naiad，是timely-dataflow的C#實作品，從微軟研究院fork過來，但沒有做任何改動。

[TimelyDataflow](https://github.com/TimelyDataflow)


### 「讨论」如何用Safe Rust实现链表

不想用Rc，也不想用Unsafe Rust，看看大家的讨论中有什么值得学习的方案？

[Read More](https://www.reddit.com/r/rust/comments/aew8kg/how_to_implement_linkedlist_in_rust_using_only/)

### Rust 2019 期望贴汇总

[rust-2019](https://readrust.net/rust-2019/)

### Rust的const类型、trait和实现

文章主要讨论了泛型的 const fn，提了一个提案增加 const trait 和 const impl 的概念，而泛型 const fn 的类型参数将要求要有相应 const impl 的类型来填入。

[Read More](https://varkor.github.io/blog/2019/01/11/const-types-traits-and-implementations-in-Rust.html)

### 「娱乐向」Rust程序员的进化

[Read More](http://antoyo.ml/evolution-rust-programmer)

---

# 学习资源

### 《Rust编程之道》读者答疑精选：函数项类型和函数指针类型

[Read More](https://zhuanlan.zhihu.com/p/54485063)

### 「系列文章」从零开始实现一个关系型数据库

计划五篇，目前作者只写了第一篇。源码仅供学习之用。

- [Read More](https://natelincoln.com/a-very-relatable-database/)
- [源码： NLincoln/relatable](https://github.com/NLincoln/relatable)

### 如何编写Rust代码

作者分享了他如何组织和构思Rust代码的一些技巧，值得参考。

[Read More](https://deterministic.space/how-to-order-rust-code.html)

### 用Rust写操作系统系列文章：介绍内存分页

这一系列文章比较有名，清华大学的操作系统课程也借鉴了这一系列文章来使用Rust进行教学。

[Read More](https://os.phil-opp.com/paging-introduction/)

### 理解Rust生命周期

[Read More](https://medium.com/nearprotocol/understanding-rust-lifetimes-e813bcd405fa)

### 简单的游戏开发指南

[rust-simple-game-dev-tutorial](https://github.com/sunjay/rust-simple-game-dev-tutorial)

### 根据数据文件生成Rust测试

使用build.rs，将数据文件生成Rust集成测试。

[Read More](https://blog.cyplo.net/posts/2018/12/generate-rust-tests-from-data.html)

### Google Cloud Endpoints 与 Rust gRPC集成

作者记录了他将Google Cloud Endpoints 和 Rust gRPC集成的故事，以便帮助后来者。

[Read More](https://www.wihlidal.com/blog/cloud/2019-01-17-google-cloud-endpoints/)

---

# 项目、框架、工具与库

### ElasticSearch的竞争者们

- [Toshi](https://github.com/toshi-search/Toshi)
- [Tantivy](https://github.com/tantivy-search/tantivy)

### Tokio作者新库loom ：Rust并发模型检查器

Rust的多线程内存模型继承自C++11，所以该库就是基于[CDSChecker: Checking Concurrent Data Structures Written with C/C++ Atomics.](http://demsky.eecs.uci.edu/publications/c11modelcheck.pdf)这篇论文来实现的。

[loom](https://github.com/carllerche/loom)

### dness: 一个动态DNS Client

[dness](https://github.com/nickbabcock/dness)

### hexyl: Rust编写的命令行十六进制查看器

常数做泛型参数的feature开始实现了: [pr53645](https://github.com/rust-lang/rust/pull/53645#issuecomment-451706747)

[hexyl](https://github.com/sharkdp/hexyl)

### 全算法支持的纯 Rust 实现 crc 库

> CRC(Cyclic Redundancy Check)：循环冗余检验，在链路层被广泛使用的检错技术。

[CRC](https://github.com/nanpuyue/crc)

### 「文章」Paw：新的Rust GUI框架

灵感来自于Flutter 1.0。作者在文章里介绍了他的想法，并且计划接下来发布0.1版本。

- [Read More](https://medium.com/@m.siglreith/paw-at-rust-guis-d4d848e14b94)
- [相关文章：Rust实现的Flexbox](https://medium.com/visly/stretch-a-flexbox-implementation-in-rust-60762b5a3331)

### nrc的新库： 为过程宏添加类似macro_rules的模式匹配功能

这样写过程宏可以更加直观一些。

- [Read More](https://www.ncameron.org/blog/proc-macro-rules/)
- [nrc/proc-macro-rules](https://github.com/nrc/proc-macro-rules)


### pickledb-rs：一个简单的Key-Value数据库实现

仿python实现的K-V数据库pickleDB

[pickledb-rs](https://github.com/seladb/pickledb-rs)

### Pi生成器

该库是根据知名数学网红3Blue1Brown的视频实现的。

[pi-generator-from-blocks](https://github.com/jakevossen5/pi-generator-from-blocks)

### rbtag: 可以添加构建信息的过程宏

该库提供了过程宏，可以为你的库添加构建信息，或者是git commit信息。

[rbtag](https://github.com/LivingInSyn/rbtag)

### breeze：实验性的文本/代码编辑器

一个实验性的，kakoune启发的以CLI为中心的文本/代码编辑器，带有`|`形状游标。

[breeze](https://github.com/dpc/breeze)

### dlarm：一个dwm的告警系统

[dlarm](https://github.com/codesections/dlarm)