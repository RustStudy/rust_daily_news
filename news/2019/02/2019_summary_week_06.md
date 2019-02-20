### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-02-17

---

# 通告两则

### RustConf Asia 大会来了

#rustconfAsia

首届RustConf Asia会议，将在北京时间 4月20~4月23号举办。其中两天为主题演讲，另外两天是workshop。现在已经可以购买早鸟票，也开放了演讲主题申请。

- [大会官网]( https://rustcon.asia/blog/hello-asia/ )
- [CFP]( https://cfp.rustcon.asia/events/rustcon-asia )
- [早鸟票]( http://www.huodongxing.com/event/6479456003900 )


### 《Rust编程之道》勘误说明

《Rust编程之道》截止今天，修订了115条勘误，第一次近距离体验到集体智慧的强大。

其中有一些重要的勘误，我把它们打上了「精选」Label，读者朋友们看书学习的时候，可以方便查看。也可以通过选择章节Label，来查看一些勘误。Closed 状态代表我已经修订了稿件。

接下来，我会对这些勘误进行归档，整理一份电子版小册子，方便读者查阅。 ​​​​

![img](https://wx3.sinaimg.cn/mw690/71684decly1g03jpc5wenj20u00yju0x.jpg)

---

# 官方新闻

### 「系列博文」Generator I：迈向MVP 

无船同志新的博文，主要探讨了接下来要对Generator进行重新设计，目标是一个MVP（最小可行性产品，minimum viable product）。当前虽然在Nightly中可用，但还是比较简陋，所以现在开始对其进行精致的设计。大目标也是为了Rust异步可以早日稳定。

- [Read More](https://boats.gitlab.io/blog/post/generators-i/)
- [Reddit 讨论贴](https://www.reddit.com/r/rust/comments/apitmm/generators_i_toward_a_minimum_viable_product/)

### 「官方」All-Hands 2019回顾

关于Miri和UCG：

这是Ralfj参加Rust官方All Hands大会后的回顾，只挑了他自己感兴趣的点进行了记录。Ralfj感兴趣的点都是和Unsafe Rust的安全性相关。比如UCG（unsafe code guidelines）和Miri。

我感兴趣的一个点是他提到：Miri已经开始运行了libcore和liballoc的单元测试。这有助于发现标准库中可能的未定义的行为。或者更确切地说，单元测试所涵盖的标准库的部分没有未定义的行为。Ralfj这几个月都是为这个目标努力。

文章里也有详细会议讨论记录的链接。

[Read More](https://www.ralfj.de/blog/2019/02/12/all-hands-recap.html)

关于WASM：

- 进一步探讨了wasm-pack 1.0 RoadMap
- 深入探讨了wasm的模块化工具包的相关细节
- 讨论如何在Cargo中添加某些通用的构建hook，可以将wasm-pack转换为针对wasm开发的hook实现
- 讨论wasm支持多线程，以及如何将Rayon库应用于wasm。

[Read More](https://rustwasm.github.io/2019/02/13/this-week-in-rust-and-wasm-010.html)

关于Async/Await：

Niko锁定了讨论await语法的issue，这意味着，await语法的讨论即将进入下一个阶段。

在All Hands会议上，官方团队讨论了async/await，事情虽然变得更加清晰，但是还未对await语法达成共识。所以暂时锁定了该issue，然后内部会进行一些针对性讨论，随后在生成报告。所以在这个阶段，issue里进一步的讨论已经起不到什么作用了，所以在官方报告出来之后，再进行下一个阶段的讨论。

[Read More](https://github.com/rust-lang/rust/issues/57640#issuecomment-464152929)




---

# 社区新闻

### 微软：70％的安全漏洞都是内存安全问题

微软安全工程师Matt Miller上周在以色列举行的BlueHat安全会议上表示，在过去的12年中，大约70％的微软补丁是为了修复内存安全漏洞。

[Read More](https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/)

### 不，不是因为「程序员菜」

针对这篇《微软：70％的安全漏洞都是内存安全问题》的文章，Diesel的作者Sean写了这篇文章。因为网上有很多言论，他们的观点是：不是因为编程语言的问题，而是因为写代码的人太菜，所以才出现这种安全问题。Sean并不同意这种观点。

本文中，Sean结合具体的案例，来说明一个问题：Rust编译器捕获的错误，是有可能超出程序员经验之外的，没有程序员是全能的。期望更好的程序员，完全不是对人类的合理期望。我们需要像Rust这样带有安全防护的语言来防止错误。 难道开车上路，有更好的司机，就不需要安全带了吗？

我们不应该对软件开发人员和编程语言放在一起评判。

(Sean 的这个观点，我比较赞同)

[Read More](https://medium.com/@sgrif/no-the-problem-isnt-bad-coders-ed4347810270)

### SemVer：语义化版本的下一步

Steve（对，就是刚从Mozilla离职的那位），他也是Semver组织的一员，他们现在准备制作新的语义化版本规范。

为什么？

语义化版本控制诞生于2009年的Ruby社区，随着Bundler的流行，也传播到了Node社区，npm也随之跟进。然后就是Rust的Cargo，都进行了支持。但是语义化版本也有很多缺点。所以他们想要改进。并且专门针对Cargo来进行尝试。

拭目以待。

- [Read More](https://words.steveklabnik.com/what-s-next-for-semver)
- [Semver Org](https://github.com/orgs/semver/)


### 「嵌入式Rust」OxidizeConf 大会CFP启动

Oxidize是一个专注于Rust嵌入式设备和微控制器的活动， OxidizeConf应该是首届嵌入式Rust大会吧。将于4.27在柏林举办。感兴趣可以关注。

[Read More](https://cfp.oxidizeconf.com/events/oxidize-2019)

### 比较各种编程语言的FFI开销

5亿次调用耗费时间（毫秒）简要排名：

1. Luajit 891
2. Julia  894
3. C      1182
4. Cpp    1182
5. Rust   1193
6. Haskell  1197
7. Java8    4505
8. Elixr    23852
9. Go       37975

[Read More](https://github.com/dyu/ffi-overhead/blob/master/README.md)

### 为TiKV贡献的机会来了

这周的三个issues比较简单，成为TiKV贡献者的机会就在眼前。除了这三个，另外还可以寻找其他标记为Easy的任务来完成。

1. 「已做」把 trait对象使用新的dyn Trait语法。 [issues/4197](https://github.com/tikv/tikv/issues/4197)
2. 「还有机会」移除 `extern crate`语句。[issues/4196](https://github.com/tikv/tikv/issues/4196)
3. 「已做」为 tikv_alloc 增加 tcmalloc  支持。[issues/4191](https://github.com/tikv/tikv/issues/4191)

###  `shared_ptr<T>`: （并不是总是）原子引用计数智能指针

（cpp用智能指针编写的程序，安全性就可以和Rust持平吗？很可能不行）

作者在几个月前，运行了Rust和Cpp的一些数据结构的基准测试，发现Rust实现的RB tree明显慢于cpp。这个现象让作者感到奇怪，因为以他的经验，Rust和Cpp的性能不会产生这么明显的差距。

所以，他就进行了进一步测试。结果发现：cpp的`shared_ptr<T>`智能指针并不总是原子性的进行引用计数，这是cpp速度快的原因。这说明，`shared_ptr<T>`在某些并发情况下，会发生数据竞争、悬空指针或内存泄漏等风险。

[Read More](http://snf.github.io/2019/02/13/shared-ptr-optimization/)

### 从Ruby到Rust经验谈

注意，该博客ban了来自中国的IP，可挂代理查看。

[Read More](https://www.reddit.com/r/rust/comments/aqonf8/moving_from_ruby_to_rust/)

### Amethyst Roadmap 2019 发布

Rust实现的游戏引擎Amethyst发布了一个简单的2019 Roadmap。

[Amethyst Roadmap 2019](https://github.com/amethyst/amethyst/blob/master/docs/ROADMAP.md)


---

# 学习资源

### 你还在用println宏来调试Rust代码？

这篇文章介绍了Rust 1.32引入的`dbg!`的用法。

[Read More](https://blog.knoldus.com/are-you-still-using-println-in-rust-for-debugging/)


### 「系列文章」Rust内存模型介绍

一系列关于Rust中内存的文章，包括栈、堆、全局内存分配器、编译器优化等内容。

[Read More](https://speice.io/2019/02/understanding-allocations-in-rust.html)

### 干掉Unwrap！

为什么要干掉Unwrap？滥用`unwrap()`会导致生产环境中出现各种恐慌。该文作者通过一些代码示例来告诉你如何干掉Unwrap！

[Read More](https://dmerej.info/blog/post/killing-unwrap/)

### trait对象一瞥

该文作者对trait对象做了一个比较系统的梳理。

[Read More](https://tratt.net/laurie/blog/entries/a_quick_look_at_trait_objects_in_rust.html)

### 「Fearless Security系列」Part 2: 线程安全

Fearless Security系列文章一共三篇，是Mozilla工程师对于Security的探讨，本文是第二篇，探讨了线程的安全性，以及Rust如何保证线程安全。

[Read More](https://hacks.mozilla.org/2019/02/fearless-security-thread-safety/)

### 使用GoReleaser分发Rust二进制文件

GoReleaser是Go语言社区的一个分发工具，它可以轻松地分发Go二进制文件。该文作者通过Hack GoReleaser的构建过程来达到支持Rust的目的。

Rust也有类似的工具，cargo-release/ cargo-deliver/ cargo-hublish，但作者说这些工具目前还不如GoReleaser完善。

[Read More](https://medium.com/@jondot/shipping-rust-binaries-with-goreleaser-d5aa42a46be0)

---

# 项目、工具与库

### rust-proofs: filecoin协议证明库

Filecoin是一个去中心化存储网络，也叫做Filecoin的区块链，Filecoin进行了ICO，代币名称为FIL。Filecoin与IPFS是两个项目，IPFS是底层协议，并没有ICO。Filecoin最近几天刚宣布开源，也有其他几个项目是Rust实现。

Filecoin是做在IPFS其上的激励层，通过token激励模式，在IPFS上构建了一个去中心化存储市场，共同点就是他们都是实现存储和检索的资源共享交换。

[rust-proofs](https://github.com/filecoin-project/rust-proofs)


### nimiq: 基于浏览器的区块链

国外一家创业公司的开源产品。

[nimiq/core-rs](https://github.com/nimiq/core-rs)

### rust-headless-chrome: Puppeteer库的Rust实现

Puppeteer库是Node实现的一个无头浏览器，通过该库可以自动化浏览器的大多数动作。rust-headless-chrome则是Rust实现的类似的库。

[rust-headless-chrome](https://github.com/atroche/rust-headless-chrome)

### 「嵌入式Rust」embedded-graphics: 小型2D图形库

一个小型2D图形库，用于在嵌入式图形LCD上绘制内容，如SSD1306 OLED显示屏。配合sh1106驱动使用。

效果：

![img](https://wx2.sinaimg.cn/mw690/71684decly1g06x5mbyzpj20sa0lihdt.jpg)

- [embedded-graphics](https://github.com/jamwaffles/embedded-graphics)
- [sh1106](https://github.com/jamwaffles/sh1106)


### Rust和Wasm写的firefox插件

并通过webpack和web-ext进行热加载。

- [rustwasm-addon](https://github.com/willdurand/rustwasm-addon)
- [webpack-webext-plugin](https://github.com/rpl/webpack-webext-plugin)

### shellfn: 支持Rust内调用shell脚本的库

该库提供了一个宏，允许在Rust函数中调用任何语言编写的shell脚本。

[shellfn](https://github.com/synek317/shellfn)


### Pyrs：将Python代码自动转换为Rust的工具

- [Read More](https://medium.com/@konchunas/transpiling-python-to-rust-766459b6ab8f)
- [pyrs](https://github.com/konchunas/pyrs)

### pycors： Python解释器管理工具

Rust实现，类似于pyenv的Python解释器管理工具，可以下载并编译指定的版本，并轻松切换它们。目前只支持MacOSX，后续支持Linux和Windows。

- [pycors](https://github.com/nbigaouette/pycors/)
- [Reddit 讨论](https://www.reddit.com/r/rust/comments/apbr6k/announcing_pycors_a_python_interpreter_manager/)

###  swc 发布1.0

swc是babel 和 closure compiler二合一的Rust实现。潜力不错，但感觉目前依旧缺乏文档。

- [swc](https://github.com/swc-project/swc)
- [Read More](https://swc-project.github.io/blog/)

### Rust实现的Kubernetes API 客户端

还属于实验状态。不过该作者是Kubernetes GitHub组织的成员。

[kubernetes-rs](https://github.com/anguslees/kubernetes-rs)

### 「嵌入式Rust」将Rust编译器移植到M68K

M68k 处理器是美国 Motorola 公司开发的高性能处理器， 具有高性价比、高集成度等特点，在工业自动化设备、控制设备、医疗仪器系统、安全系统等领域多有应用。现在为Freescale 公司所有。

有人为M68K实现了一个LLVM后端，现在该作者想把Rust编译器移植到M68K-LLVM后端。

- [Read More](https://lists.debian.org/debian-68k/2019/02/msg00003.html)
- [Reddit 讨论贴](https://www.reddit.com/r/rust/comments/apiieb/porting_the_rust_compiler_to_m68k/)


### apk-decompiler：Rust实现的apk反编译工具

- [apk-decompiler](https://github.com/robertohuertasm/apk-decompiler)
- [Read More](https://robertohuertas.com/2019/02/03/rust_cli_apk_decompiler/)

### hibp-check: 依据HIBP检查您的Keepass数据库是否泄密

KeePass是一个免费的开源密码管理器，可以帮助您以安全的方式管理密码。您可以将所有密码放在一个数据库中，该数据库使用一个主密钥或密钥文件锁定。因此，您只需记住一个主密码或选择密钥文件即可解锁整个数据库。使用当前已知的最佳和最安全的加密算法（AES和Twofish）对数据库进行加密。

Have I Been Pwned（HIBP），是一个数据泄密聚合网站，专门披露数据泄漏事件和帮助用户确认自己的密码是否泄露的账号安全网站。

[hibp-check](https://github.com/samueltardieu/hibp-check)

### dodrio: Rust和WebAssembly实现的的实验性虚拟DOM库

[dodrio](https://github.com/fitzgen/dodrio)

### protobuf-convert：序列化Protobuf3的库

[protobuf-convert](https://github.com/witnet/protobuf-convert)
