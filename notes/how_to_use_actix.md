# 如何快速实践actix和actix-web

### 学习要求：

1. 有Rust基础
2. 有Web基础


### 建议学习顺序：

一、首先，从我这次大会的[Workshop](https://github.com/ZhangHanDong/actix-workshop-rustconasia2019)  入手

1  你先看 concept_to_guide里面的概念导读，概念导读是一个脑图，其中记录了actix相关的关键概念。可以按其自行搜索相关概念的更详细的内容学习。
2   然后再看看代码。代码很简单，就是一个个简单的api。这个workshop的重点是想让大家系统理解actix和actix-web （1.0 ）。 

二、然后，再结合 PingCAP的[Workshop](https://github.com/ZhangHanDong/rust-prometheus-workshop )  继续训练，这个workshop是给actix-web的应用（内存数据库demo）加个promeheus metrix 接口。  我fork过来一份，加了一个p8模块，实现了一个宏，重构代码（那个宏没有考虑性能，Just for Fun）。 按p1 -  p8顺序实践就好了。

学习完这两个workshop，应该差不多可以上手写一个actix-web的简单微服务了。

三、进阶

actix-web进阶的话，可以参考一些开源项目：

1.  https://github.com/Aardwolf-Social/aardwolf  ，推荐该项目的理由， 是因为它的一些实践非常好： 项目结构组织、代码复用抽象（泛型、trait和宏）都值得学习
2. https://github.com/dessalines/lemmy ，使用actix-web和TypeScript仿reddit站点，推荐理由，它的功能比较完善，还包括websocket。 

但它们目前都是actix 0.7的，建议，可以帮助lemmy升级到actix 1.0 来实践。