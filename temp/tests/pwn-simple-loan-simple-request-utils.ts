import { newMockEvent } from "matchstick-as"
import { ethereum, Bytes, Address } from "@graphprotocol/graph-ts"
import { RequestMade } from "../generated/PWNSimpleLoanSimpleRequest/PWNSimpleLoanSimpleRequest"

export function createRequestMadeEvent(
  requestHash: Bytes,
  borrower: Address
): RequestMade {
  let requestMadeEvent = changetype<RequestMade>(newMockEvent())

  requestMadeEvent.parameters = new Array()

  requestMadeEvent.parameters.push(
    new ethereum.EventParam(
      "requestHash",
      ethereum.Value.fromFixedBytes(requestHash)
    )
  )
  requestMadeEvent.parameters.push(
    new ethereum.EventParam("borrower", ethereum.Value.fromAddress(borrower))
  )

  return requestMadeEvent
}
