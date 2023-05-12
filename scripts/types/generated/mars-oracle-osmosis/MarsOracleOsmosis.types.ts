// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export interface InstantiateMsg {
  base_denom: string
  owner: string
}
export type ExecuteMsg =
  | {
      set_price_source: {
        denom: string
        price_source: OsmosisPriceSource
      }
    }
  | {
      remove_price_source: {
        denom: string
      }
    }
  | {
      update_owner: OwnerUpdate
    }
export type OsmosisPriceSource =
  | {
      fixed: {
        price: Decimal
        [k: string]: unknown
      }
    }
  | {
      spot: {
        pool_id: number
        [k: string]: unknown
      }
    }
  | {
      arithmetic_twap: {
        downtime_detector?: DowntimeDetector | null
        pool_id: number
        window_size: number
        [k: string]: unknown
      }
    }
  | {
      geometric_twap: {
        downtime_detector?: DowntimeDetector | null
        pool_id: number
        window_size: number
        [k: string]: unknown
      }
    }
  | {
      xyk_liquidity_token: {
        pool_id: number
        [k: string]: unknown
      }
    }
  | {
      staked_geometric_twap: {
        downtime_detector?: DowntimeDetector | null
        pool_id: number
        transitive_denom: string
        window_size: number
        [k: string]: unknown
      }
    }
export type Decimal = string
export type Downtime =
  | 'duration30s'
  | 'duration1m'
  | 'duration2m'
  | 'duration3m'
  | 'duration4m'
  | 'duration5m'
  | 'duration10m'
  | 'duration20m'
  | 'duration30m'
  | 'duration40m'
  | 'duration50m'
  | 'duration1h'
  | 'duration15h'
  | 'duration2h'
  | 'duration25h'
  | 'duration3h'
  | 'duration4h'
  | 'duration5h'
  | 'duration6h'
  | 'duration9h'
  | 'duration12h'
  | 'duration18h'
  | 'duration24h'
  | 'duration36h'
  | 'duration48h'
export type OwnerUpdate =
  | {
      propose_new_owner: {
        proposed: string
      }
    }
  | 'clear_proposed'
  | 'accept_proposed'
  | 'abolish_owner_role'
  | {
      set_emergency_owner: {
        emergency_owner: string
      }
    }
  | 'clear_emergency_owner'
export interface DowntimeDetector {
  downtime: Downtime
  recovery: number
  [k: string]: unknown
}
export type QueryMsg =
  | {
      config: {}
    }
  | {
      price_source: {
        denom: string
      }
    }
  | {
      price_sources: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      price: {
        denom: string
      }
    }
  | {
      prices: {
        limit?: number | null
        start_after?: string | null
      }
    }
export interface ConfigResponse {
  base_denom: string
  owner?: string | null
  proposed_new_owner?: string | null
}
export interface PriceResponse {
  denom: string
  price: Decimal
}
export interface PriceSourceResponseForString {
  denom: string
  price_source: string
}
export type ArrayOfPriceSourceResponseForString = PriceSourceResponseForString[]
export type ArrayOfPriceResponse = PriceResponse[]
