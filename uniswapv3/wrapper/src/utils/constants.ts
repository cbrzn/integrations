import { BigInt } from "@web3api/wasm-as";

export const UNISWAP_ROUTER_CONTRACT = "";

export const FACTORY_ADDRESS = '0x1F98431c8aD98523631AE4a59f267346ea31F984'

export const ADDRESS_ZERO = '0x0000000000000000000000000000000000000000'

export const POOL_INIT_CODE_HASH = '0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54'
export const POOL_INIT_CODE_HASH_OPTIMISM = '0x0c231002d0970d2126e7e00ce88c3b0e5ec8e48dac71478d56245c34ea2f9447'

// historical artifact due to small compiler mismatch
export const POOL_INIT_CODE_HASH_OPTIMISM_KOVAN = '0x1fc830513acbdb1608b8c18fd3cf4a4bee3329c69bb41d56400401c40fe02fd0'

// export const TICK_SPACINGS: { [amount in FeeAmount]: number } = {
//   [FeeAmount.LOWEST]: 1,
//   [FeeAmount.LOW]: 10,
//   [FeeAmount.MEDIUM]: 60,
//   [FeeAmount.HIGH]: 200
// }

// constants used internally but not expected to be used externally
export const NEGATIVE_ONE = BigInt.from(JSBI.BigInt(-1);
export const ZERO = JSBI.BigInt(0)
export const ONE = JSBI.BigInt(1)

// used in liquidity amount math
export const Q96 = JSBI.exponentiate(JSBI.BigInt(2), JSBI.BigInt(96))
export const Q192 = JSBI.exponentiate(Q96, JSBI.BigInt(2))

 // The minimum tick that can be used on any pool.
public static MIN_TICK: number = -887272
 // The maximum tick that can be used on any pool.
public static MAX_TICK: number = -TickMath.MIN_TICK
 // The sqrt ratio corresponding to the minimum tick that could be used on any pool.
public static MIN_SQRT_RATIO: JSBI = JSBI.BigInt('4295128739')
 // The sqrt ratio corresponding to the maximum tick that could be used on any pool.
public static MAX_SQRT_RATIO: JSBI = JSBI.BigInt('1461446703485210103287273052203988822378723970342')