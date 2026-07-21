use super::{to_array, Engine};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticItemInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unique: bool,
    pub item_type: i64,
    pub subtype: i64,
    pub price: i64,
    pub expire_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticStriveItemInfo {
    pub id: i64,
    pub name: String,
    pub item_type: i64,
    pub ghp: i64,
    pub gpa: i64,
    pub gpd: i64,
    pub gma: i64,
    pub gmd: i64,
    pub gsp: i64,
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticGuardianPetPropertyInfo {
    pub level: i64,
    pub phase: i64,
    pub energy: i64,
    pub attack: i64,
    pub defend: i64,
    pub magic_attack: i64,
    pub magic_defend: i64,
    pub need_level_to_next_phase: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticTitleInfo {
    pub id: i64,
    pub title_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticMagicInfo {
    pub id: i64,
    pub name: String,
    pub item_id: String,
    pub target: i64,
    pub magic_type: i64,
    pub duration: i64,
    pub action_type: i64,
    pub app: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticPluginInfo {
    pub name: String,
    pub label: String,
    pub domain: String,
    pub version: String,
    pub command_type: String,
    pub plugin_class: String,
    pub plugin_src: String,
    pub plugin_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticTalentInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSkillInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub description2: String,
    pub power: String,
    pub pp_max: i64,
    pub property: i64,
    pub src: String,
    pub attack_type: i64,
    pub speed: i64,
    pub damage_type: i64,
    pub catch_rate: i64,
    pub super_form_id: i64,
    pub super_form_src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSpiritEvolutionEdge {
    pub target_id: i64,
    pub kind: i64,
}

impl StaticSpiritEvolutionEdge {
    pub const KIND_ORDINARY: i64 = 1;
    pub const KIND_DISPLAY: i64 = 2;

    pub fn ordinary(target_id: i64) -> Self {
        Self {
            target_id,
            kind: Self::KIND_ORDINARY,
        }
    }

    pub fn display(target_id: i64) -> Self {
        Self {
            target_id,
            kind: Self::KIND_DISPLAY,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSpiritInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub features: Vec<i64>,
    pub group: Vec<i64>,
    pub src: String,
    pub avatar: String,
    pub icon_src: String,
    pub preview_src: String,
    pub move_speed: i64,
    pub height: String,
    pub weight: String,
    pub color: String,
    pub interest: String,
    pub habitat: String,
    pub evolution: Vec<i64>,
    pub catchrate: i64,
    pub boss_phyle: String,
    pub boss_reward: String,
    pub scene_id: i64,
    pub condition: String,
    pub require_level: String,
    pub wg: i64,
    pub mg: i64,
    pub mk: i64,
    pub sm: i64,
    pub sd: i64,
    pub fy: i64,
    pub reward: i64,
    pub evolution_form_id: i64,
    pub evolution_to_ids: Vec<i64>,
    pub evolution_edges: Vec<StaticSpiritEvolutionEdge>,
    pub get_form: String,
    pub state: i64,
    pub start_time: String,
    pub end_time: String,
    pub first_id: i64,
    pub propo_level: i64,
    pub is_in_book: bool,
    pub skinnum: i64,
    pub exp_type: i64,
}

impl StaticSpiritInfo {
    pub fn unknown(spirit_id: i64) -> Self {
        Self {
            id: spirit_id,
            name: format!("未知({spirit_id})"),
            description: String::new(),
            features: Vec::new(),
            group: Vec::new(),
            src: String::new(),
            avatar: String::new(),
            icon_src: String::new(),
            preview_src: String::new(),
            move_speed: 0,
            height: String::new(),
            weight: String::new(),
            color: String::new(),
            interest: String::new(),
            habitat: String::new(),
            evolution: Vec::new(),
            catchrate: 0,
            boss_phyle: String::new(),
            boss_reward: String::new(),
            scene_id: 0,
            condition: String::new(),
            require_level: String::new(),
            wg: 0,
            mg: 0,
            mk: 0,
            sm: 0,
            sd: 0,
            fy: 0,
            reward: 0,
            evolution_form_id: 0,
            evolution_to_ids: Vec::new(),
            evolution_edges: Vec::new(),
            get_form: String::new(),
            state: 0,
            start_time: String::new(),
            end_time: String::new(),
            first_id: 0,
            propo_level: 0,
            is_in_book: false,
            skinnum: 0,
            exp_type: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSpiritInfoLookupResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub result: StaticSpiritInfo,
}

impl StaticSpiritInfoLookupResult {
    pub fn ok(result: StaticSpiritInfo) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            result,
        }
    }

    pub fn not_found(spirit_id: i64, message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
            result: StaticSpiritInfo::unknown(spirit_id),
        }
    }
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        StaticItemInfo,
        id,
        name,
        description,
        unique,
        item_type,
        subtype,
        price,
        expire_time,
    );
    register_getters!(
        engine,
        StaticStriveItemInfo,
        id,
        name,
        item_type,
        ghp,
        gpa,
        gpd,
        gma,
        gmd,
        gsp,
        src,
    );

    register_getters!(
        engine,
        StaticGuardianPetPropertyInfo,
        level,
        phase,
        energy,
        attack,
        defend,
        magic_attack,
        magic_defend,
        need_level_to_next_phase,
    );
    register_getters!(engine, StaticTitleInfo, id, title_name);
    register_getters!(
        engine,
        StaticMagicInfo,
        id,
        name,
        item_id,
        target,
        magic_type,
        duration,
        action_type,
        app,
        description,
    );
    register_getters!(
        engine,
        StaticPluginInfo,
        name,
        label,
        domain,
        version,
        command_type,
        plugin_class,
        plugin_src,
        plugin_url,
    );
    register_getters!(
        engine,
        StaticSkillInfo,
        id,
        name,
        description,
        description2,
        power,
        pp_max,
        property,
        src,
        attack_type,
        speed,
        damage_type,
        catch_rate,
        super_form_id,
        super_form_src,
    );
    register_getters!(
        engine,
        StaticSpiritInfo,
        id,
        name,
        description,
        src,
        avatar,
        icon_src,
        preview_src,
        move_speed,
        height,
        weight,
        color,
        interest,
        habitat,
        catchrate,
        boss_phyle,
        boss_reward,
        scene_id,
        condition,
        require_level,
        wg,
        mg,
        mk,
        sm,
        sd,
        fy,
        reward,
        evolution_form_id,
        get_form,
        state,
        start_time,
        end_time,
        first_id,
        propo_level,
        is_in_book,
        skinnum,
        exp_type,
    );
    engine.register_get("features", |value: &mut StaticSpiritInfo| {
        to_array(&value.features)
    });
    engine.register_get("group", |value: &mut StaticSpiritInfo| {
        to_array(&value.group)
    });
    engine.register_get("evolution", |value: &mut StaticSpiritInfo| {
        to_array(&value.evolution)
    });
    engine.register_get("evolution_to_ids", |value: &mut StaticSpiritInfo| {
        to_array(&value.evolution_to_ids)
    });
    register_getters!(engine, StaticSpiritEvolutionEdge, target_id, kind);
    engine.register_get("evolution_edges", |value: &mut StaticSpiritInfo| {
        to_array(&value.evolution_edges)
    });
    register_getters!(engine, StaticSpiritInfoLookupResult, ok, code, message);
    engine.register_get("result", |value: &mut StaticSpiritInfoLookupResult| {
        value.result.clone()
    });
}
