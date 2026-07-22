use ::rhai::Engine;

mod aquarius;
mod aries;
mod cancer;
mod capricorn;
mod gemini;
mod leo;
mod libra;
mod pisces;
mod sagittarius;
mod scorpio;
mod taurus;
mod virgo;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    aquarius::register_rhai_getters(engine);
    aries::register_rhai_getters(engine);
    cancer::register_rhai_getters(engine);
    capricorn::register_rhai_getters(engine);
    gemini::register_rhai_getters(engine);
    leo::register_rhai_getters(engine);
    libra::register_rhai_getters(engine);
    pisces::register_rhai_getters(engine);
    sagittarius::register_rhai_getters(engine);
    scorpio::register_rhai_getters(engine);
    taurus::register_rhai_getters(engine);
    virgo::register_rhai_getters(engine);
}
