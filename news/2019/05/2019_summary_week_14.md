### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust日报已经成为了Rust社区群大家每天必看的内容。

从2019年开始，日报小组成立，目前的动态由：@Chaos、 @Mike、 @Damody(台湾)轮番为大家播报。也欢迎感兴趣的朋友加入小组。

每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

独立日报订阅地址：
- [Telgram Channel](https://t.me/rust_daily_news )
- [阿里云语雀订阅](https://www.yuque.com/chaosbot/rustnews)
- [Steemit](https://steemit.com/@blackanger)
- [GitHub](https://github.com/RustStudy/rust_daily_news)

社区学习交流平台订阅：
- [Rust.cc论坛](https://rust.cc)
- [Rust Force](https://rustforce.net/)
- [微信公众号：Rust语言学习交流](https://rust.cc/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

2019-05-05

---

# 官方新闻

### 「官方」Unsafe Rust安全检查：栈借用模型 2

ralfj比较高产，他负责Unsafe下内存模型相关的工作，目的是用miri来检测unsafe中的UB行为。

他在去年引入了栈借用模型1用于定义在unsafe内存模型中允许哪种别名。建立合理的别名规则，才能基于miri来检查unsafe下的UB行为。

该模型的核心思想是： 对于一个内存位置，逐步建立可跟踪的引用，形成一个栈结构。比如有一个&mut i32，可以对其重新借用获得一个新引用。这个新引用是必须用于此位置的引用，建立在旧引用之上。当新引用过期的时候，旧引用会被激活，就好像是栈结构push和pop。

在Safe Rust中，通常有借用检查来保护内存。但是在编写Unsafe代码的时候，借用检查就无法提供帮助了。所以，Rust核心团队就必须要定义一组规则，即使对于Unsafe代码来说也是非常有意义的。

在今天这篇文章中，ralfj又带来了栈借用模型的升级，栈借用2。

在栈借用1模型中，有一个概念叫做「frozen」，处于frozen位置的指针，只能读取，不能写入。它允许可变借用也能读取（检查粒度比较粗，把可变指针和共享指针同一化处理）。但是现在该模型被发现一个问题：当使用可变借用的时候，在该模型下可能会把某些未定义行为判断为合法。为了改进这个问题，栈借用模型2将精确跟踪允许访问的原生指针（更细粒度的检查，区分了共享指针和可变指针），而不是「frozen」。

栈借用模型2还有很多已知的问题，比如其实并没有真正使用到「栈」，反而更像「树」。但这还不是最后的结论。本文比较长，去原文阅读更多信息。

[Read More](https://www.ralfj.de/blog/2019/04/30/stacked-borrows-2.html)

### Cargo 2019 Roadmap

nrc在他的博客上发布了Cargo 2019的目标：

- 交叉编译。希望支持更多的目标。
- 插件。也称自定义命令、任务或工作流程。希望使插件更加强大、可靠和实用。
- 编译时间。这个目标是为Rust的目标服务。虽然这主要是rustc的问题，但是cargo也可以给予一些辅助。目前一个想法是：允许用户构建主crate而无需等待其他crate依赖。另一个想法是：目前只能是依赖的第一个crate构建完成才能开始第二个crate，但实际上还可以改进，比如第一个依赖的crate的元数据生成以后就可以开始构建第二个的元数据。

这是Cargo在2019年要努力的重点。另外还有两个小目标：解决技术债务和完成即将完成的工作（比如Cargo features的长期改善计划）。

目前将近完成的一些工作：

- 自定义crate注册机构（私有的crates.io）（[custom registries](https://github.com/rust-lang/cargo/issues/6589)）
- 离线模式（[offline mode](https://github.com/rust-lang/cargo/issues/4686)）
- 配置文件依赖（[profile dependencies](https://github.com/rust-lang/rfcs/blob/master/text/2282-profile-dependencies.md)）
- 公共/私有依赖（[public & private dependencies](https://github.com/rust-lang/rust/issues/44663)）

[Read More](https://www.ncameron.org/blog/cargo-in-2019/)

### const generics 當前進度報告

目前還在实现中

`ConstEvaluatable(expr)` 已經做好，現在一定要传入expr

[Read more](https://github.com/rust-lang/rust/issues/44580#issuecomment-488819344?tdsourcetag=s_pctim_aiomsg)
[Read more](https://www.reddit.com/r/rust/comments/bkcgmp/const_generics_a_summary_of_progress_so_far/)

---

# 社区新闻

### crates.io的crate下载总数达到了10亿

![img](https://s2.ax1x.com/2019/05/05/E0tfMR.png)

### 建立 Rust 私有仓库

Rust 1.34.1 版本，推出了一项新功能，允许用户建立自己的私有仓库。这篇作者建立了世界上第一个 Rust 私有仓库。请参考。

[Read More](https://blog.cloudsmith.io/2019/05/01/worlds-first-private-cargo-registry/)

### 清华大学陈渝副教授：尝试将Rust/Risc-V等新技术用于系统类课程教学

看看对学生有啥帮助。这里陈教授给出了一些学习资料：

- [面向初学者的代码/文档信息](https://github.com/LearningOS/rcore_step_by_step)  
- [面向对rust比较熟悉的同学的信息](https://github.com/rcore-os/rCore)
- [以及](https://github.com/oscourse-tsinghua/rcore_plus/wiki)

有兴趣学习/参与开发 rust-based os kernel的朋友，欢迎与陈教授联系和交流（他也在Rust社区微信群和Rust编程之道的读者群里）。

### Sonic：用Rust编写的Elasticsearch的极简替代品

[Read More](https://zhuanlan.zhihu.com/p/63963140)

### 想不想在机器学习领域用上 Rust？

这是 Rust 的痛。从最早的 leaf，到后来的 [rusty-machine](https://github.com/AtheMathmo/rusty-machine), [rustlearn](https://github.com/maciejkula/rustlearn)，再到 [juice](https://github.com/spearow/juice) 。没有一个活得好的。只怪没有一个好爹？

[LukeMathWalker](https://github.com/LukeMathWalker) （ndarray 和 ndarray-stats 的重要参与者）仔细分析了这一现状。他认为 Rust 在这一领域其实是非常有潜力的。一个 ML 生态的基础有三个基石：

- n维数组运算库
- dataframes
- ML 模型接口

ndarray 已经初具成效了。现在作者准备在后面两个上做一些努力，现在他正在做一些讨论和调查。

[讨论1](https://github.com/rust-ml/discussion/issues/1)  
[讨论2](https://github.com/rust-dataframe/discussion/issues/1)  
[slide](https://docs.google.com/presentation/d/1dOqqosLPtBixIVSvNy5-vLVAapfFUMm3sV4TR9v-Fkw/edit#slide=id.g58df242dc9_0_8)

有兴趣者可以参与讨论。

### 「警惕」存在于crate中的安全风险 

[Read More](https://zhuanlan.zhihu.com/p/64586315)

### VS Code Remote 開發新天地

未來可以有更多應用

- 開啟五排開黑寫程式
- 有了rank之後，大家寫程式更有競爭動力
- 自動依rank配對等級相同的開發者
- 坐等各地區rank第一的程式寫手pk

[Read more](https://zhuanlan.zhihu.com/p/64505333)

### 使用Rust和Synthesizing进行3D模型合成展示

Synthesizing是一個歷史悠久的图形学技术，過去常用來製作背景修改、去除臉部痘痘、雀斑等

現在有人將他用來做在3D模型上

有興趣的可以來看看他怎麼做的

[Read more](https://www.reddit.com/r/rust/comments/bk4i59/synthesizing_3d_structures_with_rust_in_immense/)

### 重磅：DataFusion 性能评测，性能展露头角，内存占用少得惊人

DataFusion 的作者 Andy Grove 最近在项目 [datafusion-benchmarks](https://github.com/andygrove/datafusion-benchmarks) 仔细评测了 DataFusion 的性能。对手是大名鼎鼎的 Apache Spark。

简单查询 DataFusion 在某些地方速度已经领先了，但是涉及到 GROUP BY 这种复杂一点的，DataFusion的性能还是不够，需要改进。作者已经着手在改进了。

令人惊喜的是，DataFusion 的内存占用，几乎只是 Spark 的 1/100 （Spark 8G，DataFusion 80M）！

[Read More](https://andygrove.io/2019/04/datafusion-0.13.0-benchmarks/)

### 整个社区都在热烈讨论 await 语法问题

闹翻天了。await 这个关键字没问题，现在最大的问题是：await 如何与 ? . 号这些操作符配合，而不产生歧义，代码层面上，又要直观，好看。整个社区，包括大佬们都分成几派。目前大体分成 4 派：

- Order of Operations Solution
- Syntactic Sugar Solution
- Postfix Keyword Solution
- Postfix Sigil Solution

大的派别分为前缀派和后缀派。

这个问题，可以说是 19 年最大的问题了。很多人的东西都卡在这个问题上，所以官方压力也很大，想尽快定下来。

无船同志说希望在 1.37 版本（7月4号）把async-await稳定下来（不过只是开始，后续还有大量工作要做，比如 trait 中的异步函数，大量优化工作等）。

各位看官，下面4种形式，你选哪种？请在下面踊跃发言。

![img](https://raw.githubusercontent.com/daogangtang/picmaterials/master/0%20(1).png)


[Read More](https://paper.dropbox.com/doc/Await-Syntax-Write-Up-t9NlOSeI4RQ8AINsaSSyJ#:uid=096894980756621041377818&h2=Ergonomic-&-Readability-Consid)  
[Read More 2](https://boats.gitlab.io/blog/post/for-await-i/)  
[Read More 3](https://internals.rust-lang.org/t/await-syntax-discussion-summary/9914)  
[Read More 4](https://github.com/rust-lang/rust/issues/57640#issuecomment-487758650)

### 放弃wlroots-rs项目

wlroots-rs项目的作者宣布放弃该项目。原因是因为它碰到的问题，无法用Safe Rust去处理。他认为Safe Rust才是Rust存在的意义，不太想用Unsafe Rust来处理问题，所以就选择回到了C语言。

[Read More](http://way-cooler.org/blog/2019/04/29/rewriting-way-cooler-in-c.html)

在该话题的Reddit讨论区，rlua的作者深有同感，也写下了自己的感受：

> rlua 让他身心疲惫，也经历了类似的失败。但是，他又说了：如果有Rust无法表达的模式，那么我想我们应该努力使Rust更好，或者找到新的模式？我认为Rust的最大优势是可以把全局不安全的东西变成局部不安全的东西。我知道这对于所有任务来说都不是100％可能，但我正在努力找出剩余问题的答案。

同样也有人指出：

> 当你认为Rust的全部意义仅仅是安全的时候，那么你就错失了Rust的好处。这并不是Rust的全部观点。Rust是让你在不安全的基础上抽象安全。如果那个C库本来就不安全了，那么Rust允许你公开那个接口，而不是非得把它包装为安全的。

很多人也产生了共鸣。

> C和Rust之间的映射，确实比较困难。可能需要总结一些最佳的模式。

[Reddit 讨论区](https://www.reddit.com/r/rust/comments/biq864/giving_up_on_wlrootsrs/)

### 「手工」自制Rust吉祥物Ferris布偶

喜欢Ferris的朋友可以按这个教材手工制作一个。

[Read More](http://edunham.net/2019/04/06/rustacean_hat_pattern.html)

---

# 学习资源

### 指南：Rust Web开发中的Futures

本文以HTTP请求为示例，比较系统地介绍了Futures。

[Read More](https://dev.to/gruberb/explained-rust-futures-for-web-development-a10)

### Rust和Windows不得不说的事儿

感谢社区 @Matrix 的分享

[Read More](https://zhuanlan.zhihu.com/p/64344775)

### 如何快速实践actix和actix-web

[Read More](https://zhuanlan.zhihu.com/p/64457544)

### 「系列视频」从头开始写一个简单的x86-64 C编译器

這是一個系列影片的第一部， 有興趣的朋友可以看看。

- [Read more](https://www.reddit.com/r/rust/comments/bjvfwu/practical_video_tutorial_write_a_simple_c_to/)
- [（作者好像是国人，这个ID有意思： One HR） 源码：onehr/crust](https://github.com/onehr/crust) 

### 「系列文章」Rust：如何使用私有Cargo依赖项构建Docker镜像

本文展示了如何在构建Docker镜像时获取私有Cargo依赖项并获取它们。

[Read More](https://medium.com/@c_ameron/rust-how-to-build-a-docker-image-with-private-cargo-dependencies-ab91c25c4301)

###  「系列文章」Rust如何发送邮件 三篇

作者将通过三篇文章来讲解如何用Rust编写邮件发送的代码。

[Part I](https://blog.1aim.com/post/002-mail-1-intro/)
[Part II](https://blog.1aim.com/post/003-mail-2-crate/)
[Part III](https://blog.1aim.com/post/004-mail-3-example/)

### xv: 命令行16进制查看器

彩色输出不同类别的字节。

[Read More](https://chrisvest.github.io/xv/)

同类工具： [hexyl](https://github.com/sharkdp/hexyl)

作者分享： XV中如何使用panic

XV是一个终端16进制查看器，作者之前是Java开发者，XV是他的第一个Rust项目。他在本文主要介绍了UX中使用panic的一些经验。

- 作者认为Rust里的panic等价于Java里的异常，所以他在XV中大量使用Unwrap。（日报君友情提醒：这一条谨慎看待，不是指滥用）
- 当然，作者也是针对具体的情况来使用unwrap，在预期操作总是会成功的情况下会unwrap。所以，如果程序运行崩溃了，说明代码里有问题。
- 类型转换。在确定安全的情况下使用From，在不太安全的情况下使用TryFrom。
- 在Release模式下开启默认检查算术溢出。Rust在Debug模式下如果算术计算溢出会报错，但是在Release模型下会静默生成错误的结果。需要在Cargo.toml中设置`overflow-checks = true`来开启溢出检查。
- 自定义了一个panic处理程序，类似于`HumanPanic`库。它会捕获回溯信息，当崩溃后再次启动XV，它还会显示一条错误信息，提醒人们在GitHub上提交错误报告。

- [Read More](https://medium.com/@chrisvest/how-xv-uses-panics-ba22bd6152a5)

### 「嵌入式Rust」一些资料介绍

- [嵌入式book ，最近有一些更新](https://github.com/rust-embedded/book)
- [嵌入式book在线阅读，介绍如何使用Rust为裸机编写固件](https://rust-embedded.github.io/book/intro/index.html)
- [Discovery book,基于微控制器的嵌入式系统的入门课程](https://rust-embedded.github.io/discovery/index.html)
- [官方维护的 awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)

### Rust Web Developer Roadmap 2019

哇，第三方整理的，非常好。对于一个新人来讲，怎么快速清楚如果想用 Rust 做 Web 开发的话，要学习什么，涉及哪些模块，组织结构如何。

![img](https://raw.githubusercontent.com/csharad/rust-web-developer-roadmap/master/rust-web-developer-roadmap.png)

[Read More](https://github.com/csharad/rust-web-developer-roadmap)

### 制作落沙游戏

该文作者制作过一款落沙游戏（falling sand game）Sandspiel，这种游戏允许玩家选择不同的材质（沙子、水、石头、冰块等）放到游戏中，自然下落，形成各种造型，比较艺术。

作者在这篇文章里，介绍了他为什么要做这款游戏，以及这款游戏的架构等技术资料。

架构：

- 粒子模拟代码：Rust/WASM
- 流体模拟：JS和GLSL
- React和JS编写界面
- TypeScript和Postgresql编写CRUD后端

- [在线试玩](https://sandspiel.club/)
- [Read More](https://maxbittker.com/making-sandspiel)
- [sandspiel源码](https://github.com/MaxBittker/sandspiel)

### Sled代码评审 Part III

Sled 是一个 Rust 写的嵌入式数据库，质量相当不错。作者用了一段时间后，想了解里面的实现，就开始了这个学习和审阅过程。这是第三篇。

[Read More](https://ayende.com/blog/187073-C/reviewing-sled-part-iii)

### 使用Sonr构建pub/sub服务器 Part II

sonr建立在mio之上的网络库，相比于Tokio来说，更加轻量。

[Read More](https://hagsteel.com/posts/building-a-pub-sub-with-sonr-part-2/)

### 給「非C++開發者」的Rust并行计算教程

[Read more](https://medium.com/nearprotocol/rust-parallelism-for-non-c-c-developers-ec23f48b7e56)

### Rust图形库指南

该指南主要为那些想要使用Rust编写图形内容（视频游戏，动画，炫酷可视化等）并且不知道从哪里开始的人提供上下文。

[Read More](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019)

---

# 项目、工具与库

### Hawk：基于Rust和AWS Services的图像识别应用

用于人脸识别门禁系统的一个原型项目。

架构设计图

![img](https://s2.ax1x.com/2019/05/05/E0Y0AO.png)

- [Read More](https://blog.knoldus.com/hawk-image-recognition-project-using-rust-and-aws-services/)
- [hawk](https://github.com/knoldus/hawk)

### 用Rust和K8S交互

虽然通常使用Go和kubernetes交互，这豪无争议。但是现在随着客户端的进化，再加上Rust的泛型和过程宏，现在完全有可能使用Rust来编写一个成熟的k8s客户端了。

该文作者意见编写了好几个Rust的K8S工具，包括：k8s-openapi。并且他们也提交了新的工具：kube-rs，纯Rust实现的k8s客户端。

更多内容请阅读原文。

- [k8s-openapi](https://github.com/Arnavion/k8s-openapi)
- [kube-rs](https://github.com/clux/kube-rs)
- [operator-rs: kube-rs使用示例项目](https://github.com/clux/operator-rs)

[Read More](https://clux.github.io/probes/post/2019-04-29-rust-on-kubernetes/)

### Terminal Redox：一些用Rust编写的开发工具

该文介绍了一些终端工具，包括：

- Alacritty，跨平台、GPU加速的终端
- exa，Rust实现的`ls`命令
- dust，du (disk usage)工具的Rust版本

等等

[Read More](https://sts10.github.io//2019/04/08/terminal-redox-alacritty.html)

### webrtc-unreliable：用于编写具有多个基于WebRTC的Web客户端和类似UDP的网络的Rust服务器库

作者最近想基于wasm做一些网络游戏相关的实验，但是发现像 WebSocket 这种协议并不能满足他的要求：不可靠，无序包，需要面向具体的业务在上层做定制，不通用。而目前在Web层面，是不能直接发 UDP 包的。只有 WebRTC 这套协议里面提供了这种可能性，于是作者就开干了。

这个想法并不是他独创的，比如有一个叫 [WebUDP](https://github.com/seemk/WebUdp) 的项目。他用 Rust 实现了这个想法。

- [Read More](https://www.reddit.com/r/rust/comments/bihg1b/webrtcunreliable_a_library_for_writing_rust/)
- [webrtc-unreliable](https://github.com/kyren/webrtc-unreliable)

### RustPlayground: 用于Mac平台的Playground桌面软件

[RustPlayground](https://github.com/cmyr/RustPlayground)

### Rust 3D引擎 kiss3d 0.20发布

Kiss3d是一个跨平台（包括WASM）2D和3D图形引擎，旨在简单地用于编写演示和原型的渲染部分。 此版本增加了对基于conrod的即时模式GUI的支持。 

[kiss3d](https://github.com/sebcrozet/kiss3d)

###  rendy-pbr: 用rendy编写的小型实时基于物理的渲染器

PBR全称(Physicallly-BasedRendering)。笼统的说，就字面含义可以看出，这是一种基于物理规律模拟的一种渲染技术。它构建在gfx-hal上。该项目是rendy和Amethyst的试验场; 这里实现的大部分或全部内容最终将以某种形式添加到Amethyst渲染器中。

[rendy-pbr](https://github.com/termhn/rendy-pbr)

### SideFuzz：寻找时序旁路攻击漏洞的模糊测试库

它的工作原理是将模糊测试目标编译到WebAssembly，然后在修改后的wasmi解释器中对wasm目标进行模糊测试，该解释器计算单个指令的执行次数。

旁路攻击的一个案例：

> 举一个最简单的计时攻击的例子，某个函数负责比较用户输入的密码和存放在系统内密码是否相同，如果该函数是从第一位开始比较，发现不同就立即返回，那么通过计算返回的速度就知道了大概是哪一位开始不同的，这样就实现了电影中经常出现的按位破解密码的场景。密码破解复杂度成千上万倍甚至百万千万倍的下降。 来源： [知乎：如何通俗地解释时序攻击(timing attack)?](https://www.zhihu.com/question/20156213/answer/43377769)

[sidefuzz](https://github.com/phayes/sidefuzz)

### watchrs: 使用Rust监控AWS批量Jobs

- [Read More](https://medium.com/rusted/monitoring-aws-batch-jobs-with-rust-8f1ef6115871)
- [watchrs](https://github.com/itsHabib/watchrs)

### ruby-ext-wasm：Ruby 中执行 wasm 二进制码的扩展

wasmer.io继推出 php-ext-wasm 和 python-ext-wasm 后，又马不停蹄做出了面向 ruby 的扩展。这个扩展用来在 Ruby 中执行 wasm 二进制码。wasmer.io 是 wasm 平台化的先驱，其理念是要推动 wasm 成为一个通用的计算平台。

[Read More](https://github.com/wasmerio/ruby-ext-wasm/)

### dpar：神经网络过渡依赖解析器

这个估计懂深度学习的同学看得懂一些。是 go 版本的重写，原来的版本可以在同一仓库中找到。

[Repo](https://github.com/danieldk/dpar)

### rclc - 又一个命令行计算器

这个计算器可以执行大数计算、浮点、分数和复数运算，看起来好用。

```
> sqrt(-2)  // square root of negative number
= 0.0+1.4142135623730952i
> sqr(ans) // square root of a complex number may produce real number
= -2.0000000000000006
> 345**12 // big integer in action
= 2843342266303054544082275390625
> 1\2 + 3\5  // one half and three fifth is one and one tenth
= 1\1\10
> sqr(3\5)  // square of a rational number is a rational number
= 9\25
> sin(90°) == sin(pi/2) // degrees and radians mixed in one expression, '°' can be replaced with 'd' for easier typing 
= 1
```
使用 

```
$ cargo install rclc
```
即可安装。

[Repo](https://github.com/VladimirMarkelov/rclc)

### fui - 为你的终端命令行添加表单功能和界面

先来看看截图效果：

![img](https://raw.githubusercontent.com/xliiv/fui/master/doc/app_tar_like.png)

这个库刚刚发布了 1.0。现在支持 clap 集成了。

[Repo](https://github.com/xliiv/fui)

### Luster 使用rust实现的實驗性質的lua VM

目前產業界還是以 luajit 為大宗，希望 luster 的速度能超越 luajit 為 lua 帶來新氣象

[Read more](https://www.reddit.com/r/rust/comments/bjvt3i/luster_an_experimental_lua_vm_implemented_in_pure/)

### spruce： 硬盘空间可视化

這是他第一個發佈的 crate，歡迎大家給作者友善的建議

[Read more](https://www.reddit.com/r/rust/comments/bjx734/my_first_published_crate/)

### Risp (Lisp (in (Rust)))

這位仁兄腦洞大開

想要使用rust執行lisp

大家可以看看他怎麼做的

[Read more](https://m.stopa.io/risp-lisp-in-rust-90a0dad5b116)
[Reddit讨论](https://www.reddit.com/r/rust/comments/bjy3y9/risp_lisp_in_rust/)

### Viu：支持在命令行查看图片

命令列無法看圖片一直是個大問題

但這位大佬做到了！在命令列看图！

[Read more](https://github.com/atanunq/viu)

### bardecoder 一個 QRCode 加密解碼庫

看起來簡單好用

[Read more](https://github.com/piderman314/bardecoder)

### 「嵌入式Rust」micromath 0.3发布

micromath是一个嵌入式的Rust数学库，支持快速安全的浮点数近似计算、常用的算术运算、2D/3D向量类型、统计分析和四元数等。

[micromath](https://github.com/NeoBirth/micromath)

### pkg-version: 可在编译时获取Cargo包版本

一般情况下可以通过`CARGO_PKG_VERSION_MAJOR`环境变量来获取包的版本号，但是这种方式总是会产生字符串，只能在运行时将其解析为数字。所以这个库提供了一个`pkg_version_major!`过程宏在编译期解决这个问题。

（看源码发现依赖dtolnay的proc-macro-hack，它是一个支持表达式位置过程宏的库)

- [pkg-version](https://github.com/jonas-schievink/pkg-version)
- [proc-macro-hack](https://github.com/dtolnay/proc-macro-hack)

### Plotka： 轻松可视化浏览器中的数据

Plotka可以从stdin获取数据，解析它（作为JSON或CSV）并通过websockets进行广播。 还可以托管静态文件，可以完全通过它在浏览器中绘制数据，它可以用作Matplotlib等的替代品。

[plotka](https://github.com/micouy/plotka)

### rudolfs: 基于AWS S3的高性能缓存Git LFS服务器

LFS，Large File Storage, 大文件存储

[rudolfs](https://github.com/jasonwhite/rudolfs)