### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-02-24

---

# 通告两则

### 「正式介绍」 首届 RustCon Asia大会将在北京举行

#RustConAsia

讲师议题申请仅剩最后一周，大家抓紧。

- [Read More](https://mp.weixin.qq.com/s/vBKiFdNoCat3I9NYdV5yIA)
- [早鸟票](http://www.huodongxing.com/event/6479456003900)
- [讲师议题报名](https://cfp.rustcon.asia/events/rustcon-asia)

### 「通告」Rusty 棒球帽团购活动

发货日期： 满10人就开团了，下单以后一周左右。

截止日期： 2月28号。 28号以后就不要再问还能不能定制了。

- 材质：毛腈，Rust logo使用刺绣工艺
- 大小： 头围 56~62，均码，可调节大小
- 颜色三种： 黑、黄、橘 （参见图1~3，后两张是店家的实物效果图）
- 一顶帽子配套两个镭射标贴： Rust吉祥物螃蟹和Rust logo各一个，可以贴到帽子上，或者你想贴的其他地方。

价格构成：一共50元。

- 帽子成本： 30 （10人以上的价格）
- 镭射标签:  5元 （两个一共）
- 邮费预付： 15（看地区，多退少补），或者邮费到付也可以。

价格透明，此次定制活动的初衷是因为我自己喜欢，所以想找人平摊费用，我并没有收取中间差价，但也不能我自己亏本。

意向购买者，给我发邮件 247026628「at」qq.com ，按下面要求填写：

- 详细的收件地址和电话
- 数量和颜色

不写收获地址或者数量和颜色，我也没办法给你定。

图样：

![img](https://wx2.sinaimg.cn/mw690/71684decly1g0g9ge01jjj20d40bm0xo.jpg)
![img](https://wx2.sinaimg.cn/mw690/71684decly1g0g9gg9jw7j20co0bon2f.jpg)
![img](https://wx1.sinaimg.cn/mw690/71684decly1g0g9gilelvj20ci0bo0y0.jpg)
![img](https://wx1.sinaimg.cn/mw690/71684decly1g0g9h8tk2aj21400u0jtl.jpg)
![img](https://wx1.sinaimg.cn/mw690/71684decly1g0g9hnd0fnj20zk0qojte.jpg)

---

# 官方新闻

### 「官方」Rust官方团队变更说明

#RustTeam

Rust官方团队发出变更说明：

- Nick Cameron（nrc）离开了Mozilla加入了PingCAP，但依然会共同领导Cargo团队。
- Aaron Turon决定退出核心团队，以便专注于工程工作和语言设计团队。他依然留在Rust团队中。
- Ashley Williams加入了Cloudflare并计划在那里开展Rust和WASM集成。她将留在核心团队。
- Steve Klabnik离开了Mozilla，但仍留在核心团队中。

[Read More](https://blog.rust-lang.org/2019/02/22/Core-team-changes.html)


### 「官方思考」Rust团队在2019年将要做什么

[Read More](https://zhuanlan.zhihu.com/p/57478131)

### Generators II: 关于「问号语法糖」的问题

这是无船同志对于Generator MVP设计的第二篇文章。在第一篇文章里提到，Generator如何返回除`()`之外的类型，所以在这篇文章中探讨了Generator与问号语法集成的问题和解决思路。

- [Read More](https://boats.gitlab.io/blog/post/generators-ii/)
- [Reddit 讨论](https://www.reddit.com/r/rust/comments/arygiw/generators_ii_the_question_mark_problem/)


### RustConf 2019 FCP也启动了

这是官方举办的RustConf 2019，在4.22和23号举办。

国内的朋友可以看看RustConAsia。

- [RustConf:Read More](https://cfp.rustconf.com/events/rustconf-2019)
- [RustConAsia](https://mp.weixin.qq.com/s/vBKiFdNoCat3I9NYdV5yIA)
- [RustConAsia: 早鸟票](http://www.huodongxing.com/event/6479456003900)
- [RustConAsia: 讲师议题报名](https://cfp.rustcon.asia/events/rustcon-asia)

### 嵌入式工作组报告第15期

本期总结了Rust All Hands 2019讨论会上的要点，这些要点非常明确，和其他工作组相比，足够细化，比如具体稳定哪个API之类。

[Read More](https://rust-embedded.github.io/blog/newsletter-15/)

### WebAssembly 报告 11期

[Read More](https://rustwasm.github.io/2019/02/21/this-week-in-rust-and-wasm-011.html)


---

# 社区新闻

### 「视频」可视化Rust编译器演进历史

[Read More](https://vimeo.com/317852618)

### xi-editor最新进展

Xi Editor的作者raphlinus写的博文，记录了他最近的一些工作进展：

- Rust中的GUI：他最近的主要工作是将XiEditor中用的原生Rust GUI库druid迁移为跨平台。其中重点是2D图形抽象库piet，已经达到了最小可用阶段。piet库基于Cairo 2D图形引擎实现跨平台。
- 文本布局。这是Rust GUI生态缺失的部分。Raphlinus开了个新坑：「low level text layout」  
- 在Mac上运行Druid，是大目标，目前只有druid-shell可以运行。
- 考察了winit，决定自己实现跨平台的窗口创建库。Raph认为winit适合3D游戏，但不适合一般的GUI。不使用winit的另一个原因是VST，他需要更精细化的访问窗口创建过程。
- MarkDown解析库pulldown-cmark项目目前遇到了困难，Raph认为该代码库存在根本性的问题，所以他重启了一个分支（new_algo）。

- [Read More](https://raphlinus.github.io/personal/2019/02/20/more-small-updates.html)
- [druid](https://github.com/xi-editor/druid)
- [piet](https://github.com/linebender/piet)
- [Project roadmap: low level text layout](https://docs.google.com/document/d/1aw41q_izail-p99mN8dHrJeh9tMQ-Pldi54W6m7MHU8/edit)
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)

### 给Rust提交100个PR是什么感觉？

该文作者学习Rust一年，这一年内给Rust提交了100个PR。他写下此文作为总结。其中大部分PR是给Clippy做贡献，接下来他也打算继续给Clippy做贡献。

大家学习Rust是不是也可以定这样一个小目标呢？

- [Read More](https://phansch.net/2019/02/18/onehundred-rust-prs/)
- [100个PR列表](https://phansch.net/onehundred/rust/)

### C和Rust实现网络协议的比较练习

该文作者用C和Rust分别实现了网络协议demo，并对它们做了比较：

- Rust实现花了400行代码，依赖12个crate
- C版本911行代码和另外140行头部声明，依赖libuv和openssl

本文作者是C工程师，他用C游刃有余，但是对于用Rust实现，他认为把太多时间都用在和编译器斗争上面了。他也体验到了Rust的一些好处，比如编译通过的代码更加可靠，而C代码则需要反复测试。但是它认为，C代码即便写错了，反复修改的反馈回路也会更短，因为Rust编译时间太长（也许是因为作者编译C习惯了，所以无法忍受Rust的编译速度）。他其实更喜欢C的简单性，所以他想，也许有Go对他来说就够用了。他认为Rust试图又做底层工作，又提供了更高级的抽象，然而他没有感受到这样做的好处是什么？

（个人观点：文章描述还是客观，这篇文章是C工程师的视角，我认为可能他还是带着C语言思维和体验去看待Rust，比如，他已经习惯了在代码「编译以后再发现错误再修改」这种「常规」行为，面对Rust编译器的种种限制，反而会有些不耐烦，而没有意识到Rust编译器这样做的原因是什么。Reddit讨论贴里有很多评论者也和我持有相应的观点：即便你是一个C语言熟手，要用Rust，还需要适应一段时间Rust的概念。）

- [Read More](https://ayende.com/blog/185859-A/comparing-c-and-rust-network-protocol-exercises?Key=bd1ba87d-6e7e-4739-824d-0ca6fc232b05)
- [Reddit 讨论](https://www.reddit.com/r/rust/comments/atevrm/comparing_c_and_rust_network_protocol_exercises/)

### itch.io上面使用Rust实现的游戏合集

[Read More](https://itch.io/c/449652/rustlang-games)

### 寻找有空闲时间的Rust开发者参与开源项目

纯开源项目，没有商业化，于2013年启动，现在寻找一些Rust开发者参与游戏开发。看上去好像「我的世界」类的沙盒多人多角色RPG游戏。

- [Read More](https://www.reddit.com/r/rust/comments/aje3bs/looking_for_rust_programmer_with_lots_of_free_time/)
- [veloren/game](https://gitlab.com/veloren/game)


### Lyon 发布0.13版本

Lyon是一个基于GPU的2D图形渲染库。作者在本文回顾了2018年Lyon的进展。

[Read More](https://nical.github.io/posts/lyon-2018.html)

### 生命周期可视化讨论

本文作者探讨了为Rust编辑器或IDE实现生命周期可视化的一种方案，期待能有人接受他的想法，做出更优秀的Rust工具。去年有人实现了一个Atom编辑器插件，提供了生命周期可视化原型。但是随着NLL的引入，之前的那个工具应该不太适合使用了。所以作者设计了一种新的可视化方案。

[Read More](https://blog.adamant-lang.org/2019/rust-lifetime-visualization-ideas/)

### Filecoin的复制游戏：最快的复制算法对抗赛

Filecoin的复制证明（Proof of Replication）游戏，以鼓励参与者通过调试参数、完善代码等方式促进复制证明的优化。什么是复制证明？找了一篇介绍文：「IPFS:Filecoin和复制证明」。

- [replication-game](https://github.com/filecoin-project/replication-game)
- [IPFS:Filecoin和复制证明](https://zhuanlan.zhihu.com/p/32809642)

### Holo正在招聘Rust工程师（100%远程工作）

Holochain，区块链公司。

[Read More](https://holo.host/careers/rust-developer/)

该公司也发布了Holochain开发者预览版

[Read More](https://medium.com/holochain/developer-preview-0-0-4-alpha-and-enabling-full-authority-over-data-for-holo-users-3cc8794855d4)

### Rust Weekly本周周报

包含了本周TiKV的issues，看到有两个easy的，如果愿意可以选择趁手的提交PR。

[Read More](https://this-week-in-rust.org/blog/2019/02/19/this-week-in-rust-274/)

### 使用Rust管理嵌入式物联网设备中的安全漏洞

该文作者是一名专门从事嵌入式系统的安全顾问。他认为Rust肯定会在物联网、网络和嵌入式开发中占有一席之地，但没有人可以100%地完全杜绝漏洞，Rust也不行。因此，Rust安全代码工作组已经开始关注，如何利用Rust的内置解决方案来进一步提高关键代码的保护。比如，在生产环境中，如何知道有哪些安全漏洞需要更新？这也是Rust安全工作组在2019年要解决的问题，Rust要及时地向任何类型的生产部署提供安全更新。

除了Rust团队之外，其他安全单位的工作人员也在思考此类问题，并且有一个安全研究项目（SECONDS），该文作者及其他很多安全公司的人参与了该项目。一共评估了非Rust语言实现的37000个开源组件，发现了96321个漏洞。平均每个组件2.5个漏洞。

SECONDS项目的报告也可以作为一个现成的参考，可以丰富RustSec安全数据库。另外，Rust社区也提供了cargo-crev安全审计工具。

[Read More](https://medium.com/@flundstrom2/manage-security-vulnerabilities-in-embedded-iot-devices-with-rust-14aeabada68b)


---

# 学习资源

### 「视频」实现TCP Part I

作者介绍：

Jon，目前是MIT的一名Phd，带领小组团队实现了Noria项目（一个并发数据库）。他的导师戏称他：东半球Rust第一人（参加他的另一个演讲视频：Rust at speed — building a fast concurrent database）。他经常直播Rust视频，最早日报中介绍过的是他直播Rust异步的视频课程，长达4小时。今天这个视频长达5小时，是讲解如何用Rust实现用户态的TCP协议，遵循RFC793和RFC 1122。

- [Read More](https://www.youtube.com/watch?v=bzja9fQWzdA)
- [源码：rust-tcp](https://github.com/jonhoo/rust-tcp)


### 为Minix平台交叉编译Rust代码

Minix启发了Linus实现Linux的微内核。该文作者尝试将Rust标准库移植到Minix平台，以便Rust代码在Minix下运行。

> So there were two things I did that summer. Nothing. And read the 719 pages of 《Operating Systems: Design and Implementation》. ---  摘自Linus Torvalds自传《Just For Fun》

- [Read More](https://iandouglasscott.com/2019/02/18/cross-compiling-rust-code-to-minix/)
- [rust-minix](https://github.com/ids1024/rust-minix)

### 通过实现Punchtop来学习Rust的思考

Punchtop是该文作者用Rust和JS实现的一款使用Chromecast作为音频输出设备的音频游戏。大约有3700行Rust代码。适合新人去学习。文章中总结了他的学习心得。

[Read More](https://hyperbo.la/w/reflections-on-learning-rust/)

### Diesel核心开发者在一个旧的issues下澄清了社区对Diesel的一些误解

值得一看。

[Read More](https://github.com/rust-lang/rfcs/issues/798#issuecomment-465324544)

### rs-pbrt v0.5.1发布日志

rs_pbrt是对《Physically Based Rendering,PBRT(光线跟踪：基于物理的渲染) 》这本书中代码的Rust实现。

[Read More](https://www.rs-pbrt.org/blog/2019-02-19-v0-5-1-release-notes/)

### Rust FFI简要指南

[Read More](https://rushsteve1.us/wp/getting-started-with-rust-ffi/)

### Rust中的Builder模式

该文章介绍了Rust常用的设计模式：Builder模式。

[Read More](https://oribenshir.github.io/afternoon_rusting/blog/building-rust)

### 为什么要用Rust写WebAssembly？

这篇短文告诉你用Rust编写WebAssembly的优势，要点：

1. 其他语言具有必须包含在WebAssembly二进制文件中的巨大的运行时。
2. Rust更加安全和方便。

[Read More](https://opensource.com/article/19/2/why-use-rust-webassembly)

### 在Linux上为macOS交叉编译Rust程序

当你身边仅有一台Linux机器，如何编译到macOS呢？这篇文章告诉你怎么做。

[Read More](https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html)

---

# 项目、工具与库

### Mesh：可避免灾难性内存碎片的内存分配器

Mesh目前用于C/C ++应用程序的压缩内存管理，它是malloc的替代品。论文中显示，未来计划支持Go和Rust。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/arkbic/mesh_compacting_memory_management_for_cc/)
- [论文](https://arxiv.org/abs/1902.04738)
- [代码](https://github.com/plasma-umass/Mesh/)

### Kingston Crabfighting： 汇编指令猜谜游戏

基于「ryanisaacg/quicksilver」实现，quicksilver是一个纯Rust实现的2D 游戏引擎 ，支持桌面和Web（wasm32-unknown-unknown）

[Read More](http://rickyhan.com/jekyll/update/2019/02/17/kingston-crabfight.html)

### textalyzer： 可视化的命令行文本分析工具

目前只限分析英文，如果你学习Rust不知道该做什么项目，不妨做一个支持中文的类似的工具?

[textalyzer](https://github.com/ad-si/textalyzer)

### anevicon: 强大的基于UDP的负载生成器

负载生成器是产生虚拟用户（实际压力），用于性能测试场景。当在一台电脑上无法模拟大量的虚拟用户的时候，就可以通过多个Load Generator来完成大规模的性能负载。

[anevicon](https://github.com/Gymmasssorla/anevicon)

### 「嵌入式Rust」embedded-sdmmc-rs: 该库支持读取FAT格式的SD卡

[embedded-sdmmc-rs](https://github.com/thejpster/embedded-sdmmc-rs)

### askama 0.8发布

Askama，一个受Jinja（python模板引擎）启发的新的Rust模板渲染引擎，新版本性能提升了2倍。

[Read More](https://github.com/djc/askama/releases/tag/0.8.0)


### yarte: 号称最快的模板引擎

新库，采用handlebars风格的语法。不过，另外一个模板库Askama作者曝出yarte涉嫌抄袭Askama。但yarte的性能看上去更好一些。

- [yarte](https://github.com/rust-iendo/yarte)
- [rust-template-engine-benchmarks-in-nightly](https://github.com/rust-iendo/template-benchmarks-rs#rust-template-engine-benchmarks-in-nightly-)

### dgraph-rs: Rust实现的Dgraph客户端

Dgraph 是一个可扩展的，分布式的，低延迟的图数据库。该客户端使用gRPC和服务端通信。依赖的库是grpcio。

[dgraph-rs](https://github.com/Swoorup/dgraph-rs)

### codealong: 为项目提供可视化数据

codealong提供了一组开源工具，帮助你搜集项目的数据来进一步了解团队状态和项目进展。基于Rust和ELK和Kibana实现。

[codealong](https://github.com/codealong/codealong)


### wasm-module: 方便操作dom的js库

基于Web IDL实现，为Rust或C等其他语言提供语言无关的Dom功能。

[wasm-module](https://github.com/richardanaya/wasm-module)
