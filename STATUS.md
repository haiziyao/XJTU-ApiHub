# XJTU-ApiHub 项目状态

更新于 2026-11-24。

## 线上环境

| 项目 | 值 |
| --- | --- |
| API 地址 | `https://api.xjtu-launcher.cn` |
| 服务器 | 阿里云 `8.152.100.169` |
| 进程 | systemd `xjtu-api`，监听 `127.0.0.1:8080` |
| 反向代理 | nginx，HTTP 301 到 HTTPS |
| 数据库 | 本机 MySQL 8.0，当前库 `smart_care` |
| 证书 | Let's Encrypt，`CN=api.xjtu-launcher.cn`，有效期到 2027-02-22 |
| 部署 | GitHub Actions，static musl 二进制，SSH 到阿里云 |

健康检查：

```bash
curl -fsS https://api.xjtu-launcher.cn/health
# ok
```

## 接口

| 方法 | 路径 | 鉴权 | 说明 |
| --- | --- | --- | --- |
| `POST` | `/api/auth/register/lookup` | 无 | 注册前按学号查询昵称 |
| `POST` | `/api/auth/register` | 校方 token | 注册；昵称不写默认真实姓名；不签发社区 token |
| `POST` | `/api/auth/login` | 校方 token | 登录；账号必须已注册；签发 accessToken |
| `GET` | `/api/index` | Bearer | 首页信息：从这里，启动你的XJTU人生 |
| `GET` | `/api/me` | Bearer | 当前登录账号 |

测试接口为 `/api-test/**`，额外返回 `serve_raw` 和 `serve_data`。

## 验证状态

| 项目 | 状态 |
| --- | --- |
| 注册前查询 | 云端 `200`，`nickname: null` |
| 真实账号注册 | Launcher 插件 `200`（2026-11-24） |
| 登录 | 待真实账号测试；当前 `user_token = 0` |
| 首页信息 | 无 token `401`；临时 token `200`；临时 token 已删除 |
| `/api/me` | 临时 token `200`，返回注册账号 |

数据库当前：`user = 1`（联调注册账号），`user_token = 0`。

## 数据库

- 当前库：`smart_care`，与 SmartCare 共用；ApiHub 只新增 `user`、`user_token`
- 账号主键：学号 `stu_id`
- 凭证：只存 SHA-256，明文只在登录响应返回一次
- 后续迁移到独立库 `xjtu_apihub`，只迁移 ApiHub 自己的表
- 计划表：`posts`、`comments`、`chat_rooms`、`chat_messages`

详见 [db.md](db.md) 和 [db-doc/schema.sql](db-doc/schema.sql)。

## 部署

- 工作流：`.github/workflows/deploy.yml`
- 构建：`rust:1.88-alpine`，static musl
- 部署：SSH 上传 → `/usr/local/bin/xjtu-api` → `systemctl restart xjtu-api`
- Secrets：`ALIYUN_SSH_KEY`、`ALIYUN_HOST`、`ALIYUN_USER`
- 证书续期：宝塔面板 SSL 页面；签发时在 `/etc/hosts` 固定了 Let's Encrypt IPv4

详见 [deploy.md](deploy.md)。

## 待办

1. 用真实账号完成登录测试，拿到 accessToken
2. 登录成功后清空联调账号与 token，再由用户正式测试
3. 创建独立库 `xjtu_apihub`，迁移 ApiHub 表
4. 确认并限制阿里云 MySQL `3306` / `33060` 的公网入口（此前实测可达）
5. 停用京东云早期 API 与 MySQL 隧道
6. 证书续期检查

## 文档

- [README.md](README.md)
- [api.md](api.md)
- [deploy.md](deploy.md)
- [db.md](db.md)
- [api-doc/openapi/auth.json](api-doc/openapi/auth.json)
- [api-doc/examples/auth.md](api-doc/examples/auth.md)
