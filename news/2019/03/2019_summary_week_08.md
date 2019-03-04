### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-03-03

---

# 通告两则

### 「正式介绍」 首届 RustCon Asia大会将在北京举行

#RustConAsia

欢迎大家来面基。

- [Read More](https://mp.weixin.qq.com/s/vBKiFdNoCat3I9NYdV5yIA)
- [购票通道](http://www.huodongxing.com/event/6479456003900)

### 「通告」Rusty 棒球帽团购活动已截止

购买的朋友请关注你的邮件，将在一两周内发货。

---

# 官方新闻

### Rust 1.33 稳定版发布

1.33的更新简要：

- Pin API稳定，向async/await迈进了一步
- 整数类型在match中可以使用范围表达式穷尽了，比如u8类型，可以用0..=255来匹配，不再需要`_ => unreachable!()`了。
- `if let`和`while let`支持多模式匹配。
- const functions中现在可以使用let绑定、赋值、表达式语句以及irrefutable的模式匹配
- 支持unsafe const fn。
- 可使用cfg_attr指派多个属性
- 可以使用`#[repr(packed)] `指定特殊的对齐方式
- extern函数在恐慌时的行为默认是Abort，以前这里是未定义行为。
- LLVM支持的最低版本是6.0
- 编译器新增了很多平台的编译target支持。

[Release Note](https://github.com/rust-lang/rust/blob/stable/RELEASES.md)

### 「官方」Async/Await状态报告

[Read More](https://zhuanlan.zhihu.com/p/58101059)

### 「提案」Rust标准库即将更换新的Channel

最近有标准库即将会有两个更新，已经提交了PR：

- 使用Parking_lot替代互斥锁/读写锁等（Mutex/RwLock/condvar/），性能将会提升: [pull/56410](https://github.com/rust-lang/rust/pull/56410)
- HashMap内部会换成基于hashbrown实现的SwissTable，API未变: [pull/56241](https://github.com/rust-lang/rust/pull/56241)

这篇文章里，提议Rust标准库将Channel更换为crossbeam的Channel，并列举了mpsc的一些缺陷。该文作者是Crossbeam-rs和tokio的贡献者stjepang。

[Read More](https://stjepang.github.io/2019/03/02/new-channels.html)

---

# 社区新闻

### Rust和npm白皮书

npm Registry实用Rust来解决CPU密集型任务产生的瓶颈问题。在该PDF中阐述了具体的问题以及解决方案。

在寻找解决方案的过程中，NPM团队最先排除了C/CPP/Java。因为在他们的工程思想中，C/Cpp不再是一个合理的选择。主要是担心安全问题。由于需要部署jvm，也排除了Java。然后选择了Go或Rust。

然后团队使用Go花了两天时间重写了授权服务，在这个过程中，该团队对Go的依赖管理失望了。因为NPM本身就是一个包管理工具，他们的工程文化不接受这样的Go。而他们对Rust的评价是这样的：惊人的依赖管理，以及促使你必须思考程序的正确性。用Rust重写服务，确实比用js和Go耗费时间长。当然这段时间也包含了他们的学习成本。并且Rust社区非常友好，有问题可以及时回复。

Rust编写的第一个简单的服务程序用在生产中一年半，没有发出任何的警报。这导致他们团队很快就忘记了该程序的存在。。

- [Read More](https://www.rust-lang.org/static/pdfs/Rust-npm-Whitepaper.pdf)
- [Reddit 讨论贴](https://www.reddit.com/r/rust/comments/av1bpg/the_npm_whitepaper_is_up/)

### 用Rust重写Node.js实现的接口之后

作者用Rust重写了Node.js实现的一个小接口，但是流量很大，每秒180个请求。

之前Node.js版本技术栈：

- koa，框架
- pino，日志
- request，转发请求

Rust版本技术栈：

- actix-web, 框架
- serde-json, 序列化处理json
- slog-json，日志
- reqwest，转发请求

作者使用的是`reqwest::async`异步请求。Rust版本部署之后，CPU使用率较Node.js版本下降了90%，内存使用量降下了一半。

这些数据更有助于作者说服他的同事们使用Rust，分享出来也可以帮助大家来说服公司使用Rust。

![img](https://wx1.sinaimg.cn/mw690/71684decly1g0ob3u5cd9j21wc0q2td4.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1g0ob3w7z5mj23ec0ncjxo.jpg)

[Reddit 讨论](https://www.reddit.com/r/rust/comments/aw94xp/i_ported_a_small_api_from_nodejs_to_rust_and_was/)

### 「系列博客」 Rust与科学计算 Part 0

有博主宣布要写Rust和科学计算的系列文章，本文是该系列的首篇文章。该作者的日常工作是机器学习，他在多次使用Rust进行相关的实验之后发现，Rust语言在这个领域将大有可为，并让他感觉非常激动。

作者反观了Python统治下的机器学习世界，其生态系统爆炸的原因是因为有很多基础库，比如NumPy，SciPy，Pandas等。大多数项目是构建在NumPy和SciPy之上。

如果Rust也拥有这些核心的基础库会怎么样？抱着这样的想法，作者加入了维护ndarray库的队伍中。他贡献了一堆PR之后，诞生了一个独立的crate：ndarray-stats。这个系列的文章，将围绕ndarray来阐述。

在这篇文章中，他着重讲解了Rust中的Vector数组，以及使用Vector来计算线性代数中的点积（scalar product）。并且他和Python做了相应的性能测试比较：

```
Language	Time (us)	Notes
Python	1720	Same function, using Python’s lists
Python	12.2	NumPy, using np.array - (v * w).sum()
Python	6.1	NumPy, using np.array - v.dot(w)
Rust	1.8	Super naive
```

Rust的Vector的性能是Python List的近1000倍。是Numpy库的近6、7倍快（有点欺负Python的感觉）。本文的目的只是通过简单的例子介绍一下Rust的所有权，下一篇将进入正式的科学计算内容。

[Read More](https://www.lpalmieri.com/posts/2019-02-23-scientific-computing-a-rust-adventure-part-0-vectors/)

### tokio入选为Google编程之夏的开源项目

[Read More](https://summerofcode.withgoogle.com/organizations/5982287680765952/)

### chucklefish不再使用Rust开发该公司新游戏Witchbrook

该公司的这个决定和Rust语言本身并无关系。主要是因为该项目之前的主程kyrenn离职了。并不意味着Rust不能做游戏开发，Rust完全胜任该公司游戏的开发，估计他离职了公司没有找到合适的Rust主程，游戏也比较着急吧。

但是不可否认的是Rust在游戏的生态现在还未成熟，kyrenn说，他自己仍然会用Rust开发游戏，但是他可能会是世界上最慢的游戏开发者，因为你需要做更多的底层的「脏活累活」。

[Read More](https://www.reddit.com/r/rust/comments/avwxq1/chucklefish_is_no_longer_using_rust_for_witchbrook/)

### Firefox Reality浏览器即将登陆Microsoft's HoloLens 2

HoloLens 2是微软混合现实眼镜的第二代。Rust为HoloLens 2提供了更安全的体验。

[Read More](https://www.windowscentral.com/firefox-reality-browser-hololens-2)

###  Mozilla正在寻找懂UWP的专业人士

希望协助Moziila将Rust带到Hololens平台。目前Firefox Reality浏览器即将登陆Hololens2。

![img](https://wx3.sinaimg.cn/mw690/71684decly1g0jmu3jqmvj20xw0n0ag8.jpg)


### 「Mozilla」用Rust重写浏览器组件的意义

该文以Quantum CSS为案例，探讨「用Rust重写」对真实世界的影响。

概要： Rust虽然无法捕捉全部的安全漏洞，但是可以消除重大的安全漏洞，并且可以让开发人员专注于程序逻辑的正确性和健壮性。

[Read More](https://hacks.mozilla.org/2019/02/rewriting-a-browser-component-in-rust/)

### wasmer : 性能提升100倍

wasmer是一个服务端wasm解释器。最近发布了0.2.0版本，号称性能提升了100倍。

[Read More](https://medium.com/wasmer/running-webassembly-100x-faster-%EF%B8%8F-a8237e9a372d)

### 「视频」是时候用Rust重写操作系统了吗？

来自去年QCon San Francisco 2018的视频。作者非常看好Rust。

- [Read More](https://www.youtube.com/watch?v=HgtRAbE1nBM)
- [文本](https://www.infoq.com/presentations/os-rust?utm_source=youtube&utm_medium=link&utm_campaign=qcontalks)

### 编写桌面版和Web版Roguelike游戏

该作者准备参加Roguelike游戏挑战赛，用Rust实现了一个可跨平台运行的游戏demo。本文是该demo的一个教程。

- [Read More](https://aimlesslygoingforward.com/blog/2019/02/09/writing-a-rust-roguelike-for-the-desktop-and-the-web/)
- [代码](
https://github.com/tomassedovic/quicksilver-roguelike)

### Pop周报

System76公司(Redox背后的公司)的PopOS周报。这篇报告里说，System76公司的桌面项目都是用Rust实现的。那是不是意味着他们的PopOS中的桌面项目都是Rust实现的呢？popOS看上去还不错。

- [popOS](https://system76.com/pop)
- [Read More](https://pop-planet.info/forums/threads/this-week-in-pop-1.89/#post-420)

### 「视频」Rust Auckland 2019-02-25：Amethyst游戏引擎介绍

- [Read More](https://www.youtube.com/watch?v=qe40FqD1E1A&feature=youtu.be)
- [slides](https://gitpitch.com/azriel91/amethyst_engine_engine/master?grs=github&t=sky#/)

### skribo：底层文本布局库启动

Xi Editor的作者raphlinus之前写的博文说，文本布局是Rust GUI生态缺失的部分。skribo就是他为了弥补这个缺失的生态而开的新坑，用于全面改进文本处理。

- [Read More](https://raphlinus.github.io/rust/skribo/text/2019/02/27/text-layout-kickoff.html)
- [skribo](https://github.com/linebender/skribo)


---

# 学习资源

### 来自Rust读者群的分享：Rust生命周期

作者：月泉

> 针对Rust的生命周期及所有权机制的一些知识写了一篇文章，希望能够帮助群里的各位书友理解这些知识。

期待更多的学习分享。

[Read More](http://yuequan.org/rust_ownership_lifetime.html)

### 「嵌入式Rust」Cortex-M3 入门指南（三）：时钟总线与复位时钟控制器 

[Read More](https://zhuanlan.zhihu.com/p/57918979)

### Rust优化小技巧

[Read More](https://vfoley.xyz/rust-compilation-tip/)

### 「嵌入式Rust讨论」用什么硬件可以更容易地开始用Rust进行嵌入式开发

有人在Reddit发帖询问此问题，他用的是Arduino，但是使用Cpp和Rust都非常痛苦。即便使用针对Arduino的Rust版本avr也不太稳定，所以他想问问什么硬件更适合入门者？

回复中有人提到，使用基于ARM Cortex-M的硬件，比如STM-32 uC更好，Rust对该硬件体系支持的更好。可以配合discovery book一书学习。 评论中也有其他建议可以看看。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/aw8bwt/question_hardware_for_easy_start_in_embedded_rust/)
- [discovery book](https://rust-embedded.github.io/discovery/)

### 「视频」从零开始使用Rust构建嵌入式传感器节点平台

- [Read More](https://www.youtube.com/watch?v=S0VI70nY6Vo)
- [internet-of-streams](https://github.com/ferrous-systems/internet-of-streams)

### Rust 胖指针探秘

[Read More](https://iandouglasscott.com/2018/05/28/exploring-rust-fat-pointers/)

### 如何使用命令行参数

这篇文章帮你揭开Rust命令行参数的神秘面纱。

[Read More](https://blog.knoldus.com/working-with-command-line-arguments-in-rust/)

### 用Rust实现一个mod播放器 Part 1

mod是一种音乐格式。

- [Read More](https://www.codeslow.com/2019/01/mod-player-in-rust-part-1.html)
- [代码](https://github.com/janiorca/articles/tree/master/mod_player)

### Clojure，Rust，Pony，Erlang和Dart分别如何实现无畏并发

该文分别展示了五种语言的安全并发模型：

- Clojure： Alternative 并发模型
- Rust： 所有权机制（ownership）
- Pony：引用能力（Reference Capabilities），Pony 语言中每种变量的类型都包含了有关如何在 actor 之间分享数据的信息。有点像Rust的借用检查器。
- Erlang：Actor模型
- Dart：Actor模型，在Dart里叫Isolates（逻辑上隔离内存）。

[Read More](https://sites.google.com/a/athaydes.com/renato-athaydes/posts/fearlessconcurrencyhowclojurerustponyerlanganddartletyouachievethat)

### 「系列文章」从零开始构建数据库

这系列文章不错，教你用C从头开始构建一个sqlite数据库，感兴趣的可以换成Rust来实现。当然，去年也有人写Rust实现关系数据库的文章，可惜烂尾了。

[Read More](https://cstack.github.io/db_tutorial/)

### Rust build脚本与Meson

该文作者在尝试将Rust和Meson集成。这篇文章记录了他在此过程中使用build.rs的一些经验。

- [Read More](https://people.gnome.org/~federico/blog/rust-build-scripts.html)
- [meson](https://github.com/mesonbuild/meson)

### 「系列视频」如何用Rust构建属于自己的加密货币 Part 1

- [Read More](https://www.youtube.com/watch?v=vJdT05zl6jk&list=PLwnSaD6BDfXL0RiKT_5nOIdxTxZWpPtAv&index=2&t=0s)
- [代码](https://github.com/GeekLaunch/blockchain-rust)

### 「教程」min-sized-rust: 该库演示了如何最小化Rust二进制大小

[min-sized-rust](https://github.com/johnthagen/min-sized-rust)

### 「系列文章」WebAssembly的麻烦 Part 4 ： Microwasm

该系列文章主要探讨WebAssembly中的缺陷。

Microwasm是与Wasm兼容的格式，可以被运行时有效地使用，并由LLVM等编译器生成。它目前在Lightbeam(将wasm生成机器码的实验工具)的Microwasm分支中实现。由以下三步生成：

- Compiler IR->Microwasm;
- Wasm->Microwasm;
- Microwasm->Native.

该作者团队还在维护一个wasmtime库，它们引入了Microwasm是为了进一步提升性能，并且为Microwasm编写后端比wasm更加容易。因为Microwasm生成的汇编代码比Wasm直接生成的汇编代码更加简单。

- [Read More](http://troubles.md/posts/microwasm/)
- [lightbeam](https://github.com/CraneStation/lightbeam)
- [wasmtime](https://github.com/CraneStation/wasmtime)

### 为Rust库公开FFI接口

该文作者之前实现了battery库，该库用到了battery-ffi库，专门封装了一些FFI接口，给主库使用。他在这篇文章中写下了编写FFI的一些注意事项。可以看作是一个FFI最佳实践来学习。

[Read More](https://svartalf.info/posts/2019-03-01-exposing-ffi-from-the-rust-library/)

### 「教程」min-sized-rust: 该库演示了如何最小化Rust二进制大小

[min-sized-rust](https://github.com/johnthagen/min-sized-rust)

---

# 项目、工具与库

### tweek-rust: Rust实现的Tween动画工具包

Tween动画可以对对象进行缩小，放大，旋转，渐变，位移等操作，用于在游戏或应用中实现一些渐变类动画动作。最早是伴随Flash技术出现的。现在该技术已经用在了web前端和手机App中。

该库中包含了两个示例，可以看看。

- [tweek-rust](https://github.com/wasm-network/tweek-rust)
- [点此查看视频](https://www.bilibili.com/video/av45138427)

### McEx: Rust和Elixir共同实现的MineCraft服务器

[McEx](https://github.com/McEx/McEx)

### WAS: 用于捕获wasm编译器和应用程序内存问题的内存分配器

WAS（不是WASM）是一个简单的WASM内存分配器，旨在捕获WebAssembly编译器和应用程序中的内存问题。

[was-not-wasm](https://github.com/jedisct1/was-not-wasm)

### 使用graphviz可视化Rust代码流程图

Rust编译器可以生成flowgraph IR，然后通过graphviz这个软件就可以生成流程（FlowGraph）图。cargo inspect这个工具把graphviz调用命令集成进去了，新版本可以方便地生成流程图了。

![img](https://wx4.sinaimg.cn/mw690/71684decgy1g0izia9nlfj20u01hfqap.jpg)

[Read More](https://jonathansteyfkens.com/posts/visualizing-rust.html)

### git-req: 轻松查看GitLab和GitHub中的合并请求

Rust实现的命令工具，只需要把git-req设置好环境变量，即可成为git的子命令。

- [git-req](https://github.com/arusahni/git-req)
- [Read More](https://arusahni.github.io/git-req/)

### 「嵌入式Rust」开发环境指引

[Read More](https://josh.robsonchase.com/embedded-bootstrapping/)

### psd: PSD文件解析器

Rust实现的PSD文件解析工具，还有个在线使用wasm的demo。

- [psd](https://github.com/chinedufn/psd)
- [live demo](https://chinedufn.github.io/psd/drag-drop-demo/)

### imagene: 通用图像处理工具

基于image库实现

[imagene](https://github.com/AlmightyFloppyFish/imagene)

### battery: 用于收集笔记本电脑电池信息的库

支持Linux、Mac、Windows。通过该库提供的命令，可以在终端查看电池相关的信息。

[Read More](https://svartalf.info/posts/2019-02-25-introducing-battery-crate/)

### swym： 实验性软件事务内存库

可用于实现并发数据结构，其性能和无锁数据结构相差无几。

[Read More](https://github.com/mtak-/swym)

### cargo-deps: 可视化项目中的依赖库

它是对cargo-graph库的分支，同样是基于graphviz库，可以可视化依赖项的关联。作者号称该库是对cargo-graph的改进。

- [cargo-deps](https://github.com/m-cat/cargo-deps)
- [cargo-graph](https://github.com/kbknapp/cargo-graph)

### Forge:  Rust实现的一种轻量级的动态语言

支持Rust FFI

- [forge](https://github.com/zesterer/forge)
- [playground](https://forge.jsbarretto.com/)

### higher: Rust实现的类Haskell高阶类型

包括applicative、functor、monad以及type class等。

[higher](https://github.com/bodil/higher)

### Atto： Rust实现的一个简单的函数式语言

[atto](https://github.com/zesterer/atto)

###  laminar: 用于多人游戏的半可靠UDP协议实现

该库在UDP的基础上实现了TCP的一些功能。它被用于Amethyst游戏引擎中。

[laminar](https://github.com/amethyst/laminar)

### MIRAI: 来自Facebook实验室的Rust MIR抽象解释器

MIRAI的目标是成为Rust的静态分析工具。

[MIRAI](https://github.com/facebookexperimental/MIRAI)

### rendy-pbr: 玩具版实时物理渲染器

基于rendy和gfx-hal实现

- [rendy](https://github.com/omni-viral/rendy)
- [rendy-pbr](https://github.com/termhn/rendy-pbr)