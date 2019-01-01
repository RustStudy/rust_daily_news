# 「Rust每日新闻」2018年度盘点

---

# 「通告」2019年起「Rust每日新闻」将改名为「Rust日报」

因为「Rust每日新闻」这个名字太长了，尽量缩短一些，也符合Rust的调性。
今年我坚持每天不间断地了解、梳理、总结我所看到的Rust社区中的信息，并通过此频道分享给大家，于我个人而言，是完成了一次自我挑战。所带来的收获有很多，这个只有我自己才能体会。同时，希望大家也能在2019年跟我一样，每天能坚持自己感兴趣的事。

---

# 官方大事记

Rust 2018 关键字：生产（Production）

盘点：

1. Rust 2018 edition发布，Rust 1.31.0 是Rust 2018 edition的首个稳定版。标志着现在完全可以在生产环境使用Rust稳定版了。
2.  Rust 1.30标志着嵌入式Rust开发迎来的第一个稳定版本。你可以在不依赖unstable功能的情况下构建出完整可用的嵌入式程序。并且官方也为嵌入式开发者提供了充足的文档和生态链工具。
3. 经过官方网络工作组调查报告，2018年在Web方面，流行的框架是Rocket和Actix，分别占比27%和24%。也有其他框架的选择，但是有20%的人不选择任何框架，而是在hyper上自行搭建服务。
4. 官方web框架Tide发布首个版本，旨在为Web异步开发建立统一标准。
5. Rust和WebAssembly用于扩充JavaScript，而不是替换它。这个核心价值观推动了2018年Rust在WebAssembly领域的发展。生产链工具wasm-bindgen和wasm-pack已然成熟。
6. 万众瞩目的异步支持虽然还未稳定，但基本的前奏工作也即将就绪，预计在2019年中旬可以稳定。

---

# 社区大事记

1. 数据库领域：
    - 国内PingCAP公司基于Rust实现的开源分布式事务键值数据库TiKV，进入了云原生计算基金会（CNCF）Sandbox， 用于早期和持续发展的云原生项目。
    - Rust编写的现代嵌入式数据库sled已发布到0.16版。
    - Rust编写的图数据库indradb，已发布到0.21版。
2. Rust在游戏领域也有了十足的成长。
    - Amethyst游戏引擎基金会成立。
    - Specs倡导的ECS模式，在Rust社区颇受欢迎。并且Specs也有了继承者nitric，会为开发者提供更好的体验。
    - 曾开发 《战神：奥林匹斯之链》 和《 战神：斯巴达之魂》 的业界大厂 Ready at Dawn Studios 全面拥抱Rust。其CTO亲自动手实现了一个用于测试2D渲染的Playground库modulator_play。
    - 前EA首席设计官成立的新独立游戏工作室Embark宣布使用Rust为主要语言。
    - 各种轻量级游戏框架或库出现：比如轻量级2D游戏框架ggez、纯Rust实现的2D和3D物理引擎nphysics、2D和3D的碰撞检测库ncollide等
    - Rust和WASM结合应用于游戏也有不少例子，最有意思的是这个开源的基于浏览器的流沙游戏：MaxBittker/sandspiel
3. GUI和图形图像处理。
    - gfx-rs/gfx ，高性能无绑定的图形库，支持：Vulkan/ DirectX 12 /Metal /OpenGL 2.1+/ES2+等。到年底也发布了gfx-hal库，是对硬件的直接抽象层。
    - MaikKlein/rlsl，是支持Rust编译到SPIR-V的库，SPIR-V是一种用于GPU通用计算和图形学的中间语言。
    - GUI框架Azul：面向IMGUI的免费功能性GUI框架，支持用Rust编写桌面软件，基于Mozilla WebRender渲染引擎。
4. Web前端（WASM）和后端框架：
    - yew: 最早的基于Rust和WASM的前端框架，灵感来自于Elm和ReactJS，支持创建多线程前端App。
    - seed: Rust+WebAssembly 前端框架，基于wasm-bindgen和js-sys创建。创建应用时需要依赖web-sys。
    - Ruukh是一个前端Web框架，受到VueJS和ReactJS的启发，基于Rust和WASM。
    - 「Rust前端框架」Draco：利用Rust和Wasm编写前端代码。灵感来自于React和Elm。使用了虚拟Dom。
    - 「Web框架」Rocket v0.4发布，预计下个版本，将可以应用于Rust稳定版本。
    - 目前社区流行的Web框架：actix-web和rocket。Tower作者和Hyper作者正在联合实现Warp框架，而Rust官方在完善Tide框架，在Tide框架标准完善之后，社区的其他框架应该会统一配合官方来完成变更，比如支持统一的中间件协议、async/await语法支持等。
    - 目前最成熟的ORM框架还是Diesel。
    - Graphql框架比较成熟的是juniper。
    - 全文搜索工具tantivy-search/tantivy，对标Apache Lucene。
