前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2018-12-16

---

# 「付费阅读」系列

- [如何为Rust语言做贡献](https://zhuanlan.zhihu.com/p/51479889)
- [Rust Quiz 11](https://zhuanlan.zhihu.com/p/52032027)

---

# 官方新闻

### 官方核心成员nrc对Rust 2021 Edition的思考：

[Rust 2022](https://zhuanlan.zhihu.com/p/52181046)

### 「官方网络工作组」异步进展

#async

虽然Rust 2018发布的时候没有稳定异步编程，但在今年，异步编程也取得了很大进展。网络工作组发文，介绍了几个促进Nightly生态中使用async/await的crate。

async/await对Rust的重要性：

- async/await在其他语言中已经证明了其可用性
- 在Rust中引入async/await中需要多考虑一个元素：借用。所以，之前在编写异步代码的时候，必须使用`'static`限定，并且还经常要用到Arc和Mutex。
- async/await语法稳定之后，将不会有「借用」相关的问题。但是在这之前，还有很多工作要做，比如稳定await语法。现在google的Fuchsia项目已经在大规模使用async/await语法了。
- 标准库中支持Future等trait的工作也即将完成。
- 确定了各种API，包括Waker、Pin、与tokio兼容的futures-rs 0.1等。

发布了一些新的crate：

- [romio](https://github.com/withoutboats/romio)，之前介绍过，是对tokio最小化包装，为了支持async/await。
- [http-service](https://docs.rs/http-service)， 一个基于字节的、http和新的futures API的crate，提供HTTP通用接口。是从Tide库中提取出来的API。
- Tyger，即将推出的这个库是在Hyper上面构建的一个小的crate。提供直接的HTTP服务接口，因此可以直接使用async/await。Tyger也会对Hyper增加一些补充，提供一些更高级的抽象。也是从Tide中提取出来的包。

官方希望在2019年上半年稳定async/await。官方所考虑的不仅仅是支持这个语法，而且还在为打造async/await生态做足了准备。这也是异步编程支持如此缓慢的原因。

[Read More](https://rust-lang-nursery.github.io/wg-net/2018/12/13/async-update.html)



---

# 社区新闻

### OS2ATC2018(第六届开源操作系统年度会议)会议现场直播链接 

现在的topic： 《Design & Implementation of uCore Plus OS in Rust Lang》（清华大学ucore操作系统课程升级Rust教学）

https://flypage.chinamcloud.com/h5/tpl/index.html?id=6469&tid=810

日程： http://soft.cs.tsinghua.edu.cn/os2atc2018/rc.html

### 「安全」Crossbeam发现double-free Bug

> MsQueue和SegQueue会发生此Bug。
> 即使从队列中弹出一个元素，crossbeam也会运行它，此问题来自于crossbeam-epoch的垃圾收集器内的析构函数。 

[Read More](https://github.com/RustSec/advisory-db/blob/master/crates/crossbeam/RUSTSEC-2018-0009.toml)

### crev: 代码审查工具箱

提供了一个cargo工具：[cargo-crev](https://github.com/dpc/crev/tree/master/cargo-crev)

该工具可以判断你项目中依赖crate的安全性、质量和发现的问题。可以在公共的git仓库里发布可验证的review信息。通过这种方式期望在Rust生态系统中构建可信任的网络。将不会有人再受到未经审查和不受信任代码的困扰。

想想npm因为依赖包出了多少次安全事故。这个工具ms不错，但是否真的可以解决问题？

使用方法：

```rust
cd <your-project>
cargo crev id gen # generate your id
cargo crev verify # verify your depedencies
cargo crev review <crate> # review a dependency
cargo crev db git status # check git status of your proof database
cargo crev db git -- ci -a # commit everything
cargo crev db git push # push it to your github repository
cargo crev trust <id> # trust someone with a given CrevId
cargo crev db fetch # fetch updates from all people you trust
cargo crev verify # verify again
cargo crev help # see what other things you can do
```

其中id是可以通过[crev gitter channel](https://gitter.im/dpc/crev)来共享给大家的，形成信任网络。然后可以通过 `cargo crev trust <id>`命令从你信任的人那里获取依赖crate。

当然，这世界上没有绝对的安全，但也无法阻碍人们追求它的脚步。

[crev](https://github.com/dpc/crev)

### Rust非常适合高性能科学计算

来自某粒子物理实验室的软件性能工程师Hadrien坦言，相比于Cpp，Rust更适合高性能科学计算。并且对Rust能进入这个领域之前需要完善的工作提出了他自己的建议，同时也希望Rust 2019的目标之一可以是「高性能科学计算」。

[Read More](https://gist.github.com/HadrienG2/e9a875bdf98b528594f4e20f8176bb68)

###  杭州Rust线下Meetup回放视频

一共两场分享，视频地址是第二场分享，在列表里也能找到另外一场分享的视频。

[Bilibili](https://www.bilibili.com/video/av38044021/?share_source=qq&ts=1544706808&share_medium=iphone&bbid=dd199f5b7049675783521db5317f49ba)

### 使用Rust和WebAssembly进行edge计算

什么是边缘计算？ 边缘计算将数据的处理、应用程序的运行甚至一些功能服务的实现，由网络中心下放到网络边缘的节点上。属于一种分布式计算。一直以来，公共和企业设施的监测和维护消耗着大量的人力、物力成本；电力、制造等行业数字化转型中对海量数据的实时、智能处理也有着强烈需求。如果用常规模式构建物联网，所有数据都交给云端，那么会带来一系列的问题。边缘计算就是为了解决这个问题。

[fastlylabs](https://www.fastlylabs.com/)公司，推出了一个产品Terrarium（看着像是用webassembly.studio改造的），是一个基于浏览器的多语言编辑和部署平台。据该公司描述，此产品是为了推进边缘计算。Terrarium是基于WebAssembly沙箱而构建。Terrarium可以将几种不同的编程语言编译为WebAssembly，然后将其编译为快速，安全的本地代码，并用于为Web服务提供动力。

本文介绍了如何用Rust和Terrarium进行编写边缘计算服务。

[Read More](https://www.fastly.com/blog/edge-programming-rust-web-assembly)

### 「趣味」Rust Raps: Rust 2018 Edition首张单曲发布

由Rusta Rhymes推出的热门新单曲“Ferris Crab（Rust Raps 2018 Edition）”即将推出首张专辑“Drop for Mic”。

[在线听](http://fitzgeraldnick.com/media/rust-raps.mp3 
)

[Read More](http://fitzgeraldnick.com/2018/12/13/rust-raps.html)

### Rust开发的一款编程教学游戏预览

![img](https://wx4.sinaimg.cn/mw690/71684decly1fy847pc8j5j21bu0r0x6p.jpg)

[Read More](https://blog.roboinstruct.us/2018/12/07/looking-good.html)

---

# 学习资源

### Rust Quiz解读已更新到Quiz 15

[去专栏 Read More](https://zhuanlan.zhihu.com/time-and-spirit-hut)

### Rust 异步函数内部转换流程

本文介绍了Rust内部async/await的内部转换机制，包括generator、状态转换过程等。

[Read More](https://blag.nemo157.com/2018/12/09/inside-rusts-async-transform.html)

### 使用Cargo Test来Debug代码

本文介绍了使用cargo test命令，结合单元测试来调试代码中的问题。

[Read More](https://www.wihlidal.com/blog/general/2018-12-07-debugging-cargo-test/)

### 「系列博文」在浏览器中使用WASM Part 1

本文介绍了如何使用Rust编译wasm，并在浏览器中使用它。同时也包括了wasm的工作机制，值得一看。

[Read More](https://ljcode.org/blog/wasm-part1/)

### 在AWS Lambda上面运行Rust

本文以编写一个独立的crate为例，从代码编写到部署，介绍如何在AWS Lambda上面运行Rust库。

[Read More](https://kellenfujimoto.com/posts/dicers-rust-on-lambda/)

### 「嵌入式Rust」Cortex-M3 入门指南（一）：体系概述

[Read More](https://zhuanlan.zhihu.com/p/52235675)

### 「视频」如何在Rust/C/C++/.Net中使用SIMD

[youtube](https://www.youtube.com/watch?v=4Gs_CA_vm3o)

### 用Rust为Kubernetes动态生成Dockerfiles

[Read More](https://medium.com/docql/dynamically-generating-dockerfiles-for-k8s-d2baf7bfef5a)

### 「Slides」在Rust中使用C的va_list

以及，你为什么不应该用它。

- [PPT](http://dlrobertson.com/slides/va-list-12-13-2018.html#/)
- [Read More](https://www.reddit.com/r/rust/comments/a6j4hu/using_cs_va_list_in_rust_and_why_you_never_should/)

---

# 项目

### libui-rs: libui的Rust绑定

[libui-rs](https://github.com/LeoTindall/libui-rs)

### gba: 帮助你创建GBA游戏

- [gba](https://github.com/rust-console/gba)
- [gba book](https://rust-console.github.io/gba/)

### NASA开源的静态分析工具: ikos

虽然是为C/C++静态分析实现的工具，但据说也适用于LLVM IR，因此有助于检测Unsafe Rust的代码。

- [ikos](https://github.com/NASA-SW-VnV/ikos)
- [Read More](https://www.reddit.com/r/rust/comments/a5kyz9/ikos_21_an_open_source_static_analyzer_from_nasa/)

### seed: Rust+WebAssembly 前端框架

基于wasm-bindgen和js-sys创建。创建应用时需要依赖web-sys。

[seed](https://github.com/David-OConnor/seed)

### 「区块链」comit-rs：Comit协议的Rust实现

COMIT是一个协议，以链接不同的区块链。以便将区块链生态系统打造地更大，更具有包容性。

[comit-network/comit-rs](https://github.com/comit-network/comit-rs)

### [WIP] Rust实现的Lua解释器

[luster](https://github.com/kyren/luster)

### Rust开发安全应用程序指南

该指南不是Rust语言教程，只是记录Rust开发应用程序过程中可能出现的「坑」，特别是开发一些对安全性要求较高的程序需要注意的地方。该指南还在持续更新中。

- [rust-guide](https://github.com/ANSSI-FR/rust-guide)
- [Online Read](https://github.com/ANSSI-FR/rust-guide/blob/master/src/SUMMARY.md)

### 使用Rust配置管理kubernetes

本文作者使用Rust构建了一个用于管理k8s上运行的微服务声明格式和生命周期的标准化工具shipcat。它封装了k8s的API，目的是为了更加标准化、版本化、权限化、自动化管理k8s。本文阐述了k8s管理中的问题，以及shipcat的解决思路。

- [shipcat](https://github.com/Babylonpartners/shipcat)
- [Read More](https://clux.github.io/probes/post/2018-12-15-config-management-in-rust/)

---

# 工具与库

### spirit教程

spirit可以帮助开发者更容易地构建Unix守护进程。 文章里介绍了它的使用方法。

- [spirit](https://github.com/vorner/spirit)
- [Read More](https://vorner.github.io/2018/12/09/Spirit-Tutorial.html)

### parstream： 基于线程池以流方式计算迭代函数

并且不会打乱迭代器中元素的顺序。

[parstream](https://github.com/newpavlov/parstream)

### 命令行使用频率跟踪工具fe

[fe](https://github.com/ccheek21/fe)

### pom：又一个peg解析器

[pom](https://github.com/J-F-Liu/pom)

### 支持no_std的Curve25519加密库

椭圆曲线加密/签名/密钥交换算法Curve25519的 `#[no_std]` 版本

[Read More](https://github.com/shekohex/curve25519-rs)