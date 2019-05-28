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

2019-05-26

---

# 官方新闻

### 「官方」Rust 2019年活动记录

记录了从2018年12月到未来12月的一些活动。

[Read More](https://blog.rust-lang.org/2019/05/20/The-2019-Rust-Event-Lineup.html)

### Rust 1.35 稳定版预发布

1.35增加的一些特性摘要：

- 为`Box<FnOnce>`, `Box<FnMut>`, 和`Box<Fn>`实现了FnOnce/FnMut/Fn。（来自社区 @crlf0710的贡献 ），相关PR：[#59500](https://github.com/rust-lang/rust/pull/59500)
- 支持将闭包转换为usnafe的函数指针。
- 增加了`wasm32-unknown-wasi` Target。
- 线程在Debug模式下将显示ID。
- `alloc::System`实现了`Default`。
- `dbg!()`支持无参数调用。
- ASCII转换速度提升了4倍速。
- 稳定了一些API。

[Rust 1.35 release note](https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1350-2019-05-23)

### 「官方」Unsafe Rust安全检查：栈借用模型 2.1

ralfj比较高产，他负责Unsafe下内存模型相关的工作，目的是用miri来检测unsafe中的UB行为。为了达成这个目标，他陆续研究出以下一些借用模型：

栈借用模型1：

他在去年引入了栈借用模型1用于定义在unsafe内存模型中允许哪种别名。建立合理的别名规则，才能基于miri来检查unsafe下的UB行为。

该模型的核心思想是： 对于一个内存位置，逐步建立可跟踪的引用，形成一个栈结构。比如有一个&mut i32，可以对其重新借用获得一个新引用。这个新引用是必须用于此位置的引用，建立在旧引用之上。当新引用过期的时候，旧引用会被激活，就好像是栈结构push和pop。

在Safe Rust中，通常有借用检查来保护内存。但是在编写Unsafe代码的时候，借用检查就无法提供帮助了。所以，Rust核心团队就必须要定义一组规则，即使对于Unsafe代码来说也是非常有意义的。

栈借用模型2:

在上一篇文章中，ralfj又带来了栈借用模型的升级，栈借用2。

在栈借用1模型中，有一个概念叫做「frozen」，处于frozen位置的指针，只能读取，不能写入。它允许可变借用也能读取（检查粒度比较粗，把可变指针和共享指针同一化处理）。但是现在该模型被发现一个问题：当使用可变借用的时候，在该模型下可能会把某些未定义行为判断为合法。为了改进这个问题，栈借用模型2将精确跟踪允许访问的原生指针（更细粒度的检查，区分了共享指针和可变指针），而不是「frozen」。检查粒度比模型1更细。

栈借用模型2还有很多已知的问题，比如其实并没有真正使用到「栈」，反而更像「树」。但这还不是最后的结论。本文比较长，去原文阅读更多信息。

栈借用模型2.1:

在今天这篇文章中，ralfj又发现了上次的栈借用2模型存在一些问题：结合内部可变性，行为并不总是他们想要的。在模型2.0中，说到其实没有真正使用「栈结构」是在读取访问的时候，事实上进行「写访问」的时候，还是可以维护一个「栈结构」。

```rust
fn main() { unsafe {
    let c = &UnsafeCell::new(UnsafeCell::new(0));
    let inner_uniq = &mut *c.get();
    // stack: [c: SharedReadWrite, inner_uniq: Unique]
    let inner_shr = &*inner_uniq; // adds a SharedReadWrite
    // stack: [c: SharedReadWrite, inner_uniq: Unique, inner_shr: SharedReadWrite]
    *c.get() = UnsafeCell::new(1); // invalidates inner_shr
    // stack: [c: SharedReadWrite]
    let _val = *inner_shr.get(); // error because the tag is not on the stack any more
} }
```

对于这段代码，之前是「合法的」，但是用栈模型2.1来处理的话，就是UB。UnsafeCell是一个内部可变性容器，栈借用模型2.1会在栈中维护SharedReadWrite指针。像上面代码第4行，如果在设置了inner_shr之后，又重置了c变量容器内的值，栈借用结构就会改变，最后一行再使用inner_shr指针就可以检测到非法了，它是一个UB。但是在栈借用模式2.0中，最后代码执行的时候，堆栈将改为[c：SharedReadWrite，inner_shr：SharedReadWrite]，从而允许最终访问，这就是问题所在。

这样一来，相当于是栈模型1.0和栈模型2.0的结合？还可以在Unsafe代码导读中看到栈借用模型2.1的完整描述。

后续：ralfj将会写一篇关于栈借用模型的完整论文，当然，可能还是他自己的博士论文更重要吧，毕业最重要了。

- [Read More](https://www.ralfj.de/blog/2019/04/30/stacked-borrows-2.html)
- [Unsafe代码导读：wip/stacked-borrows](https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md)

### Rust Nightly 1.36.0中已经弃用了`mem::uninitialized`

Rust的臭名昭著的`mem::uninitialized`方法在今天的每晚构建中已被弃用。它的替代品`MaybeUninit`已经开始稳定。如果你正在使用前者，则应尽快迁移到使用后者（可能在6周内达到稳定）。因为这是一个break change的修改。

这篇文章主要讨论了未初始化内存的性质以及如何在Rust中使用它。并且探讨了`mem::uninitialized`为什么会被弃用，以及`MaybeUninit`是什么。

- [Read More](https://gankro.github.io/blah/initialize-me-maybe/)
- [Reddit 讨论](https://www.reddit.com/r/rust/comments/brek0w/heres_my_type_so_initialize_me_maybe/)
---

# 社区新闻

### github 推出了beta 版的sponsor功能

第一批里面就有diesel(Rust的ORM框架)的作者 [sgrif/sponsorship](https://github.com/users/sgrif/sponsorship)


### 经过3年零8个月的工作，WebRender将向稳定的用户发货！

今年5月21日，5％的稳定用户将开始启用WebRender。 

[Read More](https://www.reddit.com/r/rust/comments/bqmyzm/after_3_years_and_8_months_of_work_webrender_will/)

WebRender使用与游戏相同的基于GPU的加速技术重写了Firefox渲染架构，现在适用于一些选定的Win10设备。WebRender使用的现代架构主要是：

- 合成器中页面的表示不再是一组栅格化图层，而是现在的一个未经过图形化的显示列表。
- 合成和光栅化步骤已加入到单个GPU驱动的渲染步骤中。

有关更多详细信息，请参阅Lin Clark的Hacks系列文章。

- [Read More](https://mozillagfx.wordpress.com/2019/05/21/graphics-team-ships-webrender-mvp/)
- [Lin Clark的Hacks系列文章：WebRender如何摆脱jank](https://hacks.mozilla.org/2017/10/the-whole-web-at-maximum-fps-how-webrender-gets-rid-of-jank/)
- [webrender](https://github.com/servo/webrender)

### actix-web已经发布了1.0 rc版本

[web-v1.0.0-rc](https://github.com/actix/actix-web/tree/web-v1.0.0-rc)

### 为什么选择Rust - 一个视频 

Buoyant CTO Oliver Gould (@olix0r) 和他的一个同事，参加了 The Open Source Show，大谈为什么选择Rust。

Buoyant 就是开发 Linkerd 这个 service mesh 产品的那个公司，懂了吧。

00:51 哪种类型的程序员使用Rust？
02:29 为什么Linkerd使用Rust?
03:12 Rust的历史
04:24 Oliver使用Rust的经历

[Repo](https://channel9.msdn.com/Shows/The-Open-Source-Show/All-About-Rust)

### 俄罗斯的一家计算机夜校的Rust课程回顾记录

- [Read More](https://matklad.github.io/2019/05/19/rust-course-retrospective.html)
- [视频](https://www.youtube.com/playlist?list=PLlb7e2G7aSpTfhiECYNI2EZ1uAluUqE_e)
- [Slides](https://github.com/matklad/rust-course)

### Cargo 5730号issues的解决办法

Cargo无法处理启用了不同feature的不同类型的依赖项，这对于no_std项目来说是一个大问题。所以有人写了一个cargo-5730库，来帮助解决此问题。

- [#5730](https://github.com/rust-lang/cargo/issues/5730)
- [cargo-5730](https://github.com/auxoncorp/cargo-5730)

### 「学术」gbdt-rs: 用纯Safe Rust编写的梯度提升决策树库 

gbdt-rs提供训练和推理功能。 它可以使用xgboost训练的模型来完成推理任务。gbdt-rs论文已被IEEE S＆P'19接受！

- [gbdt-rs](https://github.com/mesalock-linux/gbdt-rs)
- [相关论文](https://github.com/mesalock-linux/gbdt-rs/blob/master/gbdt.pdf)

### 「Rust编写命令行应用」是悲剧？

![img](https://pic2.zhimg.com/50/v2-1323bd359788b6a77401b20417557620_r.jpg)

他说的很对，Rust近两年确实是开启了一场轰轰烈烈的命令行大替换运动。但是不是悲剧呢？

按他的说法：Rust产出的这个命令行工具大小基本是4MB以上，性能要59ms，所以，这个太悲剧了。

拿Rust编写的替代ls命令的exa来说，brew安装大小是1.4MB，其实也没超过2MB。9012年了，你的磁盘空间就这么不够用吗，都干啥了？运行一次的时间我没测过，但我觉得就算真的是59ms我也是可以接受的，因为在我的反应时间之外，并没有多卡。我在本地尝试用exa列出TiKV（算得上一个大型项目了）的树形列表，也没有感觉到有卡顿。所以悲剧在哪？

软件写出来是让人用的吧？

exa的介绍第一句话就是：
You list files hundreds of times a day. Why spend your time squinting at black and white text?
你每天列出数百次文件，为什么你要花时间眯着眼睛看黑白文字？

exa提供了很多功能，其中，高亮显示文本（区分各种元数据、文件、目录、或其他格式，比如symlinks），是大多数Rust编写的命令行工具的底线。这是Rust社区，Rust文化的体现，就是让大家感受到更好的「人体工程学」。除了高亮显示，还有很多功能，比如支持git、扩展属性等。

所以，为什么要重写exa呢？是为了给大家节省时间，提升工作效率啊，同志们！

同样，有很多优秀的Rust重写的命令行工具，比如ripgrep，你怎么不说说它的搜索性能呢？ 

说了这么多，主要是想说明：

用Rust重写或者新开发的命令行App，是为了让咱们这个世界更加安全、美好。这个重写，并不是盲目的。在需要安全、需要性能的地方，也不会含糊。大家都试试新工具吧。

所以，到底什么是悲剧呢？大家想一想吧。  

### 「讨论」对于单人主力维护的项目如何看待

楼主覺得 actix 和 rust-postgres 很棒

但發現這兩個庫都只有一個大佬在當主力開發，他覺得庫只有一人維護對大公司來說不是問題

但對無力繼續維護的小客戶來說是個問題，大家覺得呢？

（其实很多项目都是单人在撑）

[Read more](https://www.reddit.com/r/rust/comments/bsdnih/concerns_about_some_major_libs_being_onemanshows/)

### 在 Mac 下面调优 TiKV

该文介绍了如何在Mac下对TiKV进行了性能测试。

里面介绍了一个工具：DTrace，可以方便在Mac下对Rust项目进行性能测试，并可以支持生成火焰图等报告。

[Read More](https://www.jianshu.com/p/a80010878def)

---

# 学习资源

### case-studies: Rust实例探究

该库展示了一些棘手的Rust代码示例，这些代码是dtolnay（syn作者，Rust宏的高手）在使用Rust（他自己和其他人）中的各种高级宏库时遇到的问题集合。该项目致力于对Rust宏开发的一个深刻洞察：擅长使用宏的人和宏专家之间的区别主要与他们擅长“宏”的程度是无关的。

这也许是学习Rust宏的一个非常好的案例。

[case-studies](https://github.com/dtolnay/case-studies)

### 多语言混合项目的一些经验

长文预警！作者在写自己的库bitvec的时候，开始考虑，如何将其用于其他语言，比如他如果在一个C++程序中想用bitvec怎么办？所以他开始设计一套针对为Rust crate编写FFI的惯用法。这篇文章记录了他从API设计到实现的一些经验，值得一读。

- [Read More](https://myrrlyn.net/blog/misc/polyglot-projects)
- [bitvec](https://github.com/myrrlyn/bitvec)
- [bitvec ffi branch](https://github.com/myrrlyn/bitvec/tree/feature/ffi)

### 一篇好文，教你如何在编程的过程中避免克隆

作者分享了一些技巧和经验，推荐阅读。

[Read More](https://thenewwazoo.github.io/clone.html)

### 用Rust写编译器

其实用Rust尝试和玩儿写编译器的已经很多了。这篇文章讲得很详细。作者发现用Rust写编译器很舒服。他个人编码只花了大约 60 个小时。

做编译方面工作的朋友推荐阅读。

[Read More](http://thume.ca/2019/04/18/writing-a-compiler-in-rust/)

### 「小技巧」操作数组应该使用Chain而非Concat

当应用程序需要迭代来自不同源的大量数组，那么具有C/C++背景的人可能会将所有数组复制到单个Vec中并迭代此Vec。在为连续Vec缓冲区分配堆内存方面，此策略将导致高成本。相反，将数据保留在原来的位置，使用Chain将它们链在一起去迭代会省不少成本。

- [Read More](https://frehberg.com/2019/05/rust-arrays-make-chains-no-concat/)

### 交叉编译和静态链接Rust库

该文作者的团队几年前用Rust重写了Python的特定后端服务，取得了巨大的成功。 现在，为了便于开发和测试，正在探索将部分C/C++代码库移动到Rust的想法。作为第一步尝试，他们先将Rust集成到现有代码中，而不是一次性重写所有项目。所以他们做了一系列实验，在C/C++中调用Rust。

[Read More](https://medium.com/csis-techblog/cross-compiling-and-statically-linking-against-rust-libraries-2c02ee2c01af)

### 「系列」Rust for OOP系列 ：项目管理

这位博主打算写一系列主题是Rust for OOP的文章，主要是针对有一定OOP语言开发经验的人来学习。这是第一篇，介绍了Cargo和crate、模块等知识。初学者可以看看。

[Read More](https://oribenshir.github.io/afternoon_rusting/blog/project-management)

### 「异步Explained系列」Rust异步如何工作

- [Explained: How does async work in Rust](https://levelup.gitconnected.com/explained-how-does-async-work-in-rust-c406f411b2e2)
- [Explained: Futures in Rust for Web Development](https://itnext.io/explained-rust-futures-for-web-development-b1d0632490e7?sk=9962ac666e1a56bc4fe53afc902008fa)

### 使用Rust加速Ruby MRI

一个使用Rust加速Ruby程序的案例，以i18n gem为示例讲述。

[Read More](https://medium.com/swlh/speeding-up-ruby-mri-with-rust-a7c914d2f9d0)

### 切片索引检查导致的3倍性能下降问题一例

作者发现下面这两片代码：

```
pub fn insertion_sort(data: &mut [i32]) {
    for sorted in 0..data.len() {
        let min = (sorted..data.len()).min_by_key(|&i| &data[i]).unwrap();
        data.swap(sorted, min);
    }
}

pub fn insertion_sort_fast(data: &mut [i32]) {
    unsafe {
        for sorted in 0..data.len() {
            let min = (sorted..data.len())
                .min_by_key(|&i| data.get_unchecked(i))
                .unwrap();
            std::ptr::swap(data.get_unchecked_mut(sorted), data.get_unchecked_mut(min));
        }
    }
}
```

性能有3倍左右的差距

```
insertions sort          time:   [551.79 us 553.24 us 555.71 us]                               
insertions sort (fast)   time:   [187.75 us 188.15 us 188.84 us]
```

很奇怪。于是很多人给他建议。

换种写法，跟unsafe差不多快。

```
pub fn insertion_sort(data: &mut [i32]) {
    for sorted in 0..data.len() {
        let min = data
            .iter()
            .enumerate().skip(sorted)
            .min_by_key(|(i, e)| *e)
            .unwrap()
            .0;
        data.swap(sorted, min);
    }
}
```

打开 `-C opt-level=z` 编译标志，按下面方式写：

```
pub fn insertion_sort_fast(data: &mut [i32]) {
    let mut data = data;
    while data.len() > 1 {
        let (head, tail) = data.split_first_mut().unwrap();
        let tailmin = tail.iter_mut().min().unwrap();
        if head > tailmin {
            std::mem::swap(head, tailmin);
        }
        data = tail;
    }
}
```

还有一种写法：

```
pub fn insertion_sort_iter(data: &mut [i32]) {
    for sorted in 0..data.len() {
        let min = data[sorted..].iter().enumerate().min_by_key(|&(_, e)| e).unwrap().0;
        data.swap(sorted, min + sorted);
    }
}

insertions sort (fast) time:  [187.05 us 187.37 us 187.84 us]
insertions sort (iter) time:  [186.87 us 187.30 us 188.00 us]

```

跟 unsafe 差不多。

[Read More](https://www.reddit.com/r/rust/comments/bsvup3/300_performance_penalty_for_slice_index_checks/)

---

# 项目、工具与库

### cargo-permissions: 检测篡改依赖的Cargo权限

为了在crates.io中保持健康安全的包（crate），需要尽可能多地强制检测任何类型的漏洞。随着软件包之间依赖关系的使用增加，漏洞传播的风险也会增加。在NPM等其他平台上，我们已经看到了很多这样的安全问题。Rust开发人员需要一个工具来回答有关其依赖关系的问题：

- 为什么png库使用网络层？
- 为什么http库使用文件系统层？
- 可能的场景（Possible scenarios）
- 读取未授权文件
- 请求不可信域名
- 执行未授权代码
- 盗取信息
- 盗用CPU资源
- 不安全地执行代码

cargo-permissions是一个概念证明的库（PoC），基于通过查找代码中的特定路径来检测恶意代码的想法，来保证crate的安全。此项目的主要思想是拥有一组与某些特定标准包列表相关联的权限。另一方面，通过AST分析，检查crate中使用的标准库。例如，如果包A开始使用`std::net`库，则将获得net权限。所有使用包A作为依赖的crate都会间接获得net权限。遵循此方法，可以构建具有所有获取权限的依赖关系树。通过这组权限可以获取「超出控制范围的crate」尽可能多的信息。

- [讨论](https://internals.rust-lang.org/t/cargo-permissions-to-detect-tampered-dependecies/10236)
- [cargo-permissions](https://github.com/fcsonline/cargo-permissions)

### sauron ： Web 前端开发框架发布0.7 发布

#frontend

仿 elm 的风格做的。其实，也是基于 yew 之上的封装。目标是易用，好用。

作者：[ivanceras](https://github.com/ivanceras)，是一位有趣的开发者。[svgbob](https://github.com/ivanceras/svgbob)，[spongedown](https://github.com/ivanceras/spongedown)等，都是他的作品。

### 「嵌入式Rust」erkos: 用Rust编写嵌入式操作系统

erkos是日本的一名Rust开发者编写的嵌入式操作系统原型项目，目标架构是Arm Cortex-M系列。他也写了一篇文章介绍该项目。感兴趣的可以看看。

- [erkos](https://github.com/garasubo/erkos)
- [Read More](https://medium.com/@garasubo/my-project-to-write-embedded-os-in-rust-eadf83f5ee37)

### mini-aio: 新的异步IO库

AdGear公司（一家实时广告平台）开源的库。这个库采用了与Rust中大多数其他异步IO库完全不同的方法：它实际上受到了Pony编程语言的启发。 因此，它不使用Futures，它不使用async/await，它只提供简单的trait。

比如，你想要实现HTTP server，只需要实现TcpListenNotify trait。该trait包含了listening、connected等方法。然后按正常的方式使用就可以，最终使用该库提供的event loop中执行：`event_loop.run()`

之前日报里介绍过Pony的并发特点：引用能力（Reference Capabilities），Pony 语言中每种变量的类型都包含了有关如何在 actor 之间分享数据的信息。有点像Rust的借用检查器，同样保证数据安全性。所以Pony的异步是actor模型，和actix差不多。坊间有这样的说法，「Pony，当Rust遇上Erlang」。

所以，mini-aio也提供Handler trait，该trait允许接收从代码任何地方发来的消息并处理。

但要注意，此库处于alpha阶段。该库作者也写了不少Rust项目，可以看看他的个人仓库。

- [Read More](http://antoyo.ml/mini-aio-new-async-io-library)
- [一个用mini-aio实现的FTP服务器：ftp-server-mini-aio](https://github.com/FTP-rs/ftp-server-mini-aio)
- [mini-rs](https://github.com/adgear/mini-rs)
- [作者antoyo GitHub仓库](https://github.com/antoyo)

### coreutils: 为Unix和类Unix系统编写的Rust核心工具集

该项目无意与GNU的coreutils 100％兼容，如Uutils的coreutils。瞄准最小但完整的实用程序集，只添加实用程序的几个实现和真正有用的函数之间通用的功能。目前该项目在寻求Review、贡献者、和建议。想要学习Rust的朋友，也可以从此项目入手。

[coreutils](https://github.com/GrayJack/coreutils)

### pyo3-file: pyo3的辅助库，方便处理类Python文件的对象

[pyo3-file](https://github.com/omerbenamram/pyo3-file/)

### Rendy 0.2发布

Rendy是基于gfx-hal的一个渲染引擎，属于Amethyst的项目。提供各种工具，如内存分配，资源管理，渲染图执行等。gfx-hal是99％的Vulkan API。 这就是Rendy存在的原因。而不是解决内存分配和纹理上传等低级任务，用户可以专注于创建令人敬畏的高性能渲染器。

[Rendy](https://github.com/amethyst/rendy)

### 强悍性能的 blake2b_simd 和 blake2s_simd 姐妹花

BLAKE 和 BLAKE2 是密码学哈希函数，来源于 Dan Bernstein 的 ChaCha。特点是在64位机上，性能比 SHA-3, SHA-2, SHA-1, 和 MD5 等都高。属于目前最高性能的哈希函数之一。而这两个库的实现，也在追求性能的极致。

以下是一些性能评测数据：

```
╭─────────────────────────┬────────────╮
│ blake2s_simd many::hash │ 2.454 GB/s │
│ blake2s_simd BLAKE2sp   │ 2.421 GB/s │
│ sneves BLAKE2sp         │ 2.316 GB/s │
│ blake2b_simd many::hash │ 2.223 GB/s │
│ blake2b_simd BLAKE2bp   │ 2.211 GB/s │
│ sneves BLAKE2bp         │ 2.150 GB/s │
│ blake2b_simd BLAKE2b    │ 1.008 GB/s │
│ OpenSSL SHA-1           │ 0.971 GB/s │
│ sneves BLAKE2b          │ 0.949 GB/s │
│ libsodium BLAKE2b       │ 0.940 GB/s │
│ OpenSSL SHA-512         │ 0.666 GB/s │
│ blake2s_simd BLAKE2s    │ 0.647 GB/s │
╰─────────────────────────┴────────────╯
```

与 

```
╭─────────────────────┬────────────╮
│ b2sum --blake2sp    │ 1.727 GB/s │
│ b2sum --blake2bp    │ 1.618 GB/s │
│ b2sum --blake2b     │ 0.887 GB/s │
│ coreutils sha1sum   │ 0.854 GB/s │
│ coreutils b2sum     │ 0.713 GB/s │
│ coreutils md5sum    │ 0.632 GB/s │
│ coreutils sha512sum │ 0.620 GB/s │
│ b2sum --blake2s     │ 0.603 GB/s │
╰─────────────────────┴────────────╯
```

评测的环境和对比请进下面的 Repo 查看。


[Blake](https://en.wikipedia.org/wiki/BLAKE_(hash_function))  
[论文](https://blake2.net/blake2.pdf)  
[Repo](https://github.com/oconnor663/blake2_simd#performance)  


### cargo-play

一個好用的工具，讓你可以快速的編譯執行单个rust文件，并且可以支持第三方crate。不再需要把整個編譯项目目錄创建好

[Read more](https://github.com/fanzeyi/cargo-play )

### Into The Wild

有人用rust寫了一個很像lf2(Little Fighter 2)的2.5D動作遊戲

[Read more](https://azriel.im/will/2019/05/24/into-the-wild/)

### hors 0.3.0

一個類似 google search 找解答的工具

howdoi 的 rust 實作版本

比如你有個問題叫 "how to parse json in rust"

使用下列指令
```
hors "how to parse json in rust" -a
```

得到解答
```
- Answer from https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

Solved by the many helpful members of the Rust community:

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("text.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    println!("{}", json.find_path(&["Address", "Street"]).unwrap());
}
```

[Read more](https://www.reddit.com/r/rust/comments/bsg9w4/hors_030_is_released_it_supports_google_search/)

### Mockiato ：一個嚴格友好的Mock測試庫

對測試有需求的同學可以試試看

```rust
#[cfg(test)]
use mockiato::mockable;

#[cfg_attr(test, mockable)]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_the_world() {
        let mut greeter = GreeterMock::new();

        greeter
            .expect_greet(|arg| arg.partial_eq("world"))
            .returns(String::from("Hello world"));

        assert_eq!("Hello world", greeter.greet("world"));
    }
}
```

[Read more](https://www.reddit.com/r/rust/comments/bshn0f/announcing_mockiato_a_strict_yet_friendly_mocking/)

### cargo-cache 0.2.1

這個工具可以幫助你管理 ~/.cargo/ 或 ${CARGO_HOME}

```
Cargo cache '/home/matthias/.cargo':

Total size:                             7.22 GB
Size of 101 installed binaries:           909.51 MB
Size of registry:                         1.46 GB
Size of registry index:                     63.65 MB
Size of 3082 crate archives:                435.72 MB
Size of 2038 crate source checkouts:        963.10 MB
Size of git db:                           4.84 GB
Size of 107 bare git repos:                 980.81 MB
Size of 100 git repo checkouts:             3.86 GB
```

```
cargo cache query "^serde*"

Registry cache sorted by name:
    serde-0.6.15: 16988
    serde-0.7.15: 22719
    serde-0.8.23: 25824
[...]

Registry source cache sorted by name:
    serde-0.8.23: 168461
    serde-1.0.80: 477759
    serde-1.0.82: 485084
```

[Read more](https://www.reddit.com/r/rust/comments/bspb17/cargocache_021_released_conquer_your_cargo_home/)

### 漫游 Tox-rs，第一部分

长文预警。[Tox](https://github.com/tox-rs/tox) 是一个安全的P2P核心服务，加密传输，易于使用的基于DHT的网络。是toxcore的Rust实现，toxcore目前被用来做P2P安全IM服务核心。

Tox 原来是个C项目，作者用Rust通过审视发现，实现里面有不少漏洞，易被攻击。所以他用Rust重写了它。就是上面那个项目地址。现在作者，开始整理这几年的工作，开始生成文档。

有很多客户端可以使用，比如qTox。

[Read More](https://habr.com/ru/post/447994/)

### ccl - 据说是目前为止性能最高的并发哈希库

ccl 目前包含一个并发hashmap和一个并发时限缓存，初步的评测很强力。


```
20k inserts + 20k mut lookups with replace 16C/32T Xeon 2.1Ghz Hetzner CXX51

hashbrown_rwlock        time:   [64.199 ms 64.234 ms 64.266 ms]                              

chashmap                time:   [15.190 ms 15.220 ms 15.251 ms]                      

dhashmap_ccl            time:   [1.0199 ms 1.0244 ms 1.0303 ms]     

concache                time:   [126.15 ms 126.61 ms 127.03 ms]   

crossbeam-skiplist      time:   [10.648 ms 10.681 ms 10.713 ms]       

```

[Read More](https://gitlab.nebulanet.cc/xacrimon/rs-hm-bench)  
[Repo](https://gitlab.nebulanet.cc/xacrimon/ccl)