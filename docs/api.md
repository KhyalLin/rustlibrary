# 图书馆管理系统 API 文档

## 基础信息
- 基础URL: `http://localhost:8080/api`
- 所有请求和响应均使用 JSON 格式
- 认证方式：Bearer Token（在请求头中添加 `Authorization: Bearer <token>`）

## 用户相关接口

### 1. 用户注册
- **URL**: `/users/register`
- **方法**: `POST`
- **请求体**:
```json
{
    "username": "string",
    "password": "string",
    "email": "string"
}
```
- **响应**: 201 Created
```json
{
    "id": "string",
    "username": "string",
    "email": "string",
    "created_at": "datetime",
    "updated_at": "datetime"
}
```

### 2. 用户登录
- **URL**: `/users/login`
- **方法**: `POST`
- **请求体**:
```json
{
    "username": "string",
    "password": "string"
}
```
- **响应**: 200 OK
```json
{
    "token": "string",
    "user": {
        "id": "string",
        "username": "string",
        "email": "string"
    }
}
```

### 3. 用户登出
- **URL**: `/users/logout`
- **方法**: `POST`
- **请求头**: `Authorization: Bearer <token>`
- **响应**: 200 OK

## 图书相关接口

### 1. 创建图书
- **URL**: `/books`
- **方法**: `POST`
- **请求头**: `Authorization: Bearer <token>`
- **请求体**:
```json
{
    "title": "string",
    "author": "string",
    "isbn": "string",
    "description": "string",
    "type": "string",
    "quantity": "integer"
}
```
- **响应**: 201 Created
```json
{
    "id": "string",
    "title": "string",
    "author": "string",
    "isbn": "string",
    "description": "string",
    "type": "string",
    "quantity": "integer",
    "created_at": "datetime",
    "updated_at": "datetime"
}
```

### 2. 获取图书列表（支持分页和过滤）
- **URL**: `/books`
- **方法**: `GET`
- **请求头**: `Authorization: Bearer <token>`
- **查询参数**:
  - `pageNo`: 页码（默认：1）
  - `pageSize`: 每页数量（默认：10）
  - `id`: 图书ID（可选）
  - `title`: 图书标题（可选，模糊匹配）
  - `author`: 作者（可选，模糊匹配）
- **响应**: 200 OK
```json
{
    "total": "integer",
    "page_no": "integer",
    "page_size": "integer",
    "data": [
        {
            "id": "string",
            "title": "string",
            "author": "string",
            "isbn": "string",
            "description": "string",
            "type": "string",
            "quantity": "integer",
            "created_at": "datetime",
            "updated_at": "datetime"
        }
    ]
}
```

### 3. 获取单本图书
- **URL**: `/books/{id}`
- **方法**: `GET`
- **请求头**: `Authorization: Bearer <token>`
- **响应**: 200 OK
```json
{
    "id": "string",
    "title": "string",
    "author": "string",
    "isbn": "string",
    "description": "string",
    "type": "string",
    "quantity": "integer",
    "created_at": "datetime",
    "updated_at": "datetime"
}
```

### 4. 更新图书
- **URL**: `/books/{id}`
- **方法**: `PUT`
- **请求头**: `Authorization: Bearer <token>`
- **请求体**:
```json
{
    "title": "string",
    "author": "string",
    "isbn": "string",
    "description": "string",
    "type": "string",
    "quantity": "integer"
}
```
- **响应**: 200 OK（返回更新后的图书信息）

### 5. 删除图书
- **URL**: `/books/{id}`
- **方法**: `DELETE`
- **请求头**: `Authorization: Bearer <token>`
- **响应**: 204 No Content

## 错误响应
所有接口在发生错误时都会返回相应的 HTTP 状态码和错误信息：
```json
{
    "error": "错误描述信息"
}
```

常见状态码：
- 400 Bad Request: 请求参数错误
- 401 Unauthorized: 未认证或认证失败
- 403 Forbidden: 权限不足
- 404 Not Found: 资源不存在
- 500 Internal Server Error: 服务器内部错误

## 使用示例

1. 注册新用户：
```bash
curl -X POST http://localhost:8080/api/users/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"123456","email":"test@example.com"}'
```

2. 用户登录：
```bash
curl -X POST http://localhost:8080/api/users/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"123456"}'
```

3. 创建图书：
```bash
curl -X POST http://localhost:8080/api/books \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your-token>" \
  -d '{"title":"Rust Programming","author":"John Doe","isbn":"1234567890123","description":"A great book","type":"programming","quantity":10}'
```

4. 分页查询图书：
```bash
curl "http://localhost:8080/api/books?pageNo=1&pageSize=10&title=Rust&author=John" \
  -H "Authorization: Bearer <your-token>"
``` 