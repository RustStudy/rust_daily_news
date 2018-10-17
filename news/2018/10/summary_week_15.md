# 官方新闻

### Rust 1.29.2 稳定版发布

[原文](https://blog.rust-lang.org/2018/10/12/Rust-1.29.2.html)


### 嵌入式工作组报告 - 13

简要：

-  const_fn的最小子集[min_const_fn](https://github.com/rust-lang/rust/pull/54835)即将在Rust 1.31中稳定，允许在嵌入式环境中使用const_fn。
-  [rust-industrial-io](https://github.com/fpagliughi/rust-industrial-io) crate启动开发，允许Rust使用Linux Industrial（工业） I/O子系统，意味着可以操作工业传感器和执行器（加速度传感器、陀螺仪、IMUs(惯性测量单位)、电容-数字转换器(CDCs)、压力、温度和光线传感器、磁力计传感器、电能功率计、旋变数字转换器等）
-  [cortex-r-rt](https://github.com/gregokent/cortex-r-rt)开始开发，用于Cortex-R处理器的运行时库。
-  embedded-hal  嵌入式硬件抽象框架已经支持Rust  1.30-beta。
- [keypad](https://github.com/e-matteson/keypad)，平台无关的键盘驱动程序
-  bluenrg和bluetooth-hci蓝牙相关的库

[更多内容 ](https://rust-embedded.github.io/blog/2018-10-09-newsletter-13/)


---

# 社区新闻

### JavaScript和WebAssembly函数调用性能提升

JavaScript和WebAssembly这两个语言之间函数调用是出了名的慢，但是最近这种情况已经得到了改善。在最新版本的Firefox Beta中，JS和WebAssembly之间的调用比非内联JS到JS函数调用要快。万岁！🎉

在最新的在Firefox Nightly工作中，一共优化了两个方向的调用从JavaScript到WebAssembly，从WebAssembly到JavaScript，而且还优化了WebAssembly调用内置函数的性能。

所有的优化都是为了减轻浏览器引擎的工作压力。这些改进一共分为两组：

-  减少登记（Reducing bookkeeping），减少了调用栈帧
- 切断中介（Cutting out intermediaries），函数调用采用尽可能直接的方式

一、如何优化WebAssembly到JavaScript的调用？

-  浏览器引擎需要处理的内容包含两种：字节码（解释器生成）和机器码（JIT生成），这就好比是两片独立的大陆
-  引擎需要在这两片「大陆」之间穿梭，以便统筹所有需要的信息。为了组织其工作，引擎有两个文件夹来记录这些信息。（比如SpiderMonkey引擎中，这些文件夹叫做activations）
- 每次切换到不同的「大陆」，引擎都会启用新的文件夹，问题是，要启动一个文件夹必须通过C++调用，这就增加了成本。
-  再最初增加WebAssembly支持的时候，相当于是给引擎增加了第三块「大陆」
-  所以，WebAssembly调用js的过程就多了两个阻碍： 创建了不必要的文件夹和增加了C++调用成本
-  优化手段： 将JIT-JS「大陆」和WebAssembly「大陆」合并在一起。

二、如何优化JavaScript到WebAssembly调用？

- 即便在JIT编译之后，js和webassembly都使用机器码，但还存在一些问题。比如调用js动态类型。
-  wasm是静态类型，不希望付出运行时开销，当js获取的参数是动态的，也就是说，它把参数放到堆内存（装箱），wasm不知道该如何处理它
- 所以在传给wasm之前，引擎会将装箱的参数值取出来，存到寄存器里，要做到这一点，还是需要C++。即便不需要C++，但这个过程也是没法取消的。
- 优化手段： （使用C++ stub 代码）去掉了「中介（trampoline）」，当javascript调用webassembly时，装箱的值就被取出来放到了合适的位置，等待wasm直接使用。

三、更快的JavaScript到WebAssembly调用：单态（Monomorphic）调用

单态调用是指每次都调用相同的函数。在js中，存在每次都使用完全相同类型的参数来调用该函数。如果可以编写代码以便JavaScript始终将相同的类型传递给相同的WebAssembly导出函数，那么您的调用将非常快。实际上，这些调用比许多JavaScript到JavaScript调用要快。

在未来，可能会为JavaScript添加内联WebAssemly的支持。

四、如何优化WebAssembly到内置函数的调用

内置函数是浏览器提供的函数，如Math.random。有些内置函数是JS本身实现的，这种叫做自托管。有些是C++实现的，不同的浏览器引擎，有不同的实现，这方面并不统一。如果是自托管的内置函数，WebAssembly调用的时候就能用到前面说的优化，但是遇到C++实现的函数，则又回到了解放前。为此，专门为这样的内置函数构建了一条特殊的直达「通道」，允许在WebAssembly调用C++实现的内置函数时，使用该特殊通道，增加性能。

目前只支持Math相关的内置函数，这是因为WebAssembly只支持整数和浮点数。但是对于dom处理等内置函数来说则效果不佳，因此调用相关函数的时候必须使用js，这就是wasm-bindgen所要做的。

但是WebAssembly的一些实验性支持类型也会逐渐登录浏览器，一旦这些类型稳定，那么WebAssembly就可以不经过js直接调用那些内置函数了。所以当前的优化工作，在未来也会适用于其他内置函数。

但是仍然有一些内置的函数需要使用js，这些函数的问题由host-bindings来解决。

以上。

[原文](https://hacks.mozilla.org/2018/10/calls-between-javascript-and-webassembly-are-finally-fast-%F0%9F%8E%89/)

### Go的成功预示着Rust的成功

该文作者说，从理论的角度，其实很难理解Go语言现在的成功。Go语言设计的非常糟糕。一旦深入了解Go，你会发现，它应该是构建于上世纪80年代的语言，而非21世纪。然而，任何一个使用Go的人，都会告诉你，这是一门很好的语言。作者说，如果他被困在只有三门语言的小岛上面，他希望Go是其中之一。

Go为什么好？

Go有几个重要的功能掩盖了它的不好。

- go get工具轻松下载包
- 静态编译使得各种环境下移植代码比较容易
- 本机异步IO机制可以方便写出高性能网络代码
- 内置通道在go程之间轻松实现相对安全的数据
- 标准库和生态系统包含开发人员的可能需要的大多数库

说白了Go是专门为开源库、大规模并行和网络时代专门设计的语言。


反观Rust，正好是在Go完全失败的场景中拿下了高地

- Cargo胜过了go get
- Rust马上也会有高性能的异步并发
-  而且还支持天生线程安全的多线程编程，基于线程的通道
-  标准库和Go一样丰富
- Go和Rust之间的性能差距会越来越大 （当然是Rust快）

Go效应

- 一旦async/await合并到Rust中，我们可以称Rust是超越Go的优秀语言。Rust支持的并发方式非常全，还默认线程安全
-    Rust和Rust的编程概念越来越多地进入到现实世界，那么新手进入的壁垒会越来越低
-  Rust中的生命周期已经过了关键的时候。
-  Go受欢迎的程度飙升的原因也会适用于Rust，Rust的甜蜜点可能马上要到来了

等异步开发稳定之后，Rust社区会涌入大量的网络服务开发者，洪水将至。

[原文](https://medium.com/@george3d6/the-success-of-go-heralds-that-of-rust-73cb2e4c0500)

### 「Job」创建高性能的Rust系统「非远程」

待遇： 旧金山| 130 - 175K +股权| 基础设施和HPC | 全职/现场

基于机器视觉，自动结帐，构建零售业未来。从头设计系统，可以部署到数百万零售商，已经在旧金山开设了测试商店。

目前正在寻找Rust工程师。

多好的工作呀！！！

[原文](https://www.reddit.com/r/rust/comments/9ml170/job_create_high_performance_rust_systems/)

### Firefox Nightly现在可以选择使用Cranelift编译wasm

[cranelift](https://github.com/CraneStation/cranelift)是Rust实现的代码生成器

[原文](https://www.reddit.com/r/rust/comments/9mvnrk/in_firefox_nightly_an_option_has_arrived_to_use/)

### Rust 有静态的Garbage Collector

steveklabnik 写了这篇博文，主要是因为看到了类似这样的言论：

> 手动内存管理比GC要做更多的工作，当然这样是为了提高性能或使用更多底层资源的权衡。所以Rust在什么时候或在什么场景使用要取决于你自己。

steveklabnik认为这是对Rust 无GC的一种误解。

GC算法目前比较常用的有分代回收，比如Ruby里就用到分代垃圾回收技术。但是还有一种与GC相关的概念，那就是「逃逸分析（escape analysis）」，这是一种优化技术。简而言之，就是在函数内定义的对象，如果从不在函数外使用，则将其存在栈上，否则，如果函数外有使用，就将其放到堆上。比如Java，比如Golang，都使用了逃逸分析。Go在一定程度消除了堆和栈的区别，因为Go在编译的时候进行逃逸分析，来决定一个对象放栈上还是放堆上，不逃逸的对象放栈上，可能逃逸的放堆上。这也是Go没有采用分代GC的原因，因为逃逸分析的存在，分代GC也带不来多少优势。你可以把逃逸分析看成是分代GC的更强版本，任何放到堆上的对象，可以算是老一代对象。

目前，普通有个观点就是GC是运行时。既然人们把语言一分为二，动态和静态，那么为什么不能把GC也一分为二，分成动态和静态呢？ 在这方面C++做了很多的工作。Rust语言站在C++的基础上，继续开创静态GC这条路。

在某些方面，你可以把Rust借用检查看作是一种更强大的逃逸分析。就像是TypeScript里存在的渐进式类型（gradual typing）那样（TypeScript允许你同时使用动态类型和静态类型，有助于让学习者逐渐适应静态类型），你也可以把Rust里提供的这些内存自动化管理的功能看作是一种渐进式GC。

所以，Rust的GC，实际上是一种静态式GC，不要再以传统的观念来看待GC，也不要以传统的系统级语言看待Rust，以为Rust也是纯手动管理内存，实际上Rust的自动内存管理是非常先进的。

注： 以上并非翻译，只是对文章表达意思的总结，详细请看原文

[原文](https://words.steveklabnik.com/borrow-checking-escape-analysis-and-the-generational-hypothesis)

### 我想要一个2D图形crate

该文作者说，当前Rust的生态中有一个明显的缺失：跨平台的2D图形抽象库

该作者对比梳理了生态系统中现有的图形库，佐证了他的说法。通过此文也可以了解一下当前Rust 图形库的现状。

[原文](https://raphlinus.github.io/rust/graphics/2018/10/11/2d-graphics.html)

### trust-dns-proto Bug 通告

该库产生一个反序列化漏洞，可能导致栈溢出。已在新发布的版本中修复。

[原文](https://users.rust-lang.org/t/rustsec-advisory-for-trust-dns-proto-affecting-server-resolver-and-client/21179)

### debian unstable上可以使用fd-find 了

[原文](https://www.reddit.com/r/rust/comments/9n6xx8/fdfind_is_available_in_debian_unstable/)

### Rust文档化建议

作者对Rust文档化（在你编写自己的crate时）给出了一些建议：

- README 一定要写，不管多小的crate
- CONTRIBUTION要列出来，以便别人贡献
- 提出一个preRFC，提出增加features文档化配置的功能，这个感觉还是挺有意义的

[原文](https://phaazon.net/blog/rust-features-documentation)


---

# 博文与库


### 嵌入式Rust：开发蓝牙设备

该文作者在医疗设备领域工作，几乎完全是用C++。为了改变节奏，作者一直致力于学习Rust。并且在今年年初选择Rust进行嵌入式开发。

作者的核心项目是让两台无线设备通过蓝牙通信，因为没有比较标准的蓝牙相关crate，所以作者自己实现了两个crate，填补了蓝牙领域crate的空白：

-  [bluenrg](https://crates.io/crates/bluenrg)，是用于BlueNRG-MS设备的no-std Rust驱动。
- [bluetooth-hci](https://crates.io/crates/bluetooth-hci)，是一个实现了蓝牙规范的框架，默认支持4.1蓝牙版本，也具有4.2和5.0的功能。

本文介绍这两个库的用法。

[原文](https://219design.com/bluetooth-low-energy-with-rust/)

### 「嵌入式Rust」embedded-hal的mock测试库

无需访问硬件即可测试，Rust牛逼。

C++做嵌入式有这方面库吗？恕我孤陋寡闻

[embedded-hal-mock](https://github.com/dbrgn/embedded-hal-mock)


### RAII优于Haskell的bracket模式

本文比较了Haskell的bracket函数和RAII，并且阐述了bracket模式带来的问题（可能导致释放后还能使用的问题）和修复方式

[原文](https://www.snoyman.com/blog/2018/10/raii-better-than-bracket-pattern)

### 翻到一篇老文：使用Rust构建漂亮API接口的指南

虽然是旧文，但是内容还是值得学习

[原文](https://deterministic.space/elegant-apis-in-rust.html#more-method-name-conventions)

### 寻找Rust中的Bug

作者写了一个库，用来快速检查Rust标准库中数据结构的Bug。

https://github.com/blt/bughunt-rust

作者发现了什么bug吗？ 发现很多方法能在分配内存时发生恐慌。 Rust官方已经发了[RFC 2116](https://github.com/rust-lang/rfcs/blob/master/text/2116-alloc-me-maybe.md)准备进行修复。

[原文 ](https://blog.troutwine.us/2018/10/08/hunting-for-bugs-in-rust/)

### 氧化Python： 用Rust加速10倍

氧化（Oxidizing），目前在Rust社区是一个流行的词。

[原文](https://tech.blue-yonder.com/oxidizing-python-speeding-up-urlquoting-by-using-rust/)

### Heaptrack案例研究

作者基于Global Allocator尝试构建自己的内存分配器，

然而他在这个过程挖掘了一个工具：Heaptrack。可以用它来高亮内存的使用情况来帮助理解。

Heaptrack是linux下的一个工具（Mac和win可用），可以跟踪内存使用情况，并给出可视化报告。

本文就是介绍如何使用Heaptrack跟踪Rust代码中内存分配情况。

最终他得出结论：永远不要写自己的内存分配器，尽可能地使用heaptrack这种工具来优化代码吧。

[原文](https://speice.io/2018/10/case-study-optimization.html)

[heaptrack](https://github.com/KDE/heaptrack)

### 「视频」无恐惧多媒体编程

GStreamer是Linux世界中多媒体编程的首选框架，尤其用于嵌入式，但是传统都是C/C++来编写GStreamer的应用程序和插件。不幸的是，这种效率是以安全为代价的。即便是最熟练的C/C++开发人员也会在内存管理方面犯错，结果可能是灾难性的。线程安全是多媒体解决方案的核心，但是用C/C++极难实现。

在本次视频演讲中，Zeeshan介绍了使用GStreamer的Rust绑定如何安全、简单地编写多媒体应用程序。

[原文](https://media.ccc.de/v/ASG2018-172-fearless_multimedia_programming)

### 修复Clippy Crash的问题

作者分享了他修复一个Clippy Crash问题的经验

感兴趣可以看看，对于了解Clippy的机制也有一定帮助

[原文](https://phansch.net/2018/10/10/fixing-a-clippy-crash/)


### Bible.rs | 使用Actix Web和Diesel编写的在线《圣经》网站

并且开源，可以作为一个actix-web框架全栈学习案例

[bible.rs](https://bible.rs/)

[GitHub Bible.rs](https://github.com/DSpeckhals/bible.rs)


### 关于Rust中类型布局和ABI的笔记

[原文](https://gankro.github.io/blah/rust-layouts-and-abis/)


### ToDo MVC 的WASM版本

使用了wasm-bindgen

[原文](https://github.com/jonathanKingston/todomvc-wasm)


### ToDoMVC的命令行版本

[原文](https://medium.com/@devashishdxt/building-a-command-line-todo-app-in-rust-a89bb7af91c3)

### holyjit ： Rust实现的jit编译器

[holyjit](https://github.com/nbp/holyjit)
