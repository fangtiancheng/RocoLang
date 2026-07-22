use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, RocoDisplayItem, item_id, item_count, item_type);
    register_optional_getters!(engine, RocoOptionalI64);
    register_optional_getters!(engine, RocoOptionalDisplayItem);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_context_splits_domain_and_action() {
        let context = RocoRequestContext::from_raw("virgo.query_status");
        assert_eq!(context.raw, "virgo.query_status");
        assert_eq!(context.domain(), "virgo");
        assert_eq!(context.action(), "query_status");
    }

    #[test]
    fn request_context_preserves_unknown_single_segment() {
        let context = RocoRequestContext::from_raw("legacy");
        assert_eq!(context.raw, "legacy");
        assert_eq!(context.domain(), "legacy");
        assert_eq!(context.action(), "");
    }

    #[test]
    fn request_context_serde_preserves_derived_fields() {
        let context = RocoRequestContext::from_raw("virgo.query_status");
        let value = serde_json::to_value(&context).expect("request context should serialize");
        assert_eq!(value["raw"], "virgo.query_status");
        assert_eq!(value["domain"], "virgo");
        assert_eq!(value["action"], "query_status");

        let decoded: RocoRequestContext =
            serde_json::from_value(value).expect("request context should deserialize");
        assert_eq!(decoded, context);
    }

    #[test]
    fn reward_kind_has_stable_script_code() {
        assert_eq!(RocoRewardKind::Item.code(), "item");
        assert_eq!(RocoRewardKind::AssignableExp.code(), "assignable_exp");
        assert_eq!(RocoRewardKind::SpiritEquipment.code(), "spirit_equipment");
        assert_eq!(RocoRewardKind::TimedDress.code(), "timed_dress");
    }

    #[test]
    fn optional_i64_tracks_presence_without_sentinel() {
        let missing = RocoOptionalI64::from(None::<u32>);
        let present = RocoOptionalI64::from(Some(7_u32));
        assert!(!missing.is_present());
        assert_eq!(missing.value(), None);
        assert!(present.is_present());
        assert_eq!(present.value(), Some(7));
    }
}
