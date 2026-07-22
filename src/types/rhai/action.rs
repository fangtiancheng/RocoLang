use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, ActionResult, ok, code, message, error);
    register_error_info_getters!(engine, ActionResult);
}
