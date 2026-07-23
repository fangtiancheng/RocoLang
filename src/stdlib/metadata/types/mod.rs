use std::collections::{BTreeMap, VecDeque};

use super::{StdlibFieldDoc, StdlibFunctionDoc, StdlibReturnDoc};

pub(super) fn reachable_type_docs(functions: &[StdlibFunctionDoc]) -> Vec<StdlibReturnDoc> {
    let mut pending = functions
        .iter()
        .filter_map(|function| infer_return_type(&function.signature))
        .collect::<VecDeque<_>>();
    let mut docs = BTreeMap::new();

    while let Some(type_name) = pending.pop_front() {
        let normalized = normalize_type_name(&type_name);
        if normalized.is_empty() || docs.contains_key(&normalized) {
            continue;
        }
        let Some(mut doc) = return_doc_for(&normalized) else {
            continue;
        };
        doc.type_name = normalized.clone();
        pending.extend(doc.fields.iter().map(|field| field.type_name.clone()));
        docs.insert(normalized, doc);
    }

    docs.into_values().collect()
}

mod account;
mod activities;
mod core;
mod error;
mod extended;
mod friend;
mod gameplay;
mod homestead;
mod network;
mod spirit;
mod static_data;

include!(concat!(env!("OUT_DIR"), "/roco_struct_docs.rs"));

pub fn return_doc_for(type_name: &str) -> Option<StdlibReturnDoc> {
    let normalized = normalize_type_name(type_name);
    let document = core::doc(&normalized)
        .or_else(|| network::doc(&normalized))
        .or_else(|| error::doc(&normalized))
        .or_else(|| activities::doc(&normalized))
        .or_else(|| gameplay::doc(&normalized))
        .or_else(|| account::doc(&normalized))
        .or_else(|| spirit::doc(&normalized))
        .or_else(|| homestead::doc(&normalized))
        .or_else(|| friend::doc(&normalized))
        .or_else(|| static_data::doc(&normalized))
        .or_else(|| extended::doc(&normalized))
        .or_else(|| generated_struct_doc(&normalized));
    let Some((description, fields)) = document else {
        return fallback_return_doc_for(type_name, &normalized);
    };
    Some(StdlibReturnDoc {
        type_name: type_name.to_string(),
        description: description.to_string(),
        fields,
    })
}

pub fn has_complete_return_doc(type_name: &str) -> bool {
    fn complete(type_name: &str, visiting: &mut std::collections::BTreeSet<String>) -> bool {
        let normalized = normalize_type_name(type_name);
        if matches!(
            normalized.as_str(),
            "()" | "bool"
                | "int"
                | "float"
                | "char"
                | "string"
                | "dynamic"
                | "map"
                | "Map"
                | "blob"
        ) {
            return true;
        }
        if !visiting.insert(normalized.clone()) {
            return true;
        }
        let Some(document) = return_doc_for(&normalized) else {
            visiting.remove(&normalized);
            return false;
        };
        let complete = document
            .fields
            .iter()
            .all(|field| complete(&field.type_name, visiting));
        visiting.remove(&normalized);
        complete
    }

    complete(type_name, &mut std::collections::BTreeSet::new())
}

pub fn infer_return_type(signature: &str) -> Option<String> {
    let return_type = signature.split("->").nth(1)?.trim();
    if return_type.is_empty() || return_type == "()" {
        return None;
    }
    Some(return_type.to_string())
}

fn normalize_type_name(type_name: &str) -> String {
    type_name
        .trim()
        .trim_end_matches('?')
        .split('|')
        .next()
        .unwrap_or_default()
        .trim()
        .trim_end_matches("[]")
        .trim_start_matches("Vec<")
        .trim_end_matches('>')
        .to_string()
}

pub(super) fn field(name: &str, type_name: &str, description: &str) -> StdlibFieldDoc {
    StdlibFieldDoc {
        name: name.to_string(),
        type_name: type_name.to_string(),
        description: description.to_string(),
    }
}

pub(super) fn bag_candidate_fields() -> Vec<StdlibFieldDoc> {
    vec![
        field("candidate_index", "int", "候选项索引。"),
        field("spirit_id", "RocoOptionalI64", "结构化可选宠物 ID。"),
        field("bag_index", "RocoOptionalI64", "结构化可选背包位置。"),
        field("catch_time", "RocoOptionalI64", "结构化可选捕获时间。"),
        field("level", "RocoOptionalI64", "结构化可选等级。"),
        field("need_money", "RocoOptionalI64", "结构化可选所需洛克贝。"),
    ]
}

pub(super) fn exchange_display_item_fields() -> Vec<StdlibFieldDoc> {
    vec![
        field("result_code", "int", "服务器返回结果码。"),
        field("message", "string", "服务器返回信息。"),
        field("item", "RocoOptionalDisplayItem", "结构化可选展示物品。"),
        field("light_num", "int", "光数量。"),
        field("tail_num", "int", "尾巴数量。"),
        field("exchange_count0", "int", "兑换计数 0。"),
        field("exchange_count1", "int", "兑换计数 1。"),
    ]
}

