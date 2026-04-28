# RocoLang

基于 Rhai 的洛克王国自动化测试脚本语言。

## 概述

RocoLang 是一个嵌入式脚本语言库，用于编写洛克王国的自动化测试脚本。它基于 [Rhai](https://rhai.rs/) 构建，提供了一套标准库接口，可以在服务器端和客户端共享使用。

## 架构

```
测试服务器                     RocoAI 客户端
├── RocoEngine                ├── RocoEngine
├── ServerStdLib 实现         ├── ClientStdLib 实现
│   - RPC 调用客户端          │   - 直接调用本地 API
│   - 从缓存读取状态          │   - 零延迟访问状态
└── 协调双客户端              └── 执行战斗指令
```

## 特性

- ✅ 基于成熟的 Rhai 脚本引擎
- ✅ 清晰的标准库接口抽象（`RocoStdLib` trait）
- ✅ 服务器/客户端共享同一套代码
- ✅ 支持自定义函数、闭包、控制流
- ✅ 类型安全的 Rust 集成

## 快速开始

### 定义标准库实现

```rust
use roco_lang::{RocoStdLib, Result, RocoError};

struct MyStdLib {
    // 你的状态
}

impl RocoStdLib for MyStdLib {
    fn move_to_scene(&mut self, scene_id: i64) -> Result<bool> {
        // 实现场景切换
        Ok(true)
    }
    
    fn get_my_hp(&self) -> Result<i64> {
        // 返回当前血量
        Ok(450)
    }
    
    // ... 实现其他函数
}
```

### 执行脚本

```rust
use roco_lang::RocoEngine;
use std::sync::{Arc, Mutex};

let stdlib = Arc::new(Mutex::new(MyStdLib::new()));
let mut engine = RocoEngine::new(stdlib);

let script = r#"
    fn test_battle() {
        move_to_scene(102);
        
        while !is_finished() {
            if get_my_hp() < 100 {
                use_item("体力药剂");
            } else {
                use_skill(10001);
            }
            wait_round_end();
        }
        
        return get_battle_result();
    }
    
    test_battle()
"#;

match engine.eval(script) {
    Ok(result) => println!("Result: {:?}", result),
    Err(e) => eprintln!("Error: {}", e),
}
```

## 标准库函数

### 场景相关
- `move_to_scene(scene_id: i64) -> bool`
- `get_current_scene() -> i64`

### 宠物管理
- `fetch_spirit(catch_time: i64) -> bool`
- `clear_lineup() -> bool`
- `get_spirit_bag() -> SpiritBagInfo`

### 战斗相关
- `invite_pk(target_uin: i64) -> BattleInfo`
- `use_skill(skill_id: i64) -> bool`
- `use_item(item_name: string) -> bool`
- `wait_round_end() -> RoundResult`

### 状态查询
- `get_my_hp() -> i64`
- `get_rival_hp() -> i64`
- `is_finished() -> bool`

### 工具函数
- `log(message: string)`
- `assert(condition: bool, message: string)`

完整 API 文档见 `src/stdlib/traits.rs`。

## 示例脚本

查看 `scripts/` 目录下的示例脚本。

## 许可证

MIT
