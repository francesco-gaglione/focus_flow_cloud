use application::flashcards::traits::spaced_repetition_port::{
    SpacedRepetitionPort, SrsCardState, SrsNextStates,
};
use domain::flashcards::value_objects::memory_state::MemoryState;
use fsrs::{MemoryState as FsrsMemoryState, FSRS};

pub struct FsrsAdapter {
    fsrs: FSRS,
    desired_retention: f32,
}

impl FsrsAdapter {
    pub fn new(desired_retention: f32) -> Self {
        Self {
            fsrs: FSRS::default(),
            desired_retention,
        }
    }
}

impl Default for FsrsAdapter {
    fn default() -> Self {
        Self::new(0.9)
    }
}

fn to_fsrs_memory_state(ms: MemoryState) -> FsrsMemoryState {
    FsrsMemoryState {
        stability: ms.stability(),
        difficulty: ms.difficulty(),
    }
}

fn from_fsrs_memory_state(ms: FsrsMemoryState) -> MemoryState {
    MemoryState::new(ms.stability, ms.difficulty)
}

impl SpacedRepetitionPort for FsrsAdapter {
    fn next_states(
        &self,
        memory_state: Option<MemoryState>,
        elapsed_days: u32,
    ) -> SrsNextStates {
        let fsrs_state = memory_state.map(to_fsrs_memory_state);

        let next = self
            .fsrs
            .next_states(fsrs_state, self.desired_retention, elapsed_days)
            .expect("fsrs next_states failed");

        SrsNextStates {
            again: SrsCardState {
                memory_state: from_fsrs_memory_state(next.again.memory),
                interval_days: next.again.interval.round().max(1.0) as u32,
            },
            hard: SrsCardState {
                memory_state: from_fsrs_memory_state(next.hard.memory),
                interval_days: next.hard.interval.round().max(1.0) as u32,
            },
            good: SrsCardState {
                memory_state: from_fsrs_memory_state(next.good.memory),
                interval_days: next.good.interval.round().max(1.0) as u32,
            },
            easy: SrsCardState {
                memory_state: from_fsrs_memory_state(next.easy.memory),
                interval_days: next.easy.interval.round().max(1.0) as u32,
            },
        }
    }
}
