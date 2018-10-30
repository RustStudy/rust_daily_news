前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2018-10-28

# 通告一则

### 「Rust每日新闻」调查问卷  

「Rust每日新闻」持续更新也快一年了，现在想做个小调查，了解下读者群，为了在年底出一份「每日新闻」年度报告做准备。同时也为了将来可以更好地给大家提供内容。 大家有空填写一下，不涉及隐私。

填过的就无需再填了，没填过的朋友，抽空用新的问卷调查填写一下，目前的完成率只有53%，大家多支持。看大家的调查结果，也挺有意思，等问卷结束了，把统计结果分享给大家。

昨天看到有小伙伴建议在telegram channel上面增加一些方便检索的标识，建议很好，但hash值可能不好记忆，所以我加了关键字信息，因为telegram上面只适合英文检索，所以关键字是英文的，这样大家可以凭借印象来检索相关信息。

[问卷地址](https://wj.qq.com/s/2801182/f890)

---

# 官方新闻

### 「官方」Rust 1.30 稳定版发布

一大片Rust 2018 edtion功能登陆了stable。

简要：

-  Procedural Macros 新的过程宏功能发布。之前的过程宏只能写派生属性，在1.30稳定版里，过程宏也可以写像函数一样可以调用的宏了，也可以把它叫Bang（因为感叹号）宏（Rust源码里也用Bang来区分）。
-  从第三方导入宏，无需使用`#[macro_use]`属性了，而是直接使用use
-  模块系统改进：引入第三方库的现在不需要使用"::"前缀了，库名可以直接作为前缀。而"::"则代表模块相对路径的外层路径。同时，增加了`crate`关键字，代表当前包的根路径。
- `r#`原生标识符， 可以使用`r#`前缀，将关键字作为函数名来使用。这个语法的意义在于，使用FFI的时候，可以使用和Rust关键字、保留字重名的函数。
-  可以使用`no_std`来构建应用程序了。之前只能构建库，因为`#[panic_handler]`属性已经稳定，可以用它来处理运行时panic了。
- `＃[used]`属性稳定，用来阻止编译器优化静态变量。

更多详细

[Read More](https://blog.rust-lang.org/2018/10/25/Rust-1.30.0.html)

### 「官方」docs.rs 找到组织了

docs.rs现在归到了rust-lang-nersery组织下

[docs.rs](https://github.com/rust-lang-nursery/docs.rs)

### 「Rust WASM小组」多线程Rust和WASM

[Read More](https://zhuanlan.zhihu.com/p/47850829)

### 「官方」举办了第一次编译器「 筹划指导委员会」会议

该会议主要是讨论接下来Rust编译器的发展方向，制定public roadmap、编译器文档、社区如何参与等问题。

[Read More](https://internals.rust-lang.org/t/compiler-steering-committee-meeting/8588/16?u=nikomatsakis)

---

# 社区新闻

### 「研究项目」Shifgrethor：Rust实现的垃圾收集库

[Read More](https://zhuanlan.zhihu.com/p/47850166)

### WebAssembly的后MVP时代

[Read More](https://zhuanlan.zhihu.com/p/47440191)

### 一个使用Rust节约成本的故事

[Read More](https://zhuanlan.zhihu.com/p/47796236)

### 「经验之谈」改进ndarray-csv

[Read More](https://zhuanlan.zhihu.com/p/47851335)

### 「游戏」Rust网格优化器

该文由寒霜引擎（Frostbite Engine）的高级渲染工程师所写。意味着该公司也使用Rust了。

他在做三维网格GPU渲染的时候，用到了一个优化器[meshoptimizer](https://github.com/zeux/meshoptimizer)，该库提供了很多算法为GPU优化几何形状。

但它是C/C++实现，还没有Rust实现。所以他实现了一个[meshopt](https://github.com/gwihlidal/meshopt-rs)。

是通过Rust FFI对meshoptimizer做了Rust绑定。

对于为什么使用FFI，而非纯Rust重写，他给出了理由：

-  制作100%的Rust实现，还需要持续和C/C++版本的代码保持同步。因为这些C/C++的库还非常活跃。
-  那些C/C++的库已经得到了很好的优化。
-  使用FFI的时间成本要小于用Rust重新实现。尤其是当库提供C89接口时，无需处理符号解码。

同时还介绍了一些工具， bindgen、cc以及用vscode+lldb Debug代码。

[ Read More ](https://www.wihlidal.com/blog/pipeline/2018-10-20-rust-mesh-optimizer/)


### 「社区」大家还记得Atom吗？

Atom 1.32发布，携带了新的Tree-sitter解析系统，用来改进语法的高亮显示和代码折叠功能。并且Atom 1.33 Beta版将内置对Rust编程语言的支持。

[ Read More ](http://blog.atom.io/2018/10/23/atom-1-32.html)


### 「讨论」大型Rust项目的可扩展问题

85000行代码，一共分了48个crate，而且大多数crate还是包含了测试的可执行文件。现在这些可执行文件的总大小正以平方递增，目前有4.2G可执行文件。现在这些可执行文件严重拖累了Gitlab CI，因为它们必须在构建和测试阶段通过网络进行复制。作者现在不知道该如何处理这个问题？

可以关注下reddit讨论贴，看看后续。

[reddit讨论贴](https://www.reddit.com/r/rust/comments/9rb4fq/problems_scaling_a_large_multicrate_rust_project/e8fngcd/)

---

#  学习资源

### 「练手小项目」如何计算Pi

[五种计算Pi的方法](https://www.wikihow.com/Calculate-Pi)

[Read More](https://www.reddit.com/r/rust/comments/9q63dk/trying_to_calculate_pi/)

### 「用Rust写操作系统V2系列」硬件中断

本篇介绍了如何用Rust处理硬件中断，将学习到如何获取定时器中断周期以及如果从键盘获取输入。

也能看到Rust的所有权对写操作系统产生什么有益的助力。

[Read More](https://os.phil-opp.com/hardware-interrupts/)

### 「博文」学习Rust，痛并快乐着

此文是influxdata的开发者写的，他是一名Go程序员，然后最近开始学习Rust。本文记录了他的一些学习心得和过程。

他学习过程不仅仅是看书，Go社区有本书叫[《用Go实现解释器》](https://interpreterbook.com/)，这本书里实现了一个语言，叫做Monkey 语言。然后他本身是Go程序员，所以在学习Rust的时候，通过这本书，用Rust实现了Monkey语言，文中有源码仓库地址。

[ Read More ](https://www.influxdata.com/blog/rust-can-be-difficult-to-learn-and-frustrating-but-its-also-the-most-exciting-thing-in-software-development-in-a-long-time/)


### 「Web框架」使用actix-web制作一个验证用户身份的微服务 Part 1

[Read More](https://hgill.io/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/)

### 「博文」Rust的孤儿规则其实很不错

作者在实现 [amethyst-editor-sync](https://github.com/randomPoison/amethyst-editor-sync) 库，通过IPC与amethyst的editor通信功能。

注：该项目 不是官方项目。

他在实现过程中发现孤儿规则的美妙之处，保证了trait一致性。

[Read More](https://davidlegare.ghost.io/rusts-orphan-rule/)


### 「博文」Rust RwLock和Mutex性能有点古怪

作者最近用自己实现的数据结构和RwLock和Mutex的性能进行比较，在自己的笔记本上，RwLock比他实现的快5倍，而Mutex快2倍。

代码放到他公司的机器上，Mutex依旧快2倍，然而，RwLock慢了4000倍。

通过研究发现：

- 相同的测试，在macOS上RwLock执行0.01秒，而换linux机器执行需要40秒。
- 通过阅读Rust RwLock源码，发现针对底层调用的rwlock原语，macos使用的FreeBSD的用户态组件 pthread_rwlock_rdlock，而Linux使用的是不同的结构PTHREAD_RWLOCK_PREFER_READER_NP。
-  不同的底层原语意味着不同的操作系统对待读/写的策略不同。
-  Linux上选择偏好读，读取会优于写入。而macOS/BSD上偏好写，写会优于读。
-  作者的示例正好是测试写入速度，所以在macOS上，读取会优先让位给写入，写入会很快完成。而Linux正好相反，所以导致写入操作严重延迟。不过Linux上面可以更改 pthread_rwlock策略。

结论：请根据不同的硬件环境去选择适合的数据结构。

[Read More](https://fy.blackhats.net.au/blog/html/2018/10/19/rust_rwlock_and_mutex_performance_oddities.html)


### Rust，Battlecode和Halite：初学者对AI编程竞赛的体验

该作者在学习Rust的第一个主要项目是Halite II 机器人，它是Two Sigma的AI编程竞赛的升级。广义上讲，它是一个游戏，在连续的二维地图上控制船队，寻路，并在舰与舰之间对战，资源管理等等，这些都有时间限制。

后来作者参加MIT的Battlecode 2018，对方的机器人引擎是Rust实现的，而作者的是用Python，所以作者的机器人总是不断的超时。后来作者也切换到了Rust解决了这个问题。

截止到今天，Halite III发布了，是一个资源管理游戏，支持Rust语言，大家可以去玩玩，用Rust打排名赛，截止到明年1月份。

[Read More ](https://www.reddit.com/r/rust/comments/9r48v3/rust_battlecode_and_halite_a_beginners_experience/)

[ halite.io ](https://halite.io/)

[Halite III代码仓库中可以找到Rust支持套件](https://github.com/HaliteChallenge/Halite-III/blob/master/starter_kits/Rust/src/main.rs)

###「博文」Rust学习心得

该文作者是《Ruby原理剖析》原作者Pat写的。他现在也开始学习Rust，尝试用Rust改写之前的Ruby脚本，然后在这个过程中有一些心得，通过该文进行记录。

感兴趣的可以看看

[Read More](http://patshaughnessy.net/2018/10/24/summer-school-with-the-rust-compiler)


### 缓存Docker build

作者在Docker上使用Rust，但是他的Dockerfile文件是这样配置的：

```
COPY . /opt/my_build_dir
RUN cargo build
```

所以当他每次修改代码以后，重新构建docker，都需要重新下载依赖包，时间会占用很久。所以他想找一个办法解决这个问题。

他考察了Docker的构建原理，这里使用`COPY`，是告诉docker，整个`my_build_dir`如果有任何变化，请复制数据。Docker是分层缓存，COPY以后已经算是新的目录了，之前缓存的已经无效。所以build的时候必须重新下载一遍依赖。

但是作者需要的只是在Cargo.toml文件修改之后再重新构建。所以他对Dockerfile文件做了一些更改：

```
# We'll get to what this file is below!
COPY dummy.rs /your_work_dir
# If this changed likely the Cargo.toml changed so lets trigger the
# recopying of it anyways
COPY Cargo.lock /your_work_dir
COPY Cargo.toml /your_work_dir
# We'll get to what this substitution is for but replace main.rs with
# lib.rs if this is a library
RUN sed -i 's/src/main.rs/dummy.rs/' Cargo.toml
# Drop release if you want debug builds. This step cache's our deps!
RUN cargo build --release
# Now return the file back to normal
RUN sed -i 's/dummy.rs/src/main.rs/' Cargo.toml
# Copy the rest of the files into the container
COPY . /your_work_dir
# Now this only builds our changes to things like src
RUN cargo build --release
```

然后在crate根目录创建一个`dummy.rs`文件，里面只包含下面代码：

```
fn main() {}
```

仅此而已，它只是为了让Docker构建一遍，得到第三方crate依赖的缓存，而不是构建正式代码。

魔法主要是由`sed`命令来激活的，该命令先用`dummy.rs`替换`main.rs`掉，把依赖编译完，在继续用此命令，把正式的`main.rs`或`lib.rs`替换回来，然后再复制src中的其他文件。通过这样的构建，就可以让Docker缓存那些第三方库的依赖了，除非是Cargo.toml文件发生变化。

群友指出，可以使用`cargo fetch`命令来下载依赖，代替`dummy.rs`这种方式。

[Read More](https://mgattozzi.com/caching-rust-docker-builds/)


---

# 项目

### 「项目」Aardwolf：Rocket实现的联盟社交网络

愿景是建立一个类似于facebook的社交互联网。

相似的项目（Rust实现）有：

Plume-org/Plume， 基于ActivityPub协议的Blog引擎（WIP）。ActivityPub是一个去中心社交网络（decentralized social networking）的交互协议，已经成为W3C标准。目前比较知名的应用有Mastodon（长毛象，Ruby实现）。

也可通过GitHub搜索相关项目[topics/activitypub](https://github.com/topics/activitypub)

[aardwolf.social](https://aardwolf.social/)

源码：[aardwolf](https://github.com/Aardwolf-Social/aardwolf)


### 「Rust前端框架」Draco：利用Rust和Wasm编写前端代码

灵感来自于React和Elm。使用了虚拟Dom。

[draco](https://github.com/utkarshkukreti/draco)


### 「项目」frugalos： Rust实现的分布式对象存储系统

关键字： [ frugalos, Object Storage, Frugal]

对象存储可以存储非结构化数据，例如文本或二进制数据，不像文件系统中的文件，对象存储在扁平结构中。只有对象池：没有文件夹，没有目录，也没有层次体系。你只要提供对象ID，就可以请求某个对象。

[ frugalos ](https://github.com/frugalos/frugalos)


---

# 工具与库

### 「工具库」字符串操作集合

该库实现了很多字符串操作的方法，灵感来源 Voca.js 和 string.py。可以关注下

[voca_rs](https://github.com/e1r0nd/voca_rs)


### 「小工具」字符串格式化库pfmt 发布0.4版本

[Read More](https://www.reddit.com/r/rust/comments/9rp856/pfmt_a_string_formatting_library_goes_040/)


### 「小工具」编译时断言

该库可以在编译期提供断言功能

[static-assertions-rs](https://github.com/nvzqz/static-assertions-rs)

### 「小工具」Rust实现的挖矿工具

可以挖门罗币，也包括其他的某些数字货币

[powhasher](https://github.com/kazcw/powhasher)

### 「工具」K8S 的tail pod日志工具

[korq](https://github.com/vertexclique/korq)

[官网](https://vertexclique.github.io/korq/)

### 「库」快速JSON编码器

特点是：

-  利用了宏和ZST
-  更多的编译期操作，避免运行时

[json_in_type](https://github.com/lovasoa/json_in_type#json_in_type)

### 「小工具」tinyrick：形式比较自由的Rust build工具

该工具实现了一些宏，包装了Rust的一些构建命令，让开发者可以自定义自己的构建工具

[tinyrick](https://github.com/mcandre/tinyrick)


### 「工具」Reflow：统一代理规则

此程序可以自动充分利用所有代理，VPN和接口。

构建于网络层，可以自动让应用层选择每一个连接的代理。

[reflow](https://github.com/net-reflow/reflow)

[Read more Reddit](https://www.reddit.com/r/rust/comments/9qvn97/reflow_a_systemlevel_proxy_switcher_a_routing/)

### 一个在emacs中编辑rustdoc注释的好方法

Emacs 党可以看看

[Read More](https://boinkor.net/2018/10/editing-rustdoc-comments-in-emacs/)



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
