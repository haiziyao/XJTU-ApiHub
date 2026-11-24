# 认证接口使用案例

对应接口定义：[api-doc/openapi/auth.json](../openapi/auth.json)。通用约定见 [api.md](../../api.md)。

## 流程

注册、登录是三个独立接口，不再把注册和登录混在同一个调用里：

1. `POST /api/auth/register/lookup`：按学号查询是否已有昵称
2. 如果昵称为 `null`，调用 `POST /api/auth/register` 创建账号
3. 调用 `POST /api/auth/login` 换取社区登录凭证
4. 后续请求带 `Authorization: Bearer <accessToken>`

## 准备

```bash
export BASE_URL=http://127.0.0.1:8080
```

所有请求使用 `Content-Type: application/json`。成功和失败都返回统一信封：

```json
{ "code": 200, "data": { } }
```

`code` 与 HTTP 状态码一致，失败时 `data` 为 `null`。`/api-test/**` 联调接口额外返回 `serve_raw` 和 `serve_data`。

下文中的学号、昵称、凭证均为示例值。

## 1. 注册前查询昵称

### 新用户

```bash
curl -sS -X POST "$BASE_URL/api/auth/register/lookup" \
  -H 'Content-Type: application/json' \
  -d '{"studentNo":"0123456789"}'
```

```json
{
  "code": 200,
  "data": { "nickname": null }
}
```

客户端允许输入昵称，然后走注册。

### 已有昵称

```json
{
  "code": 200,
  "data": { "nickname": "梧桐" }
}
```

客户端可以回填昵称，直接走登录。

### 学号缺失或格式无效

HTTP `400`：

```json
{ "code": 400, "data": null }
```

## 2. 注册

注册会核验校方学籍凭证，但**不签发登录凭证**。

### 不写昵称：默认校方真实姓名

```bash
curl -sS -X POST "$BASE_URL/api/auth/register" \
  -H 'Content-Type: application/json' \
  -d '{
    "studentNo": "0123456789",
    "campusCredential": "<学籍 token>",
    "privacyAccepted": true
  }'
```

```json
{
  "code": 200,
  "data": {
    "studentNo": "0123456789",
    "nickname": "测试学生"
  }
}
```

`nickname` 不传或为空时，使用校方返回的真实姓名。

### 自定义昵称

```bash
curl -sS -X POST "$BASE_URL/api/auth/register" \
  -H 'Content-Type: application/json' \
  -d '{
    "studentNo": "0123456789",
    "campusCredential": "<学籍 token>",
    "privacyAccepted": true,
    "nickname": "梧桐"
  }'
```

```json
{
  "code": 200,
  "data": {
    "studentNo": "0123456789",
    "nickname": "梧桐"
  }
}
```

### 学号已经注册

HTTP `409`：

```json
{ "code": 409, "data": null }
```

### 校方学号与请求不一致

HTTP `409`：

```json
{ "code": 409, "data": null }
```

### 校园凭证无效或过期

HTTP `401`：

```json
{ "code": 401, "data": null }
```

### 未同意隐私协议

HTTP `400`：

```json
{ "code": 400, "data": null }
```

## 3. 登录

登录只接受已注册账号；未注册会返回 `404`，不会自动建号。

```bash
curl -sS -X POST "$BASE_URL/api/auth/login" \
  -H 'Content-Type: application/json' \
  -d '{
    "studentNo": "0123456789",
    "campusCredential": "<学籍 token>"
  }'
```

```json
{
  "code": 200,
  "data": {
    "accessToken": "<社区登录凭证>",
    "studentNo": "0123456789",
    "nickname": "梧桐"
  }
}
```

### 学号尚未注册

HTTP `404`：

```json
{ "code": 404, "data": null }
```

### 校园凭证无效或过期

HTTP `401`：

```json
{ "code": 401, "data": null }
```

### 校方学号与请求不一致

HTTP `409`：

```json
{ "code": 409, "data": null }
```

## 4. 使用登录凭证

```bash
curl -sS "$BASE_URL/api/me" \
  -H "Authorization: Bearer <accessToken>"
```

```json
{
  "code": 200,
  "data": {
    "studentNo": "0123456789",
    "nickname": "梧桐"
  }
}
```

## 5. 联调接口

联调接口与原接口使用相同的请求和业务响应，额外返回服务端调试字段。正式客户端使用 `/api/**`，联调时使用 `/api-test/**`。

### 注册前查询（联调）

```bash
curl -sS -X POST "$BASE_URL/api-test/auth/register/lookup" \
  -H 'Content-Type: application/json' \
  -d '{"studentNo":"0123456789"}'
```

```json
{
  "code": 200,
  "data": { "nickname": "梧桐" },
  "serve_raw": null,
  "serve_data": null
}
```

### 注册（联调）

```bash
curl -sS -X POST "$BASE_URL/api-test/auth/register" \
  -H 'Content-Type: application/json' \
  -d '{
    "studentNo": "0123456789",
    "campusCredential": "<学籍 token>",
    "privacyAccepted": true
  }'
```

```json
{
  "code": 200,
  "data": {
    "studentNo": "0123456789",
    "nickname": "测试学生"
  },
  "serve_raw": {
    "state": 200,
    "data": {
      "studentBean": {
        "sno": "0123456789",
        "name": "测试学生",
        "className": "测试班级"
      }
    }
  },
  "serve_data": {
    "studentNo": "0123456789",
    "name": "测试学生",
    "className": "测试班级",
    "grade": "2024",
    "nickname": "测试学生",
    "gender": 1,
    "provinceCode": "41",
    "degreeType": 2,
    "enrollmentYear": 2024
  }
}
```

- `serve_raw`：服务端收到的校方响应正文。
- `serve_data`：服务端实际写入的账号数据。

### 登录（联调）

```bash
curl -sS -X POST "$BASE_URL/api-test/auth/login" \
  -H 'Content-Type: application/json' \
  -d '{
    "studentNo": "0123456789",
    "campusCredential": "<学籍 token>"
  }'
```

```json
{
  "code": 200,
  "data": {
    "accessToken": "<社区登录凭证>",
    "studentNo": "0123456789",
    "nickname": "梧桐"
  },
  "serve_raw": {
    "state": 200,
    "data": {
      "studentBean": {
        "sno": "0123456789",
        "name": "测试学生",
        "className": "测试班级"
      }
    }
  },
  "serve_data": {
    "studentNo": "0123456789",
    "nickname": "梧桐"
  }
}
```

联调完成后应撤回或不对外暴露 `/api-test/**`。

## 安全注意

- `campusCredential` 只用于本次校方核验，服务端不落库、不记日志。
- `accessToken` 是客户端后续调用社区接口的凭证，不要写进 URL、日志或截图。
- 校方核验链路的已知风险见 [campus-profile-http.md](../risks/campus-profile-http.md)。
