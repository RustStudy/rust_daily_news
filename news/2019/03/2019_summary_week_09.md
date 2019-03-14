### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust日报，分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2019-03-10

---

# 通告 

### Rusty棒球帽周边定制活动状态报告

- 帽子还在制作中，店家应该周末会发货给我。
- 镭射贴第一次做的不太满意，又让店家重新制作了。再多等两天。

预计下周可以发货。

### [北京][2019年4月20日] RustCon Asia ：第一届 Rust 亚洲技术大会

[Read More](https://ruby-china.org/topics/38200?from=timeline&isappinstalled=0)

---

# 官方新闻

### Rust 2019 Roadmap 即将发布

[Read More: RoadMap RFC](https://github.com/rust-lang/rfcs/pull/2657)

### 「嵌入式Rust」嵌入式工作组报告 第16期

摘要：

- [James Munns发布了一系列使用Rust开发家庭无线传感器节点网络的视频，专注于IoT主题](https://www.youtube.com/playlist?list=PLX44HkctSkTewrL9frlUz0yeKLKecebT1)
- [工作组正在收集的带有视觉效果（图片或视频）的酷炫嵌入式Rust项目展示](https://github.com/rust-embedded/showcase)
- 一些库的发布信息

[Read More](https://rust-embedded.github.io/blog/newsletter-16/)

### Rust Tool Team的变化

主要是独立出来一个核心（core）工具组作为领导，来解决子工具组的各种协调和规划问题。

[Read More](https://internals.rust-lang.org/t/tools-team-changes/9569)

### Rust语言工作组会议跟踪贴

本贴记录了Rust语言工作组会议相关跟踪记录，感兴趣的可以关注，还会有讨论的油管视频放出。

[Read More](https://internals.rust-lang.org/t/lang-team-working-group-sync-meetings/9573)

---

# 社区新闻

### 「讨论」Rust类型系统可以保证内存空间上限吗？

该贴作者是做大数据的，使用Spark。Spark因为输入数据的大小变化和GC的原因，导致运行时内存会产生难以预测的波动。他接触了Rust之后产生了这个问题，Rust的类型系统能否减少或消除运行时内存的不确定性？

[Read More](https://www.reddit.com/r/rust/comments/ayu6un/could_rusts_type_system_guarantee_memory_space/)

### Rust实现的T1HA可以达到40GiB/s

T1HA是Fast Positive Hash的实现，基于SIMD在avx2平台上可达到40GiB/s的性能。

[Read More](https://www.reddit.com/r/rust/comments/ayla9m/rust_implementation_for_t1ha_fast_positive_hash/)

### 整理还未使用Futures 0.3的库

作者整理这个列表的目的是为了促进0.3的稳定，这些库还在用0.1，如果一直不用0.3，那么futures 0.3就很难更快地稳定。另外也方便社区的开发者为其做贡献。

[Read More](https://www.reddit.com/r/rust/comments/aygqrg/libraries_missing_for_futures_03/)


### Oxide：无限接近Rust所有权和借用本质的形式化语义模型

Rust是工业编程语言的一个重大进步，它重点是弥合了底层系统编程和高级应用程序编程之间的鸿沟。但目前Rust编程的缺点是缺乏一个用于证明Rust程序的有效工具，之前也有很多类似的工作但是不太理想。

该论文展示了Oxide了形式化语义，重新使用类型系统构建了Rust所有权和借用这个核心机制，来推理Rust的行为，并且希望研究人员能将其作为Rust工作的基础。

[氧化：Rust的本质（Oxide: The Essence of Rust）](https://arxiv.org/abs/1903.00982)

Arxiv平台Rust相关论文整理：

- [Rust在安全领域的应用： Angora: Efficient Fuzzing by Principled Search](https://arxiv.org/abs/1803.01307)
- [Rust实现的Tsetlin Machine]( https://github.com/KhaledSharif/TsetlinMachine)
- [Tsetlin Machine 对模式识别优化的论文地址](https://arxiv.org/abs/1804.01508)
- [Rust加密API的可用性如何？](https://arxiv.org/pdf/1806.04929.pdf)
- [使用Javascript安全地管理Rust数据的生命周期](https://arxiv.org/pdf/1807.00067.pdf)
- [优化编译器的未来方向](https://arxiv.org/pdf/1809.02161.pdf)
- [Mesh：可避免灾难性内存碎片的内存分配器](https://arxiv.org/abs/1902.04738)

### Google编程之夏项目

有Rust相关的项目入选，包括TiKV，Servo， Tokio-rs等，可以选择参与贡献。

- [GSoC](https://summerofcode.withgoogle.com/)

### Deno一瞥：JavaScript/TypeScript运行时

什么是Deno？

- 使用Rust实现
- JavaScript和Typescript运行时
- 实现ES5模块

可以简单地把Deno看作是Node.js的替代品。但是Deno旨在实现和浏览器相同的功能。你可以用Deno实现一个浏览器和服务器都可以使用的程序。Deno的前景可以，但是目前还属于早期阶段。

[Read More](https://43081j.com/2019/01/first-look-at-deno)


### 「招聘」「新加坡」使用Rust构建下一代支付系统

新加坡TenX公司，成长型创业公司，产品是基于云的支付平台，并且支持数字货币。其他消息不详。不过据说新加坡支付行业竞争比较激烈。

[Read More](https://functional.works-hub.com/jobs/software-engineer-in-singapore-singapore-c9d67?utm_source=reddit&utm_medium=job-tenx&utm_campaign=t.leland)

### torchbear: 为Speakeasy编程语言实现的编程环境

Speakeasy编程是一门新语言，该组织建立它的目的是为了解决更通用的问题。

> 只关注语法，语义，生态系统指导等，不包括解释器或编译器。Speakeasy旨在为每个开发人员提供更轻松，更高效的软件开发;经验丰富的退伍军人，好奇的用户，成人，儿童等。我们的范围包括来自其他语言和学科的课程和概念;我们将把目标放在一个简单，有序的开发者体验上。

Torchbear为Speakeasy编程语言提供了一个简单但功能强大的通用解释器。它可以帮助用户进行Web自动化，嵌入式编程，数据分析，数值计算......其他能想到的。目前，以Lua语言为教学语言。没有编程背景的人可以在15分钟内学习Lua。

（没搞懂这个项目，目测是给普罗大众学习编程和解决问题用的，感觉要上天。）

[torchbear](https://github.com/foundpatterns/torchbear)


### 东京Rustaceans集会

东京在3月20日，要举办一场友好的Rust比赛+派对。该派对的主题是用tokio（会有一个准备好的tokio helper crate）来编写一款多人游戏，然后用此游戏来相互对抗。

（这个活动有意思）

- [Read More](https://www.reddit.com/r/rust/comments/axua3d/tokyo_rustaceans_on_march_20th_were_throwing_a/)
- [event 详细](https://connpass.com/event/122171/)

### actix-web和rocket框架性能比较

从评论区的actix作者的回复中得知一个消息：actix-web下个版本将支持Rocket风格的路由注册机制。

[Read More](https://www.reddit.com/r/rust/comments/aybr4e/rocket_and_actix_web_benchmark/)

### Rust的channel是否应该在没有接收者的时候Panic

作者最近使用了crossbeam-channel 0.3，发现有个issues提出一个问题：[channel的send是不是默认panic？](https://github.com/crossbeam-rs/crossbeam/issues/314)，基本上每个`.send`后面都跟着`.unwrap()`。

该文作者认为这样的写法是有问题的，并且在文章中罗列了两点原因。同步状况下还没什么问题，但是一旦和future异步一起使用，就会有麻烦。

[Read More](http://www.randomhacks.net/2019/03/08/should-rust-channels-panic-on-send/)

---

# 学习资源

### 使用Criterion对Rust项目进行基准测试

Criterion是第三方Rust基准测试库，还提供了额外的统计和图表功能。

- [Read More](https://medium.com/@yamafaktory/rust-benchmarking-with-criterion-on-travis-ci-%EF%B8%8F-8b54d321e05)
- [criterion.rs](https://github.com/bheisler/criterion.rs)


### 「视频」五分钟Rust系列

该系列视频，每一集只有五分钟左右。作者是Pat Shaughnessy，《Ruby原理剖析》的原作者，现在在学习Rust。质量挺好的，大家可以看看，他的特色是图文并茂。

[Read More](https://www.youtube.com/channel/UCVgTakRms47ldJIb05JFkQw)


### 「系列译文」用Rust创造操作系统之三

[Read More](https://zhuanlan.zhihu.com/p/53745617)

### Rust实现一个Merkle（默克尔）树状数组

- [Read More](https://www.reddit.com/r/rust/comments/aww097/starling_the_binary_indexed_merkle_tree_or/)
- [merkle_bit](https://github.com/ChosunOne/merkle_bit)

### ramhorns中的动态模板

#template_engine

ramhorns是一个实验性类Moustache的模板引擎。该作者（Pairtytech的工程师）写了这篇文章，主要记录他为ramhorns中动态模板提升性能的方案。

在Rust的世界里有很多模板引擎，可以分为静态模板和动态模板。

- 静态模板引擎，在编译时渲染。比如Askama。
- 动态模板引擎，需要在运行时进行渲染，比如Moustache，Handlebars和Zola（Rust实现的静态站点生成器）使用的Tera。

对于静态站点生成器，必须使用动态模板。而动态模板和Askama的性能存在5~30倍左右的差距。这让作者比较困惑，他在阅读了Askama和其他动态模板引擎的源码之后，发现Askama可以直接使用Rust类型渲染模板，而动态模板则需要一个中间结构表示，比如这种：

```rust
#[derive(Serialize)]
struct Post<'a> {
    title: &'a str,
    content: &'a str,
}
```

然后通过序列化和HashMap这类数据结构在运行时获取相应的字段和值去渲染模板。这虽然有效，但是这种中间结构付出了沉重的代价。并且对于已经存在内存中的数据结构是完全冗余的。作者罗列了可能出现的开销：

- 如果要将字段名称和值转为字符串，则需要创建HashMap，这会双倍耗费堆内存。
- 如果有一个Vec或者是一些其他要展现的东西，比如帖子列表。将不得不创建一个新的Vec和多个HashMap。
- 每次对HashMap的插入和查找，都会有哈希处理带来的额外开销。

而静态模板引擎完全没有上述的开销。ramhorns如何优化？

> 使用宏来生成代码，并且使用比较字符串的hash值来代替直接比较字符串。使用了Fnv库。并且在模板预处理中使用相同的Hasher。

ramhorns中没有使用serde，而是使用了Content trait。利用宏，为Post结构生成如下代码：

```rust
impl<'a> Content for Post<'a> {
    fn render_field(&self, hash: u64, buf: &mut String) {
        match hash {
            // FNV-1a hash of "title"
            0xda31296c0c1b6029 => buf.push_str(&self.title),

            // FNV-1a hash of "content"
            0x420c75b526b35282 => buf.push_str(&self.content),

            // In Mustache fashion, do nothing if the field is not found
            _ => {},
        }
    }
}
```

这样就避免了在运行时使用HashMap。优化的结果如何？作者和其他的模板引擎做了性能测试，发现ramhorns不仅比其他动态引擎更快，而且还比静态引擎Askama更快（其实Askama也有很大改进空间，Wearte项目就是案例）。作者说，也许再过一段时间，Hugo就不会说自己是「世界上最快的」静态网站生成器了。

- [Read More](https://maciej.codes/2019-03-03-ramhorns.html)
- [Wearte](https://github.com/dgriffen/wearte)

## 使用Rust构建类似于Wireshark过滤器那样的执行引擎

Cloudflare公司开源的用于解析Wireshark过滤器语法，并将它们编译器为可执行的IR。该库用于该公司提供的防火墙服务规则解析，所以使用Wireshark的过滤器语法作为DSL。

解析语法一般有三种方式：

- 使用状态机、正则等按字符进行解析
- 使用解析器组合器，比如nom或combine这种工具
- 完全自动化的生成器，可以根据提供的语法自动生成一个解析器，比如pest

但是该库并没有用nom或pest，而是选了第一种解析方式。并且在文章里给出了一些提升解析器性能的经验：

- 他们认为Rust标准库提供的字符串API完全够用。
- 使用IndexMap替换了HashMap来进一步提升了两倍性能。
- 使用trait对象动态分发和闭包来避免实现JIT而带来的一些问题。动态分发的执行效率出乎他们的意料。
- 选择使用Rust语言实现，对于支持WASM提供了巨大的方便。

该库已经用于Cloudflare公司的生产项目。

说明： Cloudflare是一家提供CDN、DNS、DDoS 防护和安全服务的公司。该公司曾经声称自己抵挡“在一秒钟内的流量接近于谷歌(Google)的全球搜索引擎在一个小时内的流量”的攻击。

- [Read More](https://blog.cloudflare.com/building-fast-interpreters-in-rust/)
- [wirefilter](https://github.com/cloudflare/wirefilter)
- [indexmap](https://github.com/bluss/indexmap)

### 集成React + Rust + WASM指南

本教程教你如何用Rust提供的wasm工具链开发React App。

[Read More](https://prestonrichey.com/blog/react-rust-wasm/)

### 使用Bulletproofs进行零知识证明

该文展示了如何使用bulletproofs进行零知识证明

- [Read More](https://medium.com/coinmonks/
zero-knowledge-proofs-using-bulletproofs-4a8e2579fc82)
- [bulletproofs](https://github.com/dalek-cryptography/bulletproofs)

###  「系列」Rust开发游戏24小时经验谈

作者用Rust开发了一款个网球主题的模拟小游戏，耗费了大概24个小时，游戏虽然没完成，但是他拥有了一些经验想要分享给你。涉及ECS模式。

[Read More](http://iolivia.me/posts/24-hours-of-rust-game-dev/)

第二篇文章阐述了他在实际应用ECS模式的一些经验，结合他的网球类游戏进行了讲解，值得一看。

[Read More](http://iolivia.me/posts/entity-component-system-explained/)

### 使用Rust + Warp + Juniper + Diesel编写Graphql API的模板项目

[rust_graphql_api_boilerplate](https://github.com/mattdamon108/rust_graphql_api_boilerplate)

### 享受const fn带来的编译时函数执行

该文简单介绍了const fn的用法和注意事项。

[Read More](https://blog.knoldus.com/no-more-run-time-enjoy-compile-time-function-evaluation-using-const-fn-in-rust/)

### gen-stream: 基于生成器的Stream实现

基于futures 0.3来实现

[gen-stream](https://github.com/vorot93/gen-stream)

### 从46s到5s  - 优化350行Rust代码实现的光线跟踪器

在这篇文章中，作者将讨论如何将用C＃/C++代码库编写的光线跟踪器移植到Rust，然后利用Rust的一些特性来进行简单的优化。

这篇文章主要是写给那些认为将C#/C++代码移植为Rust代码只需要简单的代码翻译的人。如果这么想的话，就会错过真正了解Rust的机会。

[Read More](https://medium.com/@cfsamson/from-48s-to-5s-optimizing-a-350-line-pathtracer-in-rust-191ab4a1a412)

### Rust中安全访问私有字段的方法

结论：构建setter/getter方法是最安全的

[Read More](https://blog.knoldus.com/safe-way-to-access-private-fields-in-rust/)

### await!存在可能永远无法返回的情况

该文作者发现await在future被Drop的时候会出现无法返回的情况，并给出了详细的示例代码。

[Read More](http://www.randomhacks.net/2019/03/09/in-nightly-rust-await-may-never-return/)

---

# 项目、工具与库

### interact: 运行时自省框架

可以通过命令行查看运行时程序状态，这个用来调试代码很方便了。

- [interact](https://github.com/interact-rs/interact)
- [book example](https://interact-rs.github.io/interact/book/examples/actix.html)

### Gfx-rs组织宣布新的项目wgpu-rs

wgpu-rs是基于gfx-hal的原生WebGPU实现。

- [Read More](https://gfx-rs.github.io/2019/03/06/wgpu.html)
- [wgpu-rs](https://github.com/gfx-rs/wgpu)

### luster: Rust实现的lua虚拟机

[luster](https://github.com/kyren/luster)

### ffsend: Firefox Send服务的命令行客户端

状态：WIP。支持从命令行轻松安全地通过Firefox Send服务上传和下载文件。Firefox Send可以通过安全、私密且受加密的链接发送文件，链接到期后文件将从网上彻底抹除。

- [ffsend](https://github.com/timvisee/ffsend)
- [Firefox Send 服务](https://send.firefox.com/)

### persy 0.4发布

persy是一个Rust编写的简单事务存储引擎

[persy](https://gitlab.com/tglman/persy)

### 为PyTorch实现Rust绑定和OCaml绑定

- [Rust bindings for PyTorch: tch-rs](https://github.com/LaurentMazare/tch-rs)
- [ocaml-torch](https://github.com/LaurentMazare/ocaml-torch)

### 可以操作approveapi的Rust库

Approveapi服务可以通过电子邮件、短信、移动推送请求用户可以实时地对任何内容进行批准确认。

该库基于tokio实现

- [approveapi](https://approveapi.com/)
- [approveapi-rs](https://github.com/approveapi/approveapi-rs)

### Rust可信计算开发平台介绍

- [Read More](https://edp.fortanix.com/)
- [rust-sgx](https://github.com/fortanix/rust-sgx)

### Double Ratchet算法的Pure Rust实现

Double Ratchet（双棘轮）算法是端到端即时通信加密算法之一，允许两个用户安全地进行通信：它为用户提供机密和真实的通道，包括前向保密和未来保密。目前Crait和Whatsapp这两款IM产品就使用该算法加密通讯。

[Read More](https://github.com/sebastianv89/double-ratchet)

### wasp: Rust实现的用于编写wasm的Lisp方言

[wasp](https://github.com/wasplang/wasp)

### tree-sitter: 增量式解析工具

[tree-sitter](https://github.com/tree-sitter/tree-sitter)

### cargo-flamegraph: Cargo火焰图工具

#cargo #flamegraph

[cargo-flamegraph](https://github.com/ferrous-systems/cargo-flamegraph)

### Rust实现的JavaScript引擎 

[boa](https://github.com/jasonwilliams/boa)

