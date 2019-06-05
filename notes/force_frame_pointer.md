# 知识点：RUSTFLAGS='-Cforce-frame-pointers'

Use RUSTFLAGS='-Cforce-frame-pointers' when compiling any code you want to run under perf, otherwise the backtraces are unlikely to be accurate.

要编译在perf下运行的任何代码时，请使用RUSTFLAGS =' -Cforce-frame-pointers'，否则栈回溯信息可能不太准确。

目测是因为优化级别 >= 2的时候，frame pointer被省略掉了。所以后来加了个force-frame-pointers flag来禁止省略 frame pointer。相关[PR](https://github.com/rust-lang/rust/pull/48786/files)。

省略frame pointer好像是一个优化惯用法，可以优化函数调用，每个函数调用可以节省四个字节（32位架构）的栈空间，但作用仅限于此。但这种优化会导致调试器失去栈跟踪的能力，导致栈回溯信息不会精确。

