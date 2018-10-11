# 试验栏目

本周开始「每日新闻」会增加一些试验内容，选择翻译一些文章，时间有限，选择连载的方式。并且接受投稿。

[原文](https://medium.com/solana-labs/solana-at-portland-dev-meetup-72e4dc7ad32c)

「Solana 、Zcash 和Parity 三家会面讨论为什么Rust适合区块链开发 」连载1

- [Solana](https://github.com/solana-labs)是一种高性能区块链
- [Zcash](https://github.com/zcash/)是一种加密货币
- [Parity](https://github.com/paritytech/)是一个核心的区块链基础公司，以太坊相关

内容里涉及的人物我不清楚他们具体谁是对应哪家公司，但可能通过聊天内容猜的出来。

[「连载1」为什么Rust适合区块链开发](https://zhuanlan.zhihu.com/p/46123567)



# 官方新闻

### Rust 1.29.2 预发布测试


-  主要是因为与alias相关的LLVM7的bug导致编译错误
-  windows-gnu target上的rls-preview组件已经恢复

[原文](https://internals.rust-lang.org/t/rust-1-29-2-prerelease-testing/8524)

### Rust和WebAssembly本周报告 008

简要：

- wasm-pack大版本发布
- web-sys crate发布
-  《Programming WebAssembly with Rust 》Kevin Hoffman正在为Pragmatic Programmers图书出版商撰写一本关于Rust和Wasm的书
-  [SFHTML5聚会的视频录制和幻灯片](http://fitzgeraldnick.com/2018/10/01/sfhtml5-rust-and-wasm-talk.html)，都是关于wasm的，[幻灯片地址](https://rustwasm.github.io/sfhtml5-rust-and-wasm/#1)
- rustfmt编译为wasm，用于在浏览器中格式化代码
- Olin，服务端wasm平台
-  P64Lang解析器和解释器作为wasm模块运行
- [wasm-bindgen如何促进Rust和JavaScript之间的互操作](https://blog.ryanlevick.com/posts/wasm-bindgen-interop/)

详细内容请看： [原文](https://rustwasm.github.io/2018/10/01/this-week-in-rust-wasm-008.html)



---

# 社区新闻


### 微软链接器补丁

允许生成没有Rich标头的可执行文件

[link-patcher](https://github.com/mthiesen/link-patcher)

### 游戏引擎Amethyst基准测试

以60fps渲染尽可能多的 bunny sprites 来对Amethyst做基准测试

```
Results

Command                      Bunnies Rendered
-----------------------------------------------------
cargo run                       |         3,400
cargo run --release    |         109,800

```

看看release和debug的差距

[amethyst-bunnymark](https://github.com/cart/amethyst-bunnymark)


### GitHub和DigitalOcean联合举办的Hacktoberfest活动

在活动期间内（整个十月份），选择要参加活动的GitHub开源项目，并为其提交5个PR，前5w人即可获得T恤（任何时区的人都可参与）。

Rust的参赛项目是Xi-editor

[活动网站](https://hacktoberfest.digitalocean.com/)


### lolbench： 自动检测Rust的性能回退

lolbench编译了Nightly Rust下大约350个基准测试。 然后运行它们并高亮显示标准库中的潜在性能回退和编译器的输出。

该项目的目的是想要监督Rust的性能朝越来越快的方向发展

[lolbench](https://github.com/anp/lolbench)

[lolbench数据站点](https://blog.anp.lol/lolbench-data/)

[原文](https://blog.anp.lol/rust/2018/09/29/lolbench/)


### 将CUDA Path Tracer 移植到ptx-builder/linker

该文作者一直致力于将Rust应用于CUDA。目前他还不建议用Rust应用于CUDA，但是他认为现在已经有了稳定的基础。

该文记录了PTX链接器和相关构建脚本，因此可以在Rust里可靠地构建CUDA内核，这是朝Rust的GPGPU迈出重要的一步。

[GPGPU 相关前文](https://bheisler.github.io/post/state-of-gpgpu-in-rust/)

[原文](https://bheisler.github.io/post/ptx-builder-and-linker/)

### witnet 月报

Witnet去中心Oracle网络协议（Witnet Decentralized Oracle Network protocol）的第一个开源实现，是[witnet-rust](https://github.com/witnet/witnet-rust)

> Witnet协议将智能合约连接到任何在线数据源，例如股票价格，天气预报，以及其他区块链，该协议由被称为目击者的分布式矿工节点运行，这些节点可以检索并向智能合约提交数据从而获得代币奖励，建立一个分布式的，不可更改的，抗审查的数据档案。

witnet-rust 包括了一些区块链组件，比如持久化区块链的存储引擎等， 2019年会发布测试网络。感兴趣的可以看看。

[原文](https://medium.com/witnet/witnet-monthly-report-september-2018-20cb85b61a88)

### 「远程工作」编译器和分布式系统工程师

公司（很可能没有公司）在悉尼，但是你可以在全球任何地方工作。 但可能只是临时工，并且可能只提供1-2个月的工资。 不过想参与到编译器和分布式开发的开源项目的人，也可以关注下。

[原文](https://www.reddit.com/r/rust/comments/9kx94z/job_compilers_distributed_systems_engineers_in/)


### 「吐槽」小心Packt发布的书籍：低质量的内容，非常明显未经审查

该贴作者买了一本《Hands-on Concurrency with Rust》，对其很不满，发了一个吐槽贴，详细看内容和讨论吧。

[原文](https://www.reddit.com/r/rust/comments/9l2rmk/meta_beware_of_books_published_by_packt_low/)


### 介绍Ruukh框架

Ruukh是一个前端Web框架，受到VueJS和ReactJS的启发。

看起来不错

[原文](https://sharadchand.com/2018/10/03/ruukh-framework.html)


### Pest 解析器 2.0发布

提升了性能，引入了更多的wasm

Pest是一个Rust实现的通用解析器，类似于正则表达式，但是它提供了解析复杂语言的增强表达。

性能测试以解析json为例，要好过nom

[官网](https://pest.rs/)

[pest](https://github.com/pest-parser/pest)


### nom作者：不，Pest没有nom快


作者亲自做了基准测试：

- nom:
      -  canada.json: 60,734,229 ns/iter (+/- 17,775,618)
      -  data.json: 23,937 ns/iter (+/- 9,992)
- pest:
     - canada.json: 35,041,472 ns/iter (+/- 5,454,302)
     - data.json: 14,665 ns/iter (+/- 2,041)

以上是没有将json转换为Rust的类型，当然Pest比nom快。但是如果用nom4然而不转换为Rust类型会是什么结果呢？

- pest:
     - canada.json: 35,041,472 ns/iter (+/- 5,454,302)
     - data.json: 14,665 ns/iter (+/- 2,041)
- nom_spans (returning slices instead of converting to Rust types):
    - canada.json: 20,623,381 ns/iter (+/- 1,952,297)
    - data.json: 10,757 ns/iter (+/- 1,462)

这个才是公平的测试，因为Pest并不会转换为Rust类型。显然这种情况下nom更快。

（火药味十足啊）

[原文](https://unhandledexpression.com/general/2018/10/04/no-pest-is-not-faster-than-nom.html)


###  crates.io上流行的crate的作者都是谁 ？

作者写了简单的代码对最近截至2018年10月3日有超过10万次下载的crate进行统计，得出以下结果：

```rust
authors  count
----------------------------
alexcrichton  61 packages
carllerche  20 packages
7 authors  16-8 packages
8 authors  5-3 packages
14 authors  2 packages
233 authors  1 package
```

看来Alex是Rust社区的顶梁柱啊

[原文](https://words.steveklabnik.com/who-authors-the-most-popular-crates-on-crates-io)


### Red Hat 静态存储项目 Stratis 1.0 发布

前两天刚介绍过，是一个易于使用的Linux本地存储管理工具。

>  在他们决定放弃Btrfs支持之后，Stratis已经开发了两年，用于提供下一代Linux存储。 Stratis提供ZFS和Btrfs之类的功能和许多其他新功能，而上周标志着它的第一个稳定版本。

> 很快就可以向Fedora用户提供Stratis 1.0用于评估，并且它很快就会出现在其他发行版中。 Stratis组件依赖于Linux 4.14 +，Python 3和Rust 1.25+。 但值得一提的是，Stratis D-Bus API还不稳定。

[stratisd](https://github.com/stratis-storage/stratisd)

[原文](https://groveronline.com/2018/10/stratis-1-0-released/)


### Tokio文档计划

Tokio作者准备重整文档，现在需要社区帮助

1. 阅读现有文档，反馈令人困惑的部分或留下未解决的问题。
2.  在[doc-push](https://github.com/tokio-rs/doc-push)存储库上打开一个问题以报告混淆。
3. 解决问题。
4. 重复以上步骤。


[原文](https://tokio.rs/blog/2018-10-doc-blitz/)

### Compiler Explorer  ： 一个类似于play的Rust在线执行环境

与play不同的是，它还可以选择往期版本的Rust

[网址](https://www.godbolt.org/)


### Kazan：使用Rust重写

曾经GSoC 2017的一个项目Vulkan-CPU，之前是C++实现，停滞一年之后，作者宣布用Rust重写它，新项目名字是Kazan。

[kazan](https://github.com/kazan-3d/kazan)


### 用Rust实现的C标准库

Redox项目组移植了POSIX C标准库，完全用Rust实现，正在大力发展。支持Redox和Linux。

此项目动机：

- 方便Redox工作人员，
- 创建一个更安全的C标准库newlib的替代品

当然，也可以作为学习之用。Rust代码比C代码可读性高。

[relibc](https://github.com/redox-os/relibc/)


---

# 博文与库

### 测试基于reqwest的客户端

包括了一个完整的使用reqwest构建client的示例，使用json通信

使用Mock的方式来测试client，无需发送真实的请求。有一个reqwest_mock库，但是该库跟不上reqwest API的更改，所以作者只有自己寻找方法。

[原文](https://write.as/balrogboogie/testing-reqwest-based-clients)

在reddit相关讨论中，有人推荐了两个相关的库可以用来Mock测试http请求。

-  [mockito](https://github.com/lipanski/mockito)
-  [Mocktopus](https://github.com/CodeSandwich/Mocktopus)


### WebAssembly和动态内存

本文介绍了C/C++/Rust下如何在面向WebAssembly体系下进行动态内存分配，并分别比较了它们的工具链。

结论是：Rust的编译器和动态内存分配器wee_alloc的组合是明显的赢家，其生成的WebAssmebly代码非常精简，可以用在任何WebAssembly环境中。主要是因为Rust工具链没有其他的依赖。像Clang这种工具链依赖于emscripten，就导致无法在通用的WebAssembly环境中使用。

[原文](https://frehberg.wordpress.com/webassembly-and-dynamic-memory/)


### EasyReader：一个专用于读取大型文件的库

作者在两个项目中使用它，读取140GB左右的文件

[easy_reader](https://github.com/ps1dr3x/easy_reader)


### 为什么Rust不支持默认参数？

来看看这位作者怎么说的？

简要：

-  默认参数虽然方便，但是没有「显式」的好处，在开发者协作方面不太好
-  Rust使用Default类型和「..」语法配合，语义上更加明确，可读性大大提高

[原文](https://medium.com/@softprops/default-values-copy-that-ae43831781f3)


### 使用Rustler为Elixir代码编写nif

[原文](https://medium.com/@jacob.lerche/writing-rust-nifs-for-your-elixir-code-with-the-rustler-package-d884a7c0dbe3)


### Merlin：基于STROBE的可组合零知识证明消息记录（transcript）

[原文](https://medium.com/@hdevalence/merlin-flexible-composable-transcripts-for-zero-knowledge-proofs-28d9fda22d9a)

[merlin](https://github.com/dalek-cryptography/merlin)

### Xgboost的Rust绑定

XGBoost据说是机器学习界的「快刀」，现在有了Rust绑定，那就是「小李飞刀」了

[rust-xgboost](https://github.com/davechallis/rust-xgboost)


### 系列文章：用Rust实现文字冒险游戏 1

[原文](https://www.cattlegrid.info/the-undergarden-text-adventure-rust-1-sections)

### Rust文字冒险游戏  2

[原文](https://www.cattlegrid.info/the-undergarden-text-adventure-rust-2-objects)



### 使用Rust创建一个UEFI应用程序

UEFI(Unified Extensible Firmware Interface)，是硬件初始化完，操作系统启动之前的核心应用。

[原文](https://medium.com/@gil0mendes/an-efi-app-a-bit-rusty-82c36b745f49)



###  用于Emacs的godbolt编译器实现

支持Rust，可以编译到汇编，方便代码调试

[rmsbolt](https://gitlab.com/jgkamat/rmsbolt)

![nW1lVFM.gif](https://wx3.sinaimg.cn/mw690/71684decly1fvsmekfk2yg20xs0kgn92.gif)


### Twiggy 0.3发布

twiggy是.wasm二进制文件的代码大小分析器

`cargo install -f twiggy`

[原文](https://www.reddit.com/r/rust/comments/9l67p0/announcing_twiggy_030/)


### getver：命令行获取指定crate的最新版本号

[getver](https://github.com/phynalle/getver)
