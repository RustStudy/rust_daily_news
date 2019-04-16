### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-04-14

---

# 通告两则

### RustCon Aisa 2019 大会在下周六即将开启

关于大会动态，请关注此帖。包含讲师介绍、议题等信息。

[RustCon Asia 不可错过的动态](https://talk.citahub.com/t/topic/294)

大会期间吃货指南： [土生土长北京饮食达人姜军为你推荐：RustCon 期间的北京特色餐饮](https://talk.citahub.com/t/topic/477)

关于门票：

我手头有两份优惠码：IZHKfP6 和 fAp7FK1，给有需要的朋友。但不知道有没有被使用了。如果你已经买了票，但是大会期间有事不能去，可以提前联系日报小组，可以帮你吆喝一嗓子，转购给有需要的人。

### RustCC论坛坛主Mike新上线了公众号

以后Rust日报也将同步到该公众号，多了一个推送渠道。

 [微信公众号：Rust语言学习交流](https://rust.cc/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)


---

# 官方新闻

### Rust 1.34 稳定版发布

```
$ rustup update stable
```

该稳定版本包括如下变动：

- 引入了cargo registries，此功能允许你指定crates.io之外发布的crate，甚至一些闭源的crate。
- 文档测试支持`?`语法
- 过程宏中的自定义属性`#[attr()]`,`#[attr[]]`, 和`#[attr{}]` 可以接受token流。之前的unrestricted_attribute_tokens Features。
- TryFrom和TryInto稳定
- 弃用`fn before_exec`而使用`unsafe fn pre_exec`
- 稳定了一些标准库API，比如Instant::checked_add/ Instant::checked_sub等
- 支持`extern crate self as foo;`将当前crate的root导出为指定别名
- 支持新的target：`riscv64imac-unknown-none-elf` 和 `riscv64gc-unknown-none-elf`，以及`powerpc64-unknown-freebsd`
- 可以使用`-C linker-plugin-lto`启用链接器插件LTO优化, 使得rustc将Rust代码编译为LLVM bitcode，从而允许LLVM跨C / C ++ FFI边界执行LTO优化

更多内容请看官方release notes。

- [Read More](https://blog.rust-lang.org/2019/04/11/Rust-1.34.0.html)
- [Release Note](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1340-2019-04-11)
- [unrestricted_attribute_tokens features示例](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=14757b61cc384f085a63efea3002b96d)

### Rust异步进展：Future-proof the Futures API的PR已经被合并

该PR解决的问题是：`Future::poll`应该获取＆Waker还是可以获得＆Waker的＆Context？

最终答案是：Context

- [Future-proof the Futures API ：PR 59119](https://github.com/rust-lang/rust/pull/59119)
- [相关：Tracking issue for RFC 2592, futures_api](https://github.com/rust-lang/rust/issues/59113)
- [相关： [Stabilization] Future APIs #59725 ](https://github.com/rust-lang/rust/issues/59725)

---

# 社区新闻

### Rust又一次获得StackOverflow程序员最喜欢语言第一

#StackOverflow

今天SO发布的这个统计很有意思，先来看看Rust，三点：

1.  80%以上的开发者想用Rust开发
2.  SO有65%的开发者是开源贡献活跃者，这其中使用Rust、WebAssembly和Elixir的人贡献速率最高
3.  Rust的薪资较去年增高了（去年65k） ​​​​

![img](https://wx3.sinaimg.cn/mw690/71684decly1g1xakal26ij20u015hjwx.jpg)
![img](https://wx4.sinaimg.cn/mw690/71684decly1g1xake7qhej21380640ug.jpg)
![img](https://wx2.sinaimg.cn/mw690/71684decly1g1xaky1pm8j20u014wkaz.jpg)

继续SO统计。图1是SO统计各大语言生态的原图，上面没有Rust。但其实，以我运营Rust日报近两年的观察，Rust的生态目前确实比不上在各个领域已经成熟的语言，但是它正在底层默默地在这些领域构建自己的生态。如图2。用群友的话来说：Rust正在聚沙成塔。 ​​​​

![img](https://wx2.sinaimg.cn/mw690/71684decly1g1xalee3g0j20xa0u04ml.jpg)
![img](https://wx4.sinaimg.cn/mw690/71684decly1g1xalgs06jj20wn0u07wh.jpg)

另外，SO统计上关于区块链的统计也很有意思。

![img](https://wx3.sinaimg.cn/mw690/71684decly1g1xalpb809j20ym0u04fh.jpg)

[Read More](https://insights.stackoverflow.com/survey/2019)

### 「讨论」如何解读StackOverflow上面Rust连续四年成为最受开发者喜欢语言的这一事实？

该贴作者对这个结果做出了以下可能的解释：

- 跨越了学习曲线的人真的都喜欢Rust
- 开发人员喜欢新的技术，即使他们还没了解Rust，但他们认为应该重视它。（这个角度来看的话，排名第二的Python看上去就比较诡异）
- Rust社区是一个非常友好的社区
- Rust社区营销的比较好
- 有许多C++开发者因为Rust的安全性而选择它

到底是什么原因呢？这个话题下评论区讨论比较激烈。

（我喜欢Rust，是因为Rust这门语言恰好符合我对理想编程语言的某些预期。你是什么原因喜欢Rust呢？虽然目前还没有太多Rust的职位招聘，但实际上暗流涌动，有很多公司都已或多或少地在生产中使用了Rust，也就是说，Rust正在聚沙成塔）

偶尔也看到一条额外信息：Apple也在使用Rust构建分布式文件系统，据说在在西雅图举行过一次小型演讲，试图招募相关人才，在网上还找不到相关信息。

[Read More](https://www.reddit.com/r/rust/comments/bc46lc/understanding_rusts_popularity_on_stack_overflow/)


### 「远程工作」Rust软件工程师

关键字：远程(美国境内)、Rust、容器服务、创业

[Read More](https://www.cloudseal.io/hiring/rust-systems-software-engineer-sp19)

### 「通告」image的GitHub仓库现已迁移到新的image-rs组织

image是纯Rust实现的图像库，这次迁移主要是遵循以下计划：

- 准备邀请一些人可以加入组织做贡献
- 准备建立两个团队：发布团队（拥有crates.io的访问权限）和开发团队（拥有仓库代码的写入权限）。这样做是为了安全性。
- 移动其他相关的库到统一的组织下

- [image-rs组织](https://github.com/image-rs)
- [Read More](https://github.com/PistonDevelopers/image/issues/891)

### 「讨论」为什么Deno的基准测试中Hyper的最大延迟如此高？

 `max latency >1000ms `

目前讨论还未有结果，可以持续关注。

Deno作者提交了一个PR，使用O3优化，目测也未有改善。后续也有热心人提交了新的补丁，还在尝试。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/balb45/why_is_hypers_max_latency_so_high_in_deno/)
- [Deno基准测试](https://deno.land/benchmarks.html#max-latency)
- [Deno基准测试代码](https://github.com/denoland/deno/blob/master/tools/http_benchmark.py#L55-L58)
- [PR: Use -O3 instead of -O ](https://github.com/denoland/deno/pull/2070)

### 一封写给Rust宏的情书

本文不是宏教程，作者写这篇文章的目的就是为了让还没有使用过Rust宏的人，早点尝试一下宏。（真香预警）

作者比较了Rust和其他语言（C/C++/Java）中的元编程，摘要：

- C宏和Cpp模板元编程。很容易引入难以发现的错误，并且会扰乱编译时间，在构建中引入不必要的复杂性。你是否见过错误使用模板而引发的编译器错误呢？
- Java注解。Java的注解功能很强大，可以在编译时使用，也可以在运行时执行，既可以用于代码生成，又可以拿来反射。Java中最受欢迎的Spring框架大量使用Java注解。但正因为如此，Java的注解功能导致你写的代码可能超越语言自身的语义，而更像是另外一种语言。而且通过注解实现的代码，调试起来也非常痛苦。

那么Rust的宏是不是完全避免了上面的问题呢？并不是百分百的避免，但是Rust已经努力避免了常见的问题，并且可以发挥出Rust语言自身的优势。作者介绍了声明宏和过程宏的一些优势，并且在文章底部列出了一些学习宏的资料。

[Read More](https://happens.lol/posts/a-love-letter-to-rust-macros/)

### 使用Rust开发跨平台组件的新策略

Mozilla工程师最新分享，将Rust实现的核心业务逻辑封装到一个中间层，然后通过新的共享策略，达到Web、iOS、Android三端共享。实施这个策略遇到的一个问题就是如何安全地使用FFI，并且还能与Rust的所有权良好地配合。因此该团队写了一个ffi-support库来帮助达成此目的。

该库最初的实现方案是通过序列化为JSON字符串在FFI中传递。但是缺点是，JSON序列化和反序列化会很慢。最重要的问题是，Java中字符串编码是UTF-16-ish。在Kotlin中，必须反序列化每个数据结构，这个过程会可能会发生异常。最严重的问题是，没有返回JSON字符串，而是某个C指针，忘记更新Kotlin中或者ObjectiveC中的数据结构，导致严重的内存问题。

值得庆幸的是，团队发现了使用Protocol BufferV2替代JSON，可以解决上面的问题。在Rust中直接使用prost库，可以通过Rust的宏生成非常干净的结构。而且，PB比JSON的性能高。

![img](https://wx3.sinaimg.cn/mw690/71684decly1g1yo5q02ixj20y00ts44s.jpg)

- [Read More](https://hacks.mozilla.org/2019/04/crossing-the-rust-ffi-frontier-with-protocol-buffers/)
- [ffi-support](https://github.com/mozilla/application-services/blob/master/components/support/ffi/Cargo.toml)

### Atom 1.36选用了ripgrep

使用Ripgrep极大地提升了Atom项目内查找的性能，在文件量很大的情况下尤为明显。不过需要专门通过设置`使用Rip Grep`选项手动打开。

[Read More](https://blog.atom.io/2019/04/09/atom-1-36.html)

### Mozilla 準備跟一些"合格"的Linux使用者測試 WebRender 

現在Linux使用者可以手動開啟WebRender了
這是一個實驗特性

他可以開啟rust寫的gpu渲染程序
給"合格"的Linux設備使用

[Read more](https://www.reddit.com/r/rust/comments/bcb3g0/mozilla_preparing_to_test_webrender_with/)

### ndarray-stats v0.2 发布

此库为 [ndarray](https://github.com/rust-ndarray/ndarray) 的 `ArrayBase` 类型提供了统计方法。

到目前为止，`Scipy.stats (Python)` 和 `StatsBase.jl (Julia)` 中的大部分统计特性都已经移植过来了。用 Rust 做机器学习的同学是不是可以上手了？

[Repo](https://github.com/jturner314/ndarray-stats)

本库作者还邀请大家一起来讨论这个库的未来走向，好机会。

[Roadmap](https://github.com/jturner314/ndarray-stats/issues/1)

---

# 学习资源

### 如何为TiKV做贡献  

[Read More](https://zhuanlan.zhihu.com/p/62370907)

### Rust编程之道 关于闭包和所有权相关规则

读者frostRed和Yim先后总结出来的规则，大家可以参考讨论。

[Github](https://github.com/ZhangHanDong/tao-of-rust-codes/issues/164)

![pic](https://wx3.sinaimg.cn/mw690/73eff722ly1g220xsmt1uj20u010j4hd.jpg)

在随书源码选择「精选」label也可看到。

### Rust和Actix-web基本Web编程

[Read More](https://zupzup.org/rust-webapp/)

### 使用Actix Web进行人脸检测

这是上次日报里报道过「使用tensorflow与rust人脸检测」文章作者的续篇。在这篇内容里，作者介绍了如何将上一篇文章的成果通过HTTP API来提供。并且作者使用了`actix-web = "1.0.0-alpha.4"`版本。在mtcnn库的actix-web分支里可以看到具体实现。

这两篇文章是实践性很强的文章，建议大家跟着玩玩。

- [Read More](https://cetra3.github.io/blog/face-detection-with-actix-web/)
- [上篇文章人脸检测Rust库：mtcnn](https://github.com/cetra3/mtcnn)

### 「系列博客」 Rust与科学计算 Part 2: 数组

> 该作者的日常工作是机器学习，他在多次使用Rust进行相关的实验之后发现，Rust语言在这个领域将大有可为，并让他感觉非常激动。作者反观了Python统治下的机器学习世界，其生态系统爆炸的原因是因为有很多基础库，比如NumPy，SciPy，Pandas等。大多数项目是构建在NumPy和SciPy之上。如果Rust也拥有这些核心的基础库会怎么样？抱着这样的想法，作者加入了维护ndarray库的队伍中。他贡献了一堆PR之后，诞生了一个独立的crate：ndarray-stats。这个系列的文章，将围绕ndarray来阐述。

[Read More](https://www.lpalmieri.com/posts/2019-04-07-scientific-computing-a-rust-adventure-part-2-array1/)

###  「嵌入式Rust」一个TM1637的demo

来自Rust国内社区 @洛佳。查看该Demo项目Readme，有详细的内容说明。

![img](https://github.com/luojia65/tm1637-display-demo/raw/master/img/result.gif)

[tm1637-display-demo](https://github.com/luojia65/tm1637-display-demo)

### crust - 一个Rust实现的简单C编译器

从头开始写 C 编译器，很好的学习教程。谁在学习编译原理，可以参考参考。

[Repo](https://github.com/onehr/crust)

### rust-wasi-tutorial

[帅气的 wasi 教程](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-tutorial.md) 的 rust 实现版本。跟进最新的 wasi 的同学不能错过。

[Repo](https://github.com/kubkon/rust-wasi-tutorial)

### 使用actix-web实现魔术登录链接的示例

“魔术链接（Magic links）”是每次需要登录时，Web应用程序都会生成一个新的，唯一的密码，而不是单个长期存在的密码。 然后，Web应用程序会向用户发送一封电子邮件或一条带有登录链接的短信。 此链接包含此唯一密码：短暂共享密钥。

该demo是使用actix-web 0.7。

[Read More](https://blog.approveapi.com/tutorials/rust-actix-web-approveapi-magic-login-link/)

### 使用Tokio实现自定义协议（i3 IPC）

该文作者介绍了使用tokio实现i3wm（i3窗口管理器）的IPC接口

- [Read More](https://leshow.github.io/post/impl_proto_tokio/)
- [i3wm IPC接口文档](https://i3wm.org/docs/ipc.html)

### 「系列文章」Python开发者的Rust教程 Part 2: 所有权和借用

[Read More](https://medium.com/@rajasekar3eg/rust-for-python-developers-ownership-and-borrowing-cd85fc10cae4)

### electron-wasm-rust-example: 一个最小化的Electron + WebAssembly (WASM) + 🦀 Rust的示例

[electron-wasm-rust-example](https://github.com/anderejd/electron-wasm-rust-example)

### 「视频」深入WASM和WASI

[Read More](https://www.youtube.com/watch?v=Evc3T9Zk2pk)

### 如何在Rust测试中Mock时间

- [Read More](https://blog.iany.me/2019/03/how-to-mock-time-in-rust-tests-and-cargo-gotchas-we-met)
- [rust-mock-time-demo](https://github.com/doitian/rust-mock-time-demo)

### 「系列文章」JavaScript解析和求值

该文作者在之前分享了如何使用Rust实现一个基于状态机的高性能的JS词法分析器。这之后，又开始写系列文章分享他的心得。

- [Javascript evaluator part 1: Lexing](https://medium.com/@retep007/javascript-lexing-for-high-performance-f9a800ec930d)
- [Javascript evaluator part 2: Parser and Basic evaluator](https://medium.com/@retep007/javascript-evaluator-part-2-parser-and-basic-evaluator-d306ff1aec83)
- [javascript-es9-parser](https://github.com/retep007/javascript-es9-parser)

### 使用Sonr构建pub/sub服务器

sonr建立在mio之上的网络库，相比于Tokio来说，更加轻量。

- [Read More](https://hagsteel.com/posts/building-a-pub-sub-with-sonr-part-1/)
- [sonr](https://github.com/hagsteel/sonr)
- [源码：pubsub](https://github.com/hagsteel/pubsub)
- [sonr介绍](https://hagsteel.com/posts/introduction-to-sonr/)

---


# 项目、工具与库

###  BlockLang Installer 是一款专用于部署 Spring boot 项目的自动化安装工具

来自Rust国内社区 @xiaohulu 投稿。

源代码托管在 https://github.com/blocklang/blocklang-installer

BlockLang Installer 安装在应用服务器上，支持：

```
下载 JDK 和 Spring boot Jar 文件；
安装 JDK；
启动 Spring boot Jar。
```

功能示意图： ![img](https://raw.githubusercontent.com/blocklang/blocklang-installer/master/images/installer.png)

### rust-notifica: Rust实现的跨平台系统通知工具

![img](https://camo.githubusercontent.com/b5aa604901f0f5ec3264630da5110ce6b93f668f/68747470733a2f2f692e696d6775722e636f6d2f767551486878702e706e67)

[rust-notifica](https://github.com/frewsxcv/rust-notifica)

### hunter - 终端下的文件浏览器

看起来操作性很高，比linux之前的MC好用多了。本工具受 ranger 和 emacs 启发。

![hunter](https://raw.githubusercontent.com/rabite0/hunter/master/docs/hunter.png)

[Repo](https://github.com/rabite0/hunter)

### lopdf - 用于操作PDF文档的库

[Repo](https://github.com/J-F-Liu/lopdf)

### flashback - 将adobe的SWF文件转换为SVG或WASM等格式

转换后，就不需要运行模拟器来执行swf了。开坑不久，可以参与。

[Repo](https://github.com/lykenware/flashback)

### simdjson-rs - simdjson库的Rust实现

[simdjson](https://github.com/lemire/simdjson) 这个库这段时间非常火，因为它确实很快啊，用 SIMD 指令集来加速。这个库是 rust 的实现版本，不是包装版本。

[Repo](https://github.com/Licenser/simdjson-rs)

### fluid - 一个单元测试库 发布了0.4版本

写人类可读的单元测试的框架。看起来非常不错。


```
Fact

#[fact]
fn cerberus_has_3_heads() {
    number_of_faces("Cerberus").should().be_equal_to(3);
}

Theory

#[theory]
#[case("Cerberus", 3)]
#[case("Hydra", 7)]
#[case("Janus", 2)]
#[case("Normal guy", 1)]
fn each_creature_has_a_correct_number_of_faces(name: &str, nbr_faces: u8) {
    number_of_faces(name).should().be_equal_to(nbr_faces);
}
```

[Docs](https://docs.rs/fluid/latest/fluid/wiki/index.html)
[Repo](https://gitlab.com/Boiethios/fluid-rs)


### nom-peg: 基于nom实现的PEG解析器生成器工具

和pest类似，但它是基于nom实现的。

[nom-peg](https://github.com/rust-bakery/nom-peg)

### Rust实现的IRC机器人

[url-bot-rs](https://github.com/nuxeh/url-bot-rs/)

### chit: 可在终端查询crate信息的工具

```
$ cargo install chit
$ chit serde
```

[chit](https://github.com/peterheesterman/chit)

### wasmer: 可执行wasm二进制文件的Python库

[python-ext-wasm](https://github.com/wasmerio/python-ext-wasm)

### ppcp - 带进度条的文件复制工具

cp 的替代品啊。什么都不说了，炫酷上图吧

![gif](https://raw.githubusercontent.com/acidnik/ppcp/master/demo.gif)

[Repo](https://github.com/acidnik/ppcp)

### neat-flappy-bird

NEAT算法玩Flappy Bird(像素鸟) ，群友贡献，棒棒哒！JiaYe（planet0104）

![pic](https://raw.githubusercontent.com/planet0104/neat-flappy-bird/master/images/video.gif)

[Repo](https://github.com/planet0104/neat-flappy-bird)