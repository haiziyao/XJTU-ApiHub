# XJTU-ApiHub

面向学生的校园 API 网关。文档优先：接口定义、使用案例和数据库设计放在仓库根目录，代码实现位于 `xjtu-api/`。

## 目录结构

| 路径 | 内容 |
| --- | --- |
| `api.md` | 接口总览、上线状态、通用约定 |
| `api-doc/openapi/` | OpenAPI 接口定义 |
| `api-doc/examples/` | 按场景组织的接口使用案例 |
| `api-doc/risks/` | 接口相关的已知风险 |
| `db.md` | 数据库设计、ER 图、表字段说明 |
| `db-doc/schema.sql` | 建表语句 |
| `xjtu-api/` | Rust 服务实现 |

## 技术栈

- 服务实现：Rust（edition 2024）
- 数据库：待定

## 本地运行

```bash
cd xjtu-api
cargo run
```
