# tide-web-demo

使用 Rust（Tide） 写 CURD

[] CURD，并使用 uuid 串连相应 log（I/O，系统，DB）

- [x] 分层 controller, service, model
- [x] I/O log (请求参数及响应 [JSON])
- [] DB log (命令和返回)
- [] 系统 log (系统启动时间，退出错误)
- [] 测试
  - [] 单元
  - [x] 集成（http get post[json] 请求）

[] 学习 Tide

- [] 类型系统
  - [] State
  - [] Request
  - [] Response
- [] 结构
  - [] middleware
  - [] 路由
