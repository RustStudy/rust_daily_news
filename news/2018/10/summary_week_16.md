前言：

从2018年开始，我每天会花1个小时关注Rust社区动态，并且在[Rust.CC论坛](http://rust.cc)、[tg channel](https://t.me/rust_daily_news)、[Steemit](https://steemit.com/@blackanger)、[GitHub](https://github.com/RustStudy/rust_daily_news)、[语雀订阅](https://www.yuque.com/chaosbot/rustnews)都开通了Rust每日新闻，分享我每天的见闻，偶尔也夹杂了一些个人的观点。大半年过去了，Rust每日新闻已经成为了Rust社区群大家每天必看的内容。在这个知乎专栏里，每周会精选几篇Rust社区中的动态，和大家分享。分享的内容就不按时间排序了。

2018-10-21

# 通告两则

### 「Rust每日新闻」调查问卷  

「Rust每日新闻」持续更新也快一年了，现在想做个小调查，了解下读者群，为了在年底出一份「每日新闻」年度报告做准备。同时也为了将来可以更好地给大家提供内容。 大家有空填写一下，不涉及隐私。

昨天的金数据问卷调查满额了，今天用腾讯问卷新做了一份，已经填过的朋友，无需再填了。没填过的朋友，抽空用新的问卷调查填写一下。看大家的调查结果，也挺有意思，等问卷结束了，把统计结果分享给大家。

[问卷地址](https://wj.qq.com/s/2801182/f890)


### 「声明」

今天一天遇到两起评论，一条是带着阴阳怪气评论每周精选，一条是直接说我「标题党」，这让我有点不舒服。当然，我愿意相信对方也许并无恶意，只是用词不当。不过我也觉得有必要再次说明几点：

1. 「每日新闻」只是满足我自己每天的阅读需求，顺便总结出来分享给大家。
2.  每日新闻里的内容，不要当作是翻译来看待。并非翻译，只是我看完以后做了梳理和总结。
3.  人非圣贤，孰能无错？ 所以新闻的内容有错是很正常的。看见我对内容有错误的理解，欢迎指出问题，我很乐意改正，因为「每日新闻」的存在，也有一部分原因是因为我也要学习。
4.  你可以觉得我的水平很水，发的内容不值得你看，可以选择不交流不关注。但如果要评论或交流，还请携带善意。
5.  并非我敏感，只是觉得给大家说清楚比较好。
6.  有人可能认为我是宣扬Rust调门很高。但其实，我每日新闻基本都在Rust群发放，而且对原文也基本没有故意的篡改或添加私货（要加私货也是对原文中涉及的相关内容进行补充）。 不喜勿看。

特此说明

---

# 官方新闻

### 「官方」crates.io的2018年10月15日事件始末

在2018年10月15日，crates.io遭到了一次「伪装官方」的攻击事件。

事件起始：

- 有人伪造了一个用户名cratesio，然后以常用的简短的名字上传一些crate，里面除了Cargo.toml和README之外什么都没有。
-  并且指示crates.io用户，如果想要这些名字，就去crates.io issues列表发issues。
-  此用户上传crate的速度引发了GitHub的限制，导致所有软件的上传和下载速度减慢。

官方应对：

- 封禁该用户IP
- 删除该用户上传的所有crate，将cratesio用户的页面定向为404
- 该用户被举报到GitHub，封禁处理

事件从开始到结束，一共经历了五小时。

官方未来的措施：

-  考虑到单个的正常用户不可能一次性上传很多crate，所以将会限制上传crate的数量和速率
-  猜测该用户攻击行为的动机，很可能是因为之前crates.io上面的包名的「抢注」政策不满意。所以官方将在未来几周内讨论确定该政策的解决措施。
-  无论动机如何，官方团队都不欢迎这种攻击行为，如果对政策不满意，可以提交RFC或给help@crates.io发邮件。避免使用这种恶意攻击行为。
- 通过这次事件，官方团队也意识到应该多和社区进行沟通，并且会为社区的沟通制定更多的通道。

[ Read More ](https://blog.rust-lang.org/2018/10/19/Update-on-crates.io-incident.html)


### 「Web框架」Tide中的路由和提取：首个草图

该文由aaron编写，描述了Tide中路由和提取的设计，结合了Rocket、Actix和Gotham等框架的创意。

路由：从HTTP请求映射到endpoint，即，用于处理请求的一段代码。
提取： endpoint如何在HTTP请求中提取数据。

这文章只是一个草图。

通过一个simple app来展示这种设计：

```rust

#[derive(Serialize, Deserialize)]
struct Message {
    contents: String,
    author: Option<String>,
    // etc...
}

/// A handle to an in-memory list of messages
#[derive(Clone)]
struct Database { /* ... */ }

impl Database {
    /// Create a handle to an empty database
    fn new() -> Database;

    /// Add a new message, returning its ID
    fn insert(&mut self, msg: Message) -> u64;

    /// Attempt to look up a message by ID
    fn get(&mut self, id: u64) -> Option<Message>;

    /// Attempt to edit a message; returns `false`
    /// if `id` is not found.
    fn set(&mut self, id: u64, msg: Message) -> bool;
}
```
上面代码代替App的复杂的后端，可以看作是这个简单app的内存数据库层

接下来构建一个简单Web API：

```rust
fn main() {
    // The endpoints will receive a handle to the app state, i.e. a `Database` instance
    let mut app = App::new(Database::new());

    app.at("/message").post(new_message);
    app.at("/message/{}").get(get_message);
    app.at("/message/{}").put(set_message);

    app.serve();
}
```

具体的endpoint实现：

```rust
async fn new_message(mut db: AppState<Database>, msg: Json<Message>) -> Display<usize> {    
    db.insert(msg.0)
}
```

该函数签名等价于：
```
fn new_message(mut db: AppState<Database>, msg: Json<Message>)
    -> impl Future<Output = Display(usize)>
```

其中每个参数都实现Extractor trait。

Extractor trait中说明了如何从请求中提取数据。对于new_message，现在使用两个提取器：一个用于获取应用程序状态（数据库）的句柄，另一个用于提取主体（作为json编码的消息）。

在函数体内，可以直接使用参数。提取器包装器类型实现了Deref和DerefMut。在需要所有权时可以使用.0来提取内部对象：

```rust
 db.insert(msg.0)
```

最后将返回插入消息的标识符，Display(u64) 。Display类型是一个装饰器，用于将给定值序列化为vanilla HTTP 200，并通过Display trait来生成格式化的http body。

```rust
impl<T: fmt::Display> IntoResponse for Display<T> { ... }
```

更多示例看原文。

设计目标：

-  清晰地了解API和代码如何映射，将通过对路由和提取的分离和限制路由表达能力来做到这一点。
- 将提取和respond序列更加符合人体工程学。通过利用trait来实现这一点。
-  避免在核心中使用宏和代码生成。
-  为中间件和配置提供一个干净的机制，让API非常适合可扩展和自定义

路由：

-  通过「table of contents」分离路由，可以方便查看整个应用程序的结构
- 路由没有「fallback」机制，不能有两个相同的路由。
- 仅通过URL和HTTP方法选择endpoint

路由的语法：

```rust
"/users/{}"
"/users/{}/help"
"/users/new"
"/users/new/help"
```
路由只允许两条路由重叠，其中一条要比另一条更具体。

路由和资源是相匹配的：

```rust
impl<AppData> Resource<AppData> {
    pub fn get<T>(&mut self, endpoint: impl Endpoint<AppData, T>) -> &mut Config;
    pub fn put<T>(&mut self, endpoint: impl Endpoint<AppData, T>) -> &mut Config;
    pub fn post<T>(&mut self, endpoint: impl Endpoint<AppData, T>) -> &mut Config;
    pub fn delete<T>(&mut self, endpoint: impl Endpoint<AppData, T>) -> &mut Config;

    pub fn nest(&mut self, impl FnOnce(&mut Router));

    pub fn config(&mut self) -> &mut Config;
}
```
（ 有种Restful的风格）

其中AppData是路由核心类型之一，核心类型包括：

```rust
/// An application, which houses application state and other top-level concerns.
pub struct App<AppData> { .. }

/// Configures routing within an application. Routers can be nested.
pub struct Router<AppData> { .. }

/// Configures the responses for an application for a particular URL match.
pub struct Resource<AppData> { .. }

/// Embeds a typemap for providing hierarchical configuration of extractors, middleware, and more.
pub struct Config { .. }
```

路由终止于Endpoints

```rust
pub trait Endpoint<AppData, Kind> {
    type Fut: Future<Output = Response> + Send + 'static;
    fn call(&self, state: AppData, req: Request, config: &Config) -> Self::Fut;
}
```
对于框架的使用者来说endpoint，是任意一个异步函数，其中每个参数都实现了Extractor，返回类型实现IntoResponse。这点和Rocket框架相似，但没有使用宏来实现。

提取器 Extraction

```rust
pub trait Extractor<AppData>: Sized {
    type Error: IntoResponse;
    type Fut: Future<Output = Result<Self, Self::Error>> + Send + 'static;
    fn extract(state: AppData, config: &Config, req: &mut Request) -> Self::Fut;
}
```

这更像actix-web，以下所有类型都实现了Extractor：

```rust

pub struct Json<T>(pub T);
pub struct AppState<T>(pub T);
pub struct Path<T>(pub T);
pub struct Glob<T>(pub T);
pub struct Query<T>(pub T);
```

最终endpoint必须返回能转换为Response的数据：

```rust
pub trait IntoResponse: Sized {
    type Body: BufStream + Send + 'static;
    fn into_response(self) -> http::Response<Self::Body>;
}
```

下一篇文章将阐述中间件和配置API的设计

[Read](https://rust-lang-nursery.github.io/wg-net/2018/10/16/tide-routing.html)


---

# 社区新闻

### 「社区」Rust成为GitHub上增长最快的第五种编程语言

增长最快的语言：

- Kotlin
- HCL
- TypeScript
- PowerShell
- Rust

[Read More](https://octoverse.github.com/projects)


### 「研究项目」「系列文章」shifgrethor：Rust实现的垃圾收集库

Rust官方核心人员withoutboats开发了一个内存安全的垃圾收集库。它只是一个研究项目，并不能实际使用。

当然也可以用于学习目的，了解Rust的语言能力如何用于编码安全API中的复杂内存管理要求。

从长远来看，shifgrethor很可能会成为可用的高性能的垃圾回收器。当然，该项目不太可能成为Rust项目的常用功能。也就是说，请不要理解为「Rust要加GC了」。

shifgrethor的存在，只是为了要寻找一个实用的场景，用户可以使用GC来处理复杂的内存管理。比如涉及共享所有权的时候，可以使用动态的方式来管理内存。

shifgrethor是一个精准的跟踪GC

- 允许循环引用。不同于引用计数，这种GC不会导致内存泄漏。
- 精准的GC。它可以确切地知道哪些数据是活跃的，哪些不是。

Shifgrethor允许完全自由引用

- 可以解引用GC指针，可正常获得堆中的对象
- GC堆中的对象可以持有不在GC堆中对象的借用/引用，即使它们在栈上
- GC堆中的对象可以拥有栈中的对象。
- 可以讲GC指针从栈上移动到堆上。

GC是一种共享所有权，因此还需要内部可变性。

[shifgrethor](https://github.com/withoutboats/shifgrethor)

博文 [Read](https://boats.gitlab.io/blog/post/shifgrethor-i/)


### 「社区」新包 pin-cell

官方核心开发人员withoutboats开发了新的crate，pin-cell，类似于RefCell，但PinCell是针对pinned可变引用的内部可变类型。

为什么PinCell很有用？

从一个object到此object内部包含object的操作叫做「projection」，比如从结构体到其包含的字段，从Vec到其中的元素。Rust中可以按值、按引用、按可变引用来进行projection。

讲pinned引用转换为某些类型的操作叫做「pin projection」，当前讲pinned引入到RefCell是不安全的，所以必须用PinCell来安全处理「projection」。

[Read](https://boats.gitlab.io/blog/post/pin-cell/)


### Rust文档化建议

作者对Rust文档化（在你编写自己的crate时）给出了一些建议：

- README 一定要写，不管多小的crate
- CONTRIBUTION要列出来，以便别人贡献
- 提出一个preRFC，提出增加features文档化配置的功能，这个感觉还是挺有意义的

[原文](https://phaazon.net/blog/rust-features-documentation)

### Amethyst游戏引擎的最新进展

开发团队：

- 组建核心团队，负责整体管理和方向。
- 引入新的贡献者团队，这些团队专注于引擎的特定领域，并授权写入权限。
- 引入活跃策略。如果团队中有人在6个月内处于非活跃状态，将会被自动剔除团队。
-  现在有四名活跃的成员。

引擎核心：

- 基于UDP协议的net-dev正在取得进展，它会独立于引擎之外，但会被用于引擎中的高级API。这意外着你可以独立使用net-dev库。
-  3D渲染器正在完全重写，为了支持现代的渲染技术（Vulkan/Metal/DirectX12后端、VR等），正在招募相关经验的贡献者
-  正在设计和原型化支持高级部署体系结构的新的资源管道，有助于集成专业级的工作流到引擎中。（使用构建时和部署时配置来节省运行时的计算负担），并减少开发人员和艺术家的资源管理负担。希望通过引擎实现高度可扩展的开发流程，即使在大型商业游戏上也是如此。
- 实验性的Amethyst editor 发布，用于创建游戏原型
- 下一步将推出0.9版本，该版本将助力于2D游戏开发人员

[原文](https://www.amethyst.rs/blog/dev-news-10-2018/)


### const fn编译时SUBLEQ解释器

作者通过实现一个SUBLEQ解释器来测试const fn最近稳定的最小化子集mini-const-fn的可能性。

SUBLEQ是一种单指令集计算机，由其唯一指令SUbtract和Branch（如果小于或等于0的分支）组成。

指令 subleq a, b, c 将会执行 `mem[b] = mem[b] - mem[a]; if mem[b] <= 0 { goto c; }`，也就是，减并且小于等于0跳转，并且该指令是图灵完备的。其中a、b、c都是地址，`mem[b] = mem[b] - mem[a];`求减法，后面的分支进行控制挑战，可以实现通用的计算。

wiki： https://en.wikipedia.org/wiki/One_instruction_set_computer

当前Rust稳定的const-fn最小子集，包含的内容比较少，比如，包括算术运算符，结构/元组/数组创建和访问。 不允许任何种类的控制流构造，如if，match，while，for或loop。

在实现过程中得到如下事实：

-  const fn中仅仅允许使用生命周期和Sized作为泛型限定，比如还不能指定`T: Copy`
-  函数指针在const fn中是 unstable状态
-  const fn目前还不支持let绑定和重新分配可变变量
- 循环可以通过递归调用const fn来实现

```rust
const fn count(i: i32) -> i32 {
    count(i+1)
}
```

比如实现无限的计数循环，但不能指定分支条件。最终的解决方案是，手动展开循环。

结论：

 const fn的最小子集是否图灵完备？

答案是：

- 是的。因为可以用它实现图灵完备的SUBLEQ
- 不是。 只能执行有限数量的迭代。
- 也许。找到一种方式可以从循环或递归中提前中止（即使找到了中止的方法，仍然只有96次迭代的最大值，因为const fn求值器不允许超过96个栈帧）

一个有趣的事实

subleq函数可以看做一个不动点迭代器（Fixed Point iterator）。

[原文](https://www.reddit.com/r/rust/comments/9o6vzo/constfn_compiletime_subleq_interpreter/)


### 「实验项目」rails/rails_fast_attributes： 使用Rust提升Rails性能

刚在Ruby Summit China大会上讲完《Ruby的好朋友-Rust》，今天就看到了这个

是Diesel作者用Helix来帮助提升ActiveRecord的属性映射性能 ​​​​

Helix是Cargo作者带头实现的方便用Rust扩展Ruby的库

[rails/rails_fast_attributes](https://github.com/rails/rails_fast_attributes)

### 使用WebAssembly加速Markdown渲染

SpiderOak公司的博文，SpiderOak是一个云端备份工具。该公司出了一个产品叫 Semaphor，它是可协作的加密群组IM软件。起因是因为SpiderOak团队需要一个安全的群组消息传递和文件共享解决方案，而没有电子邮件或现成协作工具带来的风险。 使用私有区块链加密开发。

在Semaphor中发送的每条消息，都是Markdown文档，所以渲染性能对它们来说很重要。所以他们一直在寻找加速渲染性能的方法。最终他们想到一个方案，就是将WebAssembly集成到React组件中，取代react函数，所以他们提出了[react-wasm-bridge](https://github.com/SpiderOak/react-wasm-bridge) 实验组件。它将props传递给Rust WebAssembly模块，并提供构建React渲染树的接口。

Rust是个很自然的选择，因为Rust工具链很完善，比如有wasm-bindgen这样的工具。他们对Markdown渲染器的要求是：

- 安全，解析用户的输入需要安全
- 快速， 呈现消息展示是Semaphor中最频繁的操作

Rust+wasm方案改进后

安全性：

当前他们的Markdown渲染器用的是[markdown-it](https://github.com/markdown-it/markdown-it)，它与react一起工作并不安全，因为包含有HTML字符串，对目前的解决方案并不满意。因此新的解决方案必须没有HTML字符串，而是直接创建元素。因此通过Rust来构建React 元素树，并且wasm环境还是一个天然的沙箱，对于攻击者来说并没有那么有用。

性能：

刚开始的实现方案性能并没有多少提升，后来针对它们的产品场景，将React中循环渲染构建dom的逻辑，转移到了Rust中，从而提升了性能。因为对于它们的产品来说，只需要渲染一次即可，im中的消息是不可变的，所以不需要重新渲染。

最终的测试结果：

```
BROWSER  MARKDOWN-IT   WASM
—————————————————------------
Firefox    162ms       175ms
Chrome     197ms       84ms
```

[react-markdown-wasm](https://github.com/SpiderOak/react-markdown-wasm)

[原文](https://engineering.spideroak.com/2018/08/29/using-webassembly-to-speed-up-message-rendering/)


### 继续讨论Rust中的2D图形

这是「我想要一个2D图形Crate 」文章相关的后续讨论贴，该文作者总结了此次讨论中的几个具体的主题。

[原文](https://nical.github.io/posts/rust-2d-graphics-01.html)




---

#  学习资源

### 免费书籍推荐：CODE ARTISTRY BOOK

副标题：A Practical Guide To Rust 卷1

[官网](https://thedarkula.gitlab.io/code-artistry/book/)

[PDF](https://thedarkula.gitlab.io/code-artistry/book/VolumeI.pdf)

### 通过WebAssembly在Cloudless上玩Serverless Rust

[Read](https://blog.cloudflare.com/cloudflare-workers-as-a-serverless-rust-platform/)

### 基于 AWS Lambda 和 WebAssembly实现Serverless Rust

这篇文章介绍了 ：

1. 介绍如何使用WebAssembly创建Serverless功能。
2. 演示完全用Rust编写的AWS Lambda函数。

[Read More](https://blog.scottlogic.com/2018/10/18/serverless-rust.html)


### Rust与编程三定律

本文作者模仿阿西莫夫机器人三定律自创的叫信息学三定律（Three Laws of Informatics）

（感觉叫编程三定律好多了）

-  程序必须正确（Programs must be correct.）
-  程序必须可维护，但不能违反第一条定律（Programs must be maintainable, except where it would conflict with the First Law.）
- 程序必须高效，但不能违反前两条定律（Programs must be efficient, except where it would conflict with the First or Second Law.）

（好像有点道理）

本文很长，作者比较了众多语言，从编程语言发展历史，阐述了编程三定律背后的意义。从这个角度来看，Rust应该是目前唯一遵循此三定律的语言。

[Read](https://medium.com/@schemouil/rust-and-the-three-laws-of-informatics-4324062b322b)

### 「系列文章」Gfx-hal指南Part 2：顶点缓冲区（Vertex buffers）

[Read](https://falseidolfactory.com/2018/10/09/gfx-hal-part-2-vertex-buffers.html)

### 打通wasm <=> Fressian <=> cljs

通过Fressian，连通 Rust、wasm和cljs

Fressian是Clojure使用的数据格式， Fress是clojure.js实现的Fressian和wasm无缝转换的工具

现在用Rust实现serde-fressian，通过serde_fressian::wasm 模块与fress.wasm命名空间进行交互，这样就实现了使用Rust编写的wasm模块，直接为cljs使用。

[Read More](https://pkpkpk.github.io/wasm%E2%A5%AAfressian%E2%A5%ADcljs.html)


### 「小工具」Rust+WASM制作的浏览器扩展

用于在docs.rs上生成railroad语法图

该插件是用 Rust库macro_railroad （[之前新闻介绍链接] (https://t.me/rust_daily_news/954)），和wasm共同制作的

[macro_railroad_ext](https://github.com/lukaslueg/macro_railroad_ext) 源码

[ Read More](https://www.reddit.com/r/rust/comments/9pnk1w/show_reddit_a_browserextension_to_generate/)


### 「Rust Workshop」调试Rust分享的幻灯片


内容包括：

-  高效使用println!
- 利用断言
-  利用日志（log），以及介绍Log的生态系统
-  回溯和恐慌
- LLDB & GDB，推荐了很多LLDB/GDB的命令行debug技巧
- record & replay debug

其中log生态系统介绍的crate：

- log
- env_logger ，使用该库的时候，要理解RUST_LOG 环境变量
- stderrlog
- flexi_logger
- android_logger

关于回溯：

推荐了backtrace 包，可以按需定制生成回溯

记录和重放Debug：

推荐了工具[rr](https://rr-project.org/)，只支持Linux环境


相关代码：[debugging-workshop](https://github.com/jdm/debugging-workshop)

[Slides](https://www.joshmatthews.net/debugging-workshop/#1)

---

# 项目

### ToDo MVC 的WASM版本

使用了wasm-bindgen

[原文](https://github.com/jonathanKingston/todomvc-wasm)


### ToDoMVC的命令行版本

[原文](https://medium.com/@devashishdxt/building-a-command-line-todo-app-in-rust-a89bb7af91c3)

### 「Rust练手小项目」

- 统计git commit中的「咒骂」词语: [git-anger-management](https://github.com/sondr3/git-anger-management)
- Rust语法糖集合，该库提供了作者编写的一些语法扩展，方便使用的宏: [sugar-rs](https://github.com/harryfei/sugar-rs)

### 广告黑洞Pi-hole 开源了Rust实现的Restful API

基于Rocket实现

pi-hole可以阻止超过100,000个广告服务域

[官网](https://pi-hole.net/)

[原文](https://pi-hole.net/2018/10/15/announcing-our-restful-api-contributions-welcome/)

### 「视频」Bay Rust Meetup上排名前100的crate介绍

该视频介绍了Rust生态系统中前100个crate

文字总结：http://thume.ca/crates/

[Read](https://watch.cloudflarestream.com/6cc794b568e4b4b19153355e247ff6dd)


### 「社区项目」用Rust实现的Avatar头像生成工具

[initials.rs](https://github.com/sonmezonur/initials.rs)




---

# 工具与库


### 「社区」Rust SGX SDK v1.0.4发布

Rust SGX SDK由百度X-Lab维护，是为英特尔SGX enclaves开发安全可信计算应用程序的便捷框架。

在此版本中，添加了对最新英特尔SGX SDK 2.3.1的支持，并且（世界上首次）启用了基于SGX远程证明的内存安全Enclave-Enclave / Untrusted-to-Enclave TLS通信。

[Read](https://medium.com/baiduxlab/rust-sgx-sdk-v1-0-4-released-9c7d9056a888)

### 「新语言」Rust实现的高效编程语言Formality

可用于写智能合约

特性：

- 具有形式证明，可用于定理证明，Readme里还有一个证明`a + b == b + a`的示例
- 无GC
- 兼容EVM ，可以运行以太坊合约
- 兼容GPU，可以编译为OpenCL/CUDA
- 简单，2k行代码实现


看上去目前还未完善，还有很多工作要做

[formality](https://github.com/MaiaVictor/formality)

### K8s的Rust客户端

[kubernetes-rust](https://github.com/ynqa/kubernetes-rust)

### Welle： ApacheBench的Rust实现

[welle](https://github.com/rylev/welle)


### 「工具」envy： 新的配置工具

envy是基于serde实现的环境配置工具。它通过序列化/反序列化的手段，可以将外部环境的参数转为Rust中的类型。

看上去非常方便使用

[Read](https://medium.com/@softprops/configuration-envy-a09584386705)

[envy](https://github.com/softprops/envy)


### 「工具」ruplacer：查找并替换源文件中的文本

[Read](https://dmerej.info/blog/post/ruplacer/)

[ruplacer](https://github.com/SuperTanker/ruplacer)

### libdiffuzz：面向安全的Memory Sanitizer替代品

可用于检测未初始化内存的使用。

之前在「我如何在流行的crate里发现漏洞」中介绍过的检测未初始化内存的工具，被作者用Rust重写了。

之前新闻的链接 [Read More](https://t.me/rust_daily_news/1308)

[libdiffuzz](https://github.com/Shnatsel/libdiffuzz)


### 「社区」Inkwell：安全暴露LLVM的包装器

Inkwell旨在通过安全地包装llvm-sys来帮助你编写自己的编程语言

[Inkwell](https://github.com/TheDan64/inkwell)

### 「crate介绍」 很多派生宏 derive_more 0.13.0  发布

 derive_more 是一个提供了很多derive宏的第三方包。

在新版本中新增了自动派生Display的功能

[Readme](https://jeltef.github.io/derive_more/derive_more/display.html)

[derive_more](https://github.com/JelteF/derive_more)

### 「小工具」快速在命令终端访问你喜欢的网站

群友 Hisriver 写的一个小工具，他在去年写了一个浏览器插件「付费」Anyshortcut（https://anyshortcut.com/），今年用Rust把这个浏览器插件的功能搬到了终端上，终端功能暂不收费。

感兴趣的可以关注和使用

[Read More ](https://www.reddit.com/r/rust/comments/9ptvwq/show_reddit_a_rust_cli_tool_to_help_you_launch/)

源码 [anyshortcut-cli](https://github.com/anyshortcut/anyshortcut-cli)

---

# 招聘

### 「Job」医疗初创项目招聘Rust工程师 「旧金山/全职/非远程/薪酬/股权」

致力于改善医疗软件，拯救医生。 技术栈：Rust/node/react

也包括特定领域机器学习和细粒度的应用安全挑战

[原文](https://www.reddit.com/r/rust/comments/9ohzyd/jobs_commure_healthcare_software_startup_hiring/)
