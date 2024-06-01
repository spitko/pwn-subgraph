import { RequestMade as RequestMadeEvent } from "../generated/PWNSimpleLoanSimpleRequest/PWNSimpleLoanSimpleRequest"
import { RequestMade } from "../generated/schema"

export function handleRequestMade(event: RequestMadeEvent): void {
  let entity = new RequestMade(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.requestHash = event.params.requestHash
  entity.borrower = event.params.borrower

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}
