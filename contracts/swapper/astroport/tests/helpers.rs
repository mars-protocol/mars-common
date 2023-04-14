use astroport::router::SwapOperation;
use cosmwasm_std::{Coin, Decimal, Uint128};
use cw_it::{
    astroport::{robot::AstroportTestRobot, utils::AstroportContracts},
    cw_multi_test::ContractWrapper,
    multi_test::MultiTestRunner,
    osmosis_test_tube::OsmosisTestApp,
    robot::TestRobot,
    test_tube::{
        osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse, Account, Module,
        RunnerExecuteResult, RunnerResult, SigningAccount, Wasm,
    },
    Artifact, ContractMap, ContractType, TestRunner,
};
use mars_swapper::EstimateExactInSwapResponse;
use mars_swapper_astroport::route::AstroportRoute;
use mars_testing::wasm_oracle::WasmOracleTestRobot;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");

pub const ASTRO_ARTIFACTS_PATH: Option<&str> = Some("tests/astroport-artifacts");

const ARTIFACTS_PATH: &str = "artifacts/";
const APPEND_ARCH: bool = true;

fn get_swapper_wasm_path() -> String {
    wasm_path(ARTIFACTS_PATH, CONTRACT_NAME, APPEND_ARCH)
}

fn wasm_path(artifacts_path: &str, contract_name: &str, append_arch: bool) -> String {
    let contract_name = contract_name.replace("-", "_");
    if append_arch {
        format!("{}/{}-{}.wasm", artifacts_path, contract_name, std::env::consts::ARCH)
    } else {
        format!("{}/{}.wasm", artifacts_path, contract_name)
    }
}

fn get_local_swapper_contract(runner: &TestRunner) -> ContractType {
    match runner {
        TestRunner::PhantomData(_) => panic!("Invalid test runner"),
        TestRunner::OsmosisTestApp(_) => {
            ContractType::Artifact(Artifact::Local(get_swapper_wasm_path()))
        }
        TestRunner::MultiTest(_) => {
            ContractType::MultiTestContract(Box::new(ContractWrapper::new(
                mars_swapper_astroport::contract::execute,
                mars_swapper_astroport::contract::instantiate,
                mars_swapper_astroport::contract::query,
            )))
        }
    }
}

fn get_local_oracle_contract(runner: &TestRunner) -> ContractType {
    match runner {
        TestRunner::PhantomData(_) => panic!("Invalid test runner"),
        TestRunner::OsmosisTestApp(_) => ContractType::Artifact(Artifact::Local(wasm_path(
            ARTIFACTS_PATH,
            "mars-oracle-wasm",
            APPEND_ARCH,
        ))),
        TestRunner::MultiTest(_) => {
            ContractType::MultiTestContract(Box::new(ContractWrapper::new(
                mars_oracle_wasm::contract::entry::execute,
                mars_oracle_wasm::contract::entry::instantiate,
                mars_oracle_wasm::contract::entry::query,
            )))
        }
    }
}

/// Creates an OsmosisTestApp TestRunner
pub fn get_test_runner<'a>() -> TestRunner<'a> {
    match option_env!("TEST_RUNNER").unwrap_or("multi-test") {
        "osmosis-test-tube" => {
            let app = OsmosisTestApp::new();
            TestRunner::OsmosisTestApp(app)
        }
        "multi-test" => TestRunner::MultiTest(MultiTestRunner::new("osmo")),
        _ => panic!("Unsupported test runner type"),
    }
}

pub struct AstroportSwapperRobot<'a> {
    pub runner: &'a TestRunner<'a>,
    /// The astroport swapper contract address
    pub swapper: String,
    /// The mars wasm oracle address
    pub oracle_robot: WasmOracleTestRobot<'a>,
    pub astroport_contracts: AstroportContracts,
}

impl<'a> TestRobot<'a, TestRunner<'a>> for AstroportSwapperRobot<'a> {
    fn runner(&self) -> &'a TestRunner<'a> {
        self.runner
    }
}