fn fallback_return_doc_for(type_name: &str, normalized: &str) -> Option<StdlibReturnDoc> {
    if has_structured_reward_kind(normalized) {
        return Some(StdlibReturnDoc {
            type_name: type_name.to_string(),
            description: "奖励条目。".to_string(),
            fields: vec![
                field("reward_id", "int", "奖励 ID。"),
                field("reward_kind", "RocoRewardKind", "结构化奖励类型。"),
                field("raw_reward_type", "int", "协议原始奖励类型。"),
                field("count", "int", "奖励数量。"),
            ],
        });
    }

    if has_request_context(normalized) {
        return Some(StdlibReturnDoc {
            type_name: type_name.to_string(),
            description: format!("{type_name} 返回信息。"),
            fields: vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
            ],
        });
    }

    None
}

fn has_structured_reward_kind(type_name: &str) -> bool {
    matches!(
        type_name,
        "ThreeStartersRewardItem"
            | "MagicPioneerRewardItem"
            | "AlchemyFurnaceRewardItem"
            | "UnicornRewardItem"
            | "FourSeasonsRewardItem"
            | "DiamondTearRewardItem"
            | "IceCrystalRewardItem"
            | "MultiEvolutionRewardItem"
            | "GeminiRewardItem"
            | "SagittariusRewardItem"
    )
}

fn has_request_context(type_name: &str) -> bool {
    matches!(
        type_name,
        "CapricornStarPalaceInfo"
            | "CancerMendShapeInfo"
            | "CancerMendShapeBagInfo"
            | "CancerUnsealMemoriesInfo"
            | "CancerUnsealMemoriesBagInfo"
            | "VirgoServeGodInfo"
            | "VirgoFindHalidomInfo"
            | "VirgoBellFoxInfo"
            | "PiscesFirstInfo"
            | "PiscesSecondInfo"
            | "PiscesThirdInfo"
            | "TaurusFirstInfo"
            | "TaurusSecondInfo"
            | "TaurusThirdInfo"
            | "MagicPioneerInfo"
            | "GeminiFirstInfo"
            | "GeminiSecondInfo"
            | "GeminiThirdInfo"
            | "SagittariusFirstInfo"
            | "SagittariusSecondInfo"
            | "SagittariusThirdInfo"
            | "ScorpioFirstInfo"
            | "ScorpioSecondInfo"
            | "ScorpioThirdInfo"
            | "AriesFirstInfo"
            | "AriesSecondInfo"
            | "AriesThirdInfo"
            | "LibraFirstInfo"
            | "LibraSecondInfo"
            | "LibraThirdInfo"
            | "LeoFirstInfo"
            | "LeoSecondInfo"
            | "LeoThirdInfo"
            | "AquariusFirstInfo"
            | "AquariusSecondInfo"
            | "AquariusThirdInfo"
    )
}

pub(super) fn try_error_fields() -> Vec<StdlibFieldDoc> {
    vec![
        field(
            "error_kind_code",
            "string",
            "稳定错误类型代码；无结构化错误时为空。",
        ),
        field(
            "error_detail_kind_code",
            "string",
            "稳定错误详情代码；无结构化错误或无详情时为空。",
        ),
        field(
            "error_network_kind_code",
            "string",
            "稳定网络错误子类型代码；非网络错误或无结构化错误时为空。",
        ),
        field("error_code", "string", "具体错误代码；无结构化错误时为空。"),
        field(
            "error_message",
            "string",
            "结构化错误说明；无结构化错误时为空。",
        ),
        field(
            "error_detail",
            "RocoErrorDetail",
            "结构化错误详情；无结构化错误时为空详情。",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::return_doc_for;

    #[test]
    fn fallback_info_doc_exposes_structured_request_context() {
        let doc = return_doc_for("MagicPioneerInfo").expect("return doc");

        assert!(doc.fields.iter().any(
            |field| field.name == "request_context" && field.type_name == "RocoRequestContext"
        ));
    }

    #[test]
    fn fallback_reward_doc_exposes_structured_reward_kind() {
        let doc = return_doc_for("MagicPioneerRewardItem").expect("return doc");

        assert!(doc
            .fields
            .iter()
            .any(|field| field.name == "reward_kind" && field.type_name == "RocoRewardKind"));
    }

    #[test]
    fn fallback_info_doc_does_not_invent_request_context() {
        assert!(return_doc_for("VirgoBellFoxStatusInfo").is_none());
    }

    #[test]
    fn fallback_reward_doc_does_not_invent_reward_kind() {
        let doc = return_doc_for("MiniGameRewardItem").expect("return doc");
        assert!(!doc.fields.iter().any(|field| field.name == "reward_kind"));
    }

    #[test]
    fn action_result_doc_exposes_error_summary_and_detail() {
        let doc = return_doc_for("ActionResult").expect("return doc");
        let field_names = doc
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect::<std::collections::HashSet<_>>();

        for name in [
            "error_kind_code",
            "error_detail_kind_code",
            "error_network_kind_code",
            "error_code",
            "error_message",
            "error_detail",
        ] {
            assert!(field_names.contains(name), "missing metadata field {name}");
        }

        for name in [
            "error_invalid_param_kind_code",
            "error_bridge_code",
            "error_bridge_operation_code",
            "error_net_response_parse_target",
            "error_return_code_kind_code",
            "error_http_business_result_code",
        ] {
            assert!(
                !field_names.contains(name),
                "stale flattened detail metadata field {name}"
            );
        }
    }
}
