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
| `deploy.md` | 生产部署、GitHub Actions、域名与证书 |
| `xjtu-api/` | Rust 服务实现 |

## 技术栈

- 服务实现：Rust（edition 2024，axum + SQLx）
- 数据库：MySQL 8.0
- 构建与部署：GitHub Actions，static musl 二进制，SSH 部署到阿里云

## 生产环境

- 域名：`https://api.xjtu-launcher.cn`（HTTPS 已启用）
- 服务器：阿里云 `8.152.100.169`
- 反向代理：nginx → `127.0.0.1:8080`
- 进程：systemd `xjtu-api`
- 数据库：本机 MySQL 8.0，当前库 `smart_care`（后续迁到独立库 `xjtu_apihub`）

## 当前状态

- 注册前查询：云端已验证
- 注册：真实账号已验证（2026-11-24）
- 登录：待真实账号验证
- 首页信息：需要 accessToken，待云端验证

## 本地运行

```bash
cd xjtu-api
DATABASE_URL='mysql://user:password@127.0.0.1:3306/xjtu_apihub' \
  BIND_ADDR='127.0.0.1:8080' \
  cargo run
```
