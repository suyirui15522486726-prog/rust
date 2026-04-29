# Todo CLI

一个使用 Rust 语言实现的命令行待办事项管理工具。

## 功能特性

- ✅ 添加任务 - 创建新的待办事项
- ✅ 查看列表 - 显示所有任务及其状态
- ✅ 标记完成 - 将任务标记为已完成
- ✅ 删除任务 - 删除指定任务
- ✅ 查看详情 - 显示单个任务的详细信息
- ✅ 修改内容 - 编辑任务内容
- ✅ 交互模式 - 支持命令行交互操作

## 技术栈

- **语言**: Rust 1.70+
- **构建工具**: Cargo
- **标准库**: 纯标准库实现，无第三方依赖

## 项目结构

```
src/
├── main.rs        # 程序入口，交互模式主循环
├── todo.rs        # Todo 结构体与方法定义
├── command.rs     # 命令枚举与解析逻辑
└── storage.rs     # 任务存储与 CRUD 操作
```

## 安装方法

```bash
# 克隆仓库
git clone <repository-url>
cd todo_cli

# 编译项目
cargo build --release

# 运行程序
cargo run --release
```

## 使用说明

程序启动后进入交互模式，使用 `>` 提示符等待输入命令：

### 命令列表

| 命令 | 说明 | 示例 |
|------|------|------|
| `add <内容>` | 添加新任务 | `add "学习 Rust"` |
| `list` | 查看所有任务 | `list` |
| `done <ID>` | 标记任务完成 | `done 1` |
| `remove <ID>` | 删除任务 | `remove 2` |
| `show <ID>` | 查看任务详情 | `show 1` |
| `edit <ID> <内容>` | 修改任务内容 | `edit 1 "新内容"` |
| `exit` | 退出程序 | `exit` |

### 使用示例

```
> add "学习 Rust"
Added todo #1
> add 写报告
Added todo #2
> list
ID  状态   内容
1   [ ]   学习 Rust
2   [ ]   写报告
> edit 1 "学习 Rust 基础"
ok
> done 1
ok
> list
ID  状态   内容
1   [x]   学习 Rust 基础
2   [ ]   写报告
> show 1
ID: 1
Status: Done
Content: 学习 Rust 基础
> exit
```

## 项目规范

- ✅ 编译通过: `cargo build`
- ✅ 格式规范: `cargo fmt`
- ✅ 无严重警告: `cargo clippy`

## 边界处理

- 空任务内容检测
- 非法/不存在的任务 ID 处理
- 无效命令处理
- 重复标记完成处理

## 许可证

MIT License

## 作者

**学号**: 2410973  
**姓名**: 苏奕睿