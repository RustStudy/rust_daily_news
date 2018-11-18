前言：
从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。
2018-11-18

# 官方新闻

### 「官方」await语法方面的进展

[Read More](https://zhuanlan.zhihu.com/p/49440981)

### 「嵌入式工作组」嵌入式Rust年度回顾

[Read More](https://zhuanlan.zhihu.com/p/49921783)

### 「官方」如何检测Unsafe代码中的UB

ralfj比较高产，他负责Unsafe下内存模型相关的工作，目的是用miri来检测unsafe中的UB行为。

该模型用于定义在unsafe内存模型中允许哪种别名。建立合理的别名规则，才能基于miri来检查unsafe下的UB行为。

该模型的核心思想是： 对于一个内存位置，逐步建立可跟踪的引用，形成一个栈结构。比如有一个&mut i32，可以对其重新借用获得一个新引用。这个新引用是必须用于此位置的引用，建立在旧引用之上。当新引用过期的时候，旧引用会被激活，就好像是栈结构push和pop。

在今天这篇文章中，ralfj写了实现上述模型的进展。

在Safe Rust中，通常有借用检查来保护内存。但是在编写Unsafe代码的时候，借用检查就无法提供帮助了。所以，Rust核心团队就必须要定义一组规则，即使对于Unsafe代码来说也是非常有意义的。在本篇文章中，ralfj会再次解释这些规则，但是和上次的有所不同，因为ralfj在这三个月对栈式借用实现的过程中，对这些规则理解更深了

- [前篇：栈式借用模型]( https://www.ralfj.de/blog/2018/08/07/stacked-borrows.html)
- [关于别名规则issues]( https://github.com/nikomatsakis/rust-memory-model/issues/26)
- [Read More](https://www.ralfj.de/blog/2018/11/16/stacked-borrows-implementation.html)

---

# 社区新闻

### Rust与游戏开发

[Read More](https://zhuanlan.zhihu.com/p/50066484)

### 「安全呼吁」互联网存在一个巨大的C/C++安全隐患

[Read More](https://zhuanlan.zhihu.com/p/50069980)

### 「演讲」平台价值观、Rust以及对系统软件的启示

[Read More](https://zhuanlan.zhihu.com/p/50209719)

### GitHub上Rust颜色变更事件

之前有人修改了Rust的颜色，并给出了修改的理由：

>The Rust color on GitHub doesn't look much like rust, and I don't see where it could have come from (not on the website or part of the logo). In fact, it looks rather pale and sickly, rather than vibrant and robust.
> #a62c00 is much nicer. It resembles the red paints that have traditionally been made with iron oxide.

作者认为，新颜色是正宗氧化铁的颜色，应该改成这个。其实我比较喜欢新颜色，像「血液」的颜色，也可以赋予其象征意义：「热血」、「新鲜血液」

[肇事者PR](https://github.com/github/linguist/pull/4319)

但是引起了Rust社区的强烈反响，之后linguist项目的维护者不得不将颜色又改了回去，还向社区道了歉。

所以，如果你在GitHub你要是搜Rust项目，发现存在两种颜色的时候，不要惊讶。

### 「RustRush 2018」莫斯科举办的国际Rust大会

看来战斗民族也挺喜欢Rust。最近国际上的其他Rust大会：

- Rome at @RustFest Nov 23-28
- Cologne at @RustCologne Dec 5

[rustrush](https://www.papercall.io/rustrush)

[Read More](https://www.reddit.com/r/rust/comments/9we3oe/rustrush_2018_the_rust_conference_in_moscow/)

### Rust如何帮助Kentik提升性能

美国的网络大数据分析公司Kentik使用Rust优化了网络分析平台的性能。该公司软件工程师声称：

> Rust是Kentik每秒摄取来自互联网百万分之一的流量（十亿比特(Gigabits)/千万亿(Petabit/s)）的重要部分，并且每天存储100TB的网络流数据。我们平台的性能和可靠性依赖于Rust编写的软件，我们受益于crates.io上可用的强大的开源库生态系统。

在网络数据解析方面，使用nom，实现了高性能的零Copy解析器，对DHCP、DNS和HTTP等数据进行了高性能的解码。在数据存储方面，使用Rust编写了新的后端磁盘存储格式，可提供更高的性能和存储密度。在查询层也使用Rust编写的HyperLogLog扩展。Rust的性能和低内存使用是这里的一个关键。

这些组件和C-API一起分发，并链接到了分布式存储引擎的各个部分。Kentik大多数后端都是用Go来编写，但是Go写的共享库很大，包含了完整运行时和GC等。而且Go在运行时初始化会产生很多线程，在链接到静态二进制文件还会导致运行时的段错误。所以他们也为此增加了很多补丁，但这并不是对Go的攻击，反而，他们对Go还算满意。相比于Go，Rust更容易动态和静态链接，因为没有运行时和GC，也没有从C调用Rust函数的开销。

所以，你如果对Rust感兴趣，赶紧试试吧。

[Read More](https://www.kentik.com/blog/under-the-hood-how-rust-helps-keep-kentik's-performance-on-high/)

### 如何跟踪async/await进展状态

这篇reddit帖子评论区罗列出了async/await相关的issues、pr等，可以关注async/await的进展

[Reddit讨论](https://www.reddit.com/r/rust/comments/9wrtgs/asyncawait_status_and_tracking/)

### 「Cpp 2018」在Clang中实现C++ Core Guidelines生命周期安全

对CPP熟悉的朋友可以关注下，也许通过该视频可以对Rust的生命周期有更深入的理解。

[youtube](https://www.youtube.com/watch?v=sjnp3P9x5jA)

### 探索用Rust实现程序合成的可能性

该文作者受[程序合成是可能的](https://www.cs.cornell.edu/~asampson/blog/minisynth.html)这篇文章影响，实现了一个Rust版本。

程序合成是指按特定的规则自动生成程序。有一个设想，就是在未来没有程序员这一行业，有的只是设计师。设计师设计好功能，交给电脑，自动合成程序。 本文就是一种尝试。

现在有基于机器学习人工智能的程序合成引擎，但大家放心，还没到足以能让程序员失业的那一天。

[Read More](http://fitzgeraldnick.com/2018/11/15/program-synthesis-is-possible-in-rust.html)

[相关论文](https://people.csail.mit.edu/asolar/manual.pdf)

[代码实现](https://github.com/fitzgen/minisynth-rs)

### 「撕逼贴」Rust、Debian和librsvg

在LWN社区发布的一篇新闻。Debian支持许多（CPU）架构，即使对于那些没有正式支持的架构，也有Debian Port试图填补空白。对于大多数应用程序来说，主要是为相关架构启动并运行GCC，然后构建Debian提供的所有不同软件包。但是对于需要使用Rust构建的LLVM应用程序或库构建的软件包，则变得更加复杂。最近librsvg就引起一场风波。

[Read More](https://lwn.net/SubscriberLink/771355/1c4ca5254f22dbdf/)

### Gutenberg改名为Zola并发布0.5版

Rust实现的静态网站生成工具Gutenberg改成了Zola（之前的名字确实太长）。改名的原因是和Wrodpress的某个工具重名了。新名字Zola来源于 Emile Zola • 【埃米尔·左拉（法国作家）。

Zola名言：「生活的道路一旦选定了，就要勇敢的走下去，绝不走回头路。」

[Read More](https://www.vincentprouillet.com/blog/releasing-zola-0-5-0/)

### Cranelift： 一个rustc的新编译后端

当前还属于实验阶段。

[cranelift](https://github.com/bjorn3/rustc_codegen_cranelift)

---

# 学习资源

### 「油管」Rust中的Futures和async/await

该视频长约4小时，详细介绍了Future和async/await，包括tokio等内容。

[Read More](https://www.youtube.com/watch?v=9_3krAQtD2k)

### 「算法」探索航线谜题

在这系列文章里，作者探索了一道算法题：航线谜题（shipping puzzle）

第二篇是Rust实现方案。

航线谜题：

给定一组航段（leg）集合，每条航段分别对应于一周内的一天，包括起点和终点：

```
M PDX SEA （周一 PDX SEA）
T PDX SFO （周二 PDX SFO）
T SEA DEN （周二 SEA DEN）
W DEN PDX （周三 DEN PDX）
R PDX DEN （周四 PDX DEN）
F DEN JFK （周五 DEN JFK）
```

如何将以上航段划分为单个飞机的航线，受以下条件限制：

- 一条航段的目的地必须与下一条航段的起点相匹配
- 飞机必须持续航行（比如，一条航线不能包括：星期一 西雅图 -> 波特兰，然后是星期三 波特兰 -> 塔科马，此航线飞机在周二是闲置的。）

满足这些限制的简单方案是，将每条航段看作是一条单独的航线，但是需要多架飞机，但是一架飞机可以连续飞行的航段也可以看作是一条航线。现在问题是，如何找到最少的路线，也就是找到最少的飞机数。在此例中，我们最少需要两条航线。一种可能的解决方案是：让一架飞机飞行PDX、SEA、DEN、PDX、DEN、JFK航线，并且第二架飞机只在周二从PDX飞到SFO。

这里有一个[1w条航段的数据集](https://gist.github.com/lynaghk/0c75252b455e744ed3a8a0d09b493223)。可以开干了。

- [part 1](https://kevinlynagh.com/notes/shipping-puzzle/)
- [part 2](https://kevinlynagh.com/notes/shipping-puzzle/part-2/)

### 「长文」那些Rust不允许你干的事

本文罗列了Rust中可变系统和借用检查不允许开发者做的事情，并讨论了当前行为的理由，以及可能的解决方法。

[Read More](https://medium.com/@GolDDranks/things-rust-doesnt-let-you-do-draft-f596a3c740a5)

### Rust Flow: 数据流与Rust中方法调用链

本文探讨了Rust对实现方法调用链的支持。

观点：Rust强烈建议使用方法链来简明地表达意图和程序流，提供了能够通过使用方法调用而不是分支和循环结构来产生控制流效果的类型和方法，这些可以用来使代码更易于阅读和修改。

[Read More](https://myrrlyn.net/blog/misc/rust-flow)

### 如何使用`-Zprint-type-sizes`获取Rust类型大小

在编译时使用`-Zprint-type-sizes`可以得到比`std::mem::size_of`方法更加详细的信息。

[Read More](https://blog.mozilla.org/nnethercote/2018/11/09/how-to-get-the-size-of-rust-types-with-zprint-type-sizes/)

### 「大学课程CS242」斯坦福的Rust课程: V2版

斯坦福CS242编程语言课程升级到V2版，增加了新的内容。除了Rust，还涵盖了WebAssembly。

在官网中有详细的课程表和slides、笔记下载。

[Read More](http://cs242.stanford.edu/)

[Reddit](https://www.reddit.com/r/rust/comments/9wlcis/rust_in_stanfords_pl_course_v2_webassembly/)

### 开始使用rs-pbrt

rs_pbrt是对《Physically Based Rendering,PBRT(光线跟踪：基于物理的渲染) 》这本书中代码的Rust实现。

这篇文章是一个使用rs_pbrt的简单教程。

[Read More](https://www.rs-pbrt.org/blog/2018-11-16-getting-started/)

[rs_pbrt](https://github.com/wahn/rs_pbrt)

### 「系列文章」Pest vs Nom

该作者正在编写glsl方面的库，他之前使用了nom，现在又引入了pest。然后通过写这一系列文章，来比较这两者在实践应用中的区别和定位。

Pest:

Pest在编译时使用一个文件，其中包含定义要解析的输入格式的PEG语法。PEG, 表示解析表达式语法，是一种形式语言，使你能够用规则描述自己的语言。这些规则是用一些属于语言理论的基本块编写的。如果你曾经听说过Stephen Kleene及其着名的Kleene star，你就会对PEG感到熟悉。

作者喜欢PEG的是，通过一组非常有限的结构，可以描述很多决定论语言。在GLSL450的情况下 - 这是glsl crate可以解析的语言 - 它是一种无上下文和确定性的语言。因此，整个语言可以根据（递归）PEG规则来定义。

Pest不是解析器，它应该是一种词法分析器。AST解析还需要自己来弄。也有人推荐用[pest-ast](https://github.com/pest-parser/ast)来处理AST。

Nom：

nom是一个解析器组合器。这意味着您可以通过组合小解析器来构建更大的解析器。关于nom的正确术语是它是一个无扫描器解析器：它不需要在解析之前生成令牌，并且更喜欢同时执行两者。 nom解析器通常使用像preceded！，delimited！，take_until！，tag！，value!和do_parse!等宏，允许匹配（lexing）切片的字节/字符，并使用你选择的类型将它们解析为实际值。

然而，Pest依赖于PEG文件，表示要标记的语言的正式语法。该词法分析器阶段发生并且必须能够在返回之前对整个输入进行标记。我不确定我什么时候说这个（但我很有说服力就是这种情况）：Pest不支持流输入，因为它需要吃特殊规则EOI（End Of Input ）， 或者在返回之前吃规则错误（使用先前的规则成功或向上传播错误）。但是，nom可以用来吃掉字节流。

[Part I](https://phaazon.net/blog/glsl-pest-part-1)
[Part II](https://phaazon.net/blog/glsl-pest-part-2)

---

# 项目

### Rust实现的寄存器式虚拟机 

大多数语言vm都是栈式VM，之前介绍的一些VM实现也都是栈式。今天第一次见寄存器式VM的Rust实现。

Android虚拟机Dalvik就是寄存器式VM，可以参考Dalvik的相关资料来了解寄存器式VM的工作机制。

感兴趣的可以关注下。

[Jazz](https://github.com/playXE/Jazz)

### jsc：Rust实现的js编译器

[jsc](https://github.com/eatonphil/jsc)

### 又一个JavaScript解释器

[esprit](https://github.com/dherman/esprit)

### 将Elm架构引入Rust和WebAssembly

Elm生态系统有一个架构，叫做TEA，The Elm Architecture。然后该文作者将TEA引入了Rust和WASM体系里。并且通过此实验，了解Rust和WASM在web开发方面能走多远。结论是：他想在日程工作中都能用Rust编写webapp。

代码实现： [willow](https://github.com/sindreij/willow)

其中还包含了两个demo。

[Read More](https://sindrejohansen.no/blog/willow/rust/elm/2018/11/16/willow-elm-in-rust.html)

### 「小游戏」 使用Amethyst引擎实现的经典游戏Asteroids

基于Rust 2018 版本

[asteroids-amethyst](https://github.com/udoprog/asteroids-amethyst)

### 「库」typed-html： Rust模板库

TypedHtml是一个带类型检查的HTML模板，现在支持了纯wasm实现的虚拟dom渲染HTML

[typed-html](https://github.com/bodil/typed-html)

### 介绍Mundane 

Mundane 基于BoringSSL的Rust加密库。BoringSSL 是由谷歌从 OpenSSL 中抽出来后独立发展的作品。

之前新闻中介绍过该库。今天是相关开发者写博文详细介绍了Mundane的起源和实现。

why mundane?

Mundane存在的理由是提供一种难以滥用的API。经验表明，密码学中最常见的失败模式之一就是不正确的实现，并且，通常失败都发生在应用程序和加密库之间。鉴于此原因，Mundane采取的方法是为开发者提供尽可能少的自由度。做正确的事，很容易，但是想做错事，则会很困难，理想情况下完全不可能。（和Rust的理念相似，这也是他们使用Rust开发的理由）。

Mundane也是Google Fuchsia操作系统的主要加密库。Mundane完全依赖BoringSSL，是因为信任BoringSSL。BoringSSL团队都是密码学专家。

[mundane](https://github.com/google/mundane)

[Read More](https://joshlf.com/post/2018/11/06/introducing-mundane/)

### 「区块链」零知识证明库Bulletproofs ：pre-release版本发布

预发布版本号称性能大幅提升：

- 比libsecp实现快1.83倍（使用endomorphisms）;
- 比libsecp实现快2.00倍（没有endomorphisms）;
- 比Monero实施快4.63倍。

并且还提供了干净、安全可扩展的API。

Bulletproofs是由斯坦福大学应用加密学小组最近发表的一篇有关于保密交易的有效范围证明的研究论文，文中提出了一种可以大幅降低区块链存储数据大小（约为十倍）的方法。

[基于Bulletproofs 论文](https://crypto.stanford.edu/bulletproofs/)

[Bulletproofs](https://github.com/dalek-cryptography/bulletproofs/)

是之前这个库[ristretto-bulletproofs](https://github.com/chain/ristretto-bulletproofs/)迁移到了新的仓库里。

[Read More](https://medium.com/interstellar/bulletproofs-pre-release-fcb1feb36d4b)

---

### 工具与库

### rand库 0.6版发布

- [changelog](https://github.com/rust-random/rand/blob/master/CHANGELOG.md)
- [update guide](https://rust-random.github.io/book/update-0.6.html)
- [rand book](https://rust-random.github.io/book/)

### Rust实现可生成3D网格的库

灵感来自于structure Synth

[Read More](https://www.reddit.com/r/rust/comments/9y1efh/generate_3d_meshes_in_rust/)

[使用指南](Synthesizing Structures with immense)

### 「工具」 按时区查看日志的工具

[tztail](https://github.com/thecasualcoder/tztail)

### 「小工具」检测no_std兼容性

[cargo-nono](https://github.com/hobofan/cargo-nono)

### lifeguard 0.6 发布

lifeguard是Rust实现的一个对象池管理库，用来创建可重用的值，避免不断分配新值造成的开销。

[lifeguard](https://github.com/zslayton/lifeguard)