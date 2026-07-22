use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoNetResponseParseFailure {
    pub target: RocoNetResponseParseTarget,
    pub source: RocoNetResponseParseSource,
}

impl RocoNetResponseParseFailure {
    pub fn target_code(&self) -> String {
        self.target.code().to_string()
    }

    pub fn target_label(&self) -> String {
        self.target.label().to_string()
    }

    pub fn source_kind_code(&self) -> String {
        self.source.kind_code().to_string()
    }

    pub fn protocol_message(&self) -> String {
        self.source.protocol_message()
    }

    pub fn protocol_reason(&self) -> Option<RocoProtocolParseReason> {
        self.source.protocol_reason()
    }

    pub fn protocol_reason_code(&self) -> String {
        self.source.protocol_reason_code()
    }

    pub fn protocol_error_type_code(&self) -> String {
        self.source.protocol_error_type_code()
    }

    pub fn protocol_layer(&self) -> Option<RocoProtocolParseLayer> {
        self.source.protocol_layer()
    }

    pub fn protocol_layer_code(&self) -> String {
        self.source.protocol_layer_code()
    }

    pub fn protocol_kind_code(&self) -> String {
        self.source.protocol_kind_code()
    }

    pub fn unexpected_cmd_id(&self) -> i64 {
        self.source.unexpected_cmd_id()
    }

