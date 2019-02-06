var Migrations = artifacts.require("./Migrations.sol");
var TestContract = artifacts.require("./TestContract.sol");

module.exports = function(deployer) {
  deployer.deploy(TestContract);
	deployer.deploy(Migrations);
};
