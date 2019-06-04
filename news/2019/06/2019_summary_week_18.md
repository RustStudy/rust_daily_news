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

截止 2019-06-02

---

# 官方新闻

### 异步await语法最终确定

官方核心成员无船大佬在博客上披露，异步await语法的最终提议是继续推进后缀关键字语法： `future.await`。另外无船大佬还针对Rust语言设计给出了两点说明：

1. Rust语言很少有绝对的情况出现。除了一些必须要做的事，比如类型系统必须健壮之外，其他绝大多数的问题都是权衡问题。也就是说，需要权衡不同种类的方案，选择团队认为优先的道路。这样做必然会带来很多正面或负面的想法，所以我们需要避免非黑即白的二分思想。
2. 点关键字，只是一个方案的草图，并不带有某种角度的暗示或保证。它仍然需要走RFC流程。
3. 引入`@await`语法，也是因为权衡成本的问题，因为引入新的符号会带来成本。

下一步：

准备在Rust 1.37稳定版中引入async/await的最小稳定化版本， 1.37将于8月中旬发布，并于7月4日发布。这不会是async/await功能的结束 - 最小功能将会有很多扩展 - 但对于每个想要使用Rust进行高性能网络服务的人来说，这将是一个重要的里程碑。

[Read More](https://boats.gitlab.io/blog/post/await-decision-ii/)

### Unsafe代码指南现在线上可用

[Read More](https://rust-lang.github.io/unsafe-code-guidelines/)

### Rust标准库已经开始尝试使用 Const Generics 进行重写数组的一些实现

相关[PR](https://github.com/rust-lang/rust/pull/60466)。

[const generics](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md) 就是“基于常量值的泛型”。简单来说，Rust 中的 `[T; LEN]`，它的很多特性在之前的版本中，只支持最多 [T; 32]，而且是人 [T; 0]，[T; 1], [T;2] ... 一直到 [T; 31] 这样，[纯手撸实现](https://doc.rust-lang.org/std/primitive.array.html)的，丑不？

现在，有了 const generics，就可以把数组这种常用的类型从二等公民提升到一等公民来了。对于处女座来讲，简直要欢呼。

---

# 社区新闻

### rust.cc社区提供了国内crates镜像

快来试试

[rustcc/lernaean-deploy](https://github.com/rustcc/lernaean-deploy/)

### 知乎开源Rust实现的搜索引擎Rucene

据了解，目前开源的部分只包括 lucene library 部分，搜索引擎的部分太多业务内容而且跟内部的一个分布式框架有强绑定就没开源，等后续弄好了应该还会出一篇文章，详细介绍这个项目。未来估计还会有一次代码重构，可能会带来break change的修改，想要贡献的朋友可以缓一缓。

[Read more](https://github.com/zhihu/rucene)

### 支持Rust的多种方式

如果你想支持Rust的发展，有很多方法可以支持Rust：

- 贡献代码。
- 写文档、书、博客、视频等。
- 在论坛，Stack Overflow，Reddit，Matrix或Discord上回答问题。
- 财务支持。

尤其是财务支持，可以帮助补偿这些重点贡献人员的时间，使得开源更具可持续性。如果你愿意，可以查看Aaron发起的捐助人员列表。

- [Read More](https://readrust.net/support.html)
- [Aaron发起的捐助人员列表](http://aturon.github.io/sponsor/)

### Rust官网简体中文翻译工作正在如火如荼地进行着

 [rust-lang-cn/www.rust-lang.org](https://github.com/rust-lang-cn/www.rust-lang.org)

相关阅读 : [从官网挖掘的Rust I18N方案](https://zhuanlan.zhihu.com/p/67402361)

### Rust Audio论坛 - 讨论和询问有关Rust中音频开发的问题的地方

[Read More](https://rust-audio.discourse.group/)

### 使用WASI对区块链进行通用计算

本文来自于OasisLabs，介绍了WASI（Web Assembly System Interface ）在区块链上的应用。目前Oasis平台的技术架构目前正在围绕WASM、WASI和区块链来实施。

（OasisLabs是来自加州大学伯克利分校的Dawn Song教授和同事们创立的区块链项目，基于区块链和可信硬件想构建高性能的可信云平台。）

为什么他们想把WASI用于区块链？

WASI用于区块链的目标是利用围绕WASI快速发展的社区和基础设施。 与创建另一个Wasm接口的替代方法相比，WASI允许区块链开发人员利用更广泛的开发人员社区的资源。 通过这种方式，可以想象未来区块链将成为云开发人员工具箱中的另一个工具。

OasisLabs团队还专门提交了一个区块链WASI的RFC，想做成适合区块链上下文的WASI标注扩展。

- [Read More](https://medium.com/oasislabs/blockchain-flavored-wasi-50e3612b8eba)
- [Blockchain WASI RFC](https://github.com/oasislabs/rfcs/pull/1)

### Rust vs C++ ： 基于36核CPU的并行性能测试

#cpp #rayon

有人针对Rust/Rayon（Rust实现的多线程并发库）和C++/OpenMP（c++的类似于rayon的库）在36核的机器上进行了性能测试。

Rust 36个线程：

```rust
Running on 36 Threads
BabelStream
Version: 0.5
Implmentation: Rust
Running kernels 100 times
Precision: double
Array size: 268.4 MB (=0.3 GB)
Total size: 805.3 MB (=0.8 GB)
Function	Mbytes/sec	Min (sec)	Max		Average
Copy		50552.817	0.01062		0.02924		0.01143
Mul		39680.038	0.01353		0.01510		0.01443
Add		45828.953	0.01757		0.01874		0.01820
Triad		41769.002	0.01928		0.02206		0.02029
Dot		43584.260	0.01232		0.01327		0.01273
```

CPP/OMP 36个线程：

```c++
Running on 36 Threads
BabelStream
Version: 3.4
Implementation: OpenMP
Running kernels 100 times
Precision: double
Array size: 268.4 MB (=0.3 GB)
Total size: 805.3 MB (=0.8 GB)
Function    MBytes/sec  Min (sec)   Max         Average     
Copy        87745.870   0.00612     0.00710     0.00684     
Mul         79315.382   0.00677     0.00762     0.00736     
Add         89995.047   0.00895     0.01029     0.00992     
Triad       91574.889   0.00879     0.01012     0.00975     
Dot         118144.442  0.00454     0.00504     0.00490   
```

对于这个结果，可以看看Reddit讨论贴里的一些反馈。评论区有一半的人都认为Rust之所以慢，是因为rayon目前不支持NUMA感知（Numa aware）的原因。无论是Rust语言，还是Rayon目前都不支持该功能，除非等内存分配器稳定下来再做打算。

（NUMA 用于 x86 和 IBM® POWER® 体系结构平台上的多处理器系统。在具有 NUMA 特性的系统中，每个处理器都具有可用的本地内存，也可以访问分配给其他处理器的内存。对本地内存的内存访问速度更快。NUMA 节点是相互紧密联系的处理器和内存的集合。 在节点内的内存访问速度比在节点外更快。如果程序能够感知NUMA，那就相关计算资源，将会被放置到一个不同的物理NUMA节点。尽管仍旧在两个NUMA节点之间扩展，但资源使用将会得到优化。）

但未可知OpenMP的测试代码是否利用了NUMA感知来提升性能，但OpenMP好像是支持NUMA（不确定）。

- [Reddit 讨论](https://www.reddit.com/r/rust/comments/bto10h/update_a_scaling_comparison_between_rustrayon_and/)
- [相关代码](https://github.com/andrewpsuedonym/Dissertation-Project)
- [rayon issues: Schedulling should be NUMA aware](https://github.com/rayon-rs/rayon/issues/319)
- [介绍NUMA的一篇文章](http://cenalulu.github.io/linux/numa/)

### 「讨论」ndarray vs nalgebra

该贴针对此问题展开讨论：ndarray和nalgebra这两个库有什么区别？优缺点？哪个库更有机会被机器学习和科学计算领域应用？

评论摘要：

1. Ndarray和nalgebra针对两个不同的问题域。
2. Ndarray和nalgebra重复的部分只是线性代数系统，而nalgebra更侧重于线性代数系统，但仅限于1D向量和2D矩阵。
3. Ndarray类似于numpy，适用于n维数据处理。
4. 一旦Rust的const generics功能稳定以后，这两个库将深受影响，也许将来会有更多的功能重叠。
5. nalgebra是纯Rust的，ndarray有一个OpenBLAS后端。所以在性能上，一些程序可能ndarray执行的更好（纯Rust实现的有待优化）。
6. 如果是用于工程和数学目的，nalgebra是迄今为止最好的选择。nalgebra的最佳功能是在编译时进行维度检查，这意味着错误数学运算的代码将无法通过编译。

[Reddit讨论](https://www.reddit.com/r/rust/comments/btn1cz/ndarray_vs_nalgebra/)

### dotenv易主了

dotenv易主了，之前是diesel作者维护的，但是有段时间不维护了，连仓库都删了，现在有人接手它了，并且准备发1.0了，

[新仓库](https://github.com/dotenv-rs/dotenv)

### Rust和Blender

Blender是一个免费的开源3D创作套件。该文作者曾经在Blender工作过，编写过Python API。为了探索Blender二进制文件格式并提供读取和使用它们的工具，该文作者创建了一个Rust项目

- [Read More](https://www.janwalter.org/jekyll/blender/rust/blendinfo/2019/05/28/blend_info.html)
- [rs_blender](https://codeberg.org/wahn/rs_blender)

### Steam 上有哪些游戏是用 Rust 实现的

- [城市模拟游戏 UniverCity](https://store.steampowered.com/app/808160/UniverCity/)
- [编程游戏 Robo Instructus（预计2019 Q3上线）](https://store.steampowered.com/app/1032170/Robo_Instructus/)

[Read More](https://www.reddit.com/r/rust/comments/bvtfni/which_rust_games_have_made_it_to_steam/)


---

# 学习资源

### 「异步系列文章」Part 2: Async/Await语法之外的挑战 : 取消（Cancellation）

在这篇文章里，作者讨论了如果在应用中取消正在进行的异步任务，这非常有用，主要是因为对它们的结果不再有任何兴趣，所以继续操作就会成为资源占用。比如在超时范围内没有收到响应并且应该将错误发送回用户，则应该取消子操作。


先来看看在同步中如何取消任务：

例如

在C＃中，使用CancellationToken类型的变量传递给每个方法，该方法是可取消工作流的一部分。这些方法可以不时地检查CancellationToken.IsCancellationRequested，以便发现父任务是否已请求取消。在这种情况下，子任务可以提前返回（例如通过抛出异常）。

其他语言是这种方法的变体：

Go使用Context结构将取消请求传播到子任务。在Go中，取消请求通过Channel发出信号，因为它可以轻松地使程序等待取消请求或来自其他来源的状态更新。

Java利用线程局部中断状态。如果一个线程获得`.interrupted()`，那么如果不处理异常，某些操作将抛出并强制返回子任务。这可以被视为某种线程局部的CancellationToken，它在方法之间没有明确地传递。它具有不能在异步上下文中使用的缺点。

这些行为具有一组特定的共性:

- 取消始终只是通过取消请求来完成，它无法执行，即使请求已发出，子进程也可能运行一段时间。
- 所有方法都可以观察是否发生了取消，如果子方法受到影响，并且能够对其做出反应

在async/await异步中如何取消：

Rust之外的语言，和同步取消任务的模式类似：

- 通过专用参数或隐式任务本地参数请求取消
- 需要方法来观察取消状态。
- 即使取消后，异步方法也会完成。

比如，`C＃ async Tasks`通过CancellationToken发出取消信号。许多`.NET` core框架函数支持此参数以传播取消请求。Kotlin协程标准库识别取消请求并允许操作在取消时提前中止。Javascript没有取消的标准化类型，由于异步Javascript函数将始终运行完成，因此该机制还需要遵循上述所描述的模式。`C++`协程可以使用cppcoro库中定义的CancellationToken。

Rust对Cancellation的支持与其他支持async/await的语言有所不同。

- 只需删除表示异步操作的Future即可触发取消
- 不需要方法来转发取消请求或取消能力（例如，通过传递CancellationToken）
- 取消是同步的 - 因为drop()是同步的
- 取消不包括单独的取消请求并等待操作完成的步骤。只有一个取消/删除步骤。
- 取消被强制执行 - 子方法不能忽视或推迟取消

这些特性有优点也有缺点：

- 取消任务将变得简单
- 主要缺点是底层操作必须支持同步取消，比如处理操作系统底层IO的时候，会比较麻烦。但该文章中也给出了一些解决办法。

更多内容请查看原文。

- [Read More](https://gist.github.com/Matthias247/ffc0f189742abf6aa41a226fe07398a8)
- [文章列表](https://gist.github.com/Matthias247)

### 如何用Rust实现一个Chat App

本文介绍了如何使用Rust和简单的JavaScript在本地机器上构建简单的聊天应用程序。主要的技术栈是Rocket/websocket/JavaScript（你也可以使用TypeScript）

[Read More](https://medium.com/@steadylearner/how-to-start-rust-chat-app-499a194d0820)

### 禅与系统中移除阻塞（block）的艺术

标题仿自《禅与摩托车维修的艺术》。该文是servo的某个贡献者写的文章，他介绍了如何从并行系统中删除阻塞逻辑。长文预警！

[Read More](https://medium.com/@polyglot_factotum/programming-servo-zen-and-the-art-of-removing-blocks-from-your-system-51c1b7d404e3)

### 「系列」 使用Rust创建静态文件服务器 Part 2

[Read More](http://concisecoder.io/2019/05/27/creating-a-static-http-server-with-rust-part-2/)

### 小技巧：cargo clean的时候，不想重新编译依赖，怎么办？

`cargo clean` 之后，会重头开始编译，各种依赖会重新编译。那么如果只想重头编自己工程中的代码，不想把依赖全部重新编译呢？答案很简单：

```
cargo clean -p <your_crate_name>
```

[Read More](https://www.reddit.com/r/rust/comments/bvo0j9/how_to_cargo_clean_without_cleaning_compiled/)

### Rust 中最好的 2D 游戏开发框架一览

作者为了对比，用以下框架分别写了同一个小游戏，以做出对比。

- ggez
- tetra
- quicksilver
- coffee
- Piston
- Amethyst

文章比较长，内容非常详实，做游戏开发的同学值得一读。

[Read More](https://wiki.alopex.li/AGuideToRustGameFrameworks2019)

---

# 项目与库

### toast: 支持在docker容器中运行任务的工具

```yaml
image: ubuntu
tasks:
  greet:
    command: echo 'Hello, World!' # Toast will run this in a container.
```

![img](https://raw.githubusercontent.com/stepchowfun/toast/master/media/simple-task-0.svg?sanitize=true)

当然你还可以用它完成更多更复杂的任务：交叉编译、监控文件变化等等。但是Toast不会并行运行任务，而必须使用顺序执行计划，这是受Docker限制的，但是在单个任务中使用并行。

[toast](https://github.com/stepchowfun/toast)

### Plotters：Rust的绘图库

虽然在大多数情况下渲染图形并不需要太多的计算能力，并且诸如Python和Javascript的编程语言用于可视化目的。有时我们需要根据大量数据制作一个数字，例如，渲染一些人类基因数据可能需要对数万亿个数据点进行下采样，这对于许多高级编程语言来说甚至都不可行。在这种情况下，Rust是渲染数字的完美候选者，因为它具有高级抽象能力，但运行速度非常快。

Plotter目前支持使用HTML5画布的后端，也支持wasm，在其项目demo里有plotters+wasm的示例。

![img](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/sample.png)
![img](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/histogram.png)
![img](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/mandelbrot.png)

[plotters](https://github.com/38/plotters)

### 「物理模拟」Calcify 0.5.6 发布

Calcify是用于3-D和4-D矢量和矩阵代数的crate，被设想用于物理模拟。它基于一个基本的ThreeVec结构构建，包括内置的最常用操作。它包括物理常量，3和4-D向量和矩阵以及许多相关的操作，集合，直方图和输出树，可以在json或MessagePack中序列化。

[calcify](https://github.com/JTPond/calcify)

### terminal-typeracer: 命令行打字游戏

![img](https://gitlab.com/DarrienG/terminal-typeracer/raw/master/assets/typing.jpg)

[terminal-typeracer](https://gitlab.com/DarrienG/terminal-typeracer)

### pris:一种用于设计幻灯片和其他图形的特定领域语言

特性：

- 可以编译为PDF
- 完整的排版控制
- 一流的检查和操作支持

[pris](https://github.com/ruuda/pris)

### limber: 用于备份Elasticsearch文档的简单（但快速）工具

[limber](https://github.com/whitfin/limber)

### 「嵌入式」Hawk-Rust系列：树莓派相机的驱动

Raspberry Pi提供了一组GPIO（通用输入/输出）引脚，允许您控制用于物理计算的电子组件并探索物联网（IoT）。
相机模块是Raspberry Pi的绝佳配件，它允许用户拍摄静态照片并以全高清录制视频。

HAWK是一个基于Rust的图像识别项目，它通过使用RFID卡进行用户识别和Image进行用户验证来实现双因素身份验证。本文展示了如何使用Rust程序触发树莓派的摄像头。

- [Read More](https://blog.knoldus.com/hawk-rust-series-actuation-of-raspberry-pi-camera/)
- [hawk](https://github.com/knoldus/hawk)

### orkhon: 机器学习框架和运行时

Orkhon是用于机器学习的Rust框架，用于运行/使用用Python编写的推理/预测代码，冻结模型和处理未知（unseen）数据。 

[orkhon](https://github.com/vertexclique/orkhon)

### metrics: 高性能metrics(性能指标)库

基于trait抽象，提供稳定API，支持Prometheus

[metrics](https://github.com/metrics-rs/metrics)

### leg: 用于美化命令行输出的库

![img](https://camo.githubusercontent.com/fc0b10af3bf21ff1d561ab3bc4c50607fa956866/68747470733a2f2f692e6962622e636f2f7a667036574e4d2f6c65672d64656d6f2e706e67)

[leg](https://github.com/jesusprubio/leg)

### nameof - 替代 stringify! 的一个宏

设计这个库的目的是改进调试体验，对比 stringify!() 来说，有利于更方便地重构。

[Repo](https://github.com/SilentByte/nameof)

### j4rs 從rust调用java

在rust裡面建立 JvmBuilder 再调用 java 函數

[Read more](https://www.reddit.com/r/rust/comments/bur020/announcing_j4rs_calling_java_code_from_rust/)

### 一個乒乓球的遊戲使用WASM

[Read more](https://www.reddit.com/r/rust/comments/bv41o9/a_pong_game_with_wasmbindgen_websys_and_jssys/)

### rust-battop: 交互式终端笔记本电池电量可视化工具

基于Tui-rs实现。

[rust-battop](https://github.com/svartalf/rust-battop)

### 从Rust调用Go库：使用SQIP进行案例研究

SQIP 是基于SVG 的LQIO 方案，其能够用于生成可用的SVG格式。作者之前用Node.js中的SQIP包，但是不够稳定，然后又打算用Rust重新实现，又不打算重头编写，所以找到了SQIP的Go绑定库，想通过FFI在Rust中调用Go代码。但是和Cgo打交道，性能不知道如何。

[Read More](https://blog.arranfrance.com/post/cgo-sqip-rust/)

### dua - 并行的文件统计工具

也就是 du 的翻版啦。这个用上了Rust的并行化技术。速度比原来的 du 更快，特别是在SSD下更快。比如像下面这样测试

```
time du -sh real 0m1.003s

time dua -t 8 real 0m0.378s

time dua -t 1 real 0m0.971s
```

[Repo](https://github.com/Byron/dua-cli)

### sunfish 国际象棋游戏 

是对之前的实现的Rust重写。可以在[这里](https://lichess.org/@/sunfish_rs)试玩。

[Repo](https://github.com/Recursing/sunfish_rs/)

### minisketch-rs - @sipa 的 minisketch 库的 Rust 绑定

minisketch 是一种用于提升比特币网络的交易传输效率的技术。@sipa 是比特币开发者。

[Repo](https://github.com/eupn/minisketch-rs)

### Weave - 终端 http 路由/代理

是一个命令行工具。

[Repo](https://github.com/jsdw/weave)

### wgpu - 用 Rust 实现 WebGPU 协议

这个库基于 gfx-hal 来做。这个库会同时支持 WebGPU 和 WebGL。

[Repo](https://github.com/gfx-rs/wgpu/)

### 最近出现的Rust实现的新的语言

- [LambdaCore: Lisp家族又多了一员](https://sites.google.com/view/lcore)
- [haha: 一个简单的动态脚本语言，受js和Ruby启发。使用C实现的VM，Rust来实现解析编译为特定字节码haru](https://github.com/ffwff/hana)