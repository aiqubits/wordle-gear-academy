#![no_std]
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

// the metadata to be used by the [IDEA](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara.network) portal.
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct WordleMetadata;

impl Metadata for WordleMetadata {
    type Init = ();
    type Handle = InOut<WordleAction, WordleEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum WordleAction {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum WordleEvent {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
}
