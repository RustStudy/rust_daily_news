### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-03-17

---

# 通告 

### Rusty棒球帽周边定制活动已发货

请大家查收邮件。另外想要帽子的也没有存货了，期待下次的活动吧。

---

# 官方新闻

### Rust核心团队加入了两员大将

新加入的是Manishearth和Skade。

[Read More](https://internals.rust-lang.org/t/new-members-for-the-rust-core-team/9575)

### 「嵌入式Rust」Rust树莓派3指南新增JTAG章节

JTAG介绍（摘自网络）：

> JTAG是最基本的通讯协议之一，大家可以理解为与RX TX或者USB的道理是一样的，只是一种通讯手段，但与RX TX以及USB有很重大的不同，那就是这个JTAG协议是最底层的，说的通俗一点，一般来说，手机里边，CPU是老大，对吧？但在JTAG面前，他就不是老大了，JTAG协议就是用来控制CPU的，在JTAG面前CPU变成喽啰了。一般的协议是求着CPU读写字库的程序，但JTAG可以读写CPU的程序，命令让CPU啥活都干。

这份指南（rust-raspi3-OS-tutorials）是Rust嵌入式官方工作组出品的。该教程旨在旨在为Rust系统编程语言中的Raspberry Pi 3上的裸机编程提供简单的参考代码。重点是利用Rust的零开销抽象来编译可读，简洁和安全的精益代码（至少在裸机硬件上是安全的）。它只是针对不同的主题分出不同的教程，不是完整的OS教程，也许在不远的将来可以实现一个完整内核的教程（但不要相信我说的）。

[Read More](https://github.com/rust-embedded/rust-raspi3-OS-tutorials/tree/master/0B_hw_debug_JTAG)

### 稳定`std::task`和`std::future::Future`的RFC已被合并

`std::task`和`std::future::Future`是将async/await稳定为第一类公民的基础。该RFC涵盖了：

- std中task模块的所有API
- core和std的future模块

[RFC 2592 pr](https://github.com/rust-lang/rfcs/pull/2592)

###  「Rust和WASM工作组」让我们一起构建Gloo

Rust和WASM工作组的2019愿望是希望为Rust和WASM开发构建稳定、可持续、生产化的生态系统。为了实现这个目标，工作组推出了Gloo，它是一个模块化的工具包，作用如下：

- 构建更小且能集成到大型JavaScript系统种的WASM模块
- 可以用Rust编写整个Web应用

Gloo于WASM的目标，等价于Tide于服务端Web的目标。Gloo将进一步抽象出高级的库和框架，易于开发。该项目才刚刚开始，欢迎参与。

- [Read More](https://rustwasm.github.io/2019/03/12/lets-build-gloo-together.html)
- [rustwasm/gloo](https://github.com/rustwasm/gloo)

### 「官方通告」关于crates.io更新了索引处理方式的说明

(原来Diesel作者sgrif也参与了crates.io的维护)

之前是同步的方式，现在改成了异步。所以，如果你发布crate的时候，如果没有发现错误，也并不意味着你可以正常发布你的crate。主要是因为有延迟问题，现在最大延迟是3秒，如果依赖的包太多，你可能需要重试几遍？？当然，官方目前正在处理这个问题。

[Read More](https://internals.rust-lang.org/t/changes-to-how-crates-io-handles-index-updates/9608)

### 「官方会议讨论视频记录」Rust Unsafe代码指南讨论

这是Rust官方不安全工作组的讨论视频记录，主题是Unsafe Rust代码指南。一共六个人参与讨论，大家感兴趣可以看看。

[Read More](https://www.youtube.com/watch?v=atRKeoWbfi0)


---

# 社区新闻

### Rust有多高效？

该文章作者，在Google的Hash Code 2019编程竞赛活动中对Rust的高效能力进行了测试。

> Hash Code是一个团队编程竞赛，所有团队必须在4小时的时间窗口内解决单个工程问题。今年，来自世界各地的6000多支队伍参赛。
> Hash Code提出的问题一直是NP-hard的优化问题，需要实现近似解决方案。因为每个团队只有4个小时来实施，而且还运行他们的解决方案，所以他们不能是CPU密集型的。通过对其近似解决方案生成的结果进行评分来对团队进行排名。

问题：

今年的问题是由一组照片创建幻灯片。这些幻灯片的评分取决于它们的有趣程度。每张照片都有一组与之关联的标签。

竞争对手：

Java，C / C ++，Python和C＃等主流语言。而作者的团队当然是用Rust。

结论：

作者的团队取得了第783名的成绩（6000个团队），所以，通过这次测试，作者认为Rust是足够高效的语言。当然，编程竞赛属于一种特定场景，但在短短4小时内解决一个问题，也可以说明一门语言是否高效了。

[Read More](https://medium.com/@woutergeraedts/how-productive-is-rust-e2260db28f09)

### 基于Pathfinder和Servo实现VR中使用GPU对SVG渲染

Pathfinder 2是一款快速，实用，正在进行中的基于GPU的光栅化工具，用于使用OpenGL和OpenGL ES 2.0+进行字体和矢量图形处理。

![img](https://wx1.sinaimg.cn/mw690/71684decly1g14hozdcv3j20u00z0kjl.jpg)

[pathfinder](https://github.com/pcwalton/pathfinder)

### 开源游戏引擎GDNative发布Rust官方绑定godot-rust

该开源游戏引擎据说还支持开发AR

- [godot-rust](https://github.com/GodotNativeTools/godot-rust)
- [官网](https://godotengine.org/)

### 当Rust遇上机器学习：SIMD、BLAS和Lapack

该文作者是一名数据科学家，有90%的编程工作都是和Python打交道。几个月前，该作者对Rust产生了兴趣，但是发现Rust在机器学习的关键限制是缺乏开发体验良好的线性代数库。有nalgebra和ndarray其他几个库，但是作者认为不好用，没有针对底层的SIMD、blas和Lapack高性能数值计算。作者用的最多的是ndarray。

虽然不好用，但作者发现Rust非常适合编写高性能代码。使用blas-src和lapack-src，以及Rust内置的SIMD函数就可以编写令人惊讶的Rust代码。

> BLAS（Basic Linear Algebra Subprograms，基础线性代数程序集）是一个应用程序接口（API）标准，用以规范发布基础线性代数操作的数值库（如矢量或矩阵乘法）。该程序集最初发布于1979年，并用于建立更大的数值程序包（如LAPACK）。

这篇文章简单介绍了如何在Rust中使用SIMD、BLAS和Lapack。最后给出了一个结论：

> 摩尔定律正在失去效用，所以如果我们想要继续蓬勃发展机器学习的生态系统，就需要学会优化机器学习。与Python调用C相比，使用此方法（Rust中使用SIMD/BLAS/LAPACK）可实现性能4-10倍的提升。Rust也非常适合调试，严格的类型可以在执行数据工程时提供良好的实践。 我劝你：开始尝试使用Rust进行机器学习吧。虽然现在它会有点令人小失望，但在未来几年内会得到回报。

[Read More](https://www.erikpartridge.com/2019-03/rust-ml-simd-blas-lapack)

### 「视频」介绍RMS和Apache Arrow

演讲者是DataFusion的作者

[Read More](https://www.youtube.com/watch?v=iQcOW2fVX2c)

### hi Rustaceans! Rust有问题该去哪里问？

#Question

这个帖子里有人整理了一些提问的地方：

- StackOverflow, [Rust Tag](https://stackoverflow.com/questions/tagged/rust)
- CodeReview.Stackexchange, [Rust Tag](https://codereview.stackexchange.com/questions/tagged/rust)
- Reddit, [r/learnrust](https://www.reddit.com/r/learnrust)
- Rust官方User论坛, [users.rust-lang.org](https://users.rust-lang.org/)
- Rust相关的IRC频道， [#rust](https://client00.chat.mibbit.com/?server=irc.mozilla.org%3A%2B6697&channel=%23rust)，这样的频道还有好几个： `#rust-beginners`/ `#cargo `/ `#rust-gamedev`/ `#rust-osdev`/ `#rust-webdev`/ `#rust-networking`


以上是国外的，我整理一下国内的：

- Rust社区QQ群（253849562/813448660） /微信群/Telegram群 （https://t.me/rust_zh）
- rust.cc 论坛
- 《Rust编程之道》读者群/ 随书源码issues （最好交流和本书学习相关的）

微信加群方式可以私聊

[Read More](https://www.reddit.com/r/rust/comments/azqo9c/hey_rustaceans_got_an_easy_question_ask_here/)

### ZEIT的Now服务宣布支持Rust

ZEIT是一个Serverless服务平台，的主打产品是 now，一个一行命令就能发布Node或者Docker应用的PaaS，现在支持了Rust。因为Rust性能太好，所以ZEIT可能会推出基于1ms的定价（233）。

为了演示，他们基于now服务发布了一个简单的Rust爬虫，基于servo：rust-scraper.now.sh，可以爬Hackernews的新闻。

- [Read More](https://zeit.co/blog/introducing-now-rust)
- [now-rust](https://github.com/zeit/now-builders/tree/master/packages/now-rust)
- [rust-scraper.now.sh](https://rust-scraper.now.sh/)

###  可在线搜索Rust代码的服务

基于crates.io来搜索

[codesearch.aelve.com/rust](https://codesearch.aelve.com/rust)

### Rust编程中的好的、坏的、丑的

这篇文章总结了Rust语言、工具、库中作者认为好的地方、不好的地方，以及作者感觉有点丑陋的地方。虽然有点主观，我也不同意他的某些观点（比如他认为宏很丑陋，我觉得还好），但觉得值得分享出来让大家看看。

（这位博主说他12岁就开始写代码了）

- [Read More](https://hackernoon.com/programming-in-rust-the-good-the-bad-the-ugly-d06f8d8b7738)
- [Reddit 讨论](https://www.reddit.com/r/rust/comments/azw9cn/programming_in_rust_the_good_the_bad_the_ugly/)

### 科学Rust

任何涉及使用计算机进行科学研究的东西都算作科学编程。它包括从运行在卫星上的嵌入式软件到运行在超级计算机中的气候模型，从运行管道工具的shell脚本到使用笔记本电脑的数据分析。

该文作者在生物学领域做科学编程的工作，在这篇文章里，描述了他对于Rust的2019期望，给出了Rust如何做才能更好地面向科学工作者的建议。

看得出来Rust在生物学领域已经有了一些可用的库，可以在相关GitHub组织RustBio中找到。

- [Read More](https://blog.luizirber.org/2019/01/05/rust-2019/)
- [rust-bio](https://github.com/rust-bio)

### 使用rust-vmm构建未来的虚拟化堆栈

vmm是指VirtualMachineMonitor。Firecracker（亚马逊的）是一个基于KVM的轻量级VMM，可以在几分之一秒内启动虚拟机，内存占用少，可以实现高密度云环境。在Firecracker被开源之后，该开发团队又在2018年底启动了rust-vmm项目，旨在实现安全共享虚拟化核心组件。rust-vmm会以多个独立的crate来发布，都在RustVMM GitHub组织下开源。

- [Read More](https://opensource.com/article/19/3/rust-virtual-machine)
- [rust-vmm GitHub组织](https://github.com/rust-vmm)

### V语言： 受Rust和Go启发的新编程语言

#lang

目前还只是文档，作者并未开源，据说是2019年中开源。先来感受下代码风格：

```rust
fn add(x int, y int) int {
	return x + y
}

fn sub(x, y int) int {
	return x - y
}

fn main() {
	println(add(77, 33))
	println(sub(100, 50))
    name := 'Bob'
	age := 20
	large_number := i64(0)
	println(name)
	println(age)
}
```

[vlang.io](https://vlang.io/)

---

# 学习资源

### 《Rust编程之道》读者答疑：每个章节副标题名言代表的意义 

[Read More](https://zhuanlan.zhihu.com/p/59384453)

### 「BlogOS系列教程」分页实现

最新的这篇文章展示了如何在内核中实现分页。

该系列教程的中文翻译可以关注： [知乎专栏：做一枚爱生活的Rustacean](https://zhuanlan.zhihu.com/c_1078248076300521472)

[Read More](https://os.phil-opp.com/paging-implementation/)

### seed 发布0.3版本

Seed是一个Rust前端框架，借助wasm可以创建Web App。

[Read More](https://github.com/David-OConnor/seed/blob/master/CHANGELOG.md)


### 如何让Haskell搜索字符串与Rust一样快

这篇文章描述了作者创建Alfred-Margaret的过程，它是Aho-Corasick字符串搜索算法中最快的Haskell实现，用于支持Channable中的字符串搜索。Channable是一种Feed处理工具，用户可以在其中定义规则以优化其产品Feed。

作者最初实现的算法和上万star的明星Rust库就是用Rust实现了Aho-Corasick算法进行比较，性能差距很大。

![img](https://wx4.sinaimg.cn/mw690/71684decly1g12gqrhci0j21330u078l.jpg)

但是在经过努力优化以后，这个差距缩小了。

- [Read More](https://tech.channable.com/posts/2019-03-13-how-we-made-haskell-search-strings-as-fast-as-rust.html?)
- [aho-corasick的Rust实现](https://github.com/burntsushi/aho-corasick)
- [ripgrep](https://github.com/BurntSushi/ripgrep)

### 「视频」Rust SDL游戏开发系列 #1

本视频大概20分钟。我有个计划，希望招募（凭各人兴趣）一些社区的人，制作Rust相关的双语视频，把youtube的搬运到B站，我个人精力实在有限，如果感兴趣可以私聊我。

[Read More](https://www.youtube.com/watch?v=LMlX2tF_IsI&feature=youtu.be)

### 使用CLION远程开发和调试Rust

[Read More](https://medium.com/@zaver.max/remote-development-and-debugging-of-rust-with-clion-39c38ced7cc1)

### 为NodeJS开发者介绍Rust Web开发

一个简单的介绍，包括工具对比

[Read More](https://medium.com/@gruberbastian/intro-to-web-programming-in-rust-for-nodejs-developers-1a9c048c4de1)

### 「简单示例」用Rust扩展Python

该文作者平时使用Python来完成大数据处理，但是Python的性能让他堪忧。他考虑用C/CPP来改善代码，但是又考虑到C和Cpp很难掌握，即便难掌握，也会遇到很多内存管理和段错误的问题，所以他再三考虑之后，选择了Rust。

作者写了个简单的代码示例，介绍了如何在Python中使用Rust编译出来的动态库。

[Read More](https://www.simernes.com/?p=514)

### 在Rust中实现GADT的一些思考

该文作者是一名Haskell选手，他在尝试用Rust实现GADT（Haskell种的广义代数数据类型）

- [Read More](http://www.philipzucker.com/thoughts-on-faking-some-of-gadts-in-rust/)
- [typo](https://github.com/philzook58/typo)

### 「视频」用Rust实现Redis模块

该视频是用于学习而制作

- [代码](https://github.com/gsquire/redis-multi-map)
- [Read More](https://www.youtube.com/watch?v=tu9Lw8k-Uss&feature=youtu.be)

### 「系列博客」 Rust与科学计算 Part 1： Rust的冒险之零成本抽象

有博主宣布要写Rust和科学计算的系列文章，本文是该系列的第二篇文章。

> 该作者的日常工作是机器学习，他在多次使用Rust进行相关的实验之后发现，Rust语言在这个领域将大有可为，并让他感觉非常激动。作者反观了Python统治下的机器学习世界，其生态系统爆炸的原因是因为有很多基础库，比如NumPy，SciPy，Pandas等。大多数项目是构建在NumPy和SciPy之上。如果Rust也拥有这些核心的基础库会怎么样？抱着这样的想法，作者加入了维护ndarray库的队伍中。他贡献了一堆PR之后，诞生了一个独立的crate：ndarray-stats。这个系列的文章，将围绕ndarray来阐述。

[Read More](https://www.lpalmieri.com/posts/2019-03-12-scientific-computing-a-rust-adventure-part-1-zero-cost-abstractions/)

###  利用Rust的类型系统消除运行时越界检查

主要技巧是为实现的数据结构中的索引建立关联闭包，然后通过正确关联的闭包来访问数据，如果是空的索引，则会造成编译期错误。这个错误就证明了有越界访问。但这个技巧最好是在团队内达成共识，否则错误看上去会非常奇怪。

[Read More](https://fullyfaithful.eu/bounds-check-elision-rust/)

### 以数据处理为案例，教Pythoner使用Rust

[Read More](https://medium.com/@rajasekar3eg/making-a-case-rust-for-python-developers-1a114e2d89f4)

### pulldown_cmark 0.3 发布

pulldown_cmark是用Rust编写的CommonMark markdown标准的高性能解析器，刚发布了0.3版本。这篇文章中还介绍了该团队优化0.3版本性能的一点心得：

- 为了增加解析速度，快速构建和遍历AST，他们使用了indextree crate。
- 在没有严格要求的情况下，永远不要复制或分配内存。只用引用，以及在新版本中使用了一种新的写时复制类型（类似于`Cow<T>`)

我查了下源码，他们用的写时复制类型叫 `CowStr`

```rust
#[derive(Debug, Eq)]
pub enum CowStr<'a> {
    Boxed(Box<str>),
    Borrowed(&'a str),
    Inlined(InlineStr),
}

pub struct InlineStr {
    inner: [u8; DOUBLE_WORD_SIZE],
}

```

应该是根据自己的场景定制的。

[Read More](https://fullyfaithful.eu/pulldown-cmark/)

---

# 项目、工具与库

### purple: Rust实现的一个现代化的开放区块链协议

Purple号称是一种先进的，实验性的开放式区块链协议，从第一天开始设计就注重可扩展性和完全去中心化。它可以在高峰时间达到每秒多达50000个事务。基于一种新的共识，半同步工作证明（简称SSPoW），因为它提供的异步扩展优于传统的基于工作证明的同步共识。

不管吹的如何，新项目先关注下。代码完成度很高。

- [purple](https://github.com/purpleprotocol/purple)
- [官网](https://purpleprotocol.org/)

###  「嵌入式Rust」cargo-call-stack: 一个静态栈分析工具

用于在编译期检测栈是否溢出，对于嵌入式设备的安全很重要。所以官方嵌入式组Leader Japaric实现了这样一个库。文章介绍了关于cargo-call-stack的详细实现细节。

- [Read More](https://blog.japaric.io/stack-analysis/)
- [cargo-call-stack](https://github.com/japaric/cargo-call-stack)

### dodrio: 基于Bump内存分配的虚拟Dom实现

dodrio的作者写文章介绍了dodrio实现的一些细节。它是使用Rust和WASM实现的虚拟dom库。该库的作者为了实现此库，还专门和React、Elm和Ember团队的核心开发人员讨论了很多想法。并且Mozilla负责WebAssembly标准制定的Luke和Rust核心团队的Alex都参与了该项目的设计？看来这个库不是玩票。

Bump内存分配算法，是一种快速但有限的分配算法，分配器会维护一块内存以及指向该内存的指针，当分配一个对象时，分配器将指针会按该对象的对齐规则来分配内存，并且快速测试指针有没有溢出。

- [Read More](https://hacks.mozilla.org/2019/03/fast-bump-allocated-virtual-doms-with-rust-and-wasm/)
- [dodrio](https://github.com/fitzgen/dodrio)


### Protocol Buffers库prost发布0.5版本

该库支持proto2和proto3

[Read More](https://github.com/danburkert/prost/releases/tag/v0.5.0)

### reap: 用于解析Ruby Heap Dump（堆转储文件）的工具

此工具可以用来优化Ruby应用的内存和调试内存泄漏。

[reap](https://github.com/djudd/reap)

### rel-ptr: 相对指针

该库用于构建可移动的自引用类型

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/azvpfy/relative_pointer_an_abstraction_to_build_movable/)
- [rel-ptr](https://github.com/KrishnaSannasi/rel-ptr)

### bloom-server：基于 rust 编写的 rest api cache 中间件

位于lb 与api worker 之间，使用redis 作为缓存内容存储，我们需要做的就是配置proxy，同时他使用基于share 的概念，进行cache 的分布存储，包含了请求端口（proxy，访问数据） ，以及cache 控制端口（api 方便cache 策略的控制）

[bloom-server](https://crates.io/crates/bloom-server)

### bgrep：Rust实现的一个二进制grep工具

可以匹配任何字节模式，可以跨行。

[bgrep](https://github.com/gahag/bgrep)

### pyckitup: 用Rust实现的可运行于浏览器的Python游戏引擎

- [Read More](https://pickitup247.com/pyckitup.html)
- [pyckitup](https://github.com/pickitup247/pyckitup)

### cargo-feature-analyst: 用于分析项目种features使用情况

[cargo-feature-analyst](https://github.com/psinghal20/cargo-feature-analyst)

### cuach: 又一个编译时模板

[cuach](https://nest.pijul.com/pmeunier/cuach)

### bacon: 使用Speck算法对任意结构进行加密和解密

[bacon](https://github.com/aspera-non-spernit/bacon)

### microprofile-rust： 用于Rust的微型嵌入式分析工具

microprofile使用C/CPP实现，已经被用于多个AAA级游戏。现在有了Rust的绑定。

[microprofile-rust](https://github.com/jonasmr/microprofile-rust)

### edgy: 图像边缘检测库

该库是Sobel边缘检测算法的Rust实现，看源码的Readme文件截图，效果好像不错

[edgy](https://github.com/dangreco/edgy)

### libpnet : 提供了底层网络的跨平台Rust API

[libnet](https://github.com/libpnet/libpnet)