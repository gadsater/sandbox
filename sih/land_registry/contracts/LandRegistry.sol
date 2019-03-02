pragma solidity ^0.5.0;

contract LandRegistry {
    address owner;

    enum privilege { customer, registrar, admin }
    enum landState { owned, onSale }

    /*struct Coordinate {
        uint x;
        uint y;
        uint z;
    }

    struct Dimension {
        uint length;
        uint width;
    }*/

    struct landDetails {
        uint landId;
        string landOwnerName;
        address payable landOwnerAddress;
        address[] landPreviousOwners;
        landState landStatus;
    }

    struct userDetails {
        uint userId;
        string userName;
        address userAddress;
        privilege userPrivilege;
    }

    mapping(uint => landDetails) public Lands;
    uint public landcount;
    mapping(address => userDetails) public Users;
    uint public usercount;

    constructor() public {
      owner = msg.sender;
      userDetails memory user = userDetails({
        userId: usercount,
        userName: "admin",
        userAddress: msg.sender,
        userPrivilege: privilege.admin
      });
      Users[msg.sender] = user;
    }

    modifier isRegistrar {
        require(Users[msg.sender].userPrivilege == privilege.registrar, "User is not a registrar");
        _;
    }

    modifier isAdmin {
        require(Users[msg.sender].userPrivilege == privilege.admin, "User is not admin");
        _;
    }

    function registerLand(string memory _landOwnerName, address payable _landOwnerAddress, landState _landStatus)
    public isAdmin  {
      landcount++;
      landDetails memory land = landDetails({
        landId: landcount,
        landOwnerName: _landOwnerName,
        landOwnerAddress: _landOwnerAddress,
        landPreviousOwners: new address[](0),
        landStatus: _landStatus
      });
      Lands[landcount] = land;
    }

    function registerUser(string memory _userName, address _userAddress, privilege _userPrivilege)
    public {
      usercount++;
      userDetails memory user = userDetails({
        userId: usercount,
        userName: _userName,
        userAddress: _userAddress,
        userPrivilege: _userPrivilege
      });
      Users[_userAddress] = user;
    }

    function transactLand(uint _landId, uint amount) public payable
    returns (bool success) {
      success = false;
      if (msg.sender.balance > amount) {
        landDetails storage land = Lands[_landId];
        //land.landOwnerAddress.transfer(amount);
        land.landPreviousOwners.push(land.landOwnerAddress);
        land.landOwnerAddress = msg.sender;
        landDetails memory landmem = land;
        Lands[_landId] = landmem;
        success = true;
      }
      return success;
    }



}
