use cw_it::TestRunner;

use mars_oracle_wasm::WasmPriceSourceUnchecked;
use test_case::test_case;

mod helpers;
use helpers::*;

#[test]
fn test_contract_initialization() {
    let runner = get_test_runner();
    let admin = &runner.init_accounts()[0];
    let contract_map = get_contracts(&runner);
    setup_test(&runner, contract_map, admin);
}

#[test_case(get_test_runner(), "uusd", WasmPriceSourceUnchecked::Fixed { price: cosmwasm_std::Decimal::one()})]
fn test_set_price_source(runner: TestRunner, denom: &str, price_source: WasmPriceSourceUnchecked) {
    let admin = &runner.init_accounts()[0];
    let contract_map = get_contracts(&runner);
    let robot = setup_test(&runner, contract_map, admin);
    let admin = &robot.accs[0];

    // Execute SetPriceSource
    robot.set_price_source(&robot.mars_oracle_contract_addr, admin, denom, price_source);
}