5. 解析工具：nom、combain、pest三足鼎立。也有人写了一本书，提供了一些文档和示例，帮助你选择适合使用场景的解析工具。[Read More](https://freemasen.github.io/parsers_presentation/)
6. Rust在区块链领域发展：Zcash、Parity、秘猿、IBM超级账本等公司，都使用Rust构建自己的区块链生态。尤其是Parity的polkadot项目，将使用Rust构建异构区块链互联网。社区也发布了很多Rust编写的算法、零知识证明库等。
7. 操作系统：下一代操作系统Redox、嵌入式物联网操作系统TockOS和Google新的号称替代Fuchsia操作系统，都全部或部分使用了Rust。斯坦福大学、清华大学也开始使用Rust语言尝试进行操作系统的教学。
8. 机器学习：
    - Juice是一个开放的机器学习框架，之前的名字叫leaf，用于构建经典，深度或混合机器学习应用程序。 它受到TensorFlow，Torch，Caffe，Rust和众多研究论文背后的杰出人士的启发，并为深度学习带来了模块化，性能和便携性。
    - 第三方服务Machine Box将最先进的机器学习功能置于Docker容器中，因此像您这样的开发人员可以很快地将自然语言处理，面部检测，对象识别等功能轻松纳入您自己的应用程序中。提供了Rust SDK。
    - Rust极有可能成为构建世界级机器学习工具的语言，但它目前缺少一些重要的数学基础设施BFGS，在今年有人专门建立了一个开源项目paulkernfeld/bfgs来填补此空白。
9. 云计算相关：
    - 亚马逊AWS开源Firecracker。Firecracker是一种开源虚拟化技术，专门用于创建和管理安全，多租户容器和基于功能的服务，提供无服务器操作模型。 Firecracker在轻量级虚拟机中运行工作负载，称为microVM，它将硬件虚拟化技术提供的安全性和隔离性与容器的速度和灵活性相结合。
    - ASW Lambda Serverless全面支持Rust。
    - 红帽开源Stratis。Stratis为桌面Linux用户提供了一系列强大的高级存储功能，并且易于使用，基于Rust实现，Stratis也是是卷管理文件系统（VMF），比如ZFS和Btrfs。它始于存储“池”的核心思想，这与VMF和LVM等独立卷管理思想一致。
    - Dropbox开源了Rust实现的新的并发矢量化压缩算法DivANS，可以编译成WASM以在浏览器和服务器上进行高密度压缩。
10. Rust也用于安全领域，社区中也陆续出现了一些Rust实现的安全工具。比如：
    - 网络实时入侵检测(IDS)、嵌入式入侵防御(IPS)和网络安全监控(NSM)的引擎Suricata
    - web服务批量扫描工具lachesis。
    - 一款Android应用程序分析工具SUPER，可以检测潜在的安全漏洞并创建漂亮的报告。
11. 今年基于Rust实现的CLI App，也颇受瞩目。
    - bat，Rust实现的类cat工具，最流行。
    - exa，Rust实现的类ls工具
    - ripgrep，支持正则语法的文本搜索工具
    - tui-rs，一个强大的终端Dashborad工具
    - mcfly，更智能展示你shell历史的工具，自带小型神经网络（mall neural network）
12. 基于Rust实现的编程语言：
    - deno，Node作者的新语言，基于V8的TypeScript运行时，基于Rust实现（代码量24.2%），在今年颇受关注。
    - Gluon是函数式语言，借鉴了 F#, OCaml 和 Haskell语言，静态类型，并支持类型推断。它的目标应该是类似于Lua语言，可以嵌入在Rust语言中，也可以直接调用Rust函数，和Rust无缝集成。
    - Dyon是Piston游戏引擎组织实现的动态类型脚本语言，专门为游戏引擎和交互式应用程序而创建，其对象模型和javascript类似，但不存在null，支持类似Go的协程等。也可以和Rust无缝集成。
    - Formality，可用于写智能合约，可用于定理证明，兼容EVM（可运行以太坊合约），兼容GPU。
    - 使用Rust编写语言vm系列文章：blog.subnetzero.io 。该作者写了33篇博文，并且制作了一个寄存器式VM，源码在GitHub的subnetzero/iridium
    - 根据《 Writing an Interpreter in Go》写的Rust版本的monkey-rust语言。

# 小结

Rust社区欣欣向荣，以上盘点也只是冰山一角，最好还是能亲自加入社区体验和成长。

现在正是开始学习Rust的时候。Rust作为一种通用型语言，不管你的方向是前端、还是后端，亦或是编写嵌入式应用、还是写操作系统、网络服务等，都可以使用Rust。如果你是学生，可以尝试开始学习Rust，提升自己的竞争力。如果你是在职人员，也不妨学习一下Rust，因为它是新时代的语言。


   


