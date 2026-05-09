use rhai::{Array, Dynamic, Map};
use serde::{Deserialize, Serialize};

use crate::types::{
    ActionResult, BagItemInfo, BattleCapturedSpirit, BattleInfo, BattleResult, BattleSpiritResult,
    CombatActions, RoundResult, SceneSpiritInfo, SkillInfo, SkillPoolInfo, SkillPoolSkillInfo,
    SkillStoneResult, SkillStoneSkillInfo, SkillSwitchResult, SpiritBagInfo, SpiritInfo,
    SpiritSkillInfo, StaticGuardianPetPropertyInfo, StaticItemInfo, StaticMagicInfo,
    StaticPluginInfo, StaticSkillInfo, StaticSpiritInfo, StaticStriveItemInfo, StaticTitleInfo,
    StorageSpiritInfo, TalentRefreshResult, UserInfo,
};

const MAX_PREVIEW_CHARS: usize = 4_000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDebugBreakpoint {
    pub source: Option<String>,
    pub line: usize,
    pub column: Option<usize>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDebugConfig {
    pub source: Option<String>,
    pub breakpoints: Vec<RocoDebugBreakpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoDebugCommand {
    SetBreakpoints(Vec<RocoDebugBreakpoint>),
    Continue,
    StepInto,
    StepOver,
    Next,
    StepOut,
    Stop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDebugStackFrame {
    pub function_name: String,
    pub source: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub args_preview: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDebugLocalVariable {
    pub name: String,
    pub type_name: String,
    pub value_preview: String,
    pub is_constant: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoDebugEvent {
    Started,
    Paused {
        reason: String,
        source: Option<String>,
        line: Option<usize>,
        column: Option<usize>,
        stack: Vec<RocoDebugStackFrame>,
        locals: Vec<RocoDebugLocalVariable>,
    },
    Continued,
    Ended,
}

pub struct RocoDebugHooks {
    pub on_event: Box<dyn FnMut(RocoDebugEvent) + Send>,
    pub wait_command: Box<dyn FnMut() -> RocoDebugCommand + Send>,
}

impl RocoDebugHooks {
    pub fn new(
        on_event: impl FnMut(RocoDebugEvent) + Send + 'static,
        wait_command: impl FnMut() -> RocoDebugCommand + Send + 'static,
    ) -> Self {
        Self {
            on_event: Box::new(on_event),
            wait_command: Box::new(wait_command),
        }
    }
}

pub(crate) fn dynamic_preview(value: &Dynamic) -> String {
    if let Some(preview) = preview_array(value) {
        return preview;
    }

    if let Some(preview) = preview_map(value) {
        return preview;
    }

    if let Some(preview) = preview_roco_type(value) {
        return preview;
    }

    truncate_preview(format!("{value:?}"))
}

fn preview_array(value: &Dynamic) -> Option<String> {
    let array = value.read_lock::<Array>()?;
    let items = array
        .iter()
        .map(dynamic_preview)
        .collect::<Vec<_>>()
        .join(", ");
    Some(truncate_preview(format!("[{items}]")))
}

fn preview_map(value: &Dynamic) -> Option<String> {
    let map = value.read_lock::<Map>()?;
    let fields = map
        .iter()
        .map(|(key, value)| format!("{key}: {}", dynamic_preview(value)))
        .collect::<Vec<_>>()
        .join(", ");
    Some(truncate_preview(format!("#{{{fields}}}")))
}

fn preview_roco_type(value: &Dynamic) -> Option<String> {
    macro_rules! preview_as_json {
        ($($type:ty),+ $(,)?) => {
            $(
                if let Some(preview) = serialize_preview::<$type>(value) {
                    return Some(preview);
                }
            )+
        };
    }

    preview_as_json!(
        ActionResult,
        BagItemInfo,
        BattleCapturedSpirit,
        BattleInfo,
        BattleResult,
        BattleSpiritResult,
        CombatActions,
        RoundResult,
        SceneSpiritInfo,
        SkillInfo,
        SkillPoolInfo,
        SkillPoolSkillInfo,
        SkillStoneResult,
        SkillStoneSkillInfo,
        SkillSwitchResult,
        SpiritBagInfo,
        SpiritInfo,
        SpiritSkillInfo,
        StaticGuardianPetPropertyInfo,
        StaticItemInfo,
        StaticMagicInfo,
        StaticPluginInfo,
        StaticSkillInfo,
        StaticSpiritInfo,
        StaticStriveItemInfo,
        StaticTitleInfo,
        StorageSpiritInfo,
        TalentRefreshResult,
        UserInfo,
    );

    None
}

fn serialize_preview<T>(value: &Dynamic) -> Option<String>
where
    T: Clone + Serialize + 'static,
{
    let typed = value.read_lock::<T>()?;
    serde_json::to_string(&*typed).ok().map(truncate_preview)
}

fn truncate_preview(preview: String) -> String {
    if preview.chars().count() <= MAX_PREVIEW_CHARS {
        return preview;
    }

    let mut truncated = preview.chars().take(MAX_PREVIEW_CHARS).collect::<String>();
    truncated.push_str("...");
    truncated
}
