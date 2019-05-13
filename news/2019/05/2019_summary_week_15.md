### 前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且分享我每天的见闻，偶尔也夹杂了一些个人的观点。新的一年过去了，Rust日报已经成为了Rust社区群大家每天必看的内容。

从2019年开始，日报小组成立，目前的动态由：@Chaos、 @Mike、 @Damody(台湾)轮番为大家播报。也欢迎感兴趣的朋友加入小组。

每周也会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

独立日报订阅地址：
- [Telgram Channel](https://t.me/rust_daily_news )
- [阿里云语雀订阅](https://www.yuque.com/chaosbot/rustnews)
- [Steemit](https://steemit.com/@blackanger)
- [GitHub](https://github.com/RustStudy/rust_daily_news)

社区学习交流平台订阅：
- [Rust.cc论坛](https://rust.cc)
- [Rust Force](https://rustforce.net/)
- [微信公众号：Rust语言学习交流](https://rust.cc/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

2019-05-12

---

# 官方新闻

### Await 语法预告

![img](https://pic4.zhimg.com/v2-2d1e5c7e407e5735abd3f888c81fac43_r.jpg)

[Read More](https://zhuanlan.zhihu.com/p/64916694)

await语法，官方代码里已经安排上了

![img](https://s2.ax1x.com/2019/05/08/E6CCrj.jpg)

这里，还有一个工具 [replace-await](https://github.com/taiki-e/replace-await) 用于把之前的 await!(xxx) 这种宏代码迁移到 xxx.await 的写法。

### Cargo Vender 子命令即将登陆Cargo

cargo vender支持将crates.io中的依赖项保存到你本地目录下。

[Read More](https://github.com/rust-lang/cargo/pull/6869)

### 「嵌入式工作组」μAMP: 微控制器上的非对称多处理

microamp（就是μAMP）是用于构建针对AMP系统的裸机应用程序，它是Real Time For the Masses（RTFM）多核版本的核心基础。

从历史上看，微控制器被设计为单核系统（SoC），但较新的设计越来越多地选择异构多核架构。例如，恩智浦的LPC43xx系列将Cortex-M4处理器与一个（或多个）Cortex-M0协处理器配对在一个封装中。这些设计的目标通常是优化功耗：例如，较低功率M0可以处理所有I/O，而M4内核仅被激活以执行昂贵的浮点/ DSP计算。

μAMP模型让我们可以针对这类系统，但也可以应用于同类多核系统，如Zynq UltraScale+EG或LPC55S69（2 ARM Cortex）上的双核实时处理器（2个ARM Cortex-R5内核）微控制器。

- [Read more](https://blog.japaric.io/microamp/)
- [microamp](https://github.com/japaric/microamp)

### 「Rust Wasm工作组」：wasm-tracing-allocator

一個全局的分配器追踨器，可以追到wasm内存分配的情況

[wasm-tracing-allocator](https://github.com/rustwasm/wasm-tracing-allocator)

### 「官方文档」Rust API 指南(api-guidelines)

今天有个Reddit讨论贴，有人指出每个发布到crates.io的crate都应该加上Readme说明和Repository地址（GitHub、GitLab等），以方便用户。

```rust
# Cargo.toml
readme = "README.md"
repository = "https://github.com/user/my_awesome_crate"
```

评论中有人提到Rust官方出品的「Rust API 指南(api-guidelines)」， Rust crate作者应该将它们视为开发Rust库时的一组重要参考因素（非必须遵守）。该指南还在完善中。该指南包含两部分：

- Checklist，用于发布crate时快速检查
- 详细说明，对checklist中的内容做详细的说明

- [Read More](https://www.reddit.com/r/rust/comments/bngvml/psa_please_put_readme_and_repository_links_in/)
- [api-guidelines](https://rust-lang-nursery.github.io/api-guidelines/about.html)

### 「Cli工作组」新工具：paw

为了使Rust开发Cli应用的体验更加一流，更方便地解析命令行参数，官方Cli工作组开发了这个Paw库，目前还是WIP状态。

```rust
#[paw::main]
fn main(args: paw::Args) {
    for arg in args {
        println!("{:?}", arg);
    }
}
```

`paw::main`宏允许`fn main`接受任何实现`paw::ParseArgs trait`的参数，所以，支持将`std::env::Args`传递给`main`，还允许传递`structopt`实例。

假如paw的反响比较好，官方还将走RFC流程，将它引入标准库中。

- [Read More](https://blog.yoshuawuyts.com/paw/)
- [paw](https://github.com/rust-cli/paw)


---

# 社区新闻

### Snip开源神经网络推理引擎Tract

tract是Snips.ai公司嵌入式自然语言语音处理系统SnipFlow中的重要组件。该公司两年前在TensorFlow Lite出现之前打算将TensorFlow嵌入到库中方便他们执行模型，但是后来因为TensorFlow太过庞大复杂，不得不更改了计划。

新的计划就是使用Rust作为SnipFlow的主要语言，两年过去了，团队已经非常享受现代化软件环境带来的舒适感，而且比TensorFlow更容易交叉编译。

(其实tract上个月就开源了，只是这篇文章最近几天才发布)

- [Read More](https://medium.com/snips-ai/snips-open-sources-tract-cdc50f437ef2)
- [tract](https://github.com/snipsco/tract)
- [snips.ai](https://snips.ai/)

### JavaScript二进制AST格式的参考实现

关于JavaScript二进制AST

随着网站变得越来越复杂，JavaScript源代码的数量不断增加。依赖于大型JavaScript代码库会导致网站启动缓慢 - 通常速度慢得令人无法接受。这是因为存在两个瓶颈：解析和字节码编译JavaScript。不幸的是，浏览器几乎达到了两种操作的效率峰值。

我们（Mozilla，Bloomberg，Facebook，CloudFlare）目前正致力于针对JavaScript的特定领域编码，称为“BinAST”（“JavaScript二进制AST”的缩写）。 JavaScript二进制AST旨在打破瓶颈。当前的高级原型已经在所有最常见的框架上显示了JS解析改进了30％-50％，只需更改格式，我们相信我们可以进一步提高这一改进。编码可以构建为webdev工具链的一部分，或者由代理或CDN注入，因此可以在不更改原始网站的情况下自动提高最终用户的性能。

此编码目前在JavaScript TC39标准化过程中。它可以与现有的压缩技术（gzip，brotli等）一起使用

[binjs-ref](https://github.com/binast/binjs-ref)

### 柏林都举行了 100 场 Rust 碰面聚会了

欧洲人很爱 Rust？大家都爱嘛。他们从 2014 年起就开始进行Rust聚会了。国内要更多点才好。不仅是大会，各个城市的小会也搞起来。

[Read More](https://berline.rs/2019/05/15/rust-hack-and-learn.html) 

### RubyGems将支持带有Rust内置扩展的gem

将添加一个新的构建器CargoBuilder，它将检测Cargo.toml文件并使用Cargo构建gem原生扩展。这减轻了为Ruby用户开发和发布Rust扩展的负担。现在希望找寻贡献者来落实这项计划。

- [rubygems/issues/2726](https://github.com/rubygems/rubygems/issues/2726#issuecomment-491482467)

### 尝试在Cloudsmith上发布你的crate

Cloudsmith是Puppet Labs旗下的DevOps平台，目前支持Cargo。你可以把Cloudsmith作为crates.io之外的私人registry。

本文介绍了如何使用cloudsmith-cli工具将你的crate发布到它的平台上。

- [Read More](https://blog.cloudsmith.io/2019/05/01/worlds-first-private-cargo-registry/)
- [cargo registry 相关文档](https://doc.rust-lang.org/cargo/reference/registries.html#using-an-alternate-registry)
- [通过此文了解下Cargo Registry： 乱谈Cargo Registry ](https://zhuanlan.zhihu.com/p/64917441)

### 「论文」Rust并发的实践研究

该论文通过实现一个并发无锁HashMap来研究Rust类型系统如何影响并发数据结构的开发和改进。他们的代码库concache在GitHub上公开，是Rust语言中最快的并发HashMap之一，可以帮助降低并发程序中的瓶颈。

- [Read More](https://arxiv.org/abs/1904.12210)
- [Paper pdf](https://arxiv.org/pdf/1904.12210)
- [concache](https://github.com/saligrama/concache)

### combine-4.0.0-alpha.1 发布

combine和nom的功能类似，但它的特点是建立在Rust的trait和类型系统之上，而不是宏。

[完整的更新列表](https://github.com/Marwes/combine/blob/master/CHANGELOG.md#400-alpha1-2019-05-07)

### Xi-Editor作者新博文：现代GPU上的2D图形

该作者花了一周的时间对「传统2D成像模型在现代图形世界中的未来」做了深入思考。2D图形建立在GPU之上是否是未来？作者认为是可行的，并且阐述了他的研究。感兴趣可以看看。

[Read More](https://raphlinus.github.io/rust/graphics/gpu/2019/05/08/modern-2d.html)

### 「社区讨论」命名异步函数的返回类型

该贴的作者认为，在async趋于稳定之前，还有个重要的讨论，就是支持异步函数的返回类型的自定义命名。

```rust
async fn foo() -> impl Future<Output = usize> + Send { /**/ }
// or even to make the return type nameable
type FooReturn = impl Future<Output = usize> + Send; 
async fn foo() -> FooReturn { /**/ }
```

[Read more](https://internals.rust-lang.org/t/naming-the-return-type-of-an-async-function/10085)


---

# 学习资源

### 使用Chrome对Rust进行全自动单元测试

stretch的作者写的一篇博客。stretch是一个跨平台的FlexBox引擎。在Visly公司，该作者正参与一个为前端工程师构建的设计工具，其中用到FlexBox，需要在不使用WebView的情况下保持Web、iOS和Android三端保持相同的布局。意味着在移动设备上复制Web的布局。

这篇文章里，作者介绍了使用stretch的单元测试方案，是一套自动编写自动化测试的方案，他们称其为gentest系统。大概原理如下图：

![img](https://s2.ax1x.com/2019/05/08/E6ZLZD.png)

所有的测试用例都用html文件来描述，并且包含`id =“test-root”`的布局。然后gentest使用WebDriver将此文件加载到Chrome head-less浏览器中。加载后，gentest将通过WebDriver向浏览器询问每个DOM节点的样式，大小和位置等信息，然后利用此信息，gentest再生成Rust单元测试，用于构建三端等效的FlexBox树给stretch api使用。最后可以统一通过`cargo test`来完成测试。

gentest的另一个好处是，因为每个测试只是一个html文件，只需打开文件就可以在浏览器中显示它。并且他们还利用gentest生成一套基准测试，确保性能不会退化。

gentest有什么黑科技吗？

它也是开源的，我翻了一下源码，主要是三步：

1. 使用quote!来构建待生成测试代码的模板（TokenSteam）
2. 将这些模板填充以后从TokenSteam转称字符串。
3. 使用`fs::write`写到指定的目录文件中。

- [Read More](https://medium.com/visly/unit-testing-rust-using-chrome-b8b93572a91d)
- [stretch](https://vislyhq.github.io/stretch/)
- [webdriver](https://www.w3.org/TR/webdriver/)
- [gentest](https://github.com/vislyhq/stretch/blob/master/scripts/gentest)

### 使用sccache在CircleCI上进行Rust缓存

如果你的crates有很多依赖项，你可能已经注意到Rust编译阶段与实际运行测试相比需要花费很多时间。

缓解该问题的一种方法是缓存Rust编译的中间对象：大多数博客文章建议缓存Rust目标文件夹以减少编译时间。这种方法有一个主要问题，这个缓存文件会越来越大。

因此你需要定时清理这个缓存目录。该文推荐Mozilla的这个库sccache。该库的一个优点是可以配置文件夹的最大大小，当大小超过该限制时，会启动LRU清除算法（和Redis的差不多），清理掉部分缓存。

- [sccache](https://github.com/mozilla/sccache)
- [Read More](https://medium.com/@edouard.oger/rust-caching-on-circleci-using-sccache-c996344f0115)

### 「Handmade Rust系列」Part 4 : 创建Vulkan绑定

该系列将以Rust手工制作方式开发Vulkan渲染引擎，这是第四篇博文。该项目的特点是，不使用标准库，只使用核心库。

- [Read More](http://stevenlr.com/posts/handmade-rust-4-vulkan-bindings/)
- [HandmadeRust](https://github.com/stevenlr/HandmadeRust)

### Graphlib v0.3.0已经发布！

Graphlib，用于Rust编程语言的简单而强大的图库。提供了图数据结构的一些API。比如BFS/DFS等迭代器。

[graphlib](https://github.com/purpleprotocol/graphlib)

### Rust并发模式：通过共享sender通信

[Read More](https://medium.com/@polyglot_factotum/rust-concurrency-patterns-communicate-by-sharing-your-sender-re-visited-9d42e6dfecfa)

### 使用Usher和Hyper构建简单的API

Usher是在基于Hyper实现的一个简单的库，用于开发简单的HTTP API服务。Usher的特色是提供了一些方便从URL中提取参数的方法，类似于actix的提取器，但是它更加轻量。

[Read More](https://whitfin.io/building-simple-apis-with-hyper-and-usher/)

### 「系列文章：WebAssembly」Part II： 开始Rust

该系列文章旨在使用Rust和WASM构建一个生产级的Web应用。

[Read More](https://medium.com/tech-lah/webassembly-part-ii-a-wasm-with-rust-2356dbc6526e)

### 使用自定义工具链解决Rust和Glibc的问题

Rust和Glibc在动态链接的时候可能会失效，该文作者建议使用自定义工具链来解决此问题。

[Read More](http://redbeardlab.com/2019/05/07/rust-and-glibc-version/)

### 「视频」用Rust实现TCP Part3

我们的老朋友Jon Gjengset，他的视频通常都是5小时左右。

[Read More](https://www.youtube.com/watch?v=8GE6ltLRJA4)

### 「系列」使用Rust创建静态HTTP服务 Part I

该教程没有使用http等基础crate，而是从零开始构建http 1.0服务，可供学习使用。

[Read More](http://concisecoder.io/2019/05/11/creating-a-static-http-server-with-rust-part-1/)


---

# 项目、工具与库

### swirlr-wasm：Swirlr的wasm版本

#wasm

日报Chaos君向你问好：

![img](https://s2.ax1x.com/2019/05/12/Ehn2CV.png)

(很有意思，小图可以展示的比较清晰，但是放大以后就模糊了，感觉可以用来保护头像隐私)

swirlr可以将采集的图像沿阿基米德螺线路径的采样点渲染SVG。

- [demo](https://willdady.github.io/swirlr-wasm/)
- [swirlr-wasm](https://github.com/willdady/swirlr-wasm)


### muscli ： 基于Pandora和音乐播放器实现的Tui

音乐数据来自于Pandora流媒体平台

[muscli](https://github.com/CMatri/muscli)

### stevenarella: 用Rust编写的多协议兼容Minecraft客户端

作者声明：just doing this for fun。

[stevenarella](https://github.com/iceiix/stevenarella)

### ifmt - 插值式格式化宏库

其实其它语言很多都有了。Rust一直显得比较生硬，格式化一个带变量值的字符串，要这样写：

```
println!("x: {x}, y: {y}, x + y: {sum}", x=x, y=y, sum=x+y);
```

使用这个库，可以这么写了。

```
let four = 4;
iprintln!("four plus four is: {four + 4}");
// four plus four is: 8
iprintln!("here's a hex number: 0x{0xb0bi64 * 1321517i64 :x}");
// here's a hex number: 0xdeadbeef
iprintln!("here's a debugging value: {Some(four):?}");
// here's a debugging value: Some(4)
```

作者把一套宏全部“升级”了。

```
format!      -> iformat!
print!       -> iprint!
println!     -> iprintln!
eprint!      -> ieprint!
eprintln!    -> ieprintln!
write!       -> iwrite!
writeln!     -> iwriteln!
format_args! -> iformat_args!
```

[ifmt](https://github.com/ct-austin/ifmt)

### ansi-parser - ANSI转义序列解析库

[ANSI转义序列](https://zh.wikipedia.org/zh-hans/ANSI%E8%BD%AC%E4%B9%89%E5%BA%8F%E5%88%97) 就是这种 "This is \u{1b}[3Asome text!"，我们平时在终端下看到的文字的色彩啊，一些特效格式啊什么的，都是按这个标准来做的。

相似的库还有 [vte](https://github.com/jwilm/vte)

[Repo](https://gitlab.com/davidbittner/ansi-parser)

### test-exec - 用于测试命令行工具的库

作者也是写命令行工具的时候，觉得测试很不舒服，于是写了这个方便测试的工具。很不错。

比如：

```
let output = exec!{
    "my_bin",
    args: ["-p", "/"],
    cwd: "/tmp",
    env: {
        THREADS: "4"
    },
    stdin: b"show-hidden",
    timeout: 60000,
    log: true,

    code: 0,
    stdout: b"Started program... Done.",
    stderr: []
};

// output can be used here like a normal process::Output
```

[Repo](https://github.com/Draphar/test-exec)

### dystopia - 匿名防追踪的网络代理

用了Tor技术，其貌似是要提供一种匿名服务。比如要访问google.com

```
curl https://google.com -x 54.95.171.65:2888 -L
```

项目还在早期阶段，值得关注。

[Repo](https://github.com/tbrand/dystopia)

### Rust与sed命令不得不说的故事

x12pp是用Rust实现的可以漂亮地打印X12 EDI格式的Cli工具。作者写了一篇博文，阐述了他使用sed命令处理X12的问题，这些问题促使他使用Rust来写x12pp。并且写出来的工具性能上可以轻松击败sed这样的通用工具。

- [x12pp](https://github.com/clarkema/x12pp)
- [Read More](https://www.lambdafunctions.com/articles/racing-sed-with-rust)

### finshir - 一种 Low&Slow 流量产生器

[Low&Slow](https://www.cloudflare.com/learning/ddos/ddos-low-and-slow-attack/) 是一种DDos攻击方法，利用产生大量的慢请求来保持住对服务器资源的消耗，从而影响正常请求的访问。finshir 就是这样一种用Rust写的工具。有两点高光：

1. 使用了 [may](https://github.com/Xudong-Huang/may)，对，就是黄旭东大佬的May协程库
2. 可以配合 [Tor](https://en.wikipedia.org/wiki/Tor_%28anonymity_network%29) 使用，实现匿名性

[Repo](https://github.com/Gymmasssorla/finshir)

### riv - 图片查看工具

这是一个命令行工具。其使用 SDL2 来渲染图片显示。

[Repo](https://github.com/davejkane/riv)

### 「系列」使用 Rust 实现一种新语言 Part I

作者正在练习实现一种语言 [esta](https://github.com/epellis/esta)。第一步就是生成 AST（Abstract Syntax Tree 抽象语法树）。作者详尽地记录了怎样从头开始撸一门语言，这个文章是一个系列文章，想要自己设计语言和学习编译原理的同学强烈推荐阅读。

[Read More](http://nedellis.com/2019/05/08/esta_1/)

### rust-hypervisor-firmware: 一個簡單的 kvm firmware

intel出品，代码不多，看上去确实简单，可以学习如何用rust实现一个kvm。

- [Read more](https://www.reddit.com/r/rust/comments/bn1b47/simple_kvm_firmware_written_in_rust_from_intel/)
- [intel/rust-hypervisor-firmware](https://github.com/intel/rust-hypervisor-firmware)

### wasm-flate: 使用WASM对客户端文件进行超快压缩的工具

支持GZIP，ZLIB和DEFLATE压缩和解压缩

[wasm-flate](https://github.com/drbh/wasm-flate)

### hors: howdoi的Rust实现

Howdoi是Go实现的通过命令行获取即时的编程问题解答的工具，hors是它的Rust实现版本。

![img](https://s2.ax1x.com/2019/05/12/Ehel8O.md.png)

[hors](https://github.com/WindSoilder/hors)