impl<'a> AstroportTestRobot<'a, TestRunner<'a>> for AstroportSwapperRobot<'a> {
    fn astroport_contracts(&self) -> &AstroportContracts {
        &self.astroport_contracts
    }
}

impl<'a> AstroportSwapperRobot<'a> {
    /// Creates a new test robot with the given runner, contracts, and admin account.
    ///
    /// The contracts map must contain contracts for the following keys:
    /// - All contracts in the AstroportContracts struct
    /// - Mars swapper with key being the CARGO_PKG_NAME environment variable
    ///
    /// The contracts in the ContractMap must be compatible with the given TestRunner,
    /// else this function will panic.
    pub fn new(
        runner: &'a TestRunner,
        astroport_contracts: ContractMap,
        swapper_contract: ContractType,
        oracle_contract: ContractType,
        admin: &SigningAccount,
    ) -> Self {
        let mut contracts = astroport_contracts;
        contracts.insert("mars-oracle-wasm".to_string(), oracle_contract);
        let oracle_robot = WasmOracleTestRobot::new(runner, contracts, admin, Some("usd"));

        let swapper_code_id =
            cw_it::helpers::upload_wasm_file(runner, admin, swapper_contract).unwrap();

        let wasm = Wasm::new(runner);
        let swapper = wasm
            .instantiate(
                swapper_code_id,
                &mars_swapper::InstantiateMsg {
                    owner: admin.address(),
                },
                None,
                Some("swapper"),
                &[],
                admin,
            )
            .unwrap()
            .data
            .address;

        let astroport_contracts = oracle_robot.astroport_contracts.clone();

        Self {
            runner,
            oracle_robot,
            swapper,
            astroport_contracts,
        }
    }

    pub fn new_with_local(runner: &'a TestRunner, admin: &SigningAccount) -> Self {
        let astroport_contracts = cw_it::astroport::utils::get_local_contracts(
            runner,
            &Some(ARTIFACTS_PATH),
            APPEND_ARCH,
            &Some(std::env::consts::ARCH),
        );
        let swapper_contract = get_local_swapper_contract(runner);
        let oracle_contract = get_local_oracle_contract(runner);
        Self::new(runner, astroport_contracts, swapper_contract, oracle_contract, admin)
    }

    pub fn set_route(
        &self,
        operations: Vec<SwapOperation>,
        denom_in: impl Into<String>,
        denom_out: impl Into<String>,
        signer: &SigningAccount,
    ) -> &Self {
        self.wasm()
            .execute(
                &self.swapper,
                &mars_swapper::ExecuteMsg::SetRoute {
                    route: AstroportRoute {
                        operations,
                        router: self.astroport_contracts.router.address.clone(),
                        factory: self.astroport_contracts.factory.address.clone(),
                        oracle: self.oracle_robot.mars_oracle_contract_addr.clone(),
                    },
                    denom_in: denom_in.into(),
                    denom_out: denom_out.into(),
                },
                &[],
                signer,
            )
            .unwrap();
        self
    }

    pub fn swap(
        &self,
        coin_in: Coin,
        denom_out: impl Into<String>,
        slippage: Decimal,
        signer: &SigningAccount,
    ) -> &Self {
        self.swap_res(coin_in, denom_out, slippage, signer).unwrap();
        self
    }

    pub fn swap_res(
        &self,
        coin_in: Coin,
        denom_out: impl Into<String>,
        slippage: Decimal,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        self.wasm().execute(
            &self.swapper,
            &mars_swapper::ExecuteMsg::<AstroportRoute>::SwapExactIn {
                coin_in: coin_in.clone(),
                denom_out: denom_out.into(),
                slippage,
            },
            &[coin_in],
            signer,
        )
    }

    pub fn query_estimate_exact_in_swap(
        &self,
        coin_in: &Coin,
        denom_out: impl Into<String>,
    ) -> Uint128 {
        self.wasm()
            .query::<_, EstimateExactInSwapResponse>(
                &self.swapper,
                &mars_swapper::QueryMsg::EstimateExactInSwap {
                    coin_in: coin_in.clone(),
                    denom_out: denom_out.into(),
                },
            )
            .unwrap()
            .amount
    }
}
