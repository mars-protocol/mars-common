use std::fmt;

use astroport::{
    asset::AssetInfo,
    router::{
        ExecuteMsg as RouterExecuteMsg, QueryMsg as RouterQueryMsg, SimulateSwapOperationsResponse,
        SwapOperation,
    },
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, Decimal, Env, QuerierWrapper, Uint128, WasmMsg,
};
use mars_swapper::msgs::EstimateExactInSwapResponse;
use mars_swapper_base::{ContractError, ContractResult, Route, RouteStep};

#[cw_serde]
pub struct AstroportRoute {
    router: Addr,
    steps: Vec<AstroportRouteStep>,
}

impl AstroportRoute {
    pub fn new(router: Addr, steps: Vec<AstroportRouteStep>) -> Self {
        Self {
            router,
            steps,
        }
    }
}

impl fmt::Display for AstroportRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .steps
            .iter()
            .map(|step| {
                let denoms = step.denoms().map_err(|_| fmt::Error)?;
                Ok(format!("{}:{}", denoms.0, denoms.1))
            })
            .collect::<Result<Vec<_>, fmt::Error>>()?
            .join("|");
        write!(f, "{s}")
    }
}

impl Route<AstroportRouteStep> for AstroportRoute {
    /// Build a CosmosMsg that swaps given an input denom and amount
    fn build_exact_in_swap_msg(
        &self,
        _querier: &QuerierWrapper,
        _env: &Env,
        coin_in: &Coin,
        _slippage: Decimal,
    ) -> ContractResult<CosmosMsg> {
        let steps = &self.steps;

        steps.first().ok_or(ContractError::InvalidRoute {
            reason: "the route must contain at least one step".to_string(),
        })?;

        let operations = steps.iter().map(|step| step.to_swap_operation()).collect::<Vec<_>>();

        let swap_msg: CosmosMsg = WasmMsg::Execute {
            contract_addr: self.router.to_string(),
            msg: to_binary(&RouterExecuteMsg::ExecuteSwapOperations {
                operations,
                minimum_receive: None, //TODO: need to get oracle price to calculate this
                to: None,
                max_spread: None,
            })?,
            funds: vec![coin_in.clone()],
        }
        .into();

        Ok(swap_msg)
    }

    fn estimate_exact_in_swap(
        &self,
        querier: &QuerierWrapper,
        _env: &Env,
        coin_in: &Coin,
    ) -> ContractResult<EstimateExactInSwapResponse> {
        let operations = self.steps.iter().map(|step| step.to_swap_operation()).collect::<Vec<_>>();
        let out_amount =
            simulate_astroport_swap_operations(querier, &self.router, operations, coin_in.amount)?;
        Ok(EstimateExactInSwapResponse {
            amount: out_amount,
        })
    }

    fn steps(&self) -> &[AstroportRouteStep] {
        &self.steps
    }
}

#[cw_serde]
pub struct AstroportRouteStep(pub SwapOperation);

impl AstroportRouteStep {
    fn to_swap_operation(&self) -> SwapOperation {
        self.0.clone()
    }

    /// Returns the (offer,ask) denoms of the swap operation
    fn denoms(&self) -> ContractResult<(String, String)> {
        let operation = self.to_swap_operation();
        match operation {
            SwapOperation::NativeSwap {
                ..
            } => Err(ContractError::InvalidRoute {
                reason: "Astroport NativeSwap is not supported".to_string(),
            }),
            SwapOperation::AstroSwap {
                offer_asset_info,
                ask_asset_info,
            } => match (offer_asset_info, ask_asset_info) {
                (
                    AssetInfo::NativeToken {
                        denom: offer_denom,
                    },
                    AssetInfo::NativeToken {
                        denom: ask_denom,
                    },
                ) => Ok((offer_denom, ask_denom)),
                _ => Err(ContractError::InvalidRoute {
                    reason: "Cw20 tokens are not supported".to_string(),
                }),
            },
        }
    }
}

impl RouteStep for AstroportRouteStep {
    fn denom_out(&self) -> ContractResult<String> {
        Ok(self.denoms()?.1)
    }

    fn validate(&self, _querier: &QuerierWrapper, denom_in: &str) -> ContractResult<()> {
        // Checks that the swap operation only contains native tokens
        let denoms = self.denoms()?;
        if denoms.0 != denom_in {
            return Err(ContractError::InvalidRoute {
                reason: format!("Swap operation does not contain input denom {}", denom_in,),
            });
        }

        Ok(())
    }
}

fn simulate_astroport_swap_operations(
    querier: &QuerierWrapper,
    pair_addr: &Addr,
    operations: Vec<SwapOperation>,
    offer_amount: Uint128,
) -> ContractResult<Uint128> {
    let msg = RouterQueryMsg::SimulateSwapOperations {
        operations,
        offer_amount,
    };
    let res = querier.query_wasm_smart::<SimulateSwapOperationsResponse>(pair_addr, &msg)?;
    Ok(res.amount)
}
