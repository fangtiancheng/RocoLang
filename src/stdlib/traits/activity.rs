mod adventure;
mod alchemy;
mod aquarius;
mod aries;
mod cancer;
mod evolution;
mod gemini;
mod leo;
mod libra;
mod magic_pioneer;
mod manor;
mod news;
mod pisces;
mod sagittarius;
mod scorpio;
mod taurus;
mod three_starters;
mod tower;
mod virgo;

pub use adventure::*;
pub use alchemy::*;
pub use aquarius::*;
pub use aries::*;
pub use cancer::*;
pub use evolution::*;
pub use gemini::*;
pub use leo::*;
pub use libra::*;
pub use magic_pioneer::*;
pub use manor::*;
pub use news::*;
pub use pisces::*;
pub use sagittarius::*;
pub use scorpio::*;
pub use taurus::*;
pub use three_starters::*;
pub use tower::*;
pub use virgo::*;

pub trait RocoZodiacActivityStdLib:
    RocoAriesActivityStdLib
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
    + Send
{
}

impl<T> RocoZodiacActivityStdLib for T where
    T: RocoAriesActivityStdLib
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
        + Send
{
}

/// Event and activity APIs.
pub trait RocoActivityStdLib:
    RocoManorActivityStdLib
    + RocoNewsActivityStdLib
    + RocoTowerActivityStdLib
    + RocoAlchemyActivityStdLib
    + RocoEvolutionActivityStdLib
    + RocoMagicPioneerActivityStdLib
    + RocoAdventureActivityStdLib
    + RocoZodiacActivityStdLib
    + Send
{
}

impl<T> RocoActivityStdLib for T where
    T: RocoManorActivityStdLib
        + RocoNewsActivityStdLib
        + RocoTowerActivityStdLib
        + RocoAlchemyActivityStdLib
        + RocoEvolutionActivityStdLib
        + RocoMagicPioneerActivityStdLib
        + RocoAdventureActivityStdLib
        + RocoZodiacActivityStdLib
        + Send
{
}
