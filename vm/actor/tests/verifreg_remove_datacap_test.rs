// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod common;

use address::Address;
use common::*;
use fil_types::StoragePower;
use forest_actor::verifreg;
use forest_actor::verifreg::VerifierParams;
use forest_actor::{
    account::State, ACCOUNT_ACTOR_CODE_ID, SYSTEM_ACTOR_ADDR, SYSTEM_ACTOR_CODE_ID,
    VERIFREG_ACTOR_CODE_ID,
};
use std::convert::TryInto;
use vm::{ExitCode, Serialized};

#[test]
fn test_remove_datacap() {
    let mut rt = MockRuntime {
        receiver: Address::new_id(100),
        caller: SYSTEM_ACTOR_ADDR.clone(),
        caller_type: SYSTEM_ACTOR_CODE_ID.clone(),
        ..Default::default()
    };
    rt.expect_validate_caller_addr(vec![*SYSTEM_ACTOR_ADDR]);

    // verifier1, verifier2, verifiedClient := addrs[0], addrs[1], addrs[2]
    // let [verifier1] = create_accounts(&mut rt);
    let [verifier1, verifier2, verified_client] = create_accounts(&mut rt);
    // let verifier_allowance = StoragePower::from(2 * (32 << 30));

    // add_verifier(&mut rt, verifier1, verifier_allowance);
    // addVerifierParams := verifreg.AddVerifierParams{
    // 	Address:   verifier1,
    // 	Allowance: verifierAllowance,
    // }
    // vm.ApplyOk(t, v, vm.VerifregRoot, builtin.VerifiedRegistryActorAddr, big.Zero(), builtin.MethodsVerifiedRegistry.AddVerifier, &addVerifierParams)
}

fn add_verifier(rt: &mut MockRuntime, verifier: Address, allowance: StoragePower) {
    let params = VerifierParams {
        address: verifier,
        allowance,
    };
    rt.call(
        &*VERIFREG_ACTOR_CODE_ID,
        verifreg::Method::AddVerifier as vm::MethodNum,
        &Serialized::serialize(params).unwrap(),
    )
    .unwrap();
}

fn create_accounts<const N: usize>(rt: &mut MockRuntime) -> [Address; N] {
    let mut out = Vec::with_capacity(N);
    let seed: u8 = 0x1F;
    for i in 0..N as u8 {
        let addr = Address::new_bls(&[seed + i; address::BLS_PUB_LEN]).unwrap();
        rt.call(
            &*ACCOUNT_ACTOR_CODE_ID,
            1,
            &Serialized::serialize(addr).unwrap(),
        )
        .unwrap();

        out.push(addr);
    }
    out.try_into().unwrap()
}
/*
func CreateAccounts(ctx context.Context, t testing.TB, vm *VM, n int, balance abi.TokenAmount, seed int64) []address.Address {
    var initState initactor.State
    err := vm.GetState(builtin.InitActorAddr, &initState)
    require.NoError(t, err)

    addrPairs := make([]addrPair, n)
    for i := range addrPairs {
        addr := actor_testing.NewBLSAddr(t, seed+int64(i))
        idAddr, err := initState.MapAddressToNewID(vm.store, addr)
        require.NoError(t, err)

        addrPairs[i] = addrPair{
            pubAddr: addr,
            idAddr:  idAddr,
        }
    }
    err = vm.SetActorState(ctx, builtin.InitActorAddr, &initState)
    require.NoError(t, err)

    pubAddrs := make([]address.Address, len(addrPairs))
    for i, addrPair := range addrPairs {
        st := &account.State{Address: addrPair.pubAddr}
        initializeActor(ctx, t, vm, st, builtin.AccountActorCodeID, addrPair.idAddr, balance)
        pubAddrs[i] = addrPair.pubAddr
    }
    return pubAddrs
}
*/