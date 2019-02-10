### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-02-10

---

# 通告

### Rust日报祝大家新的一年诸事六六六

![img](https://wx2.sinaimg.cn/mw690/71684decgy1fzvfs66uf8j20sg0mynmk.jpg)


### 「招聘」NEAR寻找Rust P2P网络工程师

NEAR是一个智能合约和区块链平台，要求至少有Rust和Tokio的经验。可能无法Remote。

[Read More](https://nearprotocol.com/careers/?gh_jid=4205573002)


---

# 官方新闻

### rustc：向更好的优化器迈进

rustc引入mir已经有近三个年头了，虽然对Rust代码有了很好的优化工作，但实际上还有很多优化工作没有完成。在即将举办的2019 All Hands会议上，mir的优化将成为一个重要议题。

[Read More](https://kazlauskas.me/entries/the-road-to-bestest-optimiser.html)

### 「官方」Aturon宣布退出Rust Core Team

Aturon宣布退出Rust Core Team，意味着他将不再参与Rust工作组的管理工作，他更想作为一名Rust工程师做一些工程工作。他将继续留在Mozilla的Rust团队。

接下来他的精力将分配在：

- 完善编译器、GAT和特化。
- 继续研究Tide框架。

Aturon的退位，同时也是Rust Core Team汲取新的血液的机会。Rust即将进入新的阶段，成熟与可持续性，有新人加入，意味着有新的思维，拭目以待。

[Read More](https://internals.rust-lang.org/t/aturon-retires-from-the-core-team-but-not-from-rust/9392/2)


---

# 社区新闻

### PingCap Rust培训计划草案出炉

该课程名为「Rust网络应用实践（Practical Networked Applications in Rust）」，目前列出了课程草案计划。该课程结构受MIT分布式系统课程的影响，在实践项目中进行Rust的学习。

[草案PR]( https://github.com/pingcap/talent-training/pull/1)
[课程计划]( https://github.com/brson/talent-training/blob/master/rust/plan.md)

### rust-guide： 法国国家信息系统安全局开放了Rust安全开发指南

法国ANSSI（国家信息系统安全局）开放的Rust指南，它并不是Rust教程，而是一份使用Rust进行安全应用程序开发的示例和建议。有时间我会把它翻译成中文。

- [Reddit 讨论贴](https://www.reddit.com/r/rust/comments/aotp7d/french_national_cybersecurity_agencys_guide_to/)
- [官方网站报道](https://www.ssi.gouv.fr/en/actualite/be-part-of-anssis-new-guide-to-develop-secure-applications-with-rust/)
- [rust-guide](https://github.com/ANSSI-FR/rust-guide)

### 「安全审计」审计Rust加密库的第一步

该文章提供了一份审计清单，在你在开始审核使用Rust编写的加密软件时，你可以通过检查以下几项内容来保证一定的安全性：

- 查看clippy警告。
- 构建并运行所有单元测试。
- 查看Cargo.toml的依赖项。
- 寻找unsafe代码块，并作出安全评估。
- 寻找unwrap的使用风险。
- 寻找潜在的整数溢出（Debug模式下会在运行时panic，但是release模式下会静默）。
- 寻找是否存在不应该被公开的私有类型。
- 查找任何递归函数，评估它们是否有栈溢出的风险。
- 如果使用FFI，则需要查找调用的外部代码，如果相关，则需要将这些代码作为审计的子项目。
- 确定可以便于进行模糊测试的API，并记下来。
- 查找使用了哪些加密原语，哪些第三方库，并记下加密组件的任何新实现。
- 查找任何涉及加密和安全的RNG内容。`rand::thread_rng`在大多数情况下应该没问题，但是当OS RNG初始化失败之后，将回归到弱RNG（弱随机数生成）。
- 对于敏感值在使用后是否应该归0？在Rust里应该使用Drop trait来实现它，而不是显式地指定。

此列表并不是详尽无遗的，所以在对Rust代码做安全审计的时候，最重要的是理解代码的逻辑。

[Read More](https://research.kudelskisecurity.com/2019/02/07/auditing-rust-crypto-the-first-hours/)

### 「讨论」Web开发: Rust vs Haskell 

[Reddit 讨论贴](https://www.reddit.com/r/rust/comments/an11l9/webdevelopement_rust_vs_haskell/)

### 「讨论」寻找Rust的生产效率和故障率的相关数据

有人在Reddit寻求关于Rust生产效率和故障率的相关数据，评论中有人贴出了一些数据，以及经验。值得看看。

[Reddit 讨论贴](https://www.reddit.com/r/rust/comments/aohq6u/rust_velocity_and_defect_rates/)

### Fuchsia和Zircon的Rust接口

Zircon是Google操作系统的内核，之前这个内核名字叫Magenta，现在改名为Zircon，是C/C++开发的。
有了这个Rust API，意味着可以使用Rust来开发额外的系统代码。

[Read More](https://fuchsia-docs.firebaseapp.com/rust/auth_cache/index.html)

### 「讨论」我是否应该放弃Java而转投Rust？

这哥们最近在面试一个工作，但是公司要求他必须学习Rust。但他只会Java，所以来Reddit求助，到底是否应该学习Rust呢？Rust前景如何？

Reddit的朋友给了他很多建议。感兴趣的可以看看。

总的来说，大伙基本都认为Rust的光明未来。

[Reddit 讨论贴](https://www.reddit.com/r/rust/comments/aospj8/switching_to_rust_from_javaspring/)

### 「系列文章」WebAssembly的麻烦

日报之前介绍过第一篇文章：WebAssembly算不上一个栈虚拟机 [Part I](http://troubles.md/posts/wasm-is-not-a-stack-machine/)

下面是后两篇。

- [Part 2](http://troubles.md/posts/why-do-we-need-the-relooper-algorithm-again/)
- [Part 2](http://troubles.md/posts/the-stack-is-not-the-stack/)

### Datafusion捐赠给了Apache Arrow

Datafusion是一个Rust-Native的查询引擎，现在被用于Apache Arrow中。

[Read More](http://arrow.apache.org/blog/2019/02/04/datafusion-donation/)

### Swift 5 Release版支持独占访问内存检查

之前该功能只支持Debug，现在支持Release。这篇文章中阐述了此功能对Swift内存安全和性能策略的重要性。

这其中inout功能明显受到Rust中可变借用的启发。这是Swift引入所有权机制的基础，在Swift的所有权宣言文档中看到，Swift的共享值概念和Rust里的不可变借用是相似的。想到现在Swift又被苹果申请了专利，后续会对Rust有什么影响？还未可知。

- [Read More](https://swift.org/blog/swift-5-exclusivity/)
- [所有权宣言](https://github.com/apple/swift/blob/master/docs/OwnershipManifesto.md)

### Rust Weekly 看点介绍

Rust Weekly除了每周的博客和新闻和日报有所重复之外，也有另外的看点值得关注。

- Crate of the Week： 推荐每周的明星crate
- Call for Participation： 推荐一些可以开源做贡献的issues
- Updates from Rust Core： 关于Rust Core的一些更新动态
- Approved RFCs： 本周审核通过的一些RFC列表


[Read More](https://this-week-in-rust.org/blog/2019/02/05/this-week-in-rust-272/)

### 使用Rust挑战Roguelike游戏比赛

Roguelikes是指程序生成的Rogue模式的RPG游戏，死亡是永久性的。2005年，roguelike社区建立了一年一度的活动，即7DRL挑战赛，开发者在七天内挑战创建这样一个roguelike。距离新的挑战赛开始还有18天。

所以，有人用Rust实现了一个可以开发支持Web和Desktop的Roguelike游戏模板库quicksilver-roguelike，方便你使用Rust挑战。

- [reddit 讨论贴](https://www.reddit.com/r/rust/comments/aowv02/writing_a_rust_roguelike_for_the_desktop_and_the/)
- [7drl-challenge-2019](https://itch.io/jam/7drl-challenge-2019)
- [quicksilver-roguelike](https://github.com/tomassedovic/quicksilver-roguelike)

### 使用MPSC Channel在GTK中轻松使用线程

使用gtk-rs的人，在IRC或其他交流平台经常出现的一个问题就是：「如何从另一个线程修改UI的状态？」。由于GTK只允许主线程访问其UI状态，所以才出现这种问题。所以作者为gtk-rs实现了一个新的API：一个类似于Rust标准库的与GLib/GTK主线程集成的MPSC Channel API，方便开发者处理这个问题。该文是作者对这个过程的经验之谈。

[Read More](https://coaxion.net/blog/2019/02/mpsc-channel-api-for-painless-usage-of-threads-with-gtk-in-rust/)


---

# 学习资源

### 使用main函数的问号语法糖自定义程序退出码

作者为此还创建了一个独立的crate：exit

- [Read More](https://www.joshmcguigan.com/blog/custom-exit-status-codes-rust/)
- [exit](https://github.com/JoshMcguigan/exit)


### 浏览器插件：用于自动定向官方Rust Book的过期链接

真是有心人

[trpl-redirect](https://github.com/srishanbhattarai/trpl-redirect)

### 「Blog OS系列文章中文翻译」使用Rust创造操作系统（一）：独立式可执行程序

社区朋友 @洛佳 出品

- [Read More](https://zhuanlan.zhihu.com/p/53064186)
- [合集地址](https://github.com/luojia65/writing-an-os-in-rust)

### 「视频」将火焰图移植到Rust Part 2

本次视频，依然是5个多小时。

[Read More](https://www.youtube.com/watch?v=Qy1tQesXc7k)

### 有人尝试将worley-noise库转成了WASM

并且做了一个在线的demo。

worley-noise是一个Rust实现Worley Noise算法的库。该算法也被称为Cell Noise，属于计算机图形学方面的算法。该算法用于生成符合一定规律的花纹，比如花豹、奶牛、长颈鹿身上的花纹，或者是河滩干裂、干涸的盐碱湖形成的图纹，等等。该算法也被用于仿真地理地形生成。

- [worley-noise](https://gitlab.com/Sogomn/worley-noise)
- [webworley](https://gitlab.com/Sogomn/webworley)
- [online demo](https://sogomn.gitlab.io/noise/)

### 「私人分享」Rust奇技淫巧

作者在文章中分享了他总结的Rust的一些奇技淫巧。值得看看。

- [Read More](https://vorner.github.io/2019/02/03/hacks.html)
- [Reddit 相关讨论](https://www.reddit.com/r/rust/comments/ampf8d/personal_collection_of_rust_hacks/)

### 用Rust写一个Neovim插件

[Read More](https://medium.com/@srishanbhattarai/a-detailed-guide-to-writing-your-first-neovim-plugin-in-rust-a81604c606b1)

### 为Node开发者准备的Rust教程

该教程最近升级到了第二版，跟进了Rust 2018，适合前端人员学习。反过来，懂Rust的，是不是也可以借此教程了解下Node呢？

[rust-for-node-developers](https://github.com/Mercateo/rust-for-node-developers)

### 「嵌入式Rust」让LED开始闪烁

本篇博文介绍了如何基于STM32平台，使用Rust让LED灯闪烁。

（顺便求一本二手的《嵌入式系统：硬件、软件及软硬件协同》，谁有闲置可以联系我。）

[Read More](https://jonathanklimt.de/electrics/programming/rust-STM32F103-blink/)

### 「嵌入式Rust」用Rust实现电子纸名片

该文简单记录了作者用Rust实现电子纸名片的相关内容，并且还有源码。帅！

![img](https://wx2.sinaimg.cn/mw690/71684decly1fzxvaiw0nzj20ug0mo4qp.jpg)

- [Read More](http://www.wezm.net/technical/2019/01/linux-conf-au-rust-epaper-badge/)
- [linux-conf-au-2019-epaper-badge](https://github.com/wezm/linux-conf-au-2019-epaper-badge)

### Rust实现相对指针(Relative Pointers)

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/aokwyp/poc_relative_pointers/)
- [Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=aeadf83622b18966308f8eae6e07d2b7)
- [相对指针相关视频](https://www.youtube.com/watch?v=Z0tsNFZLxSU)

---

# 项目、工具与库

### notion：一个“省心”的JavaScript工具链管理器

#javascript

「使用简单，没有麻烦」，基于Rust实现。特点：

- 可靠：从Node版本到软件包管理器，可确保项目中的每个人都使用相同的开发环境。
- 通用：在每个shell和每个主要操作系统中享受相同的命令行体验。
- 快速：Notion使用Rust实现，并作为静态可执行文件部署以获得最佳性能。

[notion](https://github.com/notion-cli/notion)

### flutter-rs: Flutter桌面运行器

让Dark和Rust一起玩耍。

[flutter-rs](https://github.com/gliheng/flutter-rs)


### torrents.csv: bt种子存储服务

基于Rust, ripgrep, Actix, Inferno, 和 Typescript实现。

[torrents.csv](https://gitlab.com/dessalines/torrents.csv)


### 「嵌入式Rust」osaka.rs: 构建嵌入式异步生态

Osaka.rs相当于面向嵌入式设备的Tokio。作者在博文中介绍了Osaka的来龙去脉。

- [Read More](http://aep.github.io/rust-async-without-the-noise/)
- [osaka](https://github.com/aep/osaka)

### gfx-rs出品的WebGPU实现

基于gfx-hal实现的WebGPU实现。

[wgpu](https://github.com/gfx-rs/wgpu)

### sandboxfs 0.1.0 发布

该库作者是Google员工，他利用Google 20%的自由时间来实现该项目。

这是一个FUSE（用户态）文件系统，最初用Go实现，但是性能上无法满足，作者又学习了一年Rust之后，将其用Rust重新实现。当然还有一部分Go代码。

而且作者说，在用Rust实现的过程中，发现了之前Go实现代码中的一堆并发错误。新的Rust版本比Go版本至少在最初的测试上是更快一些。

- [Read More](http://julio.meroh.net/2019/02/hello-sandboxfs-0.1.0.html)
- [sandboxfs](https://github.com/bazelbuild/sandboxfs)

### pickledb-rs: 轻量级简单KV存储 0.3发布

[pickledb-rs](https://github.com/seladb/pickledb-rs)

### 「嵌入式Rust」可用于Rust和树莓派交叉编译的Docker镜像

支持的是Raspberry Pi Zero。作者认为Rust非常适合该版本的树莓派。所以制作了这个docker镜像。

[rust_armv6](https://hub.docker.com/r/mdirkse/rust_armv6)

### alloc-counter: 一个内存分配分析工具

该项目号称是对qadapt（另一个类似的工具，提供了debug_assert!宏）的重新设计。

- [Read More](https://gitlab.com/sio4/code/alloc-counter)
- [qadapt](https://github.com/bspeice/qadapt)

### TMQ: ZeroMQ的Tokio绑定

作者刚发布了0.1版本

- [Read More](https://cetra3.github.io/blog/tmq-0-1-0/)
- [tmp](https://github.com/cetra3/tmq)

### metered-rs: 帮助你测量生产项目的性能

[metered-rs](https://github.com/magnet/metered-rs)

### 「嵌入式Rust」bno055： Bosch Sensortec BNO055 9轴传感器Fusion IMU驱动程序

Bosch Sensortec是九轴运动传感器品牌，BNO055是产品型号，IMU是惯性量测单元。主要用于VR产业，防眩晕，采集运动数据之类。

[bno055](https://github.com/eupn/bno055)

### Accepted: 一个终端文本编辑器

[Accepted](https://github.com/hatoo/Accepted)

### astro: 又一个用Rust实现的新语言

号称可以用于开发快速原型设计和高性能的应用，目前是WIP状态。

[astro](https://github.com/astrolang/astro)

### procs: 代替ps命令的工具

[procs](https://github.com/dalance/procs)

### heapless: 提供了不需要动态分配的数据结构

[heapless](https://github.com/japaric/heapless)

### thank: 展示你项目中依赖的crate信息

一个有趣的crate.

[thank](https://github.com/brown121407/thank)

### sns-push-notifications：用于使用AWS SNS给iOS和Android推送通知

[sns-push-notifications](https://github.com/davidpdrsn/sns-push-notifications)

### fit-rs: 用于读取和解码运动设备生成的FIT文件

目前支持Garmin Edges 1000和520设备（Garmin出品的，用于骑行的GPS智能码表）。

[fit-rs](https://github.com/richardbrodie/fit-rs)
