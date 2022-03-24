// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use ipld_blockstore::BlockStore;
use serde::Serialize;
use std::error::Error;
use vm::ActorState;

/// Cron actor address.
pub static ADDRESS: &actorv4::CRON_ACTOR_ADDR = &actorv4::CRON_ACTOR_ADDR;

/// Cron actor method.
pub type Method = actorv4::cron::Method;

/// Cron actor state.
#[derive(Serialize)]
#[serde(untagged)]
pub enum State {
    V0(actorv0::cron::State),
    V2(actorv2::cron::State),
    V3(actorv3::cron::State),
    V4(actorv4::cron::State),
    V5(actorv5::cron::State),
    V6(actorv6::cron::State),
}

impl State {
    pub fn load<BS>(store: &BS, actor: &ActorState) -> Result<State, Box<dyn Error>>
    where
        BS: BlockStore,
    {
        if actor.code == *actorv0::CRON_ACTOR_CODE_ID {
            Ok(store
                .get(&actor.state)?
                .map(State::V0)
                .ok_or("Actor state doesn't exist in store")?)
        } else if actor.code == *actorv2::CRON_ACTOR_CODE_ID {
            Ok(store
                .get(&actor.state)?
                .map(State::V2)
                .ok_or("Actor state doesn't exist in store")?)
        } else if actor.code == *actorv3::CRON_ACTOR_CODE_ID {
            Ok(store
                .get(&actor.state)?
                .map(State::V3)
                .ok_or("Actor state doesn't exist in store")?)
        } else if actor.code == *actorv4::CRON_ACTOR_CODE_ID {
            Ok(store
                .get(&actor.state)?
                .map(State::V4)
                .ok_or("Actor state doesn't exist in store")?)
        } else if actor.code == *actorv5::CRON_ACTOR_CODE_ID {
            Ok(store
                .get(&actor.state)?
                .map(State::V5)
                .ok_or("Actor state doesn't exist in store")?)
        } else if actor.code == *actorv6::CRON_ACTOR_CODE_ID {
            Ok(store
                .get(&actor.state)?
                .map(State::V6)
                .ok_or("Actor state doesn't exist in store")?)
        } else {
            Err(format!("Unknown cron actor code {}", actor.code).into())
        }
    }
}
