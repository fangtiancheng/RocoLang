use ::rhai::Engine;

use super::super::*;

pub(super) fn register(engine: &mut Engine) {
    engine.register_type_with_name::<RocoErrorInfo>("RocoErrorInfo");
    engine.register_type_with_name::<RocoErrorDetail>("RocoErrorDetail");
    engine.register_type_with_name::<RocoErrorKind>("RocoErrorKind");
    engine.register_type_with_name::<RocoGeneralLockTarget>("RocoGeneralLockTarget");
    engine.register_type_with_name::<RocoInvalidParamInfo>("RocoInvalidParamInfo");
    engine.register_type_with_name::<RocoInvalidParamDetail>("RocoInvalidParamDetail");
    engine.register_type_with_name::<RocoInvalidParamKind>("RocoInvalidParamKind");
    engine.register_type_with_name::<RocoProtocolFieldName>("RocoProtocolFieldName");
    engine.register_type_with_name::<RocoParamRange>("RocoParamRange");
    engine.register_type_with_name::<RocoNetworkErrorKind>("RocoNetworkErrorKind");
    engine.register_type_with_name::<RocoBridgeErrorChannel>("RocoBridgeErrorChannel");
    engine.register_type_with_name::<RocoBridgeErrorInfo>("RocoBridgeErrorInfo");
    engine.register_type_with_name::<RocoNetResponseParseFailure>("RocoNetResponseParseFailure");
    engine.register_type_with_name::<RocoNetResponseParseSource>("RocoNetResponseParseSource");
    engine.register_type_with_name::<RocoNetResponseParseTarget>("RocoNetResponseParseTarget");
    engine.register_type_with_name::<RocoProtocolParseReason>("RocoProtocolParseReason");
    engine.register_type_with_name::<RocoProtocolParseContext>("RocoProtocolParseContext");
    engine
        .register_type_with_name::<RocoSpiritStorageProtoContext>("RocoSpiritStorageProtoContext");
    engine.register_type_with_name::<RocoSpiritStorageProtoField>("RocoSpiritStorageProtoField");
    engine.register_type_with_name::<RocoProtocolParseFailureKind>("RocoProtocolParseFailureKind");
    engine.register_type_with_name::<RocoProtocolParseErrorType>("RocoProtocolParseErrorType");
    engine.register_type_with_name::<RocoProtocolParseLayer>("RocoProtocolParseLayer");
    engine.register_type_with_name::<ScriptBridgeOperation>("ScriptBridgeOperation");
    engine.register_type_with_name::<ScriptBridgeFailure>("ScriptBridgeFailure");
    engine.register_type_with_name::<ScriptSystemOperation>("ScriptSystemOperation");
    engine.register_type_with_name::<ScriptSystemFailure>("ScriptSystemFailure");
    engine.register_type_with_name::<ScriptSystemFailureSource>("ScriptSystemFailureSource");
    engine.register_type_with_name::<ScriptCombatCommandFailureKind>(
        "ScriptCombatCommandFailureKind",
    );
    engine.register_type_with_name::<ScriptBackendCombatRuntimeErrorKind>(
        "ScriptBackendCombatRuntimeErrorKind",
    );
    engine.register_type_with_name::<ScriptCombatPhase>("ScriptCombatPhase");
    engine.register_type_with_name::<ScriptFunctionContextError>("ScriptFunctionContextError");
    engine.register_type_with_name::<ScriptQueryError>("ScriptQueryError");
}
