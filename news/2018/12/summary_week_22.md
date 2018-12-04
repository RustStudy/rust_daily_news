前言：
从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。
2018-12-02

# 官方新闻

### 「通告」Rust 2018 新的Beta版发布！

Rust 2018已经进入了最终的倒计时，官方刚刚发布了一个最新测试版。新的测试版包含了一些最近新稳定的功能和一些Bug修复。还包含一些工具的改进：

- Rustfmt 1.0
- RLS和Clpippy可以直接安装，不再是“preview”组件了。

来帮助检测是否存在Bug。

[Read More](https://internals.rust-lang.org/t/announcing-rust-2018-beta-release/8901)

### 「官方」Rust 2018年度调查报告

- [中文](https://zhuanlan.zhihu.com/p/51018048)
- [英文](https://blog.rust-lang.org/2018/11/27/Rust-survey-2018.html)

### 「官方」演变中的Tide中间件

[Read More](https://zhuanlan.zhihu.com/p/51048926)

### 「官方网络工作组」2018调查报告

此次是网络工作组的报告，一共收到1000多条回复。

![img](https://wx1.sinaimg.cn/mw690/71684decly1fxoutofl5jj20yg0lbq3k.jpg)

在Web方面，流行的框架是Rocket和Actix，分别占比27%和24%。也有其他框架的选择，但是有20%的人不选择任何框架，而是在hyper上自行搭建服务。

存在的问题：

- 65%的人认为缺乏示例是目前生态系统中的问题，其次是缺乏文档。这也是Tokio Doc Push和Rust异步之书正在解决的问题。
- 缺乏一个真正的框架，类似于Rails或Django那样的。这也是网络工作组正在考虑的事情。为此官方构建了Tide，为打算使用Rust构建Web应用提供一个良好的开端，另一个目标是想要深入挖掘并学习如何在Rust中编写Web框架的人提供一个文档。
- 在构建应用程序时缺乏对框架和服务的一些绑定。包括绑定应用程序，比如各种NoSql数据库、支持在K8S上运行、编排容器框架和LDAP认证协议。以及对数据库的异步访问支持等，并没有一个惯用法的指南。

[Read More](https://rust-lang-nursery.github.io/wg-net/2018/11/28/wg-net-survey.html)

### Rust官网界面改版引发争议

[官网改版测试界面](https://beta.rust-lang.org/)

![img](https://wx4.sinaimg.cn/mw690/71684decly1fxq3b5zqfbj21l00u0h5k.jpg)

### 官方Rust Book现在有Epub版本下载

[Read More](https://www.jyotirmoy.net/posts/2018-12-01-rust-book.html)

---

# 社区新闻

### AWS Lambda已经支持Rust

使用该库可以在AWS Lambda上运行Rust实现的函数

[aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)

[Read More](https://aws.amazon.com/cn/blogs/opensource/rust-runtime-for-aws-lambda/)

### 「游戏」Amethyst基金会成立了

好消息。Amethyst是一款免费的开源游戏引擎。它是用Rust编写的，其核心是ECS架构。现在成立了基金会是为了组建一个官方的非盈利组织。这对于Amethyst的发展应该是非常有好处的了。当然，现在的状态只是提交了文件，还在等待最后的审批。

[Read More](https://www.amethyst.rs/blog/non-profit-announce/)

### 「演讲」低延迟音频合成

包含了演讲视频、代码和Slides。

- [视频](https://youtu.be/-F7whGjquHI?t=3150)
- [Code](https://github.com/raphlinus/synthesizer-io)
- [Slides](https://docs.google.com/presentation/d/1cm9QaV_UjgvgBaWszHsR_w5EVre5O1D5kJUpMKQYo4I/edit?usp=sharing)

也可以多关注下后续，会有详细的博文放出。

[Read More](https://synthesize.rs/nov-2018-talk/)

### Rust Belt Rust 2018 大会 视频合集

[视频合集](https://www.youtube.com/playlist?list=PLgC1L0fKd7UlpVTHVfLYVtudVx8CzbSxW)

### AWS 出品 :  Firecracker 

Firecracker是一种开源虚拟化技术，专门用于创建和管理安全，多租户容器和基于功能的服务，提供无服务器操作模型。 Firecracker在轻量级虚拟机中运行工作负载，称为microVM，它将硬件虚拟化技术提供的安全性和隔离性与容器的速度和灵活性相结合。

看上去应该是兼容docker生态链，倾向于Serverless。

- [firecracker](https://github.com/firecracker-microvm/firecracker)
- [Read More](https://firecracker-microvm.github.io/)
- [More Read More](https://aws.amazon.com/cn/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing/)

### 「博文」Bitfury公司的工程师为俄罗斯邮政服务设计区块链

> 本月，Bitfury®工程师Yury Yanovich，Ivan Prokhorov，Darya Korepanova和Sergey Vorobyov与Skolkovo科学技术研究所（Skoltech）的计算和数据密集型科学与工程中心一起在科学研究期刊Informatics上发表; 信息传输问题研究所数据挖掘和预测建模实验室; 罗蒙诺索夫莫斯科国立大学世界经济地理系; 和高等经济学院的工作是设计一个基于区块链的供应链，以防止印章伪造。 

- [论文](http://www.mdpi.com/2227-9709/5/4/42/pdf)
- [代码原型](https://github.com/korepkorep/russian-post)

### RustQuiz: 26个在线Rust面试题

来自于RustFest 2018 大会的轻演讲主题：Rust Quiz

- [rust-quiz源码](https://github.com/dtolnay/rust-quiz)
- [在线练习和解答](https://dtolnay.github.io/rust-quiz/18)
- [视频](https://www.youtube.com/watch?v=QtDj9R6vtA8&index=6&list=PLgC1L0fKd7UlpVTHVfLYVtudVx8CzbSxW&t=0s)

### RustaCUDA v0.1.0发布

RustaCUDA是对CUDA API的Rust包装。允许分配和释放GPU内存，从GPU复制数据，加载CUDA模块和启动内核，所有这些都具有最安全，程序员友好的Rusty接口。 它可以加载和启动用任何CUDA兼容语言编写的内核，而不仅仅是Rust。

该作者对Rust支持CUDA作出了很多努力。

- [Read More](https://bheisler.github.io/post/announcing-rustacuda/)
- [RustaCUDA](https://github.com/bheisler/RustaCUDA)

### Rocket v0.4 RC2版本发布

[Read More](https://rocket.rs/v0.4/news/2018-11-30-version-0.4-rc-2/)

### TOML规范 0.5中文翻译发布了

从0.5开始，TOML规范趋于稳定。

[Read More](https://github.com/toml-lang/toml/blob/master/versions/cn/toml-v0.5.0.md)

### Atom 1.33发布，内置Rust支持

[Read More](http://blog.atom.io/2018/11/28/atom-1-33.html)

### 改进的Rust贴纸，超酷

[下载](https://i.redd.it/206f9o60u0121.png)

### 「投票结果」你主要用什么操作系统构建Rust代码

![img](https://wx2.sinaimg.cn/mw690/71684decly1fxq3ok9d25j20u00wp42n.jpg)

---

# 学习资源

### 「付费」《如何系统地学习Rust》

为你精心打造Rust必学第一课。 

[知乎Live (已完结，可回看，9.9¥)](https://www.zhihu.com/lives/1043463438202249216)

### Rust语言CheatSheet

[Cheats.rs](https://cheats.rs/)

### Rust Quiz 解读： Quiz 1

[Read More](https://zhuanlan.zhihu.com/p/51304039)

### AoC 2018谜题Rust解决方案

- [AOC官网](https://adventofcode.com/)
- [BurntSushi的解决方案](https://github.com/BurntSushi/advent-of-code)
- [历年谜题解决方案](https://github.com/BenoitZugmeyer/RustyAdventOfCode#other-rust-implementations)

### 新书：用Rust编写WebAssembly

beta版可预定。

[programming-webassembly-with-rust](https://pragprog.com/book/khrust/programming-webassembly-with-rust)

### 「系列文章」数独谜题生成器 pt.3: 使用WebAssembly

作者想用一个数独谜题生成器来检测WebAssembly的性能，他写了一系列文章，本文是第三篇，用Rust的wasm-bindgen工具来创建数独生成器的wasm模块。

[Read More](https://medium.com/@rossharrison/generating-sudoku-boards-pt-3-rust-for-webassembly-85bd7294c34a)

### 「博文」如何将Tokio的AsyncRead和AsyncWrite转换为Futures, Sinks和Streams

- AsyncRead to Future, 一次性读
- AsyncRead to Stream, 持续读
- AsyncWrite to Sink, 持续写
- AsyncWrite to Future, 一次性写

[Read More](https://jsdw.me/posts/rust-futures-tokio/)

[Sample Code](https://github.com/jsdw/jsdw.me/blob/master/content/posts/rust-futures-tokio/src/main.rs)

### 「博文」在Nightly版下使用async/await 

本文介绍了async/await，并且介绍了如何在最新的生态系统中使用老的futures 0.1。

[Read More](https://jsdw.me/posts/rust-asyncawait-preview/)

### 「视频」使用Rust和Amethyst引擎进行游戏开发

视频演讲中以一个开源的消除游戏everpuzzle为示例进行介绍。 [大约1小时]

[everpuzzle Code](https://github.com/Skytrias/everpuzzle)

[Read More](https://www.youtube.com/watch?v=P_9A7P0uNpY)

### 「嵌入式Rust」嵌入式开发环境

作者尝试玩嵌入式，选择了以自定义一个ErgoDox键盘作为尝试。本教程介绍了相关的开发环境准备。

[Read More](https://josh.robsonchase.com/embedded-bootstrapping/)

### 在Android开发中使用Rust

国内社区小伙伴写的文章，介绍如何在Android中使用Rust。

基于 [jni-rs](http://github.com/jni-rs/jni-rs)

[Read More](https://zhuanlan.zhihu.com/p/50123055)

### 使用Rust和Efflux编写 MapReduce Jobs

[Read More](https://whitfin.io/writing-mapreduce-jobs-using-rust/)

[efflux](https://github.com/whitfin/efflux)

---

# 项目

### romio：异步网络原语

无船同志写的 ：Mio + Future + Tokio = Romio。

（ 八卦： tokio作者不原意跟进Futures最新版，所以官方派出无船同志完成这项推动Rust异步历史进程的艰巨任务。） 

[romio](https://github.com/withoutboats/romio)

### Sequoia: OpenPGP的一个Rust实现

[RustFest Roma上的演讲](https://media.ccc.de/v/rustfest-rome-6-sequoia)

> 关于Sequoia：Sequoia由三位前GnuPG开发商Neal H. Walfield，Justus Winter和Kai Michaelis开发。为了缓解许多常见的安全问题，Sequoia是用强类型语言Rust编写的，它提供了时间和空间内存安全性。 Rust还为将库嵌入其他语言提供了出色的支持。 Sequoia已经提供了C绑定，并且正在积极开发Python绑定。

[sequoia](https://gitlab.com/sequoia-pgp/sequoia)

### 秘猿开源CKB和CKB-VM

Nervos CKB是一个公共授权区块链，是Nervos网络的共识层。CKB的VM是基于RISC-V指令集实现的。

- [ckb](https://github.com/nervosnetwork/ckb)
- [ckb-vm](https://github.com/nervosnetwork/ckb-vm)

### 高性能JavaScript到JavaScript编译器

基于Rust和Wasm实现

[ratel-core](https://github.com/ratel-rust/ratel-core)

### 使用Cranelift实现的玩具语言JIT

[Read More](https://github.com/CraneStation/simplejit-demo)

### Atlasr： 免费的地图预览器

[atlasr](https://github.com/atlasr-org/atlasr)

### Tange：基于任务的并行框架

“任务并行”是指一个或多个独立的任务同时运行。 可以用来做并行计算。用于：

- 数据处理
- 分布式机器学习算法
- 一般的并行计算

[tange](https://github.com/Refefer/tange)

---

# 工具与库

### tr: 国际化（i18n）库

刚发布，现在找人使用，并且想得到更多反馈。

[Read More](https://www.reddit.com/r/rust/comments/a09b0n/tr_a_crate_for_internationalizationof_rust_code/)

### 「工具」宣布RustPräzi：为crates.io构建整个调用图(call graph)

> 我们很高兴地宣布我们的第一个版本的RustPräzi，一个PoC（概念验证）项目，它从crates.io 3下载所有crate版本，构建LLVM调用图并将它们链接到一个大型版本的基于呼叫的依赖网络。与常规依赖关系网络不同，基于调用的依赖关系网络表示包内和包之间的函数调用链，支持图形分析/查询

一个CG是表示整个程序中方法（函数）之间调用关系的图，图中的节点是方法，边表示调用关系。例如方法foo()调用了方法bar()，则CG中应有一条从foo()到bar()的有向边。

[Read More](https://users.rust-lang.org/t/announcing-rustprazi-a-tool-to-build-an-entire-call-graph-of-crates-io/22696)

### 高效diff算法库

pijul开源了它们用的diff算法。pijul是基于Rust实现的类Git版本控制工具，[用法介绍](https://jneem.github.io/pijul/)

[diffs](https://docs.rs/diffs/0.1.0/diffs/)

---

# 招聘

### 「招聘」位于美国旧金山市中心的公司招Rust工程师

这是去矿上工作啊。想用Rust构建大型的数字货币和大型采矿系统。没看到说可以远程的信息，应该是不能远程了。

- 薪水 120k ~ 160k美刀 + 股权
- 旧金山软件工程师的平均工资是134,000美元
- 旧金山的失业率为3％

[Read More](https://functional.works-hub.com/jobs/rust-engineer-in-san-francisco-united-states-of-america-3a18b)