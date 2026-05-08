# RocoLang

RocoLang is a Rhai-based automation scripting library for RocoAI. It defines a shared `RocoStdLib` trait that hosts implement on the client or server side.

## Quick Start

```rust
use roco_lang::{RocoError, RocoStdLib, Result};

struct MyStdLib {
    scene_id: i64,
}

impl RocoStdLib for MyStdLib {
    fn move_to_scene(&mut self, scene_id: i64, timeout_ms: i64) -> Result<i64> {
        if timeout_ms <= 0 {
            return Err(RocoError::InvalidParam("timeout_ms must be positive".into()));
        }
        self.scene_id = scene_id;
        Ok(scene_id)
    }

    fn get_current_scene(&mut self) -> Result<i64> {
        Ok(self.scene_id)
    }

    // Implement the rest of RocoStdLib for the embedding host.
}
```

## Script Example

```rust
use roco_lang::RocoEngine;
use std::sync::{Arc, Mutex};

let stdlib = Arc::new(Mutex::new(MyStdLib { scene_id: 1 }));
let mut engine = RocoEngine::new(stdlib);

let script = r#"
    let scene_id = scene::move_to_scene(102, 5000);
    print("moved to scene: " + scene_id);
"#;

match engine.eval(script) {
    Ok(result) => println!("Result: {:?}", result),
    Err(error) => eprintln!("Error: {}", error),
}
```

## Error Model

`RocoStdLib` methods return `Result<T, RocoError>`. `EvalAltResult` is only used inside the Rhai engine boundary; stdlib implementations should report business failures with `RocoError` variants such as `InvalidParam`, `TimeoutError`, `NetworkError`, and `ServerRejected`.

Operation-style `try_*` APIs return `ActionResult` and should not raise expected business failures:

```rhai
let result = spirit::try_swap_spirits(first_position, second_position);
if !result.ok {
    print("swap failed: " + result.message);
}
```

`ActionResult` always has:

- `ok: bool`
- `code: int`
- `message: string`

Query APIs should normally return typed values directly and raise errors on missing data. If a query needs a non-throwing variant, prefer a `find_*` or `maybe_*` name instead of `try_*`.

## API Notes

- Native APIs are exposed under namespaces such as `scene::`, `combat::`, `spirit::`, `lookup::`, `profile::`, `game::`, `session::`, and `system::`.
- `scene::move_to_scene(scene_id: i64, timeout_ms: i64) -> i64` switches scene and returns the confirmed scene id. Failures are raised as script errors.
- `scene::try_move_to_scene(scene_id: i64, timeout_ms: i64) -> ActionResult` is the non-throwing operation form.
- Query methods return typed values directly.
- Action methods return `bool` when the operation has no richer result yet.
- `lookup::lookup_skills_info([ids])` and `lookup::lookup_spirits_info([ids])` preserve input order and raise if any id is missing.
- `spirit::get_storage_spirit_detail(spirit_id, catch_time)` returns full storage spirit details, including skills.
