import {
  assert,
  describe,
  test,
  clearStore,
  beforeAll,
  afterAll
} from "matchstick-as/assembly/index"
import { Bytes, Address } from "@graphprotocol/graph-ts"
import { RequestMade } from "../generated/schema"
import { RequestMade as RequestMadeEvent } from "../generated/PWNSimpleLoanSimpleRequest/PWNSimpleLoanSimpleRequest"
import { handleRequestMade } from "../src/pwn-simple-loan-simple-request"
import { createRequestMadeEvent } from "./pwn-simple-loan-simple-request-utils"

// Tests structure (matchstick-as >=0.5.0)
// https://thegraph.com/docs/en/developer/matchstick/#tests-structure-0-5-0

describe("Describe entity assertions", () => {
  beforeAll(() => {
    let requestHash = Bytes.fromI32(1234567890)
    let borrower = Address.fromString(
      "0x0000000000000000000000000000000000000001"
    )
    let newRequestMadeEvent = createRequestMadeEvent(requestHash, borrower)
    handleRequestMade(newRequestMadeEvent)
  })

  afterAll(() => {
    clearStore()
  })

  // For more test scenarios, see:
  // https://thegraph.com/docs/en/developer/matchstick/#write-a-unit-test

  test("RequestMade created and stored", () => {
    assert.entityCount("RequestMade", 1)

    // 0xa16081f360e3847006db660bae1c6d1b2e17ec2a is the default address used in newMockEvent() function
    assert.fieldEquals(
      "RequestMade",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "requestHash",
      "1234567890"
    )
    assert.fieldEquals(
      "RequestMade",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "borrower",
      "0x0000000000000000000000000000000000000001"
    )

    // More assert options:
    // https://thegraph.com/docs/en/developer/matchstick/#asserts
  })
})
