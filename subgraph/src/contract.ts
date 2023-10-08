import { BigInt } from "@graphprotocol/graph-ts"
import {
  OwnershipTransferred as OwnershipTransferredEvent,
  ParameterDecresed as ParameterDecresedEvent,
  ParameterIncresed as ParameterIncresedEvent,
  RequestAssigned as RequestAssignedEvent,
  RequestCancelled as RequestCancelledEvent,
  RequestCreated as RequestCreatedEvent,
  RequestDisputed as RequestDisputedEvent,
  RequestFinalized as RequestFinalizedEvent
} from "../generated/Contract/Contract"
import {
  OwnershipTransferred,
  ParameterDecresed,
  ParameterIncresed,
  Request,
  RequestAssigned,
  RequestCancelled,
  RequestCreated,
  RequestDisputed,
  RequestFinalized
} from "../generated/schema"
import { log } from "matchstick-as"

export function handleOwnershipTransferred(
  event: OwnershipTransferredEvent
): void {
  let entity = new OwnershipTransferred(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.previousOwner = event.params.previousOwner
  entity.newOwner = event.params.newOwner

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleParameterDecresed(event: ParameterDecresedEvent): void {
  let entity = new ParameterDecresed(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.Contract_id = event.params.id

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()


}

export function handleParameterIncresed(event: ParameterIncresedEvent): void {
  let entity = new ParameterIncresed(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.Contract_id = event.params.id

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash


  let request = Request.load(event.params.id.toString())
  if (request == null) {
    log.warning("Request {} not found", [event.params.id.toString()])
    return
  }
  request.status = "INCREASED"
  request.save()
  entity.save()
}

export function handleRequestAssigned(event: RequestAssignedEvent): void {
  let entity = new RequestAssigned(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.Contract_id = event.params.id

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  let request = Request.load(event.params.id.toString())
  if(request == null){
    log.warning("Request {} not found", [event.params.id.toString()])
    return
  }
  request.status = "ASSIGNED"
  request.save()

  entity.save()
}

export function handleRequestCancelled(event: RequestCancelledEvent): void {
  let entity = new RequestCancelled(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.Contract_id = event.params.id

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash


  let request = Request.load(event.params.id.toString())
  if (request == null) {
    log.warning("Request {} not found", [event.params.id.toString()])
    return
  }
  request.status = "CANCELLED"
  request.save()


  entity.save()
}

export function handleRequestCreated(event: RequestCreatedEvent): void {
  let entity = new RequestCreated(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.Contract_id = event.params.id
  entity.instructions = event.params.instructions

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()

  let request = new Request(event.params.id.toString())
  request.instructions = event.params.instructions.toString()
  request.status = "CREATED"
  request.save()

}

export function handleRequestDisputed(event: RequestDisputedEvent): void {
  let entity = new RequestDisputed(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.Contract_id = event.params.id

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleRequestFinalized(event: RequestFinalizedEvent): void {
  let entity = new RequestFinalized(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.Contract_id = event.params.id

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}
