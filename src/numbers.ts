import { BigDecimal, BigInt } from "@graphprotocol/graph-ts";

export const BIGINT_ZERO = BigInt.fromI32(0);
export const BIGINT_ONE = BigInt.fromI32(1);
export const BIGINT_TEN = BigInt.fromI32(10);

export function bigIntToBigDecimal(
    quantity: BigInt,
    decimals: i32 = 18
  ): BigDecimal {
    return quantity.divDecimal(BIGINT_TEN.pow(decimals as u8).toBigDecimal());
  }
  
  