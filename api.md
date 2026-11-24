# XJTU-ApiHub 接口清单

更新于 2026-11-24。认证接口的正式请求和响应定义见 [OpenAPI 文档](api-doc/openapi/auth.json)，使用案例见 [认证接口使用案例](api-doc/examples/auth.md)，数据库设计见 [db.md](db.md)。

## 文档结构

| 位置 | 内容 |
| --- | --- |
| `api.md` | 接口总览、状态、通用约定 |
| `api-doc/openapi/` | OpenAPI 接口定义 |
| `api-doc/examples/` | 按场景组织的使用案例 |
| `api-doc/risks/` | 接口相关的已知风险 |
| `db.md` | 数据库设计 |
| `db-doc/schema.sql` | 建表语句 |
| `xjtu-api/` | Rust 服务实现 |

## 接口状态

| 接口 | 原接口 | 测试接口 | 上线状态 | 验证状态 | 验证说明 |
| --- | --- | --- | --- | --- | --- |
| 注册前查询昵称 | `POST /api/auth/register/lookup` | `POST /api-test/auth/register/lookup` | 未上线 | 待验证 | 按学号查是否已有账号及昵称 |
| 注册 | `POST /api/auth/register` | `POST /api-test/auth/register` | 未上线 | 部分通过 | 校方核验后创建账号；昵称不写默认真实姓名；不签发凭证 |
| 登录 | `POST /api/auth/login` | `POST /api-test/auth/login` | 未上线 | 部分通过 | 校方核验后签发凭证；账号必须已注册，未注册返回 `404` |

## 原接口与测试接口

社区客户端接口按业务模块分组。每条原接口使用 `/api/{模块}/{接口}`，对应测试接口使用 `/api-test/{模块}/{接口}`。两者使用相同的 HTTP 方法、请求参数和业务处理，返回相同的 `code` 和 `data`。测试接口额外返回 `serve_raw` 和 `serve_data`。管理员接口另行定义。

注册与登录是独立接口：注册只创建账号，不签发登录凭证；登录只接受已注册账号，核验后签发凭证。

| 响应字段 | 返回范围 | 含义 |
| --- | --- | --- |
| `code` | 两者 | 与 HTTP 响应状态一致，成功为 `200` |
| `data` | 两者 | 原接口的业务数据，失败时为 `null` |
| `serve_raw` | 测试接口 | 服务端收到的上游响应正文，JSON 正文解析为 JSON 值。没有上游响应时为 `null` |
| `serve_data` | 测试接口 | 服务端拟保存的数据。没有待保存数据时为 `null` |

注册前查询只读取昵称，所以测试接口的两个额外字段都返回 `null`。

## 学籍核验联调记录

2026-11-23，本机通过官网 HTTPS 调用临时地址 `POST https://xjtu-launcher.cn/api/apihub-test/profile`，由服务器经 HTTP 查询校方。真实账号返回 `state=200`，学号匹配，姓名和行政班级均有值。临时路由已撤回，复查返回 HTTP 404。校方明文链路另有记录。[已知风险](api-doc/risks/campus-profile-http.md)
