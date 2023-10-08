import { newMockEvent } from "matchstick-as"
import { ethereum, Address, BigInt, Bytes } from "@graphprotocol/graph-ts"
import {
  OwnershipTransferred,
  ParameterDecresed,
  ParameterIncresed,
  RequestAssigned,
  RequestCancelled,
  RequestCreated,
  RequestDisputed,
  RequestFinalized
} from "../generated/Contract/Contract"

export function createOwnershipTransferredEvent(
  previousOwner: Address,
  newOwner: Address
): OwnershipTransferred {
  let ownershipTransferredEvent = changetype<OwnershipTransferred>(
    newMockEvent()
  )

  ownershipTransferredEvent.parameters = new Array()

  ownershipTransferredEvent.parameters.push(
    new ethereum.EventParam(
      "previousOwner",
      ethereum.Value.fromAddress(previousOwner)
    )
  )
  ownershipTransferredEvent.parameters.push(
    new ethereum.EventParam("newOwner", ethereum.Value.fromAddress(newOwner))
  )

  return ownershipTransferredEvent
}

export function createParameterDecresedEvent(id: BigInt): ParameterDecresed {
  let parameterDecresedEvent = changetype<ParameterDecresed>(newMockEvent())

  parameterDecresedEvent.parameters = new Array()

  parameterDecresedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )

  return parameterDecresedEvent
}

export function createParameterIncresedEvent(id: BigInt): ParameterIncresed {
  let parameterIncresedEvent = changetype<ParameterIncresed>(newMockEvent())

  parameterIncresedEvent.parameters = new Array()

  parameterIncresedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )

  return parameterIncresedEvent
}

export function createRequestAssignedEvent(id: BigInt): RequestAssigned {
  let requestAssignedEvent = changetype<RequestAssigned>(newMockEvent())

  requestAssignedEvent.parameters = new Array()

  requestAssignedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )

  return requestAssignedEvent
}

export function createRequestCancelledEvent(id: BigInt): RequestCancelled {
  let requestCancelledEvent = changetype<RequestCancelled>(newMockEvent())

  requestCancelledEvent.parameters = new Array()

  requestCancelledEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )

  return requestCancelledEvent
}

export function createRequestCreatedEvent(
  id: BigInt,
  instructions: Bytes
): RequestCreated {
  let requestCreatedEvent = changetype<RequestCreated>(newMockEvent())

  requestCreatedEvent.parameters = new Array()

  requestCreatedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )
  requestCreatedEvent.parameters.push(
    new ethereum.EventParam(
      "instructions",
      ethereum.Value.fromBytes(instructions)
    )
  )

  return requestCreatedEvent
}

export function createRequestDisputedEvent(id: BigInt): RequestDisputed {
  let requestDisputedEvent = changetype<RequestDisputed>(newMockEvent())

  requestDisputedEvent.parameters = new Array()

  requestDisputedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )

  return requestDisputedEvent
}

export function createRequestFinalizedEvent(id: BigInt): RequestFinalized {
  let requestFinalizedEvent = changetype<RequestFinalized>(newMockEvent())

  requestFinalizedEvent.parameters = new Array()

  requestFinalizedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )

  return requestFinalizedEvent
}
