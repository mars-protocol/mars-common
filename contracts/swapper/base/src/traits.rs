use std::fmt::{Debug, Display};

use cosmwasm_std::{Coin, CosmosMsg, Decimal, Env, QuerierWrapper};
use mars_swapper::msgs::EstimateExactInSwapResponse;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};

use crate::{helpers::hashset, ContractError, ContractResult};

pub trait RouteStep {
    /// Get the output denom of the route
    fn denom_out(&self) -> ContractResult<String>;

    /// Validate if the route step is valid
    fn validate(&self, querier: &QuerierWrapper, denom_in: &str) -> ContractResult<()>;
}

pub trait Route<RS: RouteStep>:
    Serialize + DeserializeOwned + Clone + Debug + Display + PartialEq + JsonSchema
{
    /// Build a message for executing the trade, given an input denom and amount
    fn build_exact_in_swap_msg(
        &self,
        querier: &QuerierWrapper,
        env: &Env,
        coin_in: &Coin,
        slippage: Decimal,
    ) -> ContractResult<CosmosMsg>;

    /// Query to get the estimate result of a swap
    fn estimate_exact_in_swap(
        &self,
        querier: &QuerierWrapper,
        env: &Env,
        coin_in: &Coin,
    ) -> ContractResult<EstimateExactInSwapResponse>;

    /// Get the steps of the route
    fn steps(&self) -> &[RS];

    /// Determine whether the route is valid, given a pair of input and output denoms
    fn validate(
        &self,
        querier: &QuerierWrapper,
        denom_in: &str,
        denom_out: &str,
    ) -> ContractResult<()> {
        let steps = self.steps();

        // there must be at least one step
        if steps.is_empty() {
            return Err(ContractError::InvalidRoute {
                reason: "the route must contain at least one step".to_string(),
            });
        }

        // for each step:
        // - the pool must contain the input and output denoms
        // - the output denom must not be the same as the input denom of a previous step (i.e. the route must not contain a loop)
        let mut prev_denom_out = denom_in.to_string();
        let mut seen_denoms = hashset(&[denom_in.to_string()]);
        for (_i, step) in steps.iter().enumerate() {
            step.validate(querier, &prev_denom_out)?;
            let ask_denom = step.denom_out()?;

            if seen_denoms.contains(&ask_denom) {
                return Err(ContractError::InvalidRoute {
                    reason: format!("route contains a loop: denom {} seen twice", ask_denom),
                });
            }

            prev_denom_out = ask_denom.clone();
            seen_denoms.insert(ask_denom);
        }

        // the route's final output denom must match the desired output denom
        if prev_denom_out != denom_out {
            return Err(ContractError::InvalidRoute {
                reason: format!(
                    "the route's output denom {prev_denom_out} does not match the desired output {denom_out}",
                ),
            });
        }

        Ok(())
    }
}
