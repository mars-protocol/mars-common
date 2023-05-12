use cw_it::test_tube::Account;
use mars_owner::OwnerUpdate;
use mars_testing::{
    test_runner::get_test_runner,
    wasm_oracle::{get_wasm_oracle_contract, WasmOracleTestRobot, ASTRO_ARTIFACTS_PATH},
};
use test_case::test_case;

#[test_case(true ; "caller is owner")]
#[test_case(false => panics ; "caller is not owner")]
fn test_update_admin(caller_is_owner: bool) {
    let runner = get_test_runner();
    let accs = runner.init_accounts();
    let alice = &accs[0];
    let bob = &accs[1];

    let caller = if caller_is_owner {
        alice
    } else {
        bob
    };

    let astroport_contracts =
        cw_it::astroport::utils::get_local_contracts(&runner, &ASTRO_ARTIFACTS_PATH, false, &None);
    let oracle = get_wasm_oracle_contract(&runner);
    let robot = WasmOracleTestRobot::new(&runner, oracle, astroport_contracts, alice, None);

    robot
        .owner_update(
            OwnerUpdate::ProposeNewOwner {
                proposed: bob.address(),
            },
            caller,
        )
        .assert_proposed_new_owner(bob.address());
}

#[test_case(true ; "caller is new owner")]
#[test_case(false => panics ; "caller is not new owner")]
fn test_accept_proposed(caller_is_new_owner: bool) {
    let runner = get_test_runner();
    let accs = runner.init_accounts();
    let alice = &accs[0];
    let bob = &accs[1];

    let caller = if caller_is_new_owner {
        bob
    } else {
        alice
    };

    let astroport_contracts =
        cw_it::astroport::utils::get_local_contracts(&runner, &ASTRO_ARTIFACTS_PATH, false, &None);
    let oracle = get_wasm_oracle_contract(&runner);
    let robot = WasmOracleTestRobot::new(&runner, oracle, astroport_contracts, alice, None);

    robot
        .owner_update(
            OwnerUpdate::ProposeNewOwner {
                proposed: bob.address(),
            },
            alice,
        )
        .owner_update(OwnerUpdate::AcceptProposed, caller)
        .assert_owner(bob.address());
}

#[test_case(true ; "caller is owner")]
#[test_case(false => panics ; "caller is not owner")]
fn test_clear_proposed(caller_is_owner: bool) {
    let runner = get_test_runner();
    let accs = runner.init_accounts();
    let alice = &accs[0];
    let bob = &accs[1];

    let caller = if caller_is_owner {
        alice
    } else {
        bob
    };

    let astroport_contracts =
        cw_it::astroport::utils::get_local_contracts(&runner, &ASTRO_ARTIFACTS_PATH, false, &None);
    let oracle = get_wasm_oracle_contract(&runner);
    let robot = WasmOracleTestRobot::new(&runner, oracle, astroport_contracts, alice, None);

    robot
        .owner_update(
            OwnerUpdate::ProposeNewOwner {
                proposed: bob.address(),
            },
            alice,
        )
        .owner_update(OwnerUpdate::ClearProposed, caller);
}
