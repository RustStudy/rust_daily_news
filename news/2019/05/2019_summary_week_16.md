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

2019-05-19

---

# 官方新闻

### 「官宣」Rust四周年

从2015年5月15日Rust 1.0版发布至今，取得了如下成绩：

- 连续四年StackOverflow的“最受喜爱的编程语言” （日报君感慨：近一年半看了很多社区内的各种动态，大家形容Rust用的最多的一个词是：Amazing，如果非要翻译成中文，我觉得「赞叹」比较好。）
- 稳定了嵌入式Rust
- rustc成为第一个专注于支持WASM的编译器
- Rust 2018 edition发布
- Crates.io通过了10亿次下载，拥有超过25,000个crate
- 现在全世界有超过100个聚会，分布在42个国家
- 6场新的大会在世界各地涌现（RustRush，RustCon Asia，Oxidize，Rust LATAM，Colorado Gold Rust，RustLab Italy）
- 很多大公司大平台陆续引入了Rust
- 也涌现了不少优秀的开源项目和产品

这个清单如果继续写下去会很长，Rust已经在众多领域陆续开花了。Rust社区感谢有你！

[Read More](https://blog.rust-lang.org/2019/05/15/4-Years-Of-Rust.html)

### 稳定cargo 离线模式 pr合并了

[Pr](https://github.com/rust-lang/cargo/pull/6934)

这个的意思是，以后可以指示 cargo 去本地找依赖包缓存。而不是每次都检查网络了。非常实用的进展。

等等稳定版的发布，到时有使用说明。

### Rust 1.34.2发布

该版本属于紧急发布，主要是修复CVE-2019-12083的安全问题：

Error类型提供了一个向下转换函数(downcast_ref)，可以将指定的类型转换为`Error::type_id`对应的类型。 问题是，如果你自己的类型实现Error，并让type_id返回与实际类型不同的东西。然后，当有人在你的类型上调用downcast时，它将完全通过安全代码转换为你想要的任何东西。

在Safe Rust中出现内存不安全的问题是无法容忍的。新版本的发布主要是将已经稳定的`Error::type_id`紧急改为了Unstable。后续再慢慢考虑`Error::type_id`的重新稳定化方案。

- [CVE-2019-12083](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-12083)
- [Read More](https://blog.rust-lang.org/2019/05/14/Rust-1.34.2.html)
- [安全问题演示demo：Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=90a7a46a265ac5cf46d51bd71d2e98e6)

### wasm-bindgen v0.2.44 开始支持futures 0.3和async/await

wasm 已經可以使用 async了，快看看怎麼用吧

- [Reddit讨论](https://www.reddit.com/r/rust/comments/bphw68/wasmbindgen_v0244_has_support_for_futures_03_and/)
- [Read More](https://github.com/rustwasm/wasm-bindgen/blob/master/CHANGELOG.md#0244)

### rust 1.36.0 加入了Cargo流水線加速編譯技術

利用下圖的方式加速

```
[-libA----|--------]
          [-libB----|--------]
                             [-binary-----------]
0s        5s       10s       15s                25s
```

Cargo在内部构建了一个DAG来表示crate之间的依赖关系，通常需要等待crate依赖关系构建完成才会启动编译，但是现在对Cargo进行了优化，现在只要得到编译器为了开始下一次编译而生成的“元数据”即可开始编译，这就是所谓的「流水线（pipeline，就像工厂流水线一样，节省加工时间）」加速编译技术。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/bpucwt/evaluating_pipelined_rustc_compilation/)
- [Read More](https://internals.rust-lang.org/t/evaluating-pipelined-rustc-compilation/10199)

### 安全代码工作组正在为创建可重现的构建和构建时间沙盒化而努力 

[Rust 安全代码工作组](https://github.com/rust-secure-code/wg)

[cargo-repro](https://github.com/rust-secure-code/cargo-repro)

[cargo-sandbox](https://github.com/rust-secure-code/cargo-sandbox)

相关博文：[「警惕」存在于crate中的安全风险](https://zhuanlan.zhihu.com/p/64586315)

---

# 社区新闻

### 「论文」从理论到系统：编程语言教育的基础方法

来自斯坦福大学CS242课程的设计者Will Crichton发表的关于将Rust和WebAssembly应用于编程教育的论文。

国内外高等学府，比如清华大学、斯坦福都在陆续开始将Rust应用于学生的编程教育中，国内最早教授Rust的大学没记错的话，还有新兴的上海科技大学。为什么呢？因为他们看到了Rust的教育价值。

- [CS242](http://cs242.stanford.edu/f18/)
- [论文pdf](https://arxiv.org/pdf/1904.06750)

### Rust 职位：东京，机器人创业公司

用 Rust 开发机器人？好诱惑。 这个职位在日本可能比较新鲜，创业公司，（估计）没有祖传代码，并且可以帮助办理签证，英语交流也可以，不强制要求日语。感兴趣的可以看看。

![img](https://user-images.githubusercontent.com/27893/57704364-53856800-7694-11e9-9c1c-5d69a7864868.png)

[job link](https://www.linkedin.com/jobs/cap/view/1162802661)

### 使用 Rust 來加速 Elixir 服務1100萬用戶

discord過去一直使用erlang來做為主要服務的程式語言現在他們使用了rustler讓rust為 elixir 加速。去年他们的业务有了一项新的变化，就是更新会员列表的方式：只发送更新的部分，而不是给每个会员发送更新。这个变化给服务器端造成了一个大问题：我们需要一个能够容纳数十万个条目的数据结构，以特定的方式排序，可以接受和处理大量的变动，并且可以报告添加和删除事物的位置索引。

Elixir是一种函数式语言;它的数据结构是不可变的。这非常适合在编写elixir对代码进行正确性论证以及并可以享受的大量并发性。不可变数据结构的双刃剑是通过采用现有数据结构和操作以及创建全新数据结构来对变动建模。这意味着当有人加入服务器（内部称为公会）并拥有100,000名成员的成员列表时，他们必须构建一个包含100,001名成员的新列表。 BEAM VM非常快，并且每天都在变得更快。它试图在可能的情况下利用持久性数据结构，但在我们运营的规模上，这些大型列表无法足够快地更新。

因此团队在erlang和elixir提供的各种高性能数据结构中寻找解决办法，试过MapSet、List、OrderedSet到他们自己实现的SortedSet，终于找到了一个解决方案，但是在超过250,000名成员的公会时性能达到了上限。于是Discord团队准备尝试使用Rust来加速。

这不是Discord团队第一次使用Rust了，在他们的游戏商城里也大量用到了Rust，但Discord的核心服务是Elixir，主要是因为Elixir比较适合他们的场景。为了为Elixir加速，他们预留了一周时间使用Rustler（提供了安全的NIF绑定，方便为elixir编写rust扩展）进行概念验证，最终发现，Rust支持的NIF提供了巨大的性能优势，而无需牺牲易用性或内存（可以支持100w名成员）。

从此以后，Discord团队便快乐地享受着Rust带给他们的「快感」。Discord也开源了他们的SortedSet库，点击原文看更多详细。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/bpw1aw/using_rust_to_scale_elixir_for_11_million/)
- [Read More](https://blog.discordapp.com/using-rust-to-scale-elixir-for-11-million-concurrent-users-c6f19fc029d3?gi=f41b3f0ac2b3)
- [rustler](https://github.com/rusterlium/rustler)

### 使用BinaryAST快速加载脚本

关于JavaScript二进制AST

随着网站变得越来越复杂，JavaScript源代码的数量不断增加。依赖于大型JavaScript代码库会导致网站启动缓慢 - 通常速度慢得令人无法接受。这是因为存在两个瓶颈：解析和字节码编译JavaScript。不幸的是，浏览器几乎达到了两种操作的效率峰值。

我们（Mozilla，Bloomberg，Facebook，CloudFlare）目前正致力于针对JavaScript的特定领域编码，称为“BinAST”（“JavaScript二进制AST”的缩写）。 JavaScript二进制AST旨在打破瓶颈。当前的高级原型已经在所有最常见的框架上显示了JS解析改进了30％-50％，只需更改格式，我们相信我们可以进一步提高这一改进。编码可以构建为webdev工具链的一部分，或者由代理或CDN注入，因此可以在不更改原始网站的情况下自动提高最终用户的性能。

此编码目前在JavaScript TC39标准化过程中。它可以与现有的压缩技术（gzip，brotli等）一起使用，目前有cloudflare的一个Rust实现：binjs-ref。

- [binjs-ref](https://github.com/binast/binjs-ref)
- [Read More](https://blog.cloudflare.com/binary-ast/)

### 「视频」ChromeOS使用Rust为在Chromebook上运行的Linux应用构建安全的Linux环境

本演讲视频将解释Linux for Chromebooks的架构以及使其易于使用的设计决策，包括使用Rust来构建安全的Linux沙箱环境。

[Reddit 讨论](https://www.reddit.com/r/rust/comments/bokgxr/chromeos_uses_rust_to_build_a_secure_linux/)

### Rust之父Graydon Hoare 在一个讲座中叙述了编译器的历史

是给不列颠哥伦比亚大学（在加拿大）的学生开的讲座。

分别讲了 clang, swiftc, rustc, 和 gcc 等。文章有意思。

[Slide](http://venge.net/graydon/talks/CompilerTalk-2019.pdf) 在这里。

[Repo](https://thenewstack.io/rust-creator-graydon-hoare-recounts-the-history-of-compilers/)


### 一个查看编程语言发展趋势的方法：Wikipedia Pageviews Analysis

![img](https://user-images.githubusercontent.com/27893/57599178-732f6a00-7588-11e9-9b15-9c14f3790d2e.png)
![img](https://s2.ax1x.com/2019/05/13/E4xExx.md.png)

[Read More](https://tools.wmflabs.org/pageviews/?project=en.wikipedia.org&platform=all-access&agent=user&start=2015-07&end=2019-04&pages=Rust_(programming_language)|Python|C%2B%2B|C%2B%2B11|C%2B%2B20|C)

### 「视频」Rust：后40年的语言

[Read More](https://www.youtube.com/watch?v=A3AdN7U24iU)

### Are we await!(yet)? 

有人做了个网页，可以实时关注async/await的动态，特别是await。

[Read More](https://await.pietroalbini.org/)

### Rust在demoscene圈子中开始流行

demoscene是一个国际计算机艺术亚文化，专注于制作演示：自成一体，有时甚至是极小的计算机程序，产生视听演示。 演示的目的是展示编程，视觉艺术和音乐技巧。 演示和其他demoscene制作在称为demoparties的节日上分享，由参加者投票并在线发布。

引用一段文章里提到的谈话：

> It's a pretty good language. Better than C++, worse than C... Fucking awful learning curve, at least for me, but I'm having tons of fun with it, it kind of forces you to write better code, instead of crappy single use code.

看来Rust很受这群极客的喜爱。

[Read More](http://www.pouet.net/topic.php?which=11664)

### Manticore: 一个用Rust编写的研究性操作系统

旨在探索parakernel OS架构。

> 对于希望最大限度利用硬件的服务器应用程序而言，操作系统越来越成为瓶颈。当I/O明显慢于CPU时，设计了许多传统的内核接口（例如POSIX）。但是，今天I/O变得越来越快，但单线程CPU性能却停滞不前。例如，40 GbE NIC可以比CPU访问其最后一级缓存（LLC）更快地接收缓存行大小的数据包，这使得操作系统跟上来自网络的数据包变得棘手。类似地，非易失性存储器（NVM）访问速度越来越接近DRAM速度，这对存储器的OS抽象提出了挑战。

> 为解决此操作系统瓶颈，服务器应用程序越来越多地采用内核旁路技术。例如，Seastar框架是在用户空间中实现的操作系统，它实现了自己的CPU和I/O调度程序，并尽可能地绕过Linux内核。 Parakernel是一种OS体系结构，它消除了许多操作系统抽象（类似于exokernel）并分区硬件资源（类似于多内核），以便通过增加应用程序级并行性和可预测的尾部延迟来促进高性能服务器应用程序。

[manticore](https://github.com/manticoreos/manticore)

### 一个视频，从 Node.js 到 Deno(v8+Rust)

Deno 是一个 JavaScript/TypeScript 运行时，作者其实就是 Node.js 作者。他觉得 nodejs 生态已经没办法再提高质量了，就创建了这个新项目 deno。下面是 Rafał Pocztarski 的视频分享。

- [Video](https://www.reddit.com/r/rust/comments/bo0zk0/rafa%C5%82_pocztarski_from_nodejs_to_deno/)
- [Slides](https://gitpitch.com/rsp/ntd/ntd#/)

### 一篇博文：Rust语言目前在机器学习领域的状态

这篇文章作者非常喜欢 Rust，分析了一下目前 Rust 中的机器学习生态的情况。比如：

- [const-generics](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md)
- [generic-array](https://crates.io/crates/generic-array)
- [packed_simd](https://github.com/rust-lang-nursery/packed_simd)
- [RustaCUDA](https://github.com/bheisler/RustaCUDA)
- [rsmpi](https://github.com/bsteinb/rsmpi)
- [rayon](https://github.com/rayon-rs/rayon)
- [ndarray](https://github.com/rust-ndarray/ndarray)
- [ndarray-linalg](https://crates.io/crates/ndarray-linalg)
- [ndarray-stats](https://crates.io/crates/ndarray-stats)

最后，作者打赌 Rust 在 ML/DL 领域能大展宏图。进一步的讨论可以进 [rust-ml](https://github.com/rust-ml) 进行。

[Read More](https://ehsanmkermani.com/2019/05/13/state-of-machine-learning-in-rust/)

### 为什么越来越多的知名项目用Rust来开发？

社区@Mike写的一篇文章，侧重分享了为什么Rust适合区块链开发。

[Read More](https://mp.weixin.qq.com/s/DjQlyQeushrXM7QNxbY-cA)

### 「比原生更快：在 Linux 内核中运行 WebAssembly」

[中文](https://mp.weixin.qq.com/s/F6yAE3-l_LI8l1ls5yLHqw)
[英文](https://medium.com/wasmer/running-webassembly-on-the-kernel-8e04761f1d8e)


---

# 学习资源

### 「中文」Rust Async: 标准库futures api解析 

[Read More](https://zhuanlan.zhihu.com/p/66028983)

### 如何使用纯Rust实现命令行自动补全

命令行自动补全可深可浅，深的可以对子命令，参数项等，都自动补全。文章可以借鉴。

[Read More](https://www.joshmcguigan.com/blog/shell-completions-pure-rust/)

### 「系列」Rust for OOP系列介绍

这位博主打算写一系列主题是Rust for OOP的文章，主要是针对有一定OOP语言开发经验的人来学习。可以先关注下。

[Read More](https://oribenshir.github.io/afternoon_rusting/blog/rust-for-oop)

### 在Rust中创建C/C++ API

这篇文章介绍了一些可以帮助自动生成C/C++ API的优秀工具。比如：

- bindgen， 可以根据给定的头文件自动创建Rust绑定代码。对C语言比较友好，但是对C++来说，用途有限。因为C++的继承处理比较麻烦。
- cbindgen，可以方便地为Rust项目生成C API。它还支持以C++风格输出数据类型和模板等。
- cpp，这是一个Rust crate，可以帮忙编写C++ API。它提供了一个`cpp!`宏，可以在其中嵌入c++代码。它可以方便和cbindgen搭配使用。

作者还总结了一些在Rust中创建C/C++API的准则：核心逻辑和FFI层之间应该明确分离，最好把FFI代码置于一个单独的crate中，这样做的好处是设计Rust API不会受到FFI的太多影响。

[Read More](https://karroffel.gitlab.io/post/2019-05-15-rust/)

### [教程]如何用rust为redis写一个client

主要讲解了如何通过RESP实现一个redis client，并用rust实现了一个简单的demo，目前只实现了set和get命令，可以很方便的添加命令，项目地址如下redis-simple-rs欢迎大家完善。

@readlnh 投稿 

[Repo](https://github.com/readlnh/redis-simple-rs)

### 「小技巧」利用`Option<T>`和From实现可选参数

[Read More](http://blog.keiruaprod.fr/2019/05/11/optional-parameters-in-rust/)


## 一个很有价值的问题：Rust中哪些特性是零开销抽象的

[link](https://www.reddit.com/r/rust/comments/bo13qq/what_specifically_are_all_the_zerocost/) 在这里讨论的，现在我来整理一下，下面的都是零开销的抽象：

- tuple
- gererics
- traits
- Option - 编译器最后（视情况）会把这一层包装优化掉
- Vec
- Box
- Range
- for-loops
- mod
- zero-sized types (C++ can't do that because every value needs to have an address)
- enum discriminant optimizations which I hope are done for Option<NonZeroI8> and friends (storing None as 0)
- 链式迭代器可以产生更快的代码，有时比for循环还快
- await和Futures的实现估计也会比C++的实现消耗更少的内存分配，await不是零开销的，但是会保持很少
- 宏、构建脚本和常量初始化可以输出结构化的值，也是零开销
- ...

不是零开销的部分：

- &dyn Trait
- ..

有人总结得好：

**零开销不是指没有开销，而是指与不用（Rust给出的）抽象而用手动直接模拟实现相比，没有额外的开销。**

通常来讲：当 Rust 有一个特性 F，它实现了一个编程的方面（解决了那样一种问题） A，现在你的程序要实现方面 A（解决那样一种问题），一般来说，只需要直接拿起 F 使用就对了，你手动重新实现（用 Rust 或 C 或其它语言），并不能带来更好的性能。

**C++的实现遵从零开销原则：你用不到的东西，不会为其付出代价。更进一步：对于你用到的东西，你没法再做得更好。**

对于Rust的情况来说，编译器会承担大部分的优化工作，所以在这方面（相对于C++来说）走得更远。换句话说，**实践中往往更容易写出慢的C++代码，而不是慢的Rust代码**。对于你描述的情况，元组慢是因为它们实现在编译器的上面一层，因此优化工作留给了程序员来做。而在Rust中，元组是一等公民，它们会被编译器自动优化掉。

### 零成本抽象

官方核心团队无船同志的新博文，探讨了「零成本抽象」。

零成本抽象在C++跟Rust是一個很重要的概念

簡單來說就是：不希望有很大很重的runtime，並且可以在編譯時被優化。

作者覺得 rust 有幾個很棒的 零成本抽象

1. 所有權、借用

保證内存的正確使用

2. 迭代器、閉包函數

可以輕鬆的串接 map, filter 等函數做處理

3. await 异步函數

當前的await語法雖然還沒有確定，但使用pinning 做到零成本抽象是確定的

4. Unsafe 函數、模块邊界

由於rust的語法複雜性，有很多實作會需要Unsafe的底層實作

這些Unsafe函數實作了零成本抽象的底層

讓我們在上層能安全的使用這些模块

另外无船同志还表示：trait对象目前不是零成本抽象，他想花点时间（至少需要18个月）去研究这个问题，然而总是有更优先的事情。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/bpep6h/zero_cost_abstractions/)
- [Read More](https://boats.gitlab.io/blog/post/zero-cost-abstractions/)

### 「讨论」mio异步計時器如何使用？

[官方說明文件](https://docs.rs/mio-extras/2.0.5/mio_extras/timer/index.html)

沒有範例程式碼，其實很難了解怎麼用

回覆中有人給出了答案

[Read more](https://www.reddit.com/r/rust/comments/bpbrux/could_someone_explain_to_me_how_mio_asynchronous/)

### 开发Rust的最佳IDE是什么？

本文是网络里的各种意见汇总，尝试阐明利弊，供大家参考：

- IntelliJ IDEA 中使用IntelliJ Rust插件。褒贬各一。
- CLion中使用Rust插件。 官方评论：可能是最接近「Rust专用IDE」的IDE。
- VSCode中使用Rust插件。好评比例比较多。
- (Neo)vim使用Rust插件。喜欢VIM的人用吧，也够用了。
- Sublime。 抱怨比例比较多。
- Atom。没啥评论。

你推荐哪个？

[Read More](https://medium.com/cloud-native-the-gathering/whats-the-best-ide-for-developing-in-rust-5087d46006f5)

### cerebrallib - 使用Rust写的brainfuck语言的虚拟机库

练手项目

```
// src/main.rs code
use cerebrallib::cerebral;
use std::io
fn main() {
    let code = String::from("++++");
    let mut vm = cerebral::CerebralVM::new(code, io::stdin(), io::stdout());
    vm.execute();
}
```

[Repo](https://github.com/dsouzadyn/cerebrallib)

---

# 项目、工具与库

### zemeroth - 一个六边形回合制游戏

可以在线玩儿：[Online Play](https://ozkriff.itch.io/zemeroth)。

这篇文章详细讲述了这个游戏的技术选型发展过程。目前，它综合使用了：ggez, WASM, itch.io, visuals, AI, campaign, tests 等技术。文章写得非常好，强烈推荐阅读。

[zemeroth源码](https://github.com/ozkriff/zemeroth)

### Couchbase Rust SDK 1.0 alpha.1 发布

这是官方的 Rust SDK。Couchbase 是一个商业的 NOSQL 数据库。

[Repo](https://github.com/couchbaselabs/couchbase-rs)

### ggez制作的小游戏：ggezFlappyCrabby

- [视频](https://www.youtube.com/watch?v=cPF41tl3-3c&feature=youtu.be)
- [ggezFlappyCrabby](https://github.com/AndrewJakubowicz/ggezFlappyCrabby)

### Jazz：又一个用Rust实现的编程语言

（可能是个人玩票的语言）

特点是使用了GCCJIT的静态语言，也就是libgccjit库，它提供了C接口，社区里也有它的Rust绑定库。

[Jazz](https://github.com/jazz-lang/Jazz)

### google-apis-rs: 适用于所有Google API的绑定和CLI生成器

这个项目的gen目录下包含了很多相关组件

[google-apis-rs](https://github.com/Byron/google-apis-rs)

### pegcel: syn风格的PEG解析器生成器

可以创建syn风格的语法树，配合syn库使用。

[pegcel](https://github.com/CAD97/pegcel)

### wgpu-rs: wgpu的进一步封装

适用于Rust社区的通用图形和计算需求，未来还会支持wasm和emscripten。wgpu是WebGPU的Rust实现，基于`gfx-hal`。

- [wgpu-rs](https://github.com/gfx-rs/wgpu-rs)
- [wgpu](https://github.com/gfx-rs/wgpu)

### multiqueue2 - 支持广播能力的 mpmc 管道

[Repo](https://github.com/abbychau/multiqueue)

### 使用gir crate生成GNOME库

gir是一个用于为基于glib的库生成Rust绑定和用户API的工具，可以生成sys级的crate和安全API。

[Read More](https://gtk-rs.org/docs-src/tutorial/gir_tutorial)

### cloud-hypervisor： 一个在KVM上运行的开源虚拟机监视器（VMM）

intel又一个开源项目，目前还是实验性项目，基于rust-vmm实现。

[cloud-hypervisor](https://github.com/intel/cloud-hypervisor)

### meta: 用于解析人类可读性文本的DSL解析库

可用于语言设计，自定义格式和数据驱动开发。该库提供了一种叫做「meta语言」的规则，用来告诉程序如何阅读要解析的文档。这些文档都是人类可读格式的。

最小化示例：

```rust
use piston_meta::*;

fn main() {
    let text = r#"hi James!"#;
    let rules = r#"
        1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
        2 document = say_hi
    "#;
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = syntax_errstr(rules).unwrap();
    let mut data = vec![];
    parse_errstr(&rules, text, &mut data);
    json::print(&data);
}
```

输出：` "james":true `

[meta](https://github.com/pistondevelopers/meta)

### 「嵌入式Rust」一个通用型嵌入式芯片烧录软件

目前还只支持windows和st-link，长远目标是代替OpenOCD。目前比竞品好的地方是可以同时烧两个芯片，这样做双机通讯测试就不用插拔dongle了

来自 @洛佳

[Read more](https://github.com/luojia65/nihao)

### serde-wasm-bindgen：让wasm-bindgen支持serde

cloudflare出品

[serde-wasm-bindgen](https://github.com/cloudflare/serde-wasm-bindgen)

### 【嵌入式】为ESP32构建Rust开发环境

文章在[这里](http://quickhack.net/nom/blog/2019-05-14-build-rust-environment-for-esp32.html)，不复杂。

### diesel-factories - 为测试要构建 factory_bot 的库

在单元测试/集成测试中，你经常会需要插入一些数据到数据库中。而 [factory_bot](https://github.com/thoughtbot/factory_bot) 就是这样一种库，diesel-factories  是它基于 diesel 的实现。 如果你来自Ruby社区，肯定听说过factory_girl，理念相似。

这个库像下面一样使用。

```
// A normal Diesel model
#[derive(Clone, Queryable)]
struct Country {
    pub id: i32,
    pub name: String,
}

// Our factory
#[derive(Clone, Factory)]
#[factory(model = "Country", table = "crate::schema::countries")]
struct CountryFactory {
    pub name: String,
}

// Setting up what the default values are
impl Default for CountryFactory {
    fn default() -> Self {
        Self {
            name: "Denmark".into(),
        }
    }
}

#[test]
fn some_test() {
    let con = establish_connection();

    // Using all the defaults
    let denmark = CountryFactory::default().insert(&con);
    assert_eq!("Denmark", denmark.name);

    // Defaults can be changed through builder methods
    let netherlands = CountryFactory::default()
        .name("Netherlands")
        .insert(&con);
    assert_eq!("Netherlands", netherlands.name);
}
```

[Repo](https://github.com/davidpdrsn/diesel-factories)

### memory-profiler - Nokia 用 Rust 写了一个 Linux 内存调优工具

内存调优工具，主要用来分析内存泄漏什么的。当然，还有很多其它更详细的特性。我们来先睹为快。

![img](https://raw.githubusercontent.com/nokia/memory-profiler/master/screenshot_gui_graphs.png)

![img](https://raw.githubusercontent.com/nokia/memory-profiler/master/screenshot_gui_allocations.png)

[Repo](https://github.com/nokia/memory-profiler)