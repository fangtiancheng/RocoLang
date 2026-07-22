use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionValidationKind {
    CannotSubmitAction,
    BattleFactsUnavailable,
    InvalidActivePosition {
        position: u8,
    },
    SpiritPositionMismatch {
        active: u8,
        request: u8,
    },
    ActiveSpiritNotFound {
        position: u8,
    },
    ActiveSpiritFainted {
        position: u8,
    },
    ActionAvailabilityUnavailable,
    ActionUnavailable {
        action: ScriptCombatActionAvailabilityKind,
    },
    SkillUnavailableOrNoPp {
        skill_id: u32,
    },
    TargetSpiritAlreadyActive {
        position: u8,
    },
    TargetSpiritNotFound {
        position: u8,
    },
    TargetSpiritFainted {
        position: u8,
    },
    InvalidItemId {
        item_id: u32,
    },
}

impl ScriptCombatActionValidationKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::CannotSubmitAction => "cannot_submit_action",
            Self::BattleFactsUnavailable => "battle_facts_unavailable",
            Self::InvalidActivePosition { .. } => "invalid_active_position",
            Self::SpiritPositionMismatch { .. } => "spirit_position_mismatch",
            Self::ActiveSpiritNotFound { .. } => "active_spirit_not_found",
            Self::ActiveSpiritFainted { .. } => "active_spirit_fainted",
            Self::ActionAvailabilityUnavailable => "action_availability_unavailable",
            Self::ActionUnavailable { .. } => "action_unavailable",
            Self::SkillUnavailableOrNoPp { .. } => "skill_unavailable_or_no_pp",
            Self::TargetSpiritAlreadyActive { .. } => "target_spirit_already_active",
            Self::TargetSpiritNotFound { .. } => "target_spirit_not_found",
            Self::TargetSpiritFainted { .. } => "target_spirit_fainted",
            Self::InvalidItemId { .. } => "invalid_item_id",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionAvailabilityKind {
    UseSkill,
    ChangeSpirit,
    UseItem,
    Escape,
}

impl ScriptCombatActionAvailabilityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseSkill => "use skill",
            Self::ChangeSpirit => "change spirit",
            Self::UseItem => "use item",
            Self::Escape => "escape",
        }
    }
}

impl fmt::Display for ScriptCombatActionAvailabilityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptCombatActionValidationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotSubmitAction => {
                f.write_str("cannot submit combat action in current combat runtime phase")
            }
            Self::BattleFactsUnavailable => f.write_str("combat battle facts unavailable"),
            Self::InvalidActivePosition { position } => {
                write!(f, "invalid active spirit position: {position}")
            }
            Self::SpiritPositionMismatch { active, request } => write!(
                f,
                "combat action spirit_position mismatch: active={active}, request={request}"
            ),
            Self::ActiveSpiritNotFound { position } => {
                write!(f, "active spirit not found: {position}")
            }
            Self::ActiveSpiritFainted { position } => {
                write!(f, "active spirit is fainted: {position}")
            }
            Self::ActionAvailabilityUnavailable => {
                f.write_str("combat action availability unavailable")
            }
            Self::ActionUnavailable { action } => {
                write!(f, "cannot {action} in current combat facts")
            }
            Self::SkillUnavailableOrNoPp { skill_id } => {
                write!(f, "skill unavailable or no PP: skill_id={skill_id}")
            }
            Self::TargetSpiritAlreadyActive { position } => {
                write!(f, "target spirit is already active: {position}")
            }
            Self::TargetSpiritNotFound { position } => {
                write!(f, "no spirit at target position: {position}")
            }
            Self::TargetSpiritFainted { position } => {
                write!(f, "target spirit is fainted: {position}")
            }
            Self::InvalidItemId { item_id } => write!(f, "invalid combat item id: {item_id}"),
        }
    }
}
