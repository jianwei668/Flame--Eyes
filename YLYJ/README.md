# 云链医检安全加密网关 (CC-G1) — 软件部分

CC-G1 是一款面向医疗机构的轻量化数据安全网关，通过 **Rust 高性能后端** + **Flutter Web 前端**，实现医疗数据（HL7/DICOM）的本地解析、脱敏、国密加签、单向隔离传输及区块链存证，并提供直观的安全态势看板。

## 📁 后端架构图

```
backend/
├── Cargo.toml                 # 依赖配置 + 编译优化参数 (LTO/strip)
├── Cargo.lock                 # 依赖版本锁定
├── .env                       # 环境变量 (端口/设备名/日志级别)
├── .gitignore
├── README.md                  # 项目说明 + 部署文档
│
├── src/
│   ├── main.rs                # 【入口】初始化日志/配置/状态 → 挂载路由 → 启动服务
│   │
│   ├── config.rs              # 【配置层】读取 .env，定义运行时参数
│   │   └── 提供: AppConfig { port, device_name, log_level }
│   │
│   ├── state.rs               # 【状态层】全局共享状态 (Arc + Mutex)
│   │   └── 提供: AppState { device_name, stats, crypto_keys?, db_pool? }
│   │
│   ├── models/                # 【数据层】所有请求/响应/业务数据结构
│   │   ├── mod.rs
│   │   ├── dtos.rs            # Data Transfer Objects (Flutter ↔ Rust 通信契约)
│   │   │   ├── GatewayStats   # 设备流水线状态
│   │   │   ├── HealthResponse # 健康检查响应
│   │   │   ├── UploadRequest  # Flutter 上报数据请求体
│   │   │   └── BlockchainProof # 存证回执结构
│   │   └── entities.rs        # (后续) 数据库实体 / HL7 解析结构
│   │
│   ├── handlers/              # 【路由层】HTTP 请求处理器 (薄层，只负责参数提取)
│   │   ├── mod.rs
│   │   ├── health.rs          # GET /api/health
│   │   ├── status.rs          # GET /api/device/status
│   │   ├── upload.rs          # POST /api/data/upload (Flutter 主动上报)
│   │   └── admin.rs           # (后续) 管理接口: 配置更新/日志拉取
│   │
│   ├── services/              # 【业务层】核心流水线引擎 (无 HTTP 依赖)
│   │   ├── mod.rs
│   │   ├── pipeline.rs        # 主流水线: 抓取→脱敏→加签→推送 (tokio::spawn)
│   │   ├── anonymizer.rs      # 脱敏引擎: 抹除 PID/NK1/姓名/身份证
│   │   ├── crypto.rs          # 国密模块: SM2 签名 / SM3 摘要 / SM4 加密 (占位)
│   │   ├── uploader.rs        # 推送模块: TLS 1.3 封装 + 云端 API 调用
│   │   └── blockchain.rs      # (后续) 区块链存证: 生成指纹 + 上链交互
│   │
│   ├── network/               # 【网络层】底层网卡/协议操作
│   │   ├── mod.rs
│   │   ├── sniffer.rs         # (后续) libpcap/AF_XDP 混杂模式抓包
│   │   ├── hl7_parser.rs      # (后续) HL7/DICOM 协议解析器
│   │   └── firewall.rs        # (后续) iptables 规则生成 + ip_forward 校验
│   │
│   └── middleware/            # 【中间件层】请求拦截/增强
│       ├── mod.rs
│       ├── cors.rs            # CORS 配置 (Flutter Web 联调必备)
│       ├── logger.rs          # 请求日志 + 耗时统计
│       └── auth.rs            # (后续) API Key / JWT 鉴权
│
├── tests/                     # 【测试层】集成测试
│   ├── mod.rs
│   ├── api_health_test.rs     # /api/health 响应校验
│   ├── pipeline_mock_test.rs  # 流水线逻辑模拟测试
│   └── cors_integration_test.rs # Flutter 跨域联调测试
│
├── scripts/                   # 【运维脚本】一键部署/防火墙配置
│   ├── deploy.sh              # 编译 + 拷贝二进制 + systemd 注册
│   ├── firewall_init.sh       # 配置 iptables: 关闭转发 + 单向隔离
│   └── oled_display.sh        # (后续) 前面板 OLED 状态同步
│
└── web-admin/                 # 【前端】Flutter Web 编译产物 (静态托管)
    ├── index.html
    ├── assets/
    └── (由 flutter build web --release 生成)
```



## 🎯  软件的核心功能

- **医疗协议深度解析**  
  内置 HL7、DICOM、MLLP 解析器，在边缘端完成数据语义识别与标准化。

- **国密加密与数字签名**  
  支持 SM2/SM3/SM4 国密算法，数据离院前完成签名，构建不可篡改的信任链。

- **单向安全隔离**  
  内核层关闭 IP 转发，Rust 进程实现内存级单向“数据搬运”，杜绝反向渗透。

- **区块链边缘锚定**  
  硬件作为区块链节点，数据产生瞬间上链存证，解决医疗纠纷源头信任问题。

- **可视化安全看板**  
  Flutter Web 控制台实时展示数据流向、加密状态、区块高度、拦截事件。

- **SaaS 多租户支持**  
  提供订阅管理、按量计费、API 调用计费、合规审计报告生成。

## 🧱 技术架构实现原理

### 后端 (Rust)

| 组件       | 技术选型                                    |
| ---------- | ------------------------------------------- |
| Web 框架   | Axum + Tokio (异步高性能)                   |
| 协议解析   | 自研 HL7/DICOM 解析器 (纯 Rust)             |
| 加密模块   | 国密 SM2/SM3/SM4 (纯 Rust crate)            |
| 数据序列化 | Serde + Serde JSON                          |
| 数据库驱动 | SQLx (PostgreSQL)                  |
| 跨架构编译 | cross (支持 x86_64 / loongarch64 / aarch64) |
| API 风格   | RESTful + WebSocket (实时状态推送)          |

### 前端 (Flutter Web)

| 组件        | 技术选型            |
| ----------- | ------------------- |
| UI 框架     | Flutter (Web 编译)  |
| 状态管理    | Provider / Riverpod |
| HTTP 客户端 | Dio / http          |
| 图表可视化  | fl_chart            |
| 实时通信    | WebSocket 客户端    |

### 部署模式

- **开发环境**：Rust 后端运行于 `localhost:8080`，Flutter Web 运行于 `localhost:3000` 
- **生产环境**：Nginx 托管 Flutter Web 静态文件，反向代理 Rust API；后端使用 Systemd 托管
- **信创环境**：**计划支持**统信 UOS (x86_64&aarch64) & 龙芯 LoongArch ， 争取拿下兼容性认证

### 
