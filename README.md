
# Clean Architecture Server

一个遵循`clean arch`架构原则的Rust服务器实现。

A Rust server implementation following clean architecture principles.

## 项目结构

该项目采用Clean Architecture架构，从内到外分为以下几层：

### 领域层 (Domain Layer)
`src/entities/`
- `models/` - 核心业务实体定义
  - `user.rs` - 用户实体
  - `auth.rs` - 认证相关实体
- `repositories/` - 仓储接口定义
- `services/` - 领域服务定义
- `errors.rs` - 领域错误定义

### 应用层 (Application Layer)
`src/services/`
- `auth.rs` - 认证服务实现
- 实现用例和业务流程编排

### 接口层 (Interface Layer)
`src/api/`
- `controllers/` - HTTP请求处理器
- `dto/` - 数据传输对象
- `resp.rs` - 统一响应格式

### 基础设施层 (Infrastructure Layer)
`src/infrastructure/`
- `databases/` - 数据库连接和配置
  - `postgresql.rs` - PostgreSQL数据库实现
- `repositories/` - 仓储接口实现
- `utils/` - 工具类
  - `logger.rs` - 日志工具

### 其他组件
- `src/app_state.rs` - 应用程序状态管理
- `migrations/` - 数据库迁移文件

## 环境变量

### 数据库配置
```env
# 数据库名称
DATABASE_NAME=test

# 数据库连接URL
DATABASE_URL=postgres://<username>:<password>@<host>:<port>
```

### 服务器配置
```env
# 服务器监听地址
SERVER_HOST=127.0.0.1

# 服务器端口
SERVER_PORT=8080
```

### 日志配置
```env
# 日志级别 (trace, debug, info, warn, error)
RUST_LOG=info
```

## 默认配置

如果未设置环境变量，服务器将使用以下默认值：
- SERVER_HOST: "127.0.0.1"
- SERVER_PORT: "8080"
- DATABASE_NAME: "postgres"
- RUST_LOG: "info"