# tide-web-demo

## 使用 Rust（Tide） 写 CURD

[] CURD，并使用 uuid 串连相应 log（I/O，系统，DB）

- [x] 分层 controller, service, model。 关于 web 服务结构 [掘金博文](https://juejin.im/post/5b44e62e6fb9a04fc030f216)
- [x] I/O log (请求参数及响应 [JSON])
- [] DB log (命令和返回)
- [x] 系统 log (系统启动时间，退出错误怎么搞？)
- [x] 测试
  - [x] 单元
    - [x] model
    - [x] service [使用 **mocktopus** mock model]
  - [x] 集成（http get post[json] 请求，测试 controller）
- [x] 配置（config）
  - [] watch

[] 学习 Tide

- [] 类型系统
  - [] State
  - [] Request
  - [] Response
- [] 结构
  - [] Middleware
  - [] Router

## 笔记

但跑一个测试用例并且输出

```bash
cargo test should_insert_article -- --nocapture
```
