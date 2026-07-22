use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "pet_training",
            "query",
            return_type: "PetTrainingResult",
            "查询并开始家园宠物锻炼，返回本次锻炼使用的宠物 ID。",
            params: ["training_type" => "锻炼类型，来自原版 pet_training 的 type 参数。"],
            returns: "PetTrainingResult。ok 表示 CGI 是否成功，pet_id 是后续结算需要传入的宠物 ID，rewards 在查询阶段通常为空。",
            examples: ["let result = pet_training::query(1);"]
        ),
        super::stdlib_doc!(
            "pet_training",
            "settle",
            return_type: "PetTrainingResult",
            "结算家园宠物锻炼并返回奖励列表。",
            params: ["training_type" => "锻炼类型，必须与 query 使用的 type 一致。", "pet_id" => "query 返回的宠物 ID。"],
            returns: "PetTrainingResult。rewards 包含奖励道具 ID 和数量。",
            examples: ["let result = pet_training::settle(1, query.pet_id);"]
        ),
    ]
}
