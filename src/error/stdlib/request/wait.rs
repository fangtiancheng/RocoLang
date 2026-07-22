use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptWaitContext {
    WaitNextCombatAction,
    CombatActionSettlingAfterAck,
    UseItemAction,
    TalentRefresh,
    TalentRefreshResultAfterAck,
}

impl ScriptWaitContext {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WaitNextCombatAction => "wait_next_combat_action",
            Self::CombatActionSettlingAfterAck => "combat_action_settling_after_ack",
            Self::UseItemAction => "use_item_action",
            Self::TalentRefresh => "talent_refresh",
            Self::TalentRefreshResultAfterAck => "talent_refresh_result_after_ack",
        }
    }
}

impl fmt::Display for ScriptWaitContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
