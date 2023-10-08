import { expect } from "chai";
import { ethers } from "hardhat";
import { RailGunEscrow } from "../typechain-types";

describe("RailGunEscrow", function () {
  // We define a fixture to reuse the same setup in every test.

  let RailGunEscrow: RailGunEscrow;
  before(async () => {
    const [owner] = await ethers.getSigners();
    const RailGunEscrowFactory = await ethers.getContractFactory("RailGunEscrow");
    RailGunEscrow = (await RailGunEscrowFactory.deploy(owner.address)) as RailGunEscrow;
    await RailGunEscrow.deployed();
  });

  describe("Deployment", function () {
    it("Should have the right message on deploy", async function () {
      expect(await RailGunEscrow.greeting()).to.equal("Building Unstoppable Apps!!!");
    });

    it("Should allow setting a new message", async function () {
      const newGreeting = "Learn Scaffold-ETH 2! :)";

      await RailGunEscrow.setGreeting(newGreeting);
      expect(await RailGunEscrow.greeting()).to.equal(newGreeting);
    });
  });
});
