use super::super::super::*;

pub trait RocoManorActivityStdLib: Send {
    fn manor_get_ground_info(&mut self) -> Result<ManorInfo> {
        unsupported("manor::get_ground_info")
    }
    fn manor_get_seed_bag(&mut self) -> Result<Vec<ManorItemCount>> {
        unsupported("manor::get_seed_bag")
    }
    fn manor_get_plant_status(&mut self) -> Result<Vec<ManorPlantStatus>> {
        unsupported("manor::get_plant_status")
    }
    fn manor_reclaim(&mut self, _ground_id: i64) -> Result<ManorReclaimResult> {
        unsupported("manor::reclaim")
    }
    fn manor_sow(&mut self, _seed_id: i64, _ground_id: i64) -> Result<ManorSowResult> {
        unsupported("manor::sow")
    }
    fn manor_reap(&mut self, _ground_id: i64) -> Result<ManorReapResult> {
        unsupported("manor::reap")
    }
    fn manor_uproot(&mut self, _ground_id: i64) -> Result<ManorUprootResult> {
        unsupported("manor::uproot")
    }
    fn manor_weed(&mut self, _ground_id: i64, _weed_type: i64) -> Result<ManorWeedResult> {
        unsupported("manor::weed")
    }
    fn manor_use_fertilizer(
        &mut self,
        _ground_id: i64,
        _fertilizer_item_id: i64,
    ) -> Result<ManorFertilizerResult> {
        unsupported("manor::use_fertilizer")
    }
    fn manor_submit_strawman_result(
        &mut self,
        _game_result: i64,
    ) -> Result<ManorStrawmanPlayResult> {
        unsupported("manor::submit_strawman_result")
    }
    fn manor_claim_strawman_reward(&mut self) -> Result<ManorStrawmanRewardResult> {
        unsupported("manor::claim_strawman_reward")
    }
    fn manor_claim_strawman_gift(&mut self) -> Result<Vec<ManorRewardInfo>> {
        unsupported("manor::claim_strawman_gift")
    }
    fn manor_query_coco_tree(&mut self) -> Result<ManorCocoTreeStatus> {
        unsupported("manor::query_coco_tree")
    }
    fn manor_apply_coco_tree_feed(&mut self, _feed_type: i64) -> Result<ManorCocoTreeFeedResult> {
        unsupported("manor::apply_coco_tree_feed")
    }
    fn manor_query_friend_coco_tree(
        &mut self,
        _friend_uin: i64,
    ) -> Result<ManorFriendCocoTreeStatus> {
        unsupported("manor::query_friend_coco_tree")
    }
    fn manor_apply_friend_coco_tree_feed(
        &mut self,
        _friend_uin: i64,
    ) -> Result<ManorFriendCocoTreeFeedResult> {
        unsupported("manor::apply_friend_coco_tree_feed")
    }
    fn manor_get_friend_list(&mut self, _version: i64) -> Result<Vec<ManorFriendSummary>> {
        unsupported("manor::get_friend_list")
    }
}
