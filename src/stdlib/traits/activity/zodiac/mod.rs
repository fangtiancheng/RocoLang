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
mod three_starters;
mod virgo;

pub use aquarius::*;
pub use aries::*;
pub use cancer::*;
pub use capricorn::*;
pub use gemini::*;
pub use leo::*;
pub use libra::*;
pub use pisces::*;
pub use sagittarius::*;
pub use scorpio::*;
pub use taurus::*;
pub use three_starters::*;
pub use virgo::*;

pub trait RocoZodiacActivityStdLib:
    Send
    + RocoAriesActivityStdLib
    + RocoLibraActivityStdLib
    + RocoLeoActivityStdLib
    + RocoCancerActivityStdLib
    + RocoVirgoActivityStdLib
    + RocoPiscesActivityStdLib
    + RocoTaurusActivityStdLib
    + RocoThreeStartersActivityStdLib
    + RocoGeminiActivityStdLib
    + RocoSagittariusActivityStdLib
    + RocoScorpioActivityStdLib
    + RocoAquariusActivityStdLib
    + RocoCapricornActivityStdLib
{
}

impl<T> RocoZodiacActivityStdLib for T where
    T: Send
        + RocoAriesActivityStdLib
        + RocoLibraActivityStdLib
        + RocoLeoActivityStdLib
        + RocoCancerActivityStdLib
        + RocoVirgoActivityStdLib
        + RocoPiscesActivityStdLib
        + RocoTaurusActivityStdLib
        + RocoThreeStartersActivityStdLib
        + RocoGeminiActivityStdLib
        + RocoSagittariusActivityStdLib
        + RocoScorpioActivityStdLib
        + RocoAquariusActivityStdLib
        + RocoCapricornActivityStdLib
{
}
