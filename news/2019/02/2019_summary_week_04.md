### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-02-03

---

# 全球Rust招聘信息

### 「远程工作」丹麦Impero的一家正规公司招全栈Web Rust工程师

要求全栈Web工程师，懂Rust。主要工作应该是后端API设计。可远程

- [公司招聘页面](https://impero.com/job/full-stack-web-developer-rust/)
- [Reddit讨论页](https://www.reddit.com/r/rust/comments/alzs83/rust_fullstack_developer_denmark_or_remote/)


### Discord 招聘Rust工程师

Discord游戏商店大量使用了Rust，它们现在开始招Rust工程师，来做一个顶级保密的新项目。没有看到支持远程。

[招聘页面](https://discordapp.com/jobs/4200751002)


###  「北京」移动端图形渲染开发工程师

岗位描述：

1，开发业界顶级跨平台(macOS/iOS/Android)渲染器。
2，编写Shader解析器。
3，优化Shader执行效率。

岗位要求： 

1，熟悉Rust/C++语言，有一年以上Rust开发经验更好。 
2，熟练掌握OpenGL ES /Metal/Vulkan其中两项或以上，最好熟悉SPIR-V规范。
3，拥有良好的工程意义，可编写高质量、简洁、易维护的代码。

内推邮箱，请备注简历+姓名 ： zhoujianquan 「@」kuaishou.com。欢迎推荐，转发。

### 系统工程师工作GitHub仓库

有人在GitHub上专门建立了一个帐号，建立了四个仓库：C/Cpp/Rust/Swift，用于收集世界各地用这四个语言的系统工程师招聘职位信息。

是一个新库，可以关注下，没准有需要呢？

[systems-programming-jobs](https://github.com/systems-programming-jobs)


# 官方新闻

### Rustwasm 2019 Roadmap

当前，Roadmap已经提交PR，但未合并。可以先关注。

总的来说，目标就是从「可用」到「稳定」：

- 通过模块化的工具来继续培养rust wasm的生态系统
- 为Rust生成的wasm带来多线程支持
- 在工具链中集成最佳的调试工具
- 打磨工具链和发布流程，直到wasm-pack发布1.0版

[Read More](https://github.com/fitzgen/rfcs-1/blob/2019-roadmap/text/000-2019-roadmap.md)

### Cargo的长期计划

Cargo负责人nrc发文

- 2019年：交叉编译，包括wasm和embeddded / 改进cargo，让插件更易编写
- 2020：将Cargo嵌入到构建系统和IDE中。
- 2021：为最终用户定制工作流程。

[Read More](https://www.ncameron.org/blog/cargos-next-few-years/)

### 「官方」Rust基础设施团队正在考虑停止使用Travis CI

将在下周柏林举行的Rust All Hands会议上讨论更换到哪个CI平台上面。文章里还罗列了Rust基础设施团队在Travis CI碰到的诸多问题，感兴趣可以看看。

[Read More](https://internals.rust-lang.org/t/which-ci-platform-should-rust-use/9322)


---


# 社区新闻

### RFC：为log增加dbg!风格的宏

#log #dbg #debug

[Read More](https://github.com/rust-lang-nursery/log/pull/317)


### Crossbeam 2019: 无锁Rust

文章从介绍Crossbeam开始，先后罗列了从Crossbeam库从2015走到2018期间经历的变化，并展望了接下来重点要完成的工作：

- AtomicReference
- ConcurrentHashMap

增加这两个并发数据结构的支持。另外还有一些优先级较低的零碎工作。

[Read More](https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html)


### Salsa: 增量式重新编译

Niko介绍了他的新库：salsa。该库是将rustc中的增量式重新编译技术提出来变成一个通用的框架。Salsa现在已经被用在了一些项目中，比如rust-analyzer。但rustc本身并没有使用该库。Niko还录制了两个使用Salsa的视频教程。

[Read More](http://smallcultfollowing.com/babysteps/blog/2019/01/29/salsa-incremental-recompilation/)


### Rust在基准测试排行榜中上升到了第一

在以下几个算法测试项目中，Rust都战胜了其他语言，排行上升到了第一

- [N-Body](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/nbody.html)
- [spectral-norm](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/spectralnorm.html)
- [reverse-complement](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/revcomp.html)
- [Reddit 讨论贴](https://www.reddit.com/r/rust/comments/akgmef/rust_nbody_benchmark_ranks_1/)

### 另一个性能测试

该作者制作了一个Ruby脚本，包括了多种语言的基准测试（C，C ++，Rust，Go，Java和C＃，但可以添加更多），重新标准化了平均值。

得出的结论是：Rust在基准测试游戏中的表现优于C++ 3％，比C慢4％。

[Read More](https://www.reddit.com/r/rust/comments/akluxx/rust_now_on_average_outperforms_c_in_the/)


### 「嵌入式Rust」实验：STM32 MCU 性能如何？

[Read More](https://nercury.github.io/rust/embedded/experiments/2019/01/27/rust-embedded-02-measuring-the-clock.html)


### 在TiKV中使用tower-grpc

TiKV负责人siddontang写的一篇文章。TiKV团队之前自己实现了一个grpc库，但问题是该库。不是纯Rust实现，在生产环境中遇到了一些panic，所以想彻底拥抱Rust社区，使用纯Rust的库。但是又不想发明轮子。所以从grpc-rust和tower-grpc中选择了tower-grpc。

tower-grpc并不稳定，但是它的好处也是比较明显的。比如作者是非常活跃且知名的，基于tokio等。

另外：TiKV已经升级到了Rust 2018 Edition

[Read More](https://medium.com/@siddontang/use-tower-grpc-for-tikv-6109cf8c61)

### GNome中的流媒体广播应用Gradio将用Rust重构

重构后的项目叫Shortwave

- [Shortwave](https://gitlab.gnome.org/World/Shortwave)
- [Read More](https://blogs.gnome.org/haeckerfelix/2019/01/26/hello-world/)

### 现代C++和游戏开发的一些思考

这篇文章和Rust没关系，但是可以看看CPP的现状。

长文，但是作者总结了两个观点：

- 要么，你什么都不要做。也就是说，你可以继续使用Cpp，但不要用它的任何新功能。继续用C++98。但这不是一个长期的解决方案。
- 要么，参与到Cpp的变革中。来参见CppCon，来参与Cpp委员会的讨论和议程，发出你的声音。

其实还有第三个选择：看看Rust。（作者并没有说）

[Read More](http://www.elbeno.com/blog/?p=1598)

### Apple为Swift语言申请了专利

该文作者阐述了对Apple为Swift语言申请专利的担忧。该专利覆盖了Swift语言的整体理念。比如：

> 该语言在某些领域提供C语言兼容性的功能，以提高用该语言编写的软件的固有安全性。新语言包括默认安全注意事项，例如边界和溢出检查。

大家如何看待？不知道会对Rust带来何种影响。

- [Read More](https://www.i-programmer.info/news/98-languages/12495-apple-patents-swift.html)
- [Reddit 讨论贴](https://www.reddit.com/r/rust/comments/al2xoy/apple_patents_a_programming_language_with/)


### 「系列文章」WebAssembly算不上一个栈虚拟机 Part I

本文作者指出了当前WebAssembly设计的缺陷，表面是一个栈虚拟机，但实际上在编译层面是一个性能不算好的（liveless，对变量没有活跃期分析，而导致重复计算）寄存器式虚拟机，只是在最后才表现的像一个栈虚拟机。这个缺陷是来自于之前的设计规范是基于进一步简化asm.js而导致的历史原因。所以现在出现wasmtime这样的流式WebAssembly编译器来解决这种问题。

作者在后续还会写几篇关于WebAssembly问题的文章。

[Read More](http://troubles.md/posts/wasm-is-not-a-stack-machine/)


### GitHub Actions: 自动format代码格式

这是一个GitHub Action，支持对多种语言的软件的代码进行自动格式化。支持Rustfmt和Clippy。

[actions](https://github.com/bltavares/actions)

### 「讨论」你认为Rust代码组织有哪些最佳实践或陷阱？

我个人是一个代码组织的实践就是：组件化。这也是Rust所倡导的。在单个组件之内，进行模块化，分清层次结构。

讨论中，也有喜欢扁平化的结构，这样方便测试，直到找到足够的理由再分拆单个组件包。

大家有什么分享的？

[Reddit 讨论贴](https://www.reddit.com/r/rust/comments/alsph9/rusts_modules_and_project_organization_best/)


### 「讨论」读Rust代码比写Rust代码更容易

我也同意，Rust的代码可读性很好，读代码比写代码更容易。并且还可以总结出一些高效阅读代码的技巧，这完全得益于Rust的高度一致性。

讨论中提到了Cpp、Python、JS等其他语言的可读性探讨。这种争论应该还算可以接受，感兴趣可以关注下。

[Reddit 讨论贴](https://www.reddit.com/r/rust/comments/aloxsz/reading_rust_should_be_easier_than_writing_rust/)


---

# 学习资源

### Rust「无悔」并发

无畏（fearless）并发难道已经过时了？现在是要流行无悔（regretless）并发了吗？

作者认为，无畏并发并不意味着「没有后悔」的时候。作者在本文中主要探讨了event-loop并发建模的诸多好处，并且如何利用Rust多线程来进行event-loop建模。

[Read More](https://medium.com/@polyglot_factotum/rust-regret-less-concurrency-2238b9e53333)


### Rust web框架比拼

这个GitHub项目汇集了Rust各种web框架的相关资料，包括Rust实现的前端WASM框架、Websocket、Template等项目。

还包括了很多博客、项目demo等学习资源。相当全面了。

[rust-web-framework-comparison](https://github.com/flosse/rust-web-framework-comparison)

### 「系列文章」Rust如何发送邮件 

作者将通过三篇文章来讲解如何用Rust编写邮件发送的代码。

- [1/3 Read More](https://blog.1aim.com/post/002-mail-1-intro/)
- [2/3 Read More](https://blog.1aim.com/post/003-mail-2-crate/)

### Blog OS系列： 高级分页

这是继上篇内存分页介绍之后，进一步介绍操作系统内核如何访问物理页的技术，通过这种技术可以实现虚拟地址到物理地址的转化，以及如何在页表内创建映射。

[Read More](https://os.phil-opp.com/advanced-paging/)

### 如何用Rust开发iOS应用

该文章介绍了如何使用Rust开发iOS应用，里面也有一份Rust on Android的文章链接。

[Read More](https://medium.com/visly/rust-on-ios-39f799b3c1dd)


### Rust闭包的秘密

作者总结了Rust闭包中的一些规则，可以看看。

[Read More](https://medium.com/@earthengine/rust-closures-secret-life-70d2394c5827)

### 「视频」直播合集：构建WebAssembly/WebGL渲染器

作者在油管做了一系列的直播，这个视频是这些直播的合集，所以比较长，大约3小时多。

- [Read More](https://www.youtube.com/watch?v=7_5DX_lH0kI&list=PLkzdeKCVtKYshqmgngLSqRV4UVt4QjaZ5)
- [相关代码： pure3d](https://github.com/dakom/pure3d)

### inferno: 火焰图工具移植为Rust

上次介绍过油管开直播视频讲如何移植火焰图工具那位作者的库

- [inferno](https://github.com/jonhoo/inferno)
- [flamegraphs](http://www.brendangregg.com/flamegraphs.html)


### 「长文」Rust性能：match vs 表查找

该文作者在研究生物信息学(bioinformatics)算法的过程中，发现将Rust的match改成表查找，算法的性能得到了很大的提升。[参见](https://github.com/luizirber/nthash/pull/2)

将下面的match代码：

```rust
fn match4(x: u8) -> u64 {
    match x {
        b'A' => 1,
        b'B' => 2,
        b'C' => 3,
        b'D' => 4,
        _ => 0,
    }
}
```

修改为：

```rust
use lazy_static::lazy_static;
lazy_static! {
    static ref LOOKUP4: [u64; 256] = {
        let mut l = [0; 256];

        l[b'A' as usize] = 1;
        l[b'B' as usize] = 2;
        l[b'C' as usize] = 3;
        l[b'D' as usize] = 4;

        l
    };
}

fn lookup4(x: u8) -> u64 {
    LOOKUP4[x as usize]
}
```

并且使用了lazy_static。

作者好奇，为什么编译器没有为match表达式生成最佳代码。于是他开始了一系列的测试和挖掘，从生成的汇编到LLVM都进行了一番考究。

[Read More](https://kevinlynagh.com/notes/match-vs-lookup/)


### 使用systemd、配置文件和.deb二进制构建Linux Web服务器

通过cargo-deb，将你的web项目生成.deb文件，并配合systemd和配置文件，就可以把tide、actix-web或rocket等项目，变成像nginx那样的“Linux App”。就可以通过系统命令来启动你的服务。

```
sudo systemctl restart tide-server
```

[Read More](https://gill.net.in/posts/creating-web-server-deb-binary-with-rust/)

### FFI：在Node和Rust之间交换数据

本文探讨了如何在Node和Rust之间传递数组、结构体等数据，以及如何使用回调。对于学习FFI也是一个好的案例。

[Read More](https://versbinarii.gitlab.io/blog/posts/node-rust-ffi-exchanging-data/)


### 使用rust和uinput创建Android虚拟输入设备

[Read More](https://brunodmt.github.io/rust/2018/11/03/android-virtual-input-with-rust.html)

### 用AWS和Rust进行Serverless App开发

[Read More](https://versbinarii.gitlab.io/blog/posts/serverless-app-deployment-aws/)


---

# 项目、工具与库

### tactical： wasm游戏

之前介绍过，基于Rust实现的2D回合制游戏zemeroth，现在支持wasm

- [itch.io在线玩](https://ozkriff.itch.io/zemeroth)
- [zemeroth](https://github.com/ozkriff/zemeroth)


### whatlang-rs: Rust实现的自然语言检测库

基于Rust和Wasm实现。可以检测语言属于哪国语言。

[whatlang-rs](https://github.com/greyblake/whatlang-rs)
[在线demo](https://www.greyblake.com/whatlang/)

### OOProxy: OpenID代理库

该文作者介绍了OOProxy库，支持OpenID和OAuth2。

[Read More](https://medium.com/hal24k-techblog/announcing-ooproxy-b041bab2bc85)

### 「嵌入式Rust」构建支持嵌入式系统的Future Executor

作者在实现一个模块化机械键盘的项目，在这个过程中，实现了一个可用于嵌入式的Future exector。为什么要自己实现？因为发现现在的嵌入式生态并不能满足他的需求。

- [项目Read More](https://gitlab.com/polymer-kb/polymer/blob/master/README.md)
- [stm32f103xx-futures](https://gitlab.com/polymer-kb/firmware/stm32f103xx-futures/blob/wip/src/serial.rs)
- [博文](https://josh.robsonchase.com/embedded-executor/)
- [embedded-executor](https://gitlab.com/polymer-kb/firmware/embedded-executor)

### Pushrod： Rust的UI库

是属于Piston项目中的跨平台UI Weight库

[rust-pushrod](https://www.github.com/KenSuenobu/rust-pushrod/)

### websocat: 命令行webscoket客户端工具

[websocat](https://github.com/vi/websocat)

### log-derive: 方便加log的宏工具

```rust
#[logfn(Err = "Error", fmt = "Failed Sending Packet: {:?}")]
 fn send_hi(addr: SocketAddr) -> Result<(), io::Error> {
     let mut stream = TcpStream::connect(addr)?;
     stream.write(b"Hi!")?;
     Ok( () )
 }
```

可以自动生成log输出，跟踪函数调用，可用于调试代码。

[log-derive](https://github.com/elichai/log-derive)

### Rust实现Redux

且支持no_std

[redux-rs](https://github.com/redux-rs/redux-rs)


### Fluent-rs 0.5发布

ProjectFluent是一个本地化（L18N）框架，fluent-rs是Rust实现。

- [Read More](https://projectfluent.org/)
- [fluent-rs](https://github.com/projectfluent/fluent-rs)
- [Fluent Project Wiki介绍](https://github.com/projectfluent/fluent/wiki)


### 用Rust实现Haskell中的group_by

[slice-group-by](https://github.com/Kerollmops/slice-group-by)

### 更短的UUID实现

[shorter-uuid-rs](https://github.com/seigert/shorter-uuid-rs)

### rust-numext: 扩展Rust的内建数字类型

包含了大数和散列操作。

[rust-numext](https://github.com/cryptape/rust-numext)

### derive_more  发布0.14版

现在支持no_std环境

[derive_more](https://github.com/JelteF/derive_more)

### 「嵌入式Rust」Rust实现的显示器驱动

 `Holtek CO₂ USB `显示器的驱动程序

[co2mon](https://github.com/lnicola/co2mon)

### insta: snapshot测试库

一个Rust的快照测试库，给第一次跑测试的结果生成一个快照，后面就可以用这个快照来保证代码不会被破坏。

[insta](https://github.com/mitsuhiko/insta)


---


# Rust周边

### 「Rust周边」订制活动预报

想要的朋友，可以联系我。感兴趣可发我邮件，地址: 247o26628 at qq.com。注明你想要哪种周边，目前还清楚价格，只是收集意愿，达到一定数量才会开启订制。等年后，再宣布具体的款式和价格，图案都会订制成Rust相关，但质量会和它一样的。

一： 棒球帽

![img](https://wx3.sinaimg.cn/mw690/71684decly1fzplfsr5t9j20u012eqec.jpg)


二： 书签

![img](https://wx3.sinaimg.cn/mw690/71684decly1fzpm3n7hq9j20au0hkq8r.jpg)
![img](https://wx3.sinaimg.cn/mw690/71684decly1fzpm4ijiorj20lc180q6b.jpg)