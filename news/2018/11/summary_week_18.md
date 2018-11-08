前言：
从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。
2018-11-04

# 官方新闻

### 「官方」来帮助测试Rust 2018

`$ rustup install beta`

[Read More ](https://blog.rust-lang.org/2018/10/30/help-test-rust-2018.html)

### 「Rust 2018」 Anchored and Uniform Paths

该文是官方团队成员withoutblogs所写。

锚定（Anchored）和统一（Uniform）路径，是当前Rust 2018中还存在的两种模块路径引入变体支持。但是官方计划在12月正式发布Rust 2018的时候只保留一种变体。

在Rust 2018版本中：

-  可以直接使用crate:: 命名空间来访问当前crate中的模块，而不是在顶层根目录下use。
-  使用第三方包的时候不需要使用 extern crate 语句
-  你可以在代码的任意地方像使用crate::命名空间这样去访问第三方包，而不再需要“::”前缀。比如，不再需要`impl ::std::fmt::Debug`，而只需要写`impl std::fmt::Debug`

锚定（Anchored）和统一（Uniform）路径区别：

- 锚定路径： 如果从子模块中导入，则需要写  `self::submodule::`。
- 统一路径：从子模块导入，可以直接 `use submodule::` 。
- 统一路径中，子模块如果和第三方依赖有相同的名称，则需要使用`use ::foo` 来消除歧义。
- 统一路径的优势是可以避免混淆，更加符合直觉。
- 锚定路径的优势： 可以消除模块来自于哪里的歧义。它们有专门的命名空间：crate、super和self。

经过官方团队的内部会议之后，该文作者认为以后应该走统一路径的方向。

- 统一路径优势很大，将为新用户消除`self::`这样的障碍。
- 虽然名称冲突可能发生，但情况比较稀有。使用`use ::` 这种方式，比使用`use self`要好的多。

官方会在未来3周内对此问题做出决定。即，withoutboats建议，在1.31版本中，只保留统一路径。

[Read More](https://boats.gitlab.io/blog/post/anchored-uniform/)


### Jemalloc已被彻底移出标准库

从Rust 1.28开始稳定了#[global_allocator]属性，从PR55238开始，Jemalloc已经被彻底移出了标准库。

如果想使用Jemalloc，则需要配合#[global_allocator]属性喝jemalloc第三方包。

现在默认是系统分配器。

[Read More](https://www.reddit.com/r/rust/comments/9twam5/jemalloc_was_just_removed_from_the_standard/)

[PR55238](https://github.com/rust-lang/rust/pull/55238)

### trait alias实现已经被merge

#trait_alias

```rust
trait DebugDefault = Debug + Default;
```

[RFC 1773](https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md)

[PR 55101](https://github.com/rust-lang/rust/pull/55101)


### 「官方」嵌入式工作组报告 14

简要：

一、 好消息：终于可以用Rust Stable开发嵌入式程序了。

因为`#[panic_handler] `登录了Rust 1.30稳定版，这就意味着可以用Stable Rust写Cortex-M的裸机程序了。cortex-m, cortex-m-rt 和 embedded-hal这几个库都可以用于Stable了。

但要注意在Discovery 和 The Embedded Rust Book这两本书中的代码还必须在1.30 beta或1.31 beta或最新Nightly版本中使用，因为它们用了2018 Edition版本。


- [Discovery](https://rust-embedded.github.io/discovery/)
- [嵌入式之书（The Embedded Rust Book）](https://rust-embedded.github.io/book/)

二、嵌入式工作组已经启动了一个Cortex-A团队，专注于为ARM-Cortex-A系列微处理器的裸机程序开发提供支持。

三、为了解决多个设备连接到同一个外设的问题，@Rahix开发了[shared-bus](https://github.com/Rahix/shared-bus)

四、嵌入式社区有了蓬勃的发展。工作组也从最初的8个人，增加到了现在的27名开发人员（分散在11个团队中，每个团队都在嵌入式开发中有自己的擅长领域）

现在是开始用Rust进行嵌入式开发的最好时机。

[Read More](https://rust-embedded.github.io/blog/2018-10-28-newsletter-14/)



---

# 社区新闻

### 2018 国内Rust社区调查报告

[Read More](https://zhuanlan.zhihu.com/p/48236630)

### 「研究项目」Shifgrethor：Rust实现的垃圾收集库

该文章有更新。

[Read More](https://zhuanlan.zhihu.com/p/47850166)

### 「Slides」禅与说服贵公司使用Rust的艺术

Rust核心团队成员Ashley Williams（@ag_bubs），同时也是crates.io、Rust社区的团队领导，WebAssembly工作组的成员分享了这篇主题。

「视频地址：」[How I Convinced the World's Largest Package Manager to Use Rust, and So Can You!](https://www.youtube.com/watch?v=GCsxYAxw3JQ)

[PPT](https://ashleygwilliams.github.io/gotober-2018/#1)

### 基于MIR的借用检查NLL已登陆Rust 2018

现在Rust 2018的crate已经默认是使用NLL了。在2018 测试版本稳定之后，官方也准备将NLL加到Rust 2015中。

NLL的目的是提升Rust的使用体验。也就是说，开发者只需要按自己的直觉去写代码，而不需要做一些特殊的行为来取悦借用检查器。极大降低了Rust的学习曲线。

Nll的诊断信息也得到了很大的提升，比之前的借用检查器更好，开发者可以看到更详细的错误提示信息。

虽然NLL现在还有一些轻微的开销，但是性能基本已经不再成为实践的阻碍了。

当前NLL运行在迁移模式下，也就是说，对于一些不遵循NLL的老代码，会报出警告信息。这只是一个过渡，假以时日，这些警告会变成硬性的错误。

Rust所有权和借用检查的未来如何？

还有一些小问题需要修复，以及应对未来人们开发过程中的新问题，并且还要加紧发布相关的文档。继续改进Polonius。等等

[Read More](http://smallcultfollowing.com/babysteps/blog/2018/10/31/mir-based-borrowck-is-almost-here/)

### Rust程序发布checklist

[Read More](https://zhuanlan.zhihu.com/p/47924519)

### 「rustsim」最后两月动态月刊#1

Rustsim组织是一个GitHub组织，聚焦于提供各种数值模拟的库。包括

- alga， 抽象代数库
- nalgebra， 线性代数库
- ncollide， 2D和3D的碰撞检测库
- nphysics， 2D和3D的物理模拟库

[rustsim.org](https://rustsim.org/)

今年最后的两个月，在改进ncollide和nalgebra，以便支持可变性物体的物理模拟。并且新增了两个crate：nalgebra-glm和nphysics-ecs。其中nphysics-ecs是打算让开发者方便将物理系统集成到ECS应用程序中。

还将改进文档。

[Read More](https://www.rustsim.org/blog/2018/11/01/this-month-in-rustsim/)

### 「oj平台」Kattis现在支持Rust了

#oj #kattis  #codewars  #leetcode #code_challenge

顺便统计一下国内外支持Rust的OJ平台。

- Codewars （支持）
- CodinGame （支持）
- CodeSignal （支持）
- codechef（支持）
- exercism.io （支持）
- atcoder.jp（支持）
—
- LeetCode（正在开发Rust支持）
-  hackerrank （不支持）
- coderbyte （目前不支持Rust）
-  geeksforgeeks （不支持）

欢迎补充

[Read More](https://open.kattis.com/help/rust)


### hashbrown：基于Google SwissTable哈希算法实现的Map

该库是parking_lot作者开发的新库， 基于SwissTable算法实现了HashMap和HashSet。

Rust标准库的HashMap和HashSet基于RobinHood散列，hashbrown基于SwissTable散列。

SwissTable整体性能要优于Robinhood，尤其是在搜索上面：

```rust
|  name           |stdhash ns/iter | hashbrown ns/iter|diff ns/iter| diff % |speedup
|find_existing    |  23,831        | 2,935            | -20,896    |-87.68% | x 8.12
|find_nonexisting |  25,326        | 2,283            | -23,043    |-90.99% | x 11.09
|get_remove_insert|  124           | 25               | -99        |-79.84% | x 4.96
|grow_by_insertion|  197           | 177              | -20        |-10.15% | x 1.11
|hashmap_as_queue |  72            | 18               | -54        |-75.00% | x 4.00
|new_drop         |  14            | 0                | -14        |-100.00%| x inf
|new_insert_drop  |  78            | 55               | -23        |-29.49% | x 1.42
```

[hashbrown](https://github.com/Amanieu/hashbrown)

[reddit讨论贴](https://www.reddit.com/r/rust/comments/9sn4ze/github_amanieuhashbrown_a_faster_hashmap_for_rust/)

[算法原理讲解视频: 《CppCon 2017: Matt Kulukundis “Designing a Fast, Efficient, Cache-friendly Hash Table, Step by Step”》](https://www.youtube.com/watch?v=ncHmEUmJZf4)

### 「Web框架」Rocket v0.4 发布了候选版本

正式版将于11月9号发布，到时候作者会写一篇完整介绍V0.4最大特色和变化的文章。

[Read More](https://rocket.rs/v0.4/news/2018-10-31-version-0.4-rc/)

### salsa：通用增量计算框架

Rust团队核心成员Niko又开了新坑，莎莎（salsa），是一个通用的增量计算框架。

然后我又看到Cargo的贡献者Yehuda创建了一个新库：[salsa-examples](https://github.com/wycats/salsa-examples)，我在猜测，Yehuda是不是想在salsa的基础上实现一个新的ORM框架呢？

[salsa](https://github.com/salsa-rs/salsa)

### Rust编译器现在已支持14个debian架构

本周新增的CPU架构为：

- mips
- mips64el
- mipsel
- powerpcspe

[Read More](https://lists.debian.org/debian-devel-announce/2018/11/msg00000.html)

---

#  学习资源

### 「杂谈」异常处理 vs 错误处理

[Read More](https://zhuanlan.zhihu.com/p/48200804)

### 查找并修复Hyper应用中的内存泄漏

[Read More](https://zhuanlan.zhihu.com/p/48011141)

### NLL之后：程序间冲突

[Read More](https://zhuanlan.zhihu.com/p/48302513)

### Cita-cli工具开发中的异步经验谈

本文是来自 @driftluo 的投稿。

他在使用全异步开发cita-cli过程遇到了自引用的问题，但他是在稳定版Rust下开发，使用的还是futures 0.1，为了解决这个问题，他给出了一个自己的思路。

[Read More](https://www.driftluo.com/article/19d035a0-f467-4f9d-a8d4-9c7096f7044d)

--

### 2D图形Rust开发：了解GPU内存管理

该篇文章在gfx-hal基础上构思了一个2D图形库的一些底层实现细节。他讨论的这些内容并非2D图形API，而是基础的组件，在此组件上面可以实现各种渲染技术，并且该组件将独立于这些渲染技术。

[ Read More ](https://nical.github.io/posts/rust-2d-graphics-02.html)


### 「Web框架」使用actix-web制作一个验证用户身份的微服务 Part 2

[前文索引](https://hgill.io/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/)

[Read More](https://hgill.io/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-2/)

###  如何使用Cargo构建Qt应用

目的是想让Qt的安装和运行更加简单

[Read More](https://www.vandenoever.info/blog/2018/10/30/building_qt_apps_with_cargo.html)

### Pyro: 一个轻巧的ECS

Pyro是一个快速的、小型的、带文档的ECS。

特点：

- 完全线性的迭代
- 组件中的组合始终存在于同一存储中
- 目前还仅支持单线程（作者说未来会加多线程支持）
- 不要用于生产环境，该库主要是用于教育的目的

[pyro](https://github.com/MaikKlein/pyro)

[Read More ](https://www.reddit.com/r/rust/comments/9srhuc/pyro_a_fast_small_and_documented_entity_component/)

### rs_pbrt发布0.4版本

rs_pbrt是对《Physically Based Rendering,PBRT(光线跟踪：基于物理的渲染) 》这本书中代码的Rust实现。

[Read More](https://users.rust-lang.org/t/rs-pbrt-v0-4-4-adds-support-for-nurbs-and-subdivision-surfaces/21801)

[rs_pbrt](https://github.com/wahn/rs_pbrt)

[www.pbrt.org](https://www.pbrt.org/)


---

# 项目

### ZoKrates: 以太坊的zkSNARKS工具箱

当前为POC项目。

zkSNARKS是一种零知识证明算法，以太坊据说要应用这种技术来提升以太坊的性能。

> Buterin 估计，ZK-SNARK 的整合和采用能够使以太坊每秒处理高达500笔交易，而目前的上限约为15笔。

[ZoKrates](https://github.com/Zokrates/ZoKrates)

### xorc-notifications：用于发送推送通知的Kafka Consumer

该项目是来自于生产环境实践的开源项目，从kafka topic读取protocol buffer的数据并将推送通知发送到apns2，fcm和web-push三个平台，也可以用来发送普通的http请求。

[xorc-notifications](https://github.com/xray-tech/xorc-notifications)


### sled是一款Rust实现的现代化嵌入式数据库

新的版本更加可靠，且实现了零Copy读取，Rust的所有权模型让引用安全地返回给缓存而无需防御性的拷贝。

[sled](https://github.com/spacejam/sled)

### 「GUI框架」Azul：面向IMGUI的免费功能性GUI框架

支持用Rust编写桌面软件，基于Mozilla WebRender渲染引擎。

目前还在完善中，期望特点（也就是说，当前这些功能并非全部支持）：

- 跨平台
- IMGUI （Immediate Mode GUI ）/ MVVM 编程模型
- 基于Dom的无状态组件
- CSS风格的样式引擎，支持多种通用CSS属性
- 基于Flexbox的布局
- 内置标准控件
- 自定义widgets
- SVG渲染引擎，2D绘图助手 (lines, circles, rects, etc.)
- OpenGL集成
- Async I/O
- 可选的集成日志和错误报告辅助
- 独立二进制部署，最小二进制大小（5MB）
- CPU使用率（0-4%），内存使用率（总共约50MB）
- 快速重绘时间（0.5~4ms），包括了高效的状态缓存

[azul.rs](https://azul.rs/)

### 「异步Web框架」Gotham 0.3发布

特点：

- 基于新的tokio库
- 采用hyper 0.12和新的http库
- 内部路由算法改进
- 自定义Gotham使用tokio运行时
- 通过tokio-fs提供异步静态文件
- 更多性能和可用性改进

[Read More](https://gotham.rs/blog/release/2018/10/29/gotham-0.3.html)



---

# 工具与库

### 「库」bytecount 0.4 发布

该库可以快速计算内存中给定字节或UTF8字符串出现的次数

新版本中支持了新的更快的算法，基于stdsimd。

[bytecount](https://github.com/llogiq/bytecount)


### 「库」slotmap 0.3 发布

该库是在[Rust 2018 Conf闭幕演讲](https://kyren.github.io/2018/09/14/rustconf-talk.html)中介绍的，基于 Generational Index模式实现的库。

新功能：

- 可以自定义Key的类型
- SecondaryMap，运行slotmap中的key和任意数据关联

[Read More](https://www.reddit.com/r/rust/comments/9s0hbk/slotmap_03_released_support_for_custom_key_types/)

### 「工具」aws_lambda_events 发布

aws_lambda_events 提供了AWS Lambda事件类型，以便与AWS Lambda事件源一起使用。

[aws_lambda_events](https://github.com/srijs/rust-aws-lambda/tree/master/aws_lambda_events)


### 「工具」SUPER 0.5发布

#android #super

SUPER 是一款Android应用程序分析工具，可以检测潜在的安全漏洞并创建漂亮的报告。

[Read More](https://superanalyzer.rocks/2018/11/03/super-0.5.0-release)

[SUPER](https://github.com/SUPERAndroidAnalyzer/super)


### 「工具」Roughenough 1.1.0 发布

该工具是基于Google的Roughtime协议的Rust实现，包含了客户端和服务端。可以用来做时间同步验证。比如区块链矿工可以用它来验证自己的时间早于其他矿工。

新版本中支持KMS、HTTP运行状况检查和服务器模糊测试

[Read More](https://www.reddit.com/r/rust/comments/9s51tp/roughenough_110_released_with_kms_support_an_http/)

[roughenough](https://github.com/int08h/roughenough)


### 「小工具」xcp：扩展cp命令

关键字: [xcp, cp]

提供进度条和.gitignore支持

[xcp](https://crates.io/crates/xcp)


### 「小工具」volkswagen: 检测测试何时在CI中执行并且让其通过

关键字： [volkswagen, ci, auto pass]

```rust
extern crate volkswagen;

#[cfg(test)]
mod tests {
    #[volkswagen::test]
    fn it_works() {
        assert_eq!(1 + 1, 3);
    }
}
```

比如这个测试，会失败。但是volkswagen会根据失败的结果，生成一个新的测试，让其通过。

目测，用这个工具的时候最好确定你真的需要它来自动生成测试，否则可能引起问题

[volkswagen](https://github.com/lukaslueg/volkswagen)

「附带八卦」

该作者起volkswagen这个名字，可能另有深意啊。

> 大众汽车「排放门」，2015年9月18日，美国环境保护署指控大众汽车所售部分柴油车安装了专门应对尾气排放检测的软件，可以识别汽车是否处于被检测状态，继而在车检时秘密启动，从而使汽车能够在车检时以“高环保标准”过关，而在平时行驶时，这些汽车却大量排放污染物，最大可达美国法定标准的40倍。







---

# 招聘两则

###  移动端图形渲染开发工程师

工作地点：北京

因为项目涉及保密，所以公司名和公司联系方式不便对外公布。

> 内容：开发iOS/Android短视频渲染框架
> 预算：30-50k
> 要求：
> 1. 熟悉Rust/C++语言，有一年以上Rust开发经验更好。
> 2. 熟练掌握OpenGL ES 3.x/Metal/Vulkan其中之一，拥有一个或以上已上线图形渲染项目开发经验。

联系方式： 82084788 [at] qq [dot] com

###  「今日头条EE部门」Rust开发工程师

工作城市： 上海

岗位描述：

> 1、开发跨平台高性能native客户端核心组件；
> 2、负责沟通及创作工具的系统设计, 实现, 优化和演进；
> 3、研究分析主流 IM，优化实现方案，改进产品功能；
> 4、负责设计和优化 IM 协议、弱网通信、推送、存储、网络并发、并行计算、加密以及安全等；
> 5、保证工程质量和开发效率。

岗位要求：

> 1、良好的系统编程能力. 喜爱或有Rust经验以及扎实的C/C++功底更佳；
> 2、喜爱关注新技术, 愿意尝试更优解决方案；
> 3、不设边界, 愿意探索了解事物运转原理；
> 4、熟悉开源社区；
> 5、热衷自动化完成事情；
> 6、关注代码设计，有持续学习习惯。

注：如有需要，可以联系我，帮转（猎头）简历。
