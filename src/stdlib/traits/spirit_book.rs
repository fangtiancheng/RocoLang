use super::*;

/// Runtime APIs for spirit-book ownership states.
pub trait RocoSpiritBookStdLib: Send {
    fn get_my_spirit_book_states(&mut self) -> Result<SpiritBookStates> {
        unsupported("spirit_book::get_my_states")
    }

    fn get_role_spirit_book_states(&mut self, _uin: i64) -> Result<SpiritBookStates> {
        unsupported("spirit_book::get_role_states")
    }

    fn get_my_spirit_book_spirit_state(&mut self, spirit_id: i64) -> Result<SpiritBookSpiritState> {
        let states = self.get_my_spirit_book_states()?;
        Ok(resolve_spirit_book_spirit_state(states, spirit_id))
    }

    fn get_role_spirit_book_spirit_state(
        &mut self,
        uin: i64,
        spirit_id: i64,
    ) -> Result<SpiritBookSpiritState> {
        let states = self.get_role_spirit_book_states(uin)?;
        Ok(resolve_spirit_book_spirit_state(states, spirit_id))
    }
}

fn resolve_spirit_book_spirit_state(
    states: SpiritBookStates,
    spirit_id: i64,
) -> SpiritBookSpiritState {
    let state_code = spirit_id
        .checked_sub(1)
        .and_then(|index| usize::try_from(index).ok())
        .and_then(|index| states.states.get(index).copied())
        .unwrap_or(0);
    let state = SpiritBookState::from_code(state_code);

    SpiritBookSpiritState {
        spirit_id,
        state: state.code(),
        owned: state.is_owned(),
    }
}
