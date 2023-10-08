// SPDX-License-Identifier: MIT
pragma solidity ^0.8.2;
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

struct Reputation {
    // note: he == proposer
    uint256 requestsCount; // how many accepted requests he made
    uint256 disputed; // how many disputes have been opened
    uint256 won; // how many disputes he won
    uint256 lost; // how many disputes he lost
    uint256 totalPrizePaid; // how much money he paid in total in prizes
    uint256 totalValuePaid; // how much money he paid in total
}

struct Request {
    address proposer; // who create the request
    uint256 value; // how much money in escrow
    uint256 requestTiming; // time limit for fulfilling the request since apcepted
    uint256 laudableTiming; // time within which one acceptor is entitled to a prize
    uint256 prize; // prize in escrow
    uint256 startTime;
    uint256 status; // 0 = active, 1 = closed, 2 = disputed
}

contract RailGunEscrow is Ownable {
    // IERC20 constant WETH = IERC20(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2); // mainnet
    IERC20 constant WETH = IERC20(0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6); // Goerli

    mapping(address => Reputation) public trustMeter;
    mapping(uint256 => Request) public requestMap;

    // proposer -> i < rep.pendingProposal -> id
    mapping(address => mapping(uint256 => uint256)) public proposerIdMap;
    // proposer -> pending request
    mapping(address => uint256) public pendingReqMap;
    // request ID -> position in proposer requestsCount
    mapping(uint256 => uint256) public idProposerMap;
    uint256 public lastRequestId = 1; // ID 0 is null ID

    uint256 feeDisputant = 10; // 10 = 1%

    // disputers
    mapping(address => bool) public isDisputer;

    event RequestCreated(uint256 id, bytes instructions);
    event RequestCancelled(uint256 id);

    event ParameterIncresed(uint256 id);
    event ParameterDecresed(uint256 id);

    event RequestAssigned(uint256 id);
    event RequestDisputed(uint256 id);
    event RequestFinalized(uint256 id);


    // TODO: only missing function!!!
    function _sendWethToResolutor(uint256 value) internal {}

    modifier onlyDisputant() {
        require(isDisputer[msg.sender]);
        _;
    }

    function setDisputant(address disputant, bool value) external onlyOwner {
        isDisputer[disputant] = value;
    }

    function setDisputantFee(uint256 value) external onlyOwner {
        feeDisputant = value;
    }

    function increaseRequestParameter(
        uint256 id,
        uint256 extraValue,
        uint256 extraRequestTiming,
        uint256 extraLaudableTiming,
        uint256 extraPrize
    ) public {
        Request memory req = requestMap[id];
        require(req.status == 0);
        require(req.proposer == msg.sender, "only proposer");
        uint256 totValue = extraValue + extraPrize;
        if (totValue > 0) {
            // WETH.transferFrom(msg.sender, address(this), totValue);
            req.value += extraValue;
            req.prize += extraPrize;
        }
        req.requestTiming += extraRequestTiming;
        req.laudableTiming += extraLaudableTiming;
    }

    function decreaseRequestParameter(
        uint256 id,
        uint256 deltaValue,
        uint256 deltaRequestTiming,
        uint256 deltaLaudableTiming,
        uint256 deltaPrize
    ) public {
        Request memory req = requestMap[id];
        require(req.proposer == msg.sender, "only proposer");
        require(
            req.startTime == 0,
            "request yet accepted: can't reduce parameters"
        );
        require(req.status == 0);
        uint256 totValue = deltaValue + deltaPrize;
        if (totValue > 0) {
            // WETH.transferFrom(address(this), msg.sender, totValue);
            req.value -= deltaValue;
            req.prize -= deltaPrize;
        }
        req.requestTiming -= deltaRequestTiming;
        req.laudableTiming -= deltaLaudableTiming;
        emit ParameterDecresed(id);
    }

    function createRequest(
        uint256 value,
        uint256 requestTiming,
        uint256 laudableTiming,
        uint256 prize,
        bytes calldata instructions
    ) public {
        // evoid cheating (since prizePaid is a parameter)
        require(value > 0 && requestTiming > 0, "missing values");
        if (laudableTiming > 0) {
            require(laudableTiming > block.timestamp);
            require(prize > (value * 5) / 100); // prize > 5% value
        }
        // WETH.transferFrom(msg.sender, address(this), value + prize);
        requestMap[lastRequestId] = Request({
            proposer: msg.sender,
            value: value,
            requestTiming: requestTiming,
            laudableTiming: laudableTiming,
            prize: prize,
            startTime: 0,
            status: 0
        });
        uint256 count = pendingReqMap[msg.sender];
        idProposerMap[lastRequestId] = count;
        proposerIdMap[msg.sender][count] = lastRequestId;
        pendingReqMap[msg.sender]++;
        emit RequestCreated(lastRequestId++, instructions);
    }

    function assignRequest(uint256 id) public {
        Request memory req = requestMap[id];
        require(req.proposer == msg.sender, "only proposer");
        require(req.startTime == 0, "request yet assigned");
        req.startTime = block.timestamp;
        Reputation memory rep = trustMeter[msg.sender];
        rep.requestsCount++;
        // assign to a shield...
        emit RequestAssigned(id);
    }

    function _decreaseRequestCount(uint256 id, address proposer) internal {
        uint256 count = pendingReqMap[proposer];
        if (count > 1) {
            proposerIdMap[proposer][idProposerMap[id]] = proposerIdMap[
                proposer
            ][count - 1];
        }
        idProposerMap[id] = 0;
        proposerIdMap[proposer][count - 1] = 0;
        pendingReqMap[proposer]--;
    }

    function cancelRequest(uint256 id) public {
        Request memory req = requestMap[id];
        require(req.proposer == msg.sender, "only proposer");
        require(req.startTime == 0, "request is ongoing");
        req.status = 1;
        // WETH.transferFrom(address(this), msg.sender, req.value + req.prize);
        _decreaseRequestCount(id, msg.sender);
        emit RequestCancelled(id);
    }

    function finalizeRequest(uint256 id) public {
        Request memory req = requestMap[id];
        require(req.proposer == msg.sender, "only proposer");
        require(req.startTime > 0, "request not accepted");
        req.status = 1;
        Reputation memory rep = trustMeter[msg.sender];
        uint256 totalValue = req.value;
        if (block.timestamp < req.startTime + req.laudableTiming) {
            totalValue += req.prize;
            rep.totalPrizePaid += req.prize;
        }
        rep.totalValuePaid += totalValue;
        _decreaseRequestCount(id, msg.sender);
        _sendWethToResolutor(totalValue);
        emit RequestFinalized(id);
    }

    function openDispute(uint256 id) public {
        Request memory req = requestMap[id];
        require(req.startTime > 0, "request not accepted");
        require(req.proposer == msg.sender, "only proposer");
        require(
            block.timestamp > req.startTime + req.requestTiming,
            "wait for deadline"
        );
        req.status = 2;
        Reputation memory rep = trustMeter[msg.sender];
        rep.disputed++;
        emit RequestDisputed(id);
    }

    function withdrawPrize(uint256 id) public {
        Request memory req = requestMap[id];
        require(req.proposer == msg.sender, "only proposer");
        require(block.timestamp > req.startTime + req.laudableTiming);
        // WETH.transferFrom(address(this), msg.sender, req.prize);
        req.prize = 0;
        req.laudableTiming = 0;
    }

    // result: 0: proposer wins, else: proposer loses
    function solveDispute(uint256 id, uint256 result) public onlyDisputant {
        Request memory req = requestMap[id];
        require(req.status == 2, "yet disputed");
        uint256 value = req.value;
        uint256 refund = value * (100 - feeDisputant / 1000);
        req.status = 1;
        Reputation memory rep = trustMeter[req.proposer];
        if (result == 0) {
            rep.won++;
            // WETH.transferFrom(address(this), req.proposer, refund + req.prize);
        } else {
            rep.lost++;
            rep.totalValuePaid += value;
            _sendWethToResolutor(refund);
        }
        // WETH.transferFrom(address(this), msg.sender, value - refund);
        _decreaseRequestCount(id, msg.sender);
    }
}
