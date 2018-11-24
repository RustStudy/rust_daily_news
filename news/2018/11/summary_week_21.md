前言：
从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。
2018-11-24

# 官方新闻

### 「官方」crates.io帐号审核措施

从2019-02-28开始，需要经过验证电子邮件地址的帐号才能往crates.io发布crate。所以，之前注册帐号的，2019-02-28之前，可能会收到警告，要求你去验证注册的email地址。如果不验证，2019-02-28之后就会收到发布错误了。 crates.io承诺将保护好你的隐私。

[Read More](https://users.rust-lang.org/t/a-verified-email-address-will-be-required-to-publish-to-crates-io-starting-on-2019-02-28/22425)

### 「CLI工作组」Cli Book新发布了打包章节的第一个版本

[Read More](https://rust-lang-nursery.github.io/cli-wg/tutorial/packaging.html)

---

# 社区新闻

### RustFest Roma大会开始了

[twitter话题](https://twitter.com/hashtag/RustFest?src=hash)

### 塞萨洛尼基（希腊古城） GNOME+Rust Hackfest 活动记录

[Read More](https://blog.guillaume-gomez.fr/articles/2018-11-19+GNOME%2BRust+Hackfest+in+Thessaloniki)

### TiKV Rust Client呼吁社区开发者参与

TiKV Rust Client 的RFC马上完工，现在呼吁社区开发者为其做贡献。也是一次最佳练手的机会。
当然，如果你对去PingCAP工作感兴趣的话，也许也是一次绝佳的机会。

如何参与？

开发环境配置：

- 最简单的就是本机启动一个pd（Placement Driver ），然后启动一个 Tikv，都用默认的配置应该就能跑起来。

[部署TiKV](https://github.com/tikv/tikv/blob/master/docs/op-guide/deploy-tikv-using-binary.md)

[RFC-7](https://github.com/tikv/rfcs/pull/7)

然后提交PR或Issues就可以了。

### RedHat发布对Rust、Go、Clang/LLVM6.0的支持计划

RedHat对Rust1.29、Go1.10和Clang/LLVM6.0的可用性评估之后，现在已经将它们提升到了完全支持状态。在完全支持阶段，一些合格的重要的勘误表（RHSA）和一些选定的高优先级的错误修复表（RHBA）将会在可用时及时发布。

由于这些包正在快速发展阶段，RedHat将对它们提供特别的更新支持。Rust每个季度会有更新，对于LLVM和Go，则是半年一次。

[Read More](https://developers.redhat.com/blog/2018/11/20/support-lifecycle-for-clang-llvm-go-and-rust/)

### 2018 JS状态报告里提到了Rust

>  With projects like Web Assembly arriving on the scene, writing code directly in JavaScript might soon seem quaint as developers embrace languages like Rust instead.

![img](https://wx4.sinaimg.cn/mw690/71684decly1fxgwav5lwij21lj0u01h3.jpg)

[Read More](https://2018.stateofjs.com/javascript-flavors/conclusion)

### Rust中的泛型方法：Exonum如何从Iron迁移到Actix-Web

本文是Exonum工程师记录了使用泛型编程将Iron迁移到Actix-Web框架的过程。

由于平台自身的增长需求，需要从同步迁移到异步，Iron是同步框架，已经无法满足需求，所以选择了actix-web。

他们不仅仅是迁移Web框架，更重要的是，重新设计新的API体系。本文较长，其中详细给出了如何利用泛型编程对API进行设计的思考，感兴趣的可以看看。

[Read More](https://medium.com/meetbitfury/generic-methods-in-rust-how-exonum-shifted-from-iron-to-actix-web-7a2752171388)

---

# 学习资源

### 使用Rust为PostgreSQL编写扩展函数

作者提供了一个库pgxr，可以方便地使用该库提供的宏来编写postgresql的扩展函数

[pgxr](https://github.com/clia/pgxr)

[Read More](https://www.reddit.com/r/rust/comments/9y2yjq/write_postgresql_extension_functions_as_stored/)

### 无等待线程本地存储

本文介绍了一种无等待线程本地存储。

[Read More](https://bzim.gitlab.io/blog/posts/wait-free-per-object-thread-local-storage.html)


### PHP中运行WASM 

用C语言编写的PHP扩展，通过FFI调用Rust lib，用wasmi来执行wasm。 只是一个POC版本！

- [wasmi](https://github.com/paritytech/wasmi)
- [php-ext-wasm](https://github.com/Hywan/php-ext-wasm)
- [Read More](https://www.reddit.com/r/rust/comments/9y7wl6/run_wasm_in_php_natively_php_extension_written_in/)


### 「教程」Rust中的编译时Features Flag

Features Flag用于启用或禁用features的配置。本文讲解了如何在项目里使用它。

[Read More](https://www.worthe-it.co.za/programming/2018/11/18/compile-time-feature-flags-in-rust.html)

这篇文章是作者参加一个2018 塔防游戏机器人大赛过程中总结出来的。

也是用Rust实现，源码： [entelect-challenge-tower-defence](https://github.com/JWorthe/entelect-challenge-tower-defence)

游戏官网： [challenge.entelect.co.za/](https://challenge.entelect.co.za/) ，好像挺好玩。

### 使用Pest实现EBNF语法

作者在实现一个XML解析器，这篇文章记录了使用Pest来实现使用EBNF-esque形式的词法分析器。

- [xml-grimoire](https://github.com/compenguy/xml-grimoire)
- [Read More](https://compenguy.github.io/hobbies/rust/ebnf-to-pest.html)

### 「演讲」ADC'18大会:  Rust音频编程

[Read More](https://www.reddit.com/r/rust/comments/9z4z9o/adc18_introduction_to_rust_for_audio_programming/)

[freeverb-rs](https://github.com/irh/freeverb-rs)

### Rust+WASM实现的小程序

群友@JiaYe 使用Rust和WASM实现了Gif图片制作的小程序。

让小程序加载wasm模块，还需要做一些针对性修改，可以在该源码里看到。感兴趣的可以看看。

[miniprogram-gifmaker](https://github.com/planet0104/miniprogram-gifmaker/blob/master/README.md)

---

# 项目

### 建立在mio之上的网络库：Sonr

相比于Tokio来说，更加轻量。但不晓得是不是玩票之作，至少可以满足学习的需求。

[sonr](https://github.com/hagsteel/sonr)

### TypedHtml: 带类型检查的JSX模板

这个库上次（前几天）介绍还是100左右的star，现在已经上升到800多star了。也增加了Readme说明。

[typed-html](https://github.com/bodil/typed-html)

### Vulkano：Vulkan API的Rust安全绑定

> Vulkan是由Khronos Group组织（知道OpenGL的话应该都听过这个组织）提供的一个新型的图形接口，它对现代图形显卡做了一个更好的抽象。跟现有的图形接口例如OpenGL和DirectX相比，Vulkan的性能更加强劲，驱动表现更加符合预期行为。Vulkan背后的设计思路跟 Direct3D 12 和 Metal 类似，但它具有跨平台的优势，可以让你同时在windows，linux和android上进行开发。 [来源](https://zhuanlan.zhihu.com/p/26938967)

[vulkano教程](http://vulkano.rs/guide/acquire-present)

[vulkano.rs](https://vulkano.rs/)

### 用Rust实现ecmascript 2019 parser和ast visitor 

- [swc](https://github.com/swc-project/swc)
- [swc_ecma_parser](https://github.com/swc-project/swc/blob/c816d699ca57fc50ff272dde8e441e15ecc093ac/ecmascript/parser/Cargo.toml)
- [Read More](https://www.reddit.com/r/rust/comments/9yqufy/javascript_parser_and_ast_visitor_written_in_rust/)

### wasmer: 高性能WASM JIT解释器

基于Cranelift 代码生成引擎实现的。

[wasmer](https://github.com/WAFoundation/wasmer)

### quinn : Rust实现的QUIC协议

也就是HTTP3，这两天火了呢。

注意看项目Readme，有相关演讲。

[quinn](https://github.com/djc/quinn)

### 「嵌入式」 Rust实现的闹钟

纯Rust打造的DIY液晶闹钟，很酷。

[rusty-clock](https://github.com/TeXitoi/rusty-clock)

### pijul 0.11发布

pijul是基于Rust实现的类Git版本控制工具，[用法介绍](https://jneem.github.io/pijul/)

[Read More](https://pijul.org/posts/2018-11-20-pijul-0.11/)


### amp: 类Vim的终端文本编辑器

这个库最近在GitHub趋势榜上升很快，最近几天发布了0.5.2版本。使用Vim的核心的交互模型，但是对其做了简化。其内部结构（工具集）已经独立为scribe库，包含数据结构、语法高亮、工作空间管理等功能。

- [amp](https://github.com/jmacdonald/amp)
- [scribe](https://github.com/jmacdonald/scribe)

### drone: Rust无人机项目

四轴飞行器

[drone](https://github.com/martindeegan/drone)

### Proptest：Rust实现的属性测试框架

Proptest受Python Hypotheis测试框架启发，而Hypotheis是由Haskell的QuickCheck包启发而成的。

本质上它是用来产生测试数据的，但是它会注意到内存泄露以及越界等会使测试崩溃的问题，因而，在某种程度上它可以为你编写单元测试。

[proptest](https://github.com/AltSysrq/proptest)

---

# 工具与库

### Wither 0.6发布

wither是为Mongodb编写的Rust驱动。可以参考宣传文章，有更多介绍。

- [wither](https://github.com/thedodd/wither)
- [Read More](https://medium.com/docql/https-medium-com-docql-rust-mongodb-wither-13e803c9ae72)

### 命令行HTTP客户端

一个Rust实现的类crul命令行http客户端，灵感来自于Python的HTTPie

[rust_client](https://gitlab.com/rakenodiax/rust-client)
