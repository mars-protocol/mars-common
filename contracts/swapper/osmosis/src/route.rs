use std::{fmt, str::FromStr};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    BlockInfo, Coin, CosmosMsg, Decimal, Env, Fraction, QuerierWrapper, StdResult, Uint128,
};
use mars_osmosis::helpers::{query_arithmetic_twap_price, query_pool};
use mars_swapper::msgs::EstimateExactInSwapResponse;
use mars_swapper_base::{ContractError, ContractResult, Route, RouteStep};
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute};

/// 10 min in seconds (Risk Team recommendation)
const TWAP_WINDOW_SIZE_SECONDS: u64 = 600u64;

#[cw_serde]
pub struct OsmosisRoute(pub Vec<OsmosisRouteStep>);

impl From<Vec<SwapAmountInRoute>> for OsmosisRoute {
    fn from(routes: Vec<SwapAmountInRoute>) -> Self {
        Self(routes.into_iter().map(OsmosisRouteStep).collect())
    }
}

impl fmt::Display for OsmosisRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|step| format!("{}:{}", step.0.pool_id, step.0.token_out_denom))
            .collect::<Vec<_>>()
            .join("|");
        write!(f, "{s}")
    }
}

#[cw_serde]
pub struct OsmosisRouteStep(pub SwapAmountInRoute);

impl OsmosisRouteStep {
    /// Get the liquidity of the pool
    fn get_pool_liquidity(&self, querier: &QuerierWrapper) -> ContractResult<Vec<Coin>> {
        let pool = query_pool(querier, self.0.pool_id)?;
        Ok(pool
            .pool_assets
            .iter()
            .flat_map(|asset| asset.token.clone())
            .map(|token| {
                let amount = token.amount;
                let denom = token.denom;
                Ok(Coin {
                    amount: Uint128::from_str(&amount)?,
                    denom,
                })
            })
            .collect::<StdResult<Vec<_>>>()?)
    }
}

impl RouteStep for OsmosisRouteStep {
    fn ask_denom(&self) -> ContractResult<String> {
        Ok(self.0.token_out_denom.clone())
    }

    fn validate(&self, querier: &QuerierWrapper, denom_in: &str) -> ContractResult<()> {
        let pool_liquidity = self.get_pool_liquidity(querier)?;
        let pool_denoms = pool_liquidity.into_iter().map(|c| c.denom).collect::<Vec<_>>();

        if !pool_denoms.contains(&denom_in.to_string()) {
            return Err(ContractError::InvalidRoute {
                reason: format!(
                    "pool {} does not contain input denom {}",
                    self.0.pool_id, denom_in,
                ),
            });
        }

        if !pool_denoms.contains(&self.ask_denom()?) {
            return Err(ContractError::InvalidRoute {
                reason: format!(
                    "pool {} does not contain output denom {}",
                    self.0.pool_id,
                    self.ask_denom()?,
                ),
            });
        }

        Ok(())
    }
}

impl Route<OsmosisRouteStep> for OsmosisRoute {
    /// Build a CosmosMsg that swaps given an input denom and amount
    fn build_exact_in_swap_msg(
        &self,
        querier: &QuerierWrapper,
        env: &Env,
        coin_in: &Coin,
        slippage: Decimal,
    ) -> ContractResult<CosmosMsg> {
        let steps = self.swap_routes();

        steps.first().ok_or(ContractError::InvalidRoute {
            reason: "the route must contain at least one step".to_string(),
        })?;

        let out_amount = query_out_amount(querier, &env.block, coin_in, &steps)?;
        let min_out_amount = (Decimal::one() - slippage) * out_amount;

        let swap_msg: CosmosMsg = MsgSwapExactAmountIn {
            sender: env.contract.address.to_string(),
            routes: steps.to_vec(),
            token_in: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                denom: coin_in.denom.clone(),
                amount: coin_in.amount.to_string(),
            }),
            token_out_min_amount: min_out_amount.to_string(),
        }
        .into();
        Ok(swap_msg)
    }

    fn estimate_exact_in_swap(
        &self,
        querier: &QuerierWrapper,
        env: &Env,
        coin_in: &Coin,
    ) -> ContractResult<EstimateExactInSwapResponse> {
        let out_amount = query_out_amount(querier, &env.block, coin_in, &self.swap_routes())?;
        Ok(EstimateExactInSwapResponse {
            amount: out_amount,
        })
    }

    fn steps(&self) -> &[OsmosisRouteStep] {
        &self.0
    }
}

impl OsmosisRoute {
    pub fn swap_routes(&self) -> Vec<SwapAmountInRoute> {
        self.0.iter().map(|step| step.0.clone()).collect::<Vec<_>>()
    }
}

/// Query how much amount of denom_out we get for denom_in.
///
/// Example calculation:
/// If we want to swap atom to usdc and configured routes are [pool_1 (atom/osmo), pool_69 (osmo/usdc)] (no direct pool of atom/usdc):
/// 1) query pool_1 to get price for atom/osmo
/// 2) query pool_69 to get price for osmo/usdc
/// 3) atom/usdc = (price for atom/osmo) * (price for osmo/usdc)
/// 4) usdc_out_amount = (atom amount) * (price for atom/usdc)
fn query_out_amount(
    querier: &QuerierWrapper,
    block: &BlockInfo,
    coin_in: &Coin,
    steps: &[SwapAmountInRoute],
) -> ContractResult<Uint128> {
    let start_time = block.time.seconds() - TWAP_WINDOW_SIZE_SECONDS;

    let mut price = Decimal::one();
    let mut denom_in = coin_in.denom.clone();
    for step in steps {
        let step_price = query_arithmetic_twap_price(
            querier,
            step.pool_id,
            &denom_in,
            &step.token_out_denom,
            start_time,
        )?;
        price = price.checked_mul(step_price)?;
        denom_in = step.token_out_denom.clone();
    }

    let out_amount =
        coin_in.amount.checked_multiply_ratio(price.numerator(), price.denominator())?;
    Ok(out_amount)
}
