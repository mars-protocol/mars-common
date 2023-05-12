// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from '@tanstack/react-query'
import { ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee, Coin } from '@cosmjs/amino'
import {
  InstantiateMsg,
  ExecuteMsg,
  OsmosisPriceSource,
  Decimal,
  Downtime,
  OwnerUpdate,
  DowntimeDetector,
  QueryMsg,
  ConfigResponse,
  PriceResponse,
  PriceSourceResponseForString,
  ArrayOfPriceSourceResponseForString,
  ArrayOfPriceResponse,
} from './MarsOracleOsmosis.types'
import { MarsOracleOsmosisQueryClient, MarsOracleOsmosisClient } from './MarsOracleOsmosis.client'
export const marsOracleOsmosisQueryKeys = {
  contract: [
    {
      contract: 'marsOracleOsmosis',
    },
  ] as const,
  address: (contractAddress: string | undefined) =>
    [{ ...marsOracleOsmosisQueryKeys.contract[0], address: contractAddress }] as const,
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsOracleOsmosisQueryKeys.address(contractAddress)[0], method: 'config', args },
    ] as const,
  priceSource: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsOracleOsmosisQueryKeys.address(contractAddress)[0], method: 'price_source', args },
    ] as const,
  priceSources: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsOracleOsmosisQueryKeys.address(contractAddress)[0], method: 'price_sources', args },
    ] as const,
  price: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...marsOracleOsmosisQueryKeys.address(contractAddress)[0], method: 'price', args }] as const,
  prices: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsOracleOsmosisQueryKeys.address(contractAddress)[0], method: 'prices', args },
    ] as const,
}
export interface MarsOracleOsmosisReactQuery<TResponse, TData = TResponse> {
  client: MarsOracleOsmosisQueryClient | undefined
  options?: Omit<
    UseQueryOptions<TResponse, Error, TData>,
    "'queryKey' | 'queryFn' | 'initialData'"
  > & {
    initialData?: undefined
  }
}
export interface MarsOracleOsmosisPricesQuery<TData>
  extends MarsOracleOsmosisReactQuery<ArrayOfPriceResponse, TData> {
  args: {
    limit?: number
    startAfter?: string
  }
}
export function useMarsOracleOsmosisPricesQuery<TData = ArrayOfPriceResponse>({
  client,
  args,
  options,
}: MarsOracleOsmosisPricesQuery<TData>) {
  return useQuery<ArrayOfPriceResponse, Error, TData>(
    marsOracleOsmosisQueryKeys.prices(client?.contractAddress, args),
    () =>
      client
        ? client.prices({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsOracleOsmosisPriceQuery<TData>
  extends MarsOracleOsmosisReactQuery<PriceResponse, TData> {
  args: {
    denom: string
  }
}
export function useMarsOracleOsmosisPriceQuery<TData = PriceResponse>({
  client,
  args,
  options,
}: MarsOracleOsmosisPriceQuery<TData>) {
  return useQuery<PriceResponse, Error, TData>(
    marsOracleOsmosisQueryKeys.price(client?.contractAddress, args),
    () =>
      client
        ? client.price({
            denom: args.denom,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsOracleOsmosisPriceSourcesQuery<TData>
  extends MarsOracleOsmosisReactQuery<ArrayOfPriceSourceResponseForString, TData> {
  args: {
    limit?: number
    startAfter?: string
  }
}
export function useMarsOracleOsmosisPriceSourcesQuery<TData = ArrayOfPriceSourceResponseForString>({
  client,
  args,
  options,
}: MarsOracleOsmosisPriceSourcesQuery<TData>) {
  return useQuery<ArrayOfPriceSourceResponseForString, Error, TData>(
    marsOracleOsmosisQueryKeys.priceSources(client?.contractAddress, args),
    () =>
      client
        ? client.priceSources({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsOracleOsmosisPriceSourceQuery<TData>
  extends MarsOracleOsmosisReactQuery<PriceSourceResponseForString, TData> {
  args: {
    denom: string
  }
}
export function useMarsOracleOsmosisPriceSourceQuery<TData = PriceSourceResponseForString>({
  client,
  args,
  options,
}: MarsOracleOsmosisPriceSourceQuery<TData>) {
  return useQuery<PriceSourceResponseForString, Error, TData>(
    marsOracleOsmosisQueryKeys.priceSource(client?.contractAddress, args),
    () =>
      client
        ? client.priceSource({
            denom: args.denom,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsOracleOsmosisConfigQuery<TData>
  extends MarsOracleOsmosisReactQuery<ConfigResponse, TData> {}
export function useMarsOracleOsmosisConfigQuery<TData = ConfigResponse>({
  client,
  options,
}: MarsOracleOsmosisConfigQuery<TData>) {
  return useQuery<ConfigResponse, Error, TData>(
    marsOracleOsmosisQueryKeys.config(client?.contractAddress),
    () => (client ? client.config() : Promise.reject(new Error('Invalid client'))),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsOracleOsmosisUpdateOwnerMutation {
  client: MarsOracleOsmosisClient
  msg: OwnerUpdate
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsOracleOsmosisUpdateOwnerMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsOracleOsmosisUpdateOwnerMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsOracleOsmosisUpdateOwnerMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) => client.updateOwner(msg, fee, memo, funds),
    options,
  )
}
export interface MarsOracleOsmosisRemovePriceSourceMutation {
  client: MarsOracleOsmosisClient
  msg: {
    denom: string
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsOracleOsmosisRemovePriceSourceMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsOracleOsmosisRemovePriceSourceMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsOracleOsmosisRemovePriceSourceMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.removePriceSource(msg, fee, memo, funds),
    options,
  )
}
export interface MarsOracleOsmosisSetPriceSourceMutation {
  client: MarsOracleOsmosisClient
  msg: {
    denom: string
    priceSource: OsmosisPriceSource
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsOracleOsmosisSetPriceSourceMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsOracleOsmosisSetPriceSourceMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsOracleOsmosisSetPriceSourceMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.setPriceSource(msg, fee, memo, funds),
    options,
  )
}
