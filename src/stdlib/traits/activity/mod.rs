mod adventure;
mod alchemy_furnace;
mod evolution;
mod homestead;
mod magic_pioneer;
mod news;
mod tower;
mod zodiac;

pub use adventure::*;
pub use alchemy_furnace::*;
pub use evolution::*;
pub use homestead::*;
pub use magic_pioneer::*;
pub use news::*;
pub use tower::*;
pub use zodiac::*;

/// Event and activity APIs.
pub trait RocoActivityStdLib:
    RocoHomeActivityStdLib
    + RocoManorActivityStdLib
    + RocoPetTrainingActivityStdLib
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
    T: RocoHomeActivityStdLib
        + RocoManorActivityStdLib
        + RocoPetTrainingActivityStdLib
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
