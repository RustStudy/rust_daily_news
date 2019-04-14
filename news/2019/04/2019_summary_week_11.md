### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-04-07

---

# 通告两则：

### Rust日报组成立

「Rust日报」2019每周精选前两周因为有事所以中断两期，今天开始继续。借此也成立了一个Rust日报小组，目前已有两人加入。

发日报有什么好处？我来给大家透露一下。至少有三点好处：

1. 了解Rust生态最新动态
2. 锻炼自己的总结能力。对于感兴趣的文章，可以快速汲取其主要观点。
3. 为自己的提升和学习铺垫了资料库。

所以，也欢迎大家参与Rust日报的内容建设中。

### 与 Rust 大神面基指南（一） | RustCon Asia

[Read More](https://zhuanlan.zhihu.com/p/61461452)

首届RustCon Asia大会还有差不多两周就临近了，还没上车的可以上车了。来欣赏一下大会举办方的精美Banner，顺便扫描二维码可以购票。

![img](https://wx1.sinaimg.cn/mw690/71684decly1g1ucbj5ufij20u00hsgn5.jpg)
![img](https://wx2.sinaimg.cn/mw690/71684decly1g1ucbrkcsnj20u90hz474.jpg)
![img](https://wx2.sinaimg.cn/mw690/71684decly1g1ucbrwac0j20u00htwmp.jpg)
![img](https://wx4.sinaimg.cn/mw690/71684decly1g1ucbrnyt1j20u00hsjsk.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1g1ucbrr0upj20u80hyq50.jpg)
![img](https://wx1.sinaimg.cn/mw690/71684decly1g1ucbrp7nej20u80hyjs9.jpg)

---

# 官方新闻

### Gloo更新：Onion层、计时器和事件

Gloo是一个用Rust和Wasm构建快速可靠的Web应用程序和库的模块化工具包，于两周前开始构建。最新的进展如下：

- 增加了「“洋葱”分层API」。

也就是说，API的构建就像是洋葱一样，分好几个抽象层。他们希望在raw-sys库之上构建一层中级的API，然后在中级API层再构建一层高级的面向用户的API。并且每一层都是公开暴露接口并可复用。这样设计是为了最高限度地提高大型生态系统的可重用性、通用性等。

其中核心层API是构建在wasm-bindgen，js-sys和web-sys之上的原始绑定。中间层是回调（callback）层，比如gloo_timers就包含在这一层。高级层是Futures和Steams层，当然现在是futures 0.1，等异步稳定以后再切换到最新的。未来也有可能增加更高的抽象层。比如某些Web API或是为了生态系统而集成的层。

- 另一个正在进行的设计是如何制作事件目标和监听器层。

[Read More](https://rustwasm.github.io/2019/03/26/gloo-onion-layers.html)

###  当前Async已经修复的问题

在最近的一条已merge的PR中，顺便列出了所有已被修复的async中已知的生命周期问题。

[rust/pull/59286](https://github.com/rust-lang/rust/pull/59286)

### Future API終於要定案了！

[Read more](https://www.reddit.com/r/rust/comments/b9se8s/stabilizing_future_apis/)

### HashMap的最新版实现要合并了

[Read more](https://www.reddit.com/r/rust/comments/b9ol2l/the_new_hashmap_is_ready_for_merging/)

---

# 社区新闻

### 愚人节合集

提议整个Rust项目由bors机器人管理。  

[https://github.com/rust-lang/rfcs/pull/2671](https://github.com/rust-lang/rfcs/pull/2671)

提议将Cow来默认导出  （这个我真信了） 

[https://github.com/rust-lang/rfcs/pull/2672](https://github.com/rust-lang/rfcs/pull/2672)   

Rust 2.0要开始筹备了，主要目标有：

- 为了更健康的crate生态，准备提供一个审查工具，帮助用户剔除掉一些依赖包
- 语言简化。 考虑删除过程宏功能，准备汲取Zig语言的编译器动态反射来替代当前的过程宏。另外，还要放弃ref和mut修饰符，用于简化用户的开发体验。
- 为了进一步增加安全性，将会添加： 依赖类型和有状态视图的支持。依赖类型有助于提升代码安全性，有状态视图有助于减少unsafe代码的使用。
- 编译器改进。考虑让rust编译器比gcc编译的更快。

![img](https://wx2.sinaimg.cn/mw690/71684decly1g1n18k48ekj22um0u045z.jpg)

祝大家愚人节快乐。

### Witnet Foundation： Rust区块链工程师招聘

新加坡公司，非远程。Witnet-将智能合约连接到任何外部数据源。

- [Read More](https://angel.co/witnet-foundation-1/jobs/342272-rust-developer-blockchain-medior-senior)

### 成功故事：Rust在企业领域的应用

[Read More](https://zhuanlan.zhihu.com/p/61410107)

### PingCAP Talent Plan 第二期火热来袭，线上课程全面开放！

[Read More](https://zhuanlan.zhihu.com/p/61340679)

另外，我给准备参与此人才计划的选择阅读《Rust编程之道》的读者，提供一些提高学习效率的建议。

1. 先看第一章，对Rust有整体了解，重点是了解Rust社区、Rust语言架构和Rust编译过程
2. 使用检索式阅读，把Rust By example过一遍，把Rust语法先通览一遍。
3. 然后再看《Rust编程之道》第二章，重点理解「一切皆表达式」的概念，借此把语法再次梳理一遍
4. 攻克所有权，同样检索式阅读看三四五章。
5. 可以结合其他的学习资料，进行主题式阅读（针对一个主题，比较式阅读学习）

下图为检视阅读方法：

![img](https://wx2.sinaimg.cn/mw690/71684decly1g1pufa5bjej21jl0klju3.jpg)

### Linkerd v2: 从产品中吸取了什么了教训导致重写了服务网格？

[Read More](https://zhuanlan.zhihu.com/p/61617593)

### 台湾 COSCUP 開源人年會 

COSCUP 2019 - Aug 17th-18th. NTUST, Taiwan 無論您是開放原始碼的開發者、推廣者、使用者、還是想了解軟體的新手，都歡迎您來參加為開放原始碼開發者、使用者和推廣者舉辦的「開源人年會」

在寻求Rust Topic征集。

[Read More](https://blog.coscup.org/2019/04/2019-cfp-open.html#rust?tdsourcetag=s_pcqq_aiomsg)   
[COSCUP 2019](https://coscup.org/2019/)

### GitHub 上有哪些值得关注的 Rust 项目？

可以关注下这个知乎问题，下面链接是对区块链框架CITA的介绍。

[Read More](https://www.zhihu.com/question/30511494/answer/643314455)


### 「讨论」Rust是不是一个好的C替代品？

针对这篇文章「Rust不是一个好的C替代品（Rust is not a good C replacement）」引发的讨论。

- [Rust is not a good C replacement](https://drewdevault.com/2019/03/25/Rust-is-not-a-good-C-replacement.html)
- [Reddit讨论](https://www.reddit.com/r/C_Programming/comments/b8cokd/rust_is_not_a_good_c_replacement/)

### 「讨论」旁观Rust目前的状态

有人在reddit发帖，探讨了他目前观察到Rust的一些状态，他关注的点是：

- 异步语法。正在积极地走向稳定。
- 消息队列（Graphql订阅、mqtt等）。被异步耽误了，目前Graphql订阅支持最有希望的是Actix。
- 嵌入式Rust。看起来得2019年年底成熟。
- WASM（Graphql UI Client）。Yew好像最受欢迎，但是不支持wasm-bindgen，只能用stdweb，所以导致很难用webpack来处理css模块。当然也有人在解决这些问题。
- 云支持。看上去支持的不错。

结论： 目前Rust生态已经处在一种「马上能成事」的边缘。作者表示，如果现在用Rust构建他想要实现的产品，可能需要自己构建或者等待一些工具（大约一年）。他也知道现在开始学习Rust正好，也可以提交一些PR来改进生态，但是他不想这样（233，估计是时间关系），然而他说，他可以出钱赞助这些加速生态发展的项目维护者。

[Reddit 讨论](https://www.reddit.com/r/rust/comments/ba7yci/state_of_rust_from_an_outside_pov/)

### nom 5.0 发布预告

nom是Rust社区的一个重要的解析工具，作者在reddit发帖表示将要发布5.0版本。（这是一个你没有看过的船新版本，挤需体验三分钟，里造会干我一样，爱象这个版本。）

你可以在nomfun项目里体验到nom的新设计。在这个库里完全看不到宏的影子了，取而代之的是Functor，用法和另一个解析库combine趋于一致。新版本据说性能会有5%-20%的提升，并且有更好的错误处理系统，同时保持大部分向后兼容。因为作者家里有事，可能5.0版本需要几个月才能发布，不过近期他会先发一个alpha版本，作者也列出了一个5.0的路线图。

- [Read More](https://www.reddit.com/r/rust/comments/ba366j/call_for_help_releasing_nom_50/)
- [nomfun](https://github.com/Geal/nomfun)
- [combine](https://github.com/Marwes/combine)

### Diesel的作者离开了Rails准备全职投入Rust

Sgrif在Rails社区服务了6年多，提交了1452个commit，重写了Rails 4.2的大部分库。对Rust产生兴趣之后，创建了Diesel库，Rust实现的ORM框架。2018年4月，接手管理crates.io，但是2018年10月他从Shopify公司离职，想全职投入Rust社区。

他现在想要以下支持：

- 一份来自大公司的赞助，以便支持crates.io的维护。
- 一份可以兼职的工作，以便维持生计。

[Read More](https://blog.seantheprogrammer.com/moving-on-from-rails-and-whats-next)

### rustsim 报告 #5

简要：

- alga 0.9 和 nalgebra 0.18开始支持复数
- 并且与`＃[no-std]`保持兼容
- nalgebra开始在几何代数上添加对三角函数和卷积的支持等等。

[Read More](https://www.rustsim.org/blog/2019/04/01/this-month-in-rustsim/)

### 一个Elixir/Javascript程序员准备切换到Rust的思考

他发帖主要是想寻求一个快速学习的方法，如果有同样需求的初学者，可以看看讨论区的回复。

[Reddit 讨论](https://www.reddit.com/r/rust/comments/b7qjmq/elixirjavascript_programmer_thinking_of_switching/)

### 已经有人写了 WASI 和 Lucet 的两篇使用文章

[Post 1](https://hermanradtke.com/2019/04/01/wasi-example-using-rust-and-lucet.html)  
[Post 2](https://hermanradtke.com/2019/03/31/lucet-in-five-minutes.html)

### Mesalink v1.0.0发布

MesaLink TLS是百度安全实验室研发的下一代传输层安全(Transport Layer Security, TLS)库。正式支持TLS 1.3和IPv6，支持CMake编译，支持Windows，实现生产环境可用。

[Read More](https://mp.weixin.qq.com/s/O6rRwJGMGWuy7bziwul6eg)

### 「远程工作」Chorus One寻找加密软件工程师

要求懂得Go或Rust。该团队成立15个月，有6名团队成员，工作主要是区块链相关。公司在美国西海岸，可全职远程。

[Read More](https://blockchain.works-hub.com/jobs/remote-cryptonomic-software-engineer-1f007?utm_source=reddit&utm_medium=chorus%20one&utm_campaign=j.gretton)

### 采访Gleam的作者：使用Rust编写Erlang VM

长文。关键问题：

为什么选择Rust来实现Gleam编译器？ （而不是选择erlang / elixir等）

> Gleam最初是用Elixir来写的，但很快又换成了Erlang，后来他想要重构代码但是意识到，没有一个好的静态类型来帮助它纠正设计中的错误，所以就对重构失去了信心。后来选择了Rust，花了3个月时间重构了，功能大致相同，但是有更少的错误和更少的技术债务。

Rust是实现编程语言的好语言吗？

> 作者表示Rust不错，很适合。并且用Rust实现Erlang VM之后启动和加载各种模块不再有延迟。但是他认为，Rust还不是实现编程语言的完美语言，主要是编写类型检查的代码让他很沮丧。但总的来说，他对自己用Rust的决定还是挺满意的。

[Read More](https://notamonadtutorial.com/an-interview-with-the-creator-of-gleam-an-ml-like-language-for-the-erlang-vm-with-a-compiler-e94775f60dc7)


### 「系列文章」glium指南

这应该是对官方Glium book的中文翻译

[Read More](https://zhuanlan.zhihu.com/p/57805534)

---


# 学习资源

### 编译器性能和LLVM

本文作者以他自己实现的Cone编译器为案例深度探索了编译器架构和LLVM的性能，并比较全面地阐述了LLVM的性能概况。

- 编译器架构。Cone选择了C，而非Rust，是因为作者对性能更关心一些，他认为C更好。还介绍了一些前端架构的优化原则。
- LLVM后端性能。作者测量了LLVM后端各个编译阶段的性能，顺带阐述了LLVM的构建流程。

LLVM构建流程：

- Setup。初始化有关目标计算机，数据布局和全局上下文的信息。这个阶段的执行时间是固定的，不会随着源程序的增大而变长。
- Gen LLVM IR。Cone和LLVM的混合阶段，作者的Cone编译器会生成LLVM IR，就像Rust一样。同时作者也实现了一个Cone IR，类似于Rust的MIR。但Cone可能比Rust更快一些，因为没有Rust这么多分析。
- 验证LLVM IR。这个过程是对LLVM IR的有效语义进行分析。确保IR的格式正确、通过类型检查，其算法复杂度为O(n)，与LLVM IR节点的数量成正比。
- 优化LLVM IR。将执行6次LLVM优化：将栈变量转换为寄存器、函数内联、窥孔优化（peephole optimization）、位操作优化（bit twiddling）、公共子表达式消除和控制流程简化。一些优化过程可能是O(n)，但某些复杂的可能是指数级的。
- 生成目标，并将其存储于磁盘。此阶段占整个LLVM运行时间的73%。

小百科

> 窥孔优化可以在四个方面寻找优化机会：冗余指令删除，包括冗余的load和store指令以及死代码(不会执行的代码);控制流优化；强度削弱；利用特有指令。

作者也从LLVM架构上探讨了LLVM缓慢的原因。并且作者还打算继续研究优化LLVM的方案以及替代LLVM的方案。更多详细请阅读原文。

- [Read More](http://pling.jondgoodwin.com/post/compiler-performance/)
- [Reddit 讨论](https://www.reddit.com/r/rust/comments/b98v3c/compiler_performance_and_llvm/)

### 「系列文章」Rust中实现DSL的挑战

- [Part I](https://blog.yoshuawuyts.com/dsls-1/)
- [Part II](https://blog.yoshuawuyts.com/2019-03-03-dsls-2/)

### 解释：Rust异步如何工作？

[Read More](https://dev.to/gruberb/explained-how-does-async-work-in-rust-46f8)

### 「系列文章」理解Futrue Part I

[Read More](https://www.viget.com/articles/understanding-futures-in-rust-part-1/)

### 使用Tensorflow Rust进行人脸识别

本文将引导你基于一些现有模型来使用rust和tensorflow完成面部识别。 使用的是名为mtcnn的预训练模型进行人脸识别。

- [Read More](https://cetra3.github.io/blog/face-detection-with-tensorflow-rust/)
- [mtcnn](https://github.com/cetra3/mtcnn)

### 高性能JavaScript词法分析器

该篇文章分享了如何使用Rust实现一个基于状态机的高性能的JS词法分析器。

- [Read More](https://medium.com/@retep007/javascript-lexing-for-high-performance-f9a800ec930d)
- [javascript-es9-parser](https://github.com/retep007/javascript-es9-parser)


### 介绍Seqlocks

seqlocks（顺序锁）对读写锁的一种优化。使用顺序锁，读执行单元绝不会被写执行单元阻塞，也就是说，读执行单元可以在写执行单元对被顺序锁保护的共享资源进行写操作时仍然可以继续读，而不必等待写执行单元完成操作，写操作也不需要等待所有读执行单元完成读操作才去进行写操作。用于受保护的资源很小，简单且经常访问，适用于写操作很少但必须很快的场景。Linux内核处理中断的函数使用了seqlock。 

该文作者也是实验性软件事务内存库swym的作者，swym是对seqlock的一种实现，基于Transactional Locking II 论文。

- [Read More](https://mtak-blog.github.io/generalizing-seqlocks)
- [swym](https://github.com/mtak-/swym)
- [Transactional Locking II 论文](https://people.csail.mit.edu/shanir/publications/Transactional_Locking.pdf)

### Rust中的惯用monad

「长文预警」本文作者描述了一种在Rust中表达Monad的新方法，用于证明在Rust中实现Monad的可行性。作者说这是他见过的最简单的设计。

[Read More](https://varkor.github.io/blog/2019/03/28/idiomatic-monads-in-rust.html)


### 使用Rust的条件编译来实现Mock功能

写单元测试的时候经常需要mock一些场景，比如访问外部服务。但是传统的mock服务是在测试代码中创建一些模拟的对象。Rust中的一些mock框架也是类似的做法，参加「mock框架比较」。但是最大的问题是无法mock出代码中使用的外部结构。Rust中没有继承的概念，所以无法mock出标准库或者外部包中使用的结构类型。有一种解决办法是用trait或泛型，但是这种方法可能会使代码更复杂。而另一种方法则是利用条件编译，这也是本文作者想要介绍的。

```rust
#[cfg(test)]
use fake_clock::FakeClock as Instant;
#[cfg(not(test))]
use std::time::Instant;
```

在测试的时候使用fake_clock的mokc对象，而非测试的情况则使用Instant。

但这样也有一些缺点：

- 所有测试用例共享一个mock，但是如果每个测试用例需要不同的mock行为，则需要想想其他策略
- 更加需要集成测试。

在使用该文作者编写的http mock库mockito就可以使用条件编译：

```rust
#[cfg(test)]
use mockito;

#[cfg(not(test))]
let url = "https://api.twitter.com";

#[cfg(test)]
let url = &mockito::server_url();
```

- [Read More](https://klausi.github.io/rustnish/2019/03/31/mocking-in-rust-with-conditional-compilation.html)
- [mockito](https://github.com/lipanski/mockito)
- [Rust社区mock库比较](https://asomers.github.io/mock_shootout/)

### “高阶多态双向检查”的Rust实现

该库是对论文《Complete and Easy Bidirectional Typechecking for Higher-Rank Polymorphism》的实现

- [BidirectionalTypechecking](https://github.com/JDemler/BidirectionalTypechecking)
- [Complete and Easy Bidirectional Typechecking for Higher-Rank Polymorphism](https://www.cl.cam.ac.uk/~nk480/bidir.pdf)

### Rust和性能测试指南

这篇文章主要关注在Rust中如何创建令人满意的基准测试

[Read More](https://nbsoftsolutions.com/blog/guidelines-on-benchmarking-and-rust)

### 「系列文章」审阅Sled源码 Part I

Sled项目是一个用Rust编写的嵌入式数据库。该文作者在日程工作中用到了它，为了掌握它的工作原理，准备开始写这个系列的博客。也可以通过学习此文，掌握一些阅读开源项目源码的技巧或其他启示，比如：

- 先找你擅长领域的项目去阅读，事半功倍。
- 从整体上先把握代码组织结构、依赖库，从所获得的信息中去推测更多信息。
- 携带某个主要的问题去看源码是如何实现的。

该文作者还有一个完整的review LevelDB的系列文章：reviewing-leveldb

- [Read More](https://ayende.com/blog/186753-A/reviewing-sled-part-i)
- [reviewing-leveldb](https://ayende.com/blog/posts/series/161410/reviewing-leveldb)

### 案例：使用Rust和Lucet

Lucet是一个本地WASM编译器和运行时。基于Lucet运行时，Rust编译为wasm32-unknown-wasi target就可以创建一个运行于服务端的wasm程序。目前wasm32-unknown-wasi只支持Rust Nightly（2019-04-01版本确定可用）

- [Rust官方PR： Add a new wasm32-unknown-wasi target](https://github.com/rust-lang/rust/pull/59464)
- [Read More](https://hermanradtke.com/2019/04/01/wasi-example-using-rust-and-lucet.html)

### Arenas vs. Indices：為型別寫函數要寫在型別裡還是外面？ 

[Read more](https://www.reddit.com/r/rust/comments/b9y8ov/arenas_vs_indices/)

### ArcSwap的袖里乾坤

ArcSwap可以自动存储和加载Arc，类似于`RwLock<Arc <T >>`但没有锁。适合于频繁读取但不经常修改的数据，如配置或内存数据库每秒请求数百万次查询等。这篇文章中，作者揭示了ArcSwap的工作机制。

- [Read More](https://vorner.github.io/2019/04/06/tricks-in-arc-swap.html)
- [arc-swap](https://github.com/vorner/arc-swap)


### 「讨论」Rust项目配置的最佳实践是什么？

有人在reddit上面开贴询问这个问题，评论里也有很多人讨论。大家还有什么推荐？

（我个人用的是dotenv了）

[Reddit讨论](https://www.reddit.com/r/rust/comments/ba4dr3/configurations_best_practices/)

### 不要序列化默认值

默认情况下，serde在序列化结构时包括所有字段，即使它们的值是默认值。 这可能导致一些包含空值的「污染」。本篇文章教你如何跳过这些默认值。

[Read More](https://www.mth.st/blog/skip-default/)


---

# 项目、工具与库

### DataFusion 0.13 发布了

本次是作为 Apache Arrow 的一部分发布的。

目前有几大特性支持：

- Parquet 支持
- 自定义数据源支持
- 实验性数据帧风格 API
- Query 优化

[Read More](https://andygrove.io/2019/04/datafusion-0.13.0/)

###  yarte 0.2 现在支持actix-web 1.0.0-alpha

yarte号称最快的模板引擎，之前和Askama模板的作者有过Lisense相关的争议。

- [Read More](https://github.com/actix/examples/tree/master/template_yarte)
- [yarte](https://github.com/rust-iendo/yarte)

### bstr: 提供不需要验证UTF-8有效性的字符串

该库提供了BString和BStr两种字符串类型，与标准库的String和str类型的不同之处在于它们不需要是有效的UTF-8。

该库不是对标准库Sring的包装，值得看看源码。

[bstr](https://github.com/BurntSushi/bstr)

### amiquip: 纯Rust实现的RabbitMQ客户端

[amiquip](https://github.com/jgallagher/amiquip)

### HugoToJSON: 用于生成Hugo文档的关键内容的json

主要用来给Hugo静态站点提供搜索，目前该库请求review。

[HugoToJSON](https://github.com/arranf/HugoToJSON)

### git-rs  -  用 rust 重新实现的 git

作者只是为了好玩儿和教育目的？

[Read More](https://github.com/chrisdickinson/git-rs)

### evtx - Windows XML 事件日志解析器

可能是（世界上）最快的 Windows XML Event Log 解析器了。

[Read More](https://github.com/omerbenamram/evtx)

作者还做了一个 py 的封装。

[Read More](https://github.com/omerbenamram/pyevtx-rs)

### Pushrod - 基于 piston_window 的 GUI 库

不过看起来好像还很嫩。Rust 目前为止还没有一个靠谱的原生 GUI 库。

[Repo](https://github.com/KenSuenobu/rust-pushrod/)

### Lens: 纯Rust实现的Linux用户空间

可替代Linux用户空间，但目前只是简陋版。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/b8qy70/my_new_project_a_purerust_userspace_for_the_linux/)
- [lens-os.gitlab.io](https://lens-os.gitlab.io/)

### aerosol 0.2 发布

#DI

支持编译时依赖注入

[aerosol](https://github.com/Diggsey/aerosol)

### desse: 用于编译时已知大小类型的超快序列化库

[desse](https://github.com/devashishdxt/desse)

###  mips-simulator: Rust实现的MIPS汇编模拟器

MIPS汇编常用于教学目标，MIPS指令集属于精简指令集

[mips-simulator](https://github.com/salahsheikh/mips-simulator)

### undermoon: 支持Redis集群的Server端Redis代理

#redis

由国内Rust社区成员 @黄光星 开发

[undermoon](https://github.com/doyoubi/undermoon)

### hazptr: 基于Hazard指针的并发内存回收

问题：在多线程程序中，某线程通过一个指针访问一段内存时，如何保证指针所指向的那块内存是有效的？

普通青年： 加锁。
文艺青年： 无锁实现，使用HazardPointer。
二逼青年： 根本没有意识到这是个问题。

HazardPointer可以理解为是一种线程安全的智能指针。相比于crossbeam-epoch的基于代的回收方案效率更低。但如果追求回收的可靠性，HazardPointer更可靠点。

- [hazptr](https://github.com/oliver-giersch/hazptr)
- [Reddit 讨论](https://www.reddit.com/r/rust/comments/b8go1z/hazptr_hazard_pointer_based_memory_reclamation/)

### tract: 适用于TF和ONNX的小型推理引擎

对语音处理等实时应用也提供半实验性支持。

[tract](https://github.com/snipsco/tract)

### trassh: 一个简单的SSH蜜罐

类似于Endlessh，它会打开一个套接字并伪装成一个 SSH 服务器，非常缓慢地发送一个无休止的随机 SSH banner，使 SSH 客户端一次锁定数小时甚至数天。目的是将真正的 SSH 服务器放在另一个端口上，而在虚假的服务器上卡住入侵者，防止其影响到真正的服务器。

[tarssh](https://github.com/Freaky/tarssh)

### Inko: Rust实现的面向对象语言

号称安全无痛地处理并发，主要是支持类Erlang轻量级进程来处理并发。

[inko-lang.org](https://inko-lang.org/)

### Smithy： 一个Rust实现的WebAssembly框架

状态：0.0.2 Alpha版本。

- [Read More](https://medium.com/@robert.balicki_2494/introducing-smithy-webassembly-framework-for-rust-679d8fe9c16)
- [smithy](https://github.com/rbalicki2/smithy)
- [在线Demo](https://smithy-todolist.robertbalicki.com/)
- [demo源码：smithy_todolist](https://github.com/rbalicki2/smithy_todolist)

### UFO: 用于无人机/无人机/四轴飞行器/RC航模的Rust库

状态：WIP

[ufo](https://github.com/ajmwagar/ufo)

### minitt-rs:  Mini-TT的Rust实现

Mini-TT是一个简单的依赖类型语言。minitt-rs是它的Rust实现，并且还提供了一个REPL。 作者是@ice1000

- [minitt-rs](https://github.com/owo-lang/minitt-rs)
- [Mini-TT论文](http://59.80.44.49/www.cse.chalmers.se/~bengt/papers/GKminiTT.pdf)

### Rust source code行數變化

![img](https://wx3.sinaimg.cn/mw690/71684decly1g1svyy01bzj21800r0gtm.jpg)

使用工具 https://github.com/src-d/hercules

[Read more](https://www.reddit.com/r/rust/comments/b9shaz/burndown_chart_of_rust_source_code_lines_by_year/)

### Zola 0.6.0

一個快速的靜態網站產生器，讓人快速的產生blog上傳到github上

[Read more](https://www.reddit.com/r/rust/comments/b9rc50/zola_060_released/)
