### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-01-27

---

### 《Rust编程之道》相关内容三则

勘误说明：

[勘误列表](https://github.com/ZhangHanDong/tao-of-rust-codes/issues)，同时也是随书源码。大家有什么问题反映上去就好了。最近我在忙别的事，不能及时回复，不过我每一条都会看。

最终会形成一份电子版的勘误集《Rust编程之道•求真集》。「求真」，是为了致敬这些认真的读者，也是表达准确传播知识的信念和责任。写书之时，开始动笔之前，我就已经知道错误在所难免。不惧错误，只有正视错误，才能让自己成长，这是我一贯坚守的信念。

大家也可以私聊我，加入读者交流群。

[读者答疑精选：对书中值语义、引用语义、栈拷贝、按位复制等概念的澄清](https://zhuanlan.zhihu.com/p/55572688)

随书源码仓库新增了「精选」Label，涉及比较重要的勘误内容，大家看书的时候可以了解了解。

![img](https://wx4.sinaimg.cn/mw690/71684decgy1fzhkz30n56j213r0u04qp.jpg)

---

# 官方新闻

### Rust异步开发支持又前进了一小步

RFC: stabilize `std::task` and `std::future::Future` 准备Merge了

[Read More](https://github.com/rust-lang/rfcs/pull/2592)

### 「Rust异步」Weaker API何去何从？

无船大佬写了两篇博客，来讨论Waker API。考虑要统一Waker API，而去掉之前的LocalWaker，可能引来一些争议。因为在某些OS下，无法使用TLS。

- [The Waker API I: what does a waker do?](https://boats.gitlab.io/blog/post/wakers-i/)
- [Waker II: waking across threads](https://boats.gitlab.io/blog/post/wakers-ii/)


---

# 社区新闻

### 约翰•卡马克说他写了点Rust。

没错，是传奇程序员「约翰•卡马克」。

这个推文实际上是Cpp vs Rust的小范围论战之一，Reddit上讨论贴，

![img](https://wx1.sinaimg.cn/mw690/71684decly1fzkzqt2ukjj21n60t0anr.jpg)

[Read More](https://www.reddit.com/r/rust/comments/ak69zp/john_carmack_started_writing_some_rust/)

### 「Job」Prevoty寻找高级C/CPP/Rust工程师

Prevoty是一家网络安全公司，提供运行时监控和保护程序。

Prevoty正在寻找一位才华横溢（Talented）的高级软件工程师加入他们的团队。

位于洛杉矶。有竞争力的福利包括股权，综合健康福利，(误：以为401K是薪水，经查，401k是社保。 401(k)退休福利社保。 😂）。

[Read More](https://www.prevoty.com/about/careers?gh_jid=4032159002)

###  rust-analyzer指南

Rust Analyzer是给IDE使用的Rust实验性模块化编译器前端。也是RLS的潜在替换者。

[Read More](https://github.com/rust-analyzer/rust-analyzer/blob/e0d8c86563b72e5414cf10fe16da5e88201447e2/guide.md)

与此相关新闻：

sourcegraph移除了对Rust的支持

代码搜索和智能托管服务Sourcegraph宣布移除对Rust的支持，原因是RLS现在还不够完善。因为性能的关系，Sourcegraph需要等待RLS的初始化时间（编译整个项目，需要几分钟）得到改善，或者就是可以希望Rust-analyzer能替换掉RLS（可能需要等几年），然后才考虑支持Rust。Sourcegraph对初始化时间非常敏感。

[Read More](https://github.com/sourcegraph/sourcegraph/issues/7)


### “实现请求”列表

Serde作者dtolnay在GitHub上发起了一个列表，旨在让人们提交那些「本应该存在但还不存在」的库，请求实现它。

“实现请求”列表适用于那些已经进行了明确、清晰、成熟的大部分设计工作的想法，它应该准备好可以让参与者直接开始编写代码。

[request-for-implementation](https://github.com/dtolnay/request-for-implementation)


### quinn 发布 0.2版

Quinn是QUIC协议（传说中的HTTP3， IETF标准化的下一代TCP替换协议）的纯Rust实现，并且兼容Tokio API。并且已经与另一个Rust的QUIC实现quicr合并。

Quinn 0.2.0是最新QUIC草案（草案17）中最符合要求的实现之一。Quinn项目正在努力实现模块化开发，比如拆解出独立协议逻辑的quinn-proto库和可以利用tokio API的quinn库。

[quinn](https://github.com/djc/quinn/releases/tag/0.2.0)

### nrc博客宣布加入PingCAP

作为在「Rust内部」工作了五年的人，现在跳到「Rust外部」来使用它，并且面向的是分布式数据库这样一个新兴的领域，nrc感到很兴奋。并且他将继续在新西兰远程工作。

（我很希望一睹nrc的真容，因为很多年前nrc给Rust社区做贡献的时候，就只看到他的大猩猩头像，很好奇他长啥样）

[Read More](https://www.ncameron.org/blog/starting-at-pingcap/)

### 请享受QUIC和Rust吧！

cloudflare公司开源了他们用Rust实现的QUIC协议库，可以和C/C++及其他语言方便集成。

可以用它方便构建QUIC Server。但目前还不是很成熟，可以先关注。

- [Read More](https://blog.cloudflare.com/enjoy-a-slice-of-quic-and-rust/)
- [quiche](https://github.com/cloudflare/quiche)

### 「宏」proc_macro2库的文档已经更新

[/0.4/proc_macro2](https://docs.rs/proc-macro2/0.4/proc_macro2/)


### NLL之后： Polonius 和 区间错误

Polonius是Niko一直在研究的新的借用检查器库，还实验性地被集成到了Rust中，但它目前还不完整。

该文主要介绍了Polonius对「核心借用检查」分析中忽略的检查：不考虑生命周期之间的关系。

如感兴趣可以查看原文了解。

- [polonius](https://github.com/rust-lang-nursery/polonius/)
- [Read More](http://smallcultfollowing.com/babysteps/blog/2019/01/17/polonius-and-region-errors/)

### Niko: Polonius与Hereditary Harrop谓词

Niko在这篇博文里探讨了Polonius目前的不足。为了解决高阶子类型和trait匹配的问题，需要寻求更丰富的约束概念（存在量词等）。 这篇太学术。

不过，Polonius是Niko受面向逻辑编程语言prolog的启发而实现，这里面的术语Hereditary Harrop就是来自于它。感兴趣的可以深入了解下。

- [wiki/Harrop_formula](https://en.wikipedia.org/wiki/Harrop_formula)
- [Read More](http://smallcultfollowing.com/babysteps/blog/2019/01/21/hereditary-harrop-region-constraints/)

### 「社区提案」Unsafe Rust社区代码规范

[Read More](http://sanxiyn.blogspot.com/2019/01/proposed-rust-community-norm-for-unsafe.html)

### DataFusion 0.6发布

DataFusion是一个在Rust中实现的内存查询引擎，它的目标是构建分布式计算平台。

目前使用Apache Arrow作为内存模型。但目前它只是一个概念证明，还不适合实际应用。 目前支持针对CSV文件运行简单的SQL查询。DataFusion也支持Parquet。 Rust Parquet项目最近被捐赠给Apache Arrow，作者希望在DataFusion中进行持续的集成工作。作者最近提议将DataFusion捐赠给Apache Arrow项目，以成为Arrow中的默认Rust-native查询引擎。

[Read More](https://andygrove.io/2019/01/datafusion-0.6.0/)

### Rust 1.31 vs Rust 1.32 基准性能测试

Rust 1.32中完全移除了jemalloc，所以有人想看看，1.32到底性能如何？这次性能测试是基于[Rust vs Cpp Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/faster/rust-gpp.html)来做的。

结果：

![img](https://wx4.sinaimg.cn/mw690/71684decly1fzhj4foy7dj20s40jytbw.jpg)

目测1.32略胜一筹？

[Read More](https://www.reddit.com/r/rust/comments/aj2pwx/comparison_of_rust_131_vs_132_in_the_benchmarks/)

### 「宣传向」微软的 Azure IoT Hub设备流服务使用Actix实现

Actix的作者fafhrd91就是在Azure IoT团队中。

[Read More](https://www.reddit.com/r/rust/comments/aj3fi7/azure_iot_device_streams_built_with_actix/)

---

# 学习资源

### Rust-for-undergrads： 一个鼓励大学生将C/C++问题重新用Rust实现的项目

这个项目去年2月份介绍过，今天看到它上了GitHub趋势榜。

[Rust-for-undergrads](https://github.com/rustindia/Rust-for-undergrads)

### Rust实现LC3虚拟机

Little Computer 3或LC-3是一种计算机教育编程语言，一种汇编语言，是一种低级编程语言。

它具有相对简单的指令集，但可用于编写中等复杂的汇编程序，并且是C编译器理论上可行的目标。该语言不如x86程序集复杂，但具有许多类似于更复杂语言的功能。这些功能使其对于开始教学非常有用，因此它最常用于向计算机科学和计算机工程专业的学生讲授编程和计算机体系结构的基础知识。

想写自己VM的可以跟着学习。

相关系列文章：[Write your Own Virtual Machine](https://justinmeiners.github.io/lc3-vm/index.html#1:12)

[Read More](https://github.com/KuldeepSinh/lc3_vm)

### 将Serde类型导出为TypeScript的类型

利用wasm-bindgen和serde序列化库，让Rust和Webassembly的开发体验更上一层。

1. 使用serde序列化，可以让前端和Rust端在开发过程中数据的更改自动同步，减少手工更改的工作量。
2.  可以同时对Rust代码和前端组件进行类型检查，在编译期间而不是在运行时捕获类型错误。

考虑到WebAssembly运行时错误的调试难度，这一点尤为重要！

[Read More](http://timryan.org/2019/01/22/exporting-serde-types-to-typescript.html)

### 使用Rust创建Ubuntu Touch应用程序

基于Rust和Qt。

[Read More](https://timsueberkrueb.io/posts/2019/01/18/rust-ubuntu-touch-app-dev/)

### 基于Rust 1.32实现静态文件服务器

[Read More](https://medium.com/@zwegrzyniak/attempting-a-bare-bones-static-file-server-in-rust-1-32-d784545b64b0)

### Rust+WebGL 基础指南

[Read More](http://www.chinedufn.com/3d-webgl-basic-water-tutorial/)

### 「视频」将火焰图移植到Rust

上次讲异步的那哥们，又出了视频。他每次视频都是四到五个小时。

[Read More](https://www.reddit.com/r/rust/comments/ak6s89/porting_flamegraph_to_rust_video/)


### Rust web生态 vs Python Flask Web 生态

ORM：

- Rust：Diesel，（如果用acitx-web，推荐 https://github.com/mehcode/actix-diesel 这个库，抽象的非常干净漂亮）
- Python: SQLAlchemy

I18N:

- Rust: ? （原帖中说Rust没有等价的库，但实际上有，rocket-i18n，也有rust-unic和tr 这样的国际化库）
- Python: Flask-Babel

登录/邮件/表单处理/密码验证等组件：

- Rust: ? （这个确实没有什么通用的组件，但这其实不算啥重要的，actix-web写登录和发邮件都挺方便，也有很多库可以帮助处理密码验证，至于表单处理，rocket和actix-web都支持提取器非常方便，但可能也有瑕疵，比如actix-web的Form提取器不支持数组参数之类 ）
- Python: Flask-Login/Flask-mail

（总的来说，和flask生态相比，目前actix-web和rocket只差在通用的业务组件上面）

具体可以看讨论

[Reddit 讨论](https://www.reddit.com/r/rust/comments/ai3p1z/the_rust_web_ecosystem_vs_the_flask_ecosystem/)

### 「系列文章」在Rust中使用TLS 之 用Tokio处理异步IO

- [Read More](https://ayende.com/blog/185793-A/using-tls-in-rust-going-to-async-i-o-with-tokio)
- [using-tls-with-rust系列](https://ayende.com/blog/posts/series/185698-A/using-tls-with-rust)

### 「Hack技」基于类型反射来构造编译时可确定variant的枚举类型

[Read More](https://guiand.xyz/blog-posts/compile-time-unions.html)

### 「Cli」分形查看器

该库实现了分形算法，并将其生成图片，方便查看。该库比较适合新人学习Rust时模仿和借鉴。

[mandelbrot-viewer](https://github.com/agherzan/mandelbrot-viewer)

### 将论文中的Haskell示例移植为Rust

论文《A Play on Regular Expressions》中使用Haskell实现了连续八个强大的正则表达式引擎，同时使用来自集合理论的数学结构的GADT表示来实现从解析结果中提取数据，称为半环。

作者昨天用Haskell实现了前两个引擎，今天将第一个版本移植到Rust。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/ajj8xq/a_small_project_taking_an_academic_paper_with/)
- [代码（包括论文地址）](https://github.com/elfsternberg/riggedregex)


---

# 项目、框架、工具与库

### typetag: 序列化/反序列化trait对象

Dtolnay大佬这么高产，又开了一个新库，是将trait对象无痛地序列化和反序列化。

[typetag](https://github.com/dtolnay/typetag)

### mnemonic: 帮助你记忆各种终端命令的工具

mnemonic(助记符)是一个小的Cli应用，目的是帮助开发者记忆那些难记的shell命令。

![img](https://github.com/codesections/mnemonic/raw/master/mn.gif)

[mnemonic](https://github.com/codesections/mnemonic)


### n-sql: Sql解析/优化/生成器

目前支持Oracle, MySQL, PostgreSQL, SqlServer, sqlite等关系数据库。作者 来自于国内Rust社区成员 @ng

作者说：

>  n自然是numbers of的意思，目标是尽量覆盖所有数据库，主要目标是在查询上面，nosql后面再扩展吧。

[n-sql](https://github.com/mokeyish/n-sql)


### oxide-auth: 0.4发布

一个OAuth2 Server库，支持actix、rocket等框架。

[oxide-auth](https://github.com/HeroicKatora/oxide-auth)

### warmy 0.11.1中支持通用JSON

warmy是一个Rust编写的可实现热加载/重载的库： [warmy](https://github.com/phaazon/warmy)

支持同步和异步加载

[介绍](http://phaazon.net/blog/asynchronous_warmy_prequel)

[Read More](https://phaazon.net/blog/warmy-universal-json)

### 在Rust应用中嵌入Webassembly

wasmer-runtime库提供了易于使用的安全API，用于帮助开发者在自己的库中调用WebAssembly。

[Read More](https://medium.com/wasmer/executing-webassembly-in-your-rust-application-d5cd32e8ce46)


### 使用Rust构建JavaScript开发工具

作者构建了三个crate，希望为开发者创造一套用于处理JavaScript的通用Rust库。

- RESS，用于词法扫描，标记化词条
- RESSA，用于语法分析，解析语法树
- RESW，用于编写上层代码

作者提供了一份简单的book来说明这三个crate的用法。

[rusty-ecma-book](https://freemasen.github.io/rusty-ecma-book/)

### arraystring： 基于栈的通用字符串

谨慎使用，该库使用了很多unsafe代码。不过适合学习。

[arraystring](https://github.com/paulocsanz/arraystring)

### ryu: 快速浮点数到字符串的转换

dtolnay的新库。是对一种快速将浮点数转换为十进制字符串的算法的纯Rust实现。算法相关论文：

- [Ryū: fast float-to-string conversion](https://dl.acm.org/citation.cfm?id=3192369)
- [ryu](https://github.com/dtolnay/ryu)