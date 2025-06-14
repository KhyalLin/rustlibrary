# 图书馆管理系统需求文档

## 1. 项目概述

### 1.1 项目背景
针对暨南大学图书馆大规模图书的存储和维护难题，为减轻图书馆管理人员与志愿者的负担，本项目旨在开发一个现代化的图书馆管理系统，用于管理图书资源、用户信息和借阅记录。系统采用 Rust 语言开发，提供 RESTful API 接口，支持前后端分离架构。

### 1.2 项目目标
本项目的目标是实现图书资源的数字化管理，提供用户友好的接口；确保数据安全和系统稳定性，支持系统的可扩展性。

## 2. 功能需求

### 2.1 用户管理
#### 2.1.1 用户注册
- 用户可以通过提供用户名、密码和邮箱进行注册
- 用户名和邮箱必须唯一
- 密码需要进行加密存储
- 注册成功后返回用户基本信息

#### 2.1.2 用户认证
- 支持用户登录功能
- 使用 JWT 进行身份认证
- 登录成功后返回访问令牌
- 支持用户登出功能

### 2.2 图书管理
#### 2.2.1 图书信息管理
- 支持添加新图书
- 支持修改图书信息
- 支持删除图书
- 支持查询单本图书详情

#### 2.2.2 图书查询
- 支持分页查询图书列表
- 支持按图书ID精确查询
- 支持按标题模糊查询
- 支持按作者模糊查询
- 支持多条件组合查询

### 2.3 数据管理
#### 2.3.1 图书信息
- 图书ID（UUID）
- 标题
- 作者
- ISBN
- 描述
- 类型
- 库存数量
- 创建时间
- 更新时间

#### 2.3.2 用户信息
- 用户ID（UUID）
- 用户名
- 密码（加密存储）
- 邮箱
- 创建时间
- 更新时间

#### 2.3.3 令牌信息
- 令牌ID（UUID）
- 用户ID
- 令牌内容
- 过期时间
- 创建时间

## 3. 非功能需求

### 3.1 性能需求
- API 响应时间：95% 的请求应在 500ms 内响应
- 并发处理：支持至少 100 个并发用户
- 系统吞吐量：每秒至少处理 50 个请求

### 3.2 安全需求
- 所有密码通过 crypt 加密存储
- 使用 JWT 进行身份认证
- 所有 API 接口需要进行鉴权
- 防止 SQL 注入攻击
- 防止 XSS 攻击

### 3.3 可用性需求
- 系统可用性：99.9%
- 支持 7*24 小时运行
- 系统维护时间：每月不超过 4 小时

### 3.4 可扩展性需求
- 支持水平扩展
- 支持数据库读写分离
- 支持分布式部署

### 3.5 可维护性需求
- 代码需要良好的注释
- 遵循 Rust 编码规范
- 提供完整的 API 文档
- 提供部署文档

## 4. 技术架构

### 4.1 开发环境
- 编程语言：Rust
- Web 框架：Actix-web
- 数据库：MySQL
- ORM：SQLx
- 认证：JWT

### 4.2 系统架构
- 采用 RESTful API 设计
- 前后端分离架构
- 使用 Docker 容器化部署
- 使用 Nginx 作为反向代理

## 5. 项目规划（单位:人天）

1. 需求分析和设计（3）
2. 数据库设计和实现（1）
3. 核心功能开发（7）
4. 测试和优化（3）
