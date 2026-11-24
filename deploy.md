# 生产部署

更新于 2026-11-24。

## 拓扑

| 组件 | 位置 | 说明 |
| --- | --- | --- |
| API 服务 | 阿里云 `8.152.100.169` | `xjtu-api`，监听 `127.0.0.1:8080` |
| 反向代理 | 阿里云 nginx | `api.xjtu-launcher.cn` → `127.0.0.1:8080` |
| 数据库 | 阿里云本机 MySQL 8.0 | 当前使用 `smart_care` 库；后续迁移到独立 `xjtu_apihub` |
| 构建与部署 | GitHub Actions | `.github/workflows/deploy.yml` |
| 域名 | `api.xjtu-launcher.cn` | DNS 已指向阿里云；HTTPS 已启用 |

早期京东云上的部署不再作为生产目标，保留为临时环境。

## 构建与部署

GitHub Actions 每次 `main` 更新时执行：

1. 在 `rust:1.88-alpine` 容器中构建 **static musl** 二进制
2. gzip 压缩后通过 SSH 上传
3. 解压到 `/usr/local/bin/xjtu-api`
4. `systemctl restart xjtu-api`
5. 检查 `http://127.0.0.1:8080/health`

### GitHub Secrets

- `ALIYUN_SSH_KEY`：部署私钥
- `ALIYUN_HOST`：阿里云服务器地址
- `ALIYUN_USER`：登录用户

## 服务器上的文件

| 路径 | 用途 |
| --- | --- |
| `/usr/local/bin/xjtu-api` | 服务二进制 |
| `/etc/xjtu-apihub/api.env` | 运行环境变量，权限 600 |
| `/etc/systemd/system/xjtu-api.service` | systemd 服务 |
| `/www/server/panel/vhost/nginx/api.xjtu-launcher.cn.conf` | nginx vhost |
| `/www/wwwlogs/api.xjtu-launcher.cn.log` | nginx 访问日志 |

## 运维命令

```bash
systemctl status xjtu-api
systemctl restart xjtu-api
journalctl -u xjtu-api -n 100 --no-pager
curl -sS http://127.0.0.1:8080/health
nginx -t && nginx -s reload
```

## 域名与 HTTPS

- `api.xjtu-launcher.cn` 的 DNS 已指向阿里云 `8.152.100.169`
- nginx 已启用 443，并将 HTTP 强制 301 到 HTTPS
- 证书由 Let's Encrypt 签发，当前有效期到 **2027-02-22**，路径：
  - `/www/server/panel/vhost/letsencrypt/api.xjtu-launcher.cn/fullchain.pem`
  - `/www/server/panel/vhost/letsencrypt/api.xjtu-launcher.cn/privkey.pem`
- ACME HTTP-01 验证目录：`/srv/xjtu-launcher/acme`
- 续期在宝塔面板 SSL 页面操作，续期后执行 `nginx -s reload`
- 签发时为了绕过 IPv6 链路问题，在 `/etc/hosts` 固定了 `acme-v02.api.letsencrypt.org` 的 IPv4；续期失败时先检查这条记录
- 健康检查：`curl -fsS https://api.xjtu-launcher.cn/health`

## 数据库说明

- 当前 API 连接本机 MySQL 的 `smart_care` 库
- `smart_care` 同时被 SmartCare 应用使用，ApiHub 只新增了 `user` 和 `user_token` 两张表
- 后续迁移到独立库 `xjtu_apihub` 时，只需迁移 ApiHub 自己的表，不影响 SmartCare

## 注意

- 阿里云服务器可用内存约 300 MiB，API 二进制本身占用较小；不要在该服务器上编译 Rust
- 数据库密码、SSH 私钥不进入仓库；只放在服务器 `api.env` 和 GitHub Secrets
