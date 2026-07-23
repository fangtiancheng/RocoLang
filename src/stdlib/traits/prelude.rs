use super::*;

/// Native APIs exposed to Rhai scripts.
///
/// Convention: operation-style `try_*` APIs return `ActionResult`
/// (`ok/code/message`) instead of raising expected business failures. Plain
/// APIs may raise or return their domain result directly. Query APIs should not
/// use `try_*` unless they also follow the `ActionResult` convention.
pub trait RocoStdLib:
    RocoRuntimeStdLib
    + RocoRemoteStateStdLib
    + RocoIncubativeMachineStdLib
    + RocoPetEggStdLib
    + RocoSpiritStdLib
    + RocoActivityStdLib
    + RocoLookupStdLib
    + RocoCombatStdLib
    + RocoFriendStdLib
    + RocoSpiritBookStdLib
    + RocoSystemStdLib
    + RocoTaskStdLib
    + Send
{
}

impl<T> RocoStdLib for T where
    T: RocoRuntimeStdLib
        + RocoRemoteStateStdLib
        + RocoIncubativeMachineStdLib
        + RocoPetEggStdLib
        + RocoSpiritStdLib
        + RocoActivityStdLib
        + RocoLookupStdLib
        + RocoCombatStdLib
        + RocoFriendStdLib
        + RocoSpiritBookStdLib
        + RocoSystemStdLib
        + RocoTaskStdLib
        + Send
{
}
