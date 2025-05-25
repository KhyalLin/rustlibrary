# 图书馆管理系统设计文档

## 1. 系统架构设计

### 1.1 整体架构
```mermaid
graph TB
    Client[客户端] --> Nginx[Nginx反向代理]
    Nginx --> App[应用服务器]
    App --> DB[(MySQL数据库)]
    App --> Cache[(Redis缓存)]
    
    subgraph 应用服务器
        App --> Auth[认证模块]
        App --> User[用户模块]
        App --> Book[图书模块]
    end
```

### 1.2 模块划分
```mermaid
graph LR
    A[系统模块] --> B[用户管理]
    A --> C[图书管理]
    A --> D[认证管理]
    
    B --> B1[用户注册]
    B --> B2[用户信息]
    B --> B3[用户认证]
    
    C --> C1[图书CRUD]
    C --> C2[图书查询]
    C --> C3[库存管理]
    
    D --> D1[JWT认证]
    D --> D2[令牌管理]
    D --> D3[权限控制]
```

## 2. 数据库设计

### 2.1 ER图
```mermaid
erDiagram
    USERS ||--o{ TOKENS : has
    USERS {
        string id PK
        string username UK
        string password
        string email UK
        datetime created_at
        datetime updated_at
    }
    
    BOOKS {
        string id PK
        string title
        string author
        string isbn UK
        string description
        string type
        int quantity
        datetime created_at
        datetime updated_at
    }
    
    TOKENS {
        string id PK
        string user_id FK
        string token UK
        datetime expires_at
        datetime created_at
    }
```

### 2.2 实体关系说明

1. 用户（USERS）
   - 主键：id（UUID）
   - 唯一约束：username, email
   - 与令牌是一对多关系

2. 图书（BOOKS）
   - 主键：id（UUID）
   - 唯一约束：isbn
   - 包含库存数量字段

3. 令牌（TOKENS）
   - 主键：id（UUID）
   - 外键：user_id（关联USERS表）
   - 唯一约束：token
   - 包含过期时间

## 3. 类设计

### 3.1 核心类图
```mermaid
classDiagram
    class User {
        +String id
        +String username
        +String password
        +String email
        +DateTime created_at
        +DateTime updated_at
        +register()
        +login()
        +logout()
    }
    
    class Book {
        +String id
        +String title
        +String author
        +String isbn
        +String description
        +String type
        +int quantity
        +DateTime created_at
        +DateTime updated_at
        +create()
        +update()
        +delete()
        +query()
    }
    
    class Token {
        +String id
        +String user_id
        +String token
        +DateTime expires_at
        +DateTime created_at
        +generate()
        +validate()
        +revoke()
    }
    
    User "1" -- "n" Token : has
```

### 3.2 数据模型类
```mermaid
classDiagram
    class UserModel {
        +String id
        +String username
        +String password
        +String email
        +DateTime created_at
        +DateTime updated_at
    }
    
    class BookModel {
        +String id
        +String title
        +String author
        +String isbn
        +String description
        +String type
        +int quantity
        +DateTime created_at
        +DateTime updated_at
    }
    
    class TokenModel {
        +String id
        +String user_id
        +String token
        +DateTime expires_at
        +DateTime created_at
    }
```

## 4. 接口设计

### 4.1 接口时序图
```mermaid
sequenceDiagram
    participant Client
    participant Auth
    participant User
    participant Book
    
    Client->>Auth: 登录请求
    Auth->>User: 验证用户
    User-->>Auth: 返回验证结果
    Auth-->>Client: 返回JWT令牌
    
    Client->>Book: 请求图书列表(带令牌)
    Book->>Auth: 验证令牌
    Auth-->>Book: 验证结果
    Book-->>Client: 返回图书列表
```

### 4.2 认证流程
```mermaid
stateDiagram-v2
    [*] --> 未认证
    未认证 --> 已认证: 登录成功
    已认证 --> 未认证: 登出/令牌过期
    已认证 --> 已认证: 刷新令牌
```

## 5. 安全设计

### 5.1 认证流程
```mermaid
graph LR
    A[请求] --> B{验证令牌}
    B -->|有效| C[处理请求]
    B -->|无效| D[返回401]
    C --> E[返回响应]
```

### 5.2 数据加密
```mermaid
graph TB
    A[明文密码] --> B[加密存储]
    B --> C[数据库]
    D[登录请求] --> E[验证密码]
    E --> F[生成令牌]
``` 