    pub fn description(&self) -> String {
        format!(
            "failed to parse network response {}: {}",
            self.target.label(),
            self.source
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetResponseParseTarget {
    ChangeScene,
    ReturnCode,
    QueryServerTime,
    RemoteSceneData,
    RemoteNpcValue,
    QuerySpiritBookStates,
    PauseState,
    QuerySpiritBag,
    QuerySpiritItemList,
    QuerySpiritStorageList,
    QuerySpiritStorageListPush,
    QueryAbandonedStorageList,
    QuerySpiritStorageDetail,
    WakeupSkills,
    SwitchSkill,
    UseSpiritItem,
    EquipmentBag,
    SpiritEquipmentOperation,
    HomeOverview,
    HomeFriendList,
    HomeTrainingSpirits,
    HomeTrainingSpiritReport,
    HomeTakeTrainingSpirit,
    HomeCoachSpirits,
    ManorGroundInfo,
    ManorItemCount,
    ManorPlantStatus,
    ManorApplyReclaim,
    ManorApplySow,
    ManorApplyReap,
    ManorApplyUproot,
    ManorApplyWeed,
    ManorUseFertilizer,
    ManorStrawmanPlay,
    ManorStrawmanReward,
    ManorStrawmanGift,
    ManorCocoTreeStatus,
    ManorCocoTreeFeed,
    ManorFriendCocoTreeStatus,
    ManorFriendCocoTreeFeed,
    ManorFriendList,
    ManorFriendDetails,
    NewsTimesQueryReports,
    QueryActivities,
    TaskInfoList,
    TaskProgress,
    IncubativeMachine,
    PetEgg,
    CompleteTask,
    TaskAchievementList,
    MagicGrowupInfo,
    TaskConditionApplyComplete,
    TaskConditionStatus,
    AdventureStatus,
    AdventureOperation,
    AdventureRewards,
    LadderPersonalInfo,
    LadderRank,
    TypeLadderInfo,
    TypeLadderRank,
    CapricornPalaceNotes,
    CapricornInviteList,
    VirgoBellFoxStatus,
    VirgoBellFoxExchange,
    AquariusSecondStatus,
    AquariusSecondExchange,
    AriesThirdStatus,
    AriesThirdExchange,
    LibraThirdStatus,
    LibraThirdExchange,
    LeoFirstStatus,
    LeoFirstExchange,
    CapricornTeamOperation,
    CombatFightRequest,
    CombatItems,
    CombatStartPacket,
}

impl RocoNetResponseParseTarget {
    pub const fn code(self) -> &'static str {
        match self {
            Self::ChangeScene => "change_scene",
            Self::ReturnCode => "return_code",
            Self::QueryServerTime => "query_server_time",
            Self::RemoteSceneData => "remote_scene_data",
            Self::RemoteNpcValue => "remote_npc_value",
            Self::QuerySpiritBookStates => "query_spirit_book_states",
            Self::PauseState => "pause_state",
            Self::QuerySpiritBag => "query_spirit_bag",
            Self::QuerySpiritItemList => "query_spirit_item_list",
            Self::QuerySpiritStorageList => "query_spirit_storage_list",
            Self::QuerySpiritStorageListPush => "query_spirit_storage_list_push",
            Self::QueryAbandonedStorageList => "query_abandoned_storage_list",
            Self::QuerySpiritStorageDetail => "query_spirit_storage_detail",
            Self::WakeupSkills => "wakeup_skills",
            Self::SwitchSkill => "switch_skill",
            Self::UseSpiritItem => "use_spirit_item",
            Self::EquipmentBag => "equipment_bag",
            Self::SpiritEquipmentOperation => "spirit_equipment_operation",
            Self::HomeOverview => "home_overview",
            Self::HomeFriendList => "home_friend_list",
            Self::HomeTrainingSpirits => "home_training_spirits",
            Self::HomeTrainingSpiritReport => "home_training_spirit_report",
            Self::HomeTakeTrainingSpirit => "home_take_training_spirit",
            Self::HomeCoachSpirits => "home_coach_spirits",
            Self::ManorGroundInfo => "manor_ground_info",
            Self::ManorItemCount => "manor_item_count",
            Self::ManorPlantStatus => "manor_plant_status",
            Self::ManorApplyReclaim => "manor_apply_reclaim",
            Self::ManorApplySow => "manor_apply_sow",
            Self::ManorApplyReap => "manor_apply_reap",
            Self::ManorApplyUproot => "manor_apply_uproot",
            Self::ManorApplyWeed => "manor_apply_weed",
            Self::ManorUseFertilizer => "manor_use_fertilizer",
            Self::ManorStrawmanPlay => "manor_strawman_play",
            Self::ManorStrawmanReward => "manor_strawman_reward",
            Self::ManorStrawmanGift => "manor_strawman_gift",
            Self::ManorCocoTreeStatus => "manor_coco_tree_status",
            Self::ManorCocoTreeFeed => "manor_coco_tree_feed",
            Self::ManorFriendCocoTreeStatus => "manor_friend_coco_tree_status",
            Self::ManorFriendCocoTreeFeed => "manor_friend_coco_tree_feed",
            Self::ManorFriendList => "manor_friend_list",
            Self::ManorFriendDetails => "manor_friend_details",
            Self::NewsTimesQueryReports => "news_times_query_reports",
            Self::QueryActivities => "query_activities",
            Self::TaskInfoList => "task_info_list",
            Self::TaskProgress => "task_progress",
            Self::IncubativeMachine => "incubative_machine",
            Self::PetEgg => "pet_egg",
            Self::CompleteTask => "complete_task",
            Self::TaskAchievementList => "task_achievement_list",
            Self::MagicGrowupInfo => "magic_growup_info",
            Self::TaskConditionApplyComplete => "task_condition_apply_complete",
            Self::TaskConditionStatus => "task_condition_status",
            Self::AdventureStatus => "adventure_status",
            Self::AdventureOperation => "adventure_operation",
            Self::AdventureRewards => "adventure_rewards",
            Self::LadderPersonalInfo => "ladder_personal_info",
            Self::LadderRank => "ladder_rank",
            Self::TypeLadderInfo => "type_ladder_info",
            Self::TypeLadderRank => "type_ladder_rank",
            Self::CapricornPalaceNotes => "capricorn_palace_notes",
            Self::CapricornInviteList => "capricorn_invite_list",
            Self::VirgoBellFoxStatus => "virgo_bell_fox_status",
            Self::VirgoBellFoxExchange => "virgo_bell_fox_exchange",
            Self::AquariusSecondStatus => "aquarius_second_status",
            Self::AquariusSecondExchange => "aquarius_second_exchange",
            Self::AriesThirdStatus => "aries_third_status",
            Self::AriesThirdExchange => "aries_third_exchange",
            Self::LibraThirdStatus => "libra_third_status",
            Self::LibraThirdExchange => "libra_third_exchange",
            Self::LeoFirstStatus => "leo_first_status",
            Self::LeoFirstExchange => "leo_first_exchange",
            Self::CapricornTeamOperation => "capricorn_team_operation",
            Self::CombatFightRequest => "combat_fight_request",
            Self::CombatItems => "combat_items",
            Self::CombatStartPacket => "combat_start_packet",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ChangeScene => "ChangeSceneResponse",
            Self::ReturnCode => "ReturnCode",
            Self::QueryServerTime => "QueryServerTimeResponse",
            Self::RemoteSceneData => "RemoteSceneDataResponse",
            Self::RemoteNpcValue => "RemoteNpcValueResponse",
            Self::QuerySpiritBookStates => "SpiritBookStateResponse",
            Self::PauseState => "PauseStateResponse",
            Self::QuerySpiritBag => "QuerySpiritBagResponse",
            Self::QuerySpiritItemList => "QuerySpiritItemListResponse",
            Self::QuerySpiritStorageList => "QuerySpiritStorageListResponse",
            Self::QuerySpiritStorageListPush => "QuerySpiritStorageListResponse push",
            Self::QueryAbandonedStorageList => "QuerySpiritStorageListResponse abandoned",
            Self::QuerySpiritStorageDetail => "QuerySpiritStorageDetailResponse",
            Self::WakeupSkills => "WakeupSkillsResponse",
            Self::SwitchSkill => "SwitchSkillResponse",
            Self::UseSpiritItem => "UseSpiritItemResponse",
            Self::EquipmentBag => "EquipmentBagResponse",
            Self::SpiritEquipmentOperation => "SpiritEquipmentOperationResponse",
            Self::HomeOverview => "HomeOverviewResponse",
            Self::HomeFriendList => "HomeFriendListResponse",
            Self::HomeTrainingSpirits => "HomeTrainingSpiritListResponse",
            Self::HomeTrainingSpiritReport => "HomeTrainingSpiritReportResponse",
            Self::HomeTakeTrainingSpirit => "HomeTakeTrainingSpiritResponse",
            Self::HomeCoachSpirits => "HomeCoachSpiritListResponse",
            Self::ManorGroundInfo => "ManorGroundInfoResponse",
            Self::ManorItemCount => "ManorItemCountResponse",
            Self::ManorPlantStatus => "ManorPlantStatusResponse",
            Self::ManorApplyReclaim => "ManorApplyReclaimResponse",
            Self::ManorApplySow => "ManorApplySowResponse",
            Self::ManorApplyReap => "ManorApplyReapResponse",
            Self::ManorApplyUproot => "ManorApplyUprootResponse",
            Self::ManorApplyWeed => "ManorApplyWeedResponse",
            Self::ManorUseFertilizer => "ManorUseFertilizerResponse",
            Self::ManorStrawmanPlay => "ManorOnScarecrowResultResponse",
            Self::ManorStrawmanReward => "ManorGetStrawmanRewardResponse",
            Self::ManorStrawmanGift => "ManorGetStrawmanGiftResponse",
            Self::ManorCocoTreeStatus => "ManorCocoTreeStatusResponse",
            Self::ManorCocoTreeFeed => "ManorCocoTreeFeedResponse",
            Self::ManorFriendCocoTreeStatus => "ManorFriendCocoTreeStatusResponse",
            Self::ManorFriendCocoTreeFeed => "ManorFriendCocoTreeFeedResponse",
            Self::ManorFriendList => "ManorQqSimpleInfoResponse",
            Self::ManorFriendDetails => "ManorQqDetailInfoResponse",
            Self::NewsTimesQueryReports => "NewsTimesQueryReportsResponse",
            Self::QueryActivities => "QueryActivitiesResponse",
            Self::TaskInfoList => "TaskInfoListResponse",
            Self::TaskProgress => "ReportTaskRsp",
            Self::IncubativeMachine => "IncubativeMachineResponse",
            Self::PetEgg => "PetEggResponse",
            Self::CompleteTask => "CompleteTaskResponse",
            Self::TaskAchievementList => "TaskAchievementListResponse",
            Self::MagicGrowupInfo => "MagicGrowupInfoResponse",
            Self::TaskConditionApplyComplete => "ApplyTaskConditionCompleteResponse",
            Self::TaskConditionStatus => "TaskConditionStatusResponse",
            Self::AdventureStatus => "AdventureStatusResponse",
            Self::AdventureOperation => "AdventureOperationResponse",
            Self::AdventureRewards => "AdventureRewardResponse",
            Self::LadderPersonalInfo => "LadderPersonalInfoResponse",
            Self::LadderRank => "LadderRankResponse",
            Self::TypeLadderInfo => "TypeLadderInfoResponse",
            Self::TypeLadderRank => "TypeLadderRankResponse",
            Self::CapricornPalaceNotes => "CapricornPalaceNotesResponse",
            Self::CapricornInviteList => "CapricornInviteListResponse",
            Self::VirgoBellFoxStatus => "VirgoBellFoxStatusResponse",
            Self::VirgoBellFoxExchange => "VirgoBellFoxExchangeResponse",
            Self::AquariusSecondStatus => "AquariusSecondStatusResponse",
            Self::AquariusSecondExchange => "AquariusSecondExchangeResponse",
            Self::AriesThirdStatus => "AriesThirdStatusResponse",
            Self::AriesThirdExchange => "AriesThirdExchangeResponse",
            Self::LibraThirdStatus => "LibraThirdStatusResponse",
            Self::LibraThirdExchange => "LibraThirdExchangeResponse",
            Self::LeoFirstStatus => "LeoFirstStatusResponse",
            Self::LeoFirstExchange => "LeoFirstExchangeResponse",
            Self::CapricornTeamOperation => "CapricornTeamOperationResponse",
            Self::CombatFightRequest => "CombatFightRequestResponse",
            Self::CombatItems => "CombatItemsResponse",
            Self::CombatStartPacket => "CombatStartPacketResponse",
        }
    }
}

impl RocoNetResponseParseSource {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::Protocol { .. } => "protocol",
            Self::UnexpectedCommand { .. } => "unexpected_command",
        }
    }

    pub fn protocol_message(&self) -> String {
        match self {
            Self::Protocol { reason, .. } => reason.message(),
            _ => String::new(),
        }
    }

    pub fn protocol_reason(&self) -> Option<RocoProtocolParseReason> {
        match self {
            Self::Protocol { reason, .. } => Some(reason.clone()),
            _ => None,
        }
    }

    pub fn protocol_reason_code(&self) -> String {
        match self {
            Self::Protocol { reason, .. } => reason.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn protocol_error_type_code(&self) -> String {
        match self {
            Self::Protocol { error_type, .. } => error_type.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn protocol_layer(&self) -> Option<RocoProtocolParseLayer> {
        match self {
            Self::Protocol { layer, .. } => Some(*layer),
            _ => None,
        }
    }

    pub fn protocol_layer_code(&self) -> String {
        match self {
            Self::Protocol { layer, .. } => layer.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn protocol_kind_code(&self) -> String {
        match self {
            Self::Protocol { kind, .. } => kind.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn unexpected_cmd_id(&self) -> i64 {
        match self {
            Self::UnexpectedCommand { cmd_id } => i64::from(*cmd_id),
            _ => 0,
        }
    }
}
