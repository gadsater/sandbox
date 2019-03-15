pragma solidity ^0.5.0;

contract Land {
  address owner;

  enum privilege { admin, registrar, customer } // 0 -> admin, 1 -> registrar, 2-> customer
  enum landState { owned, onSale } // 0 -> owned, 1 -> onSale

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
    uint landTag;
    string landOwnerName;
    address payable landOwnerAddress;
    address[] landPreviousOwners;
    address landOwnerValidator;
    landState landStatus;
    address specifyLandTo;
		uint payableAmount;
  }
  
  struct userDetails {
    uint userId;
    string userName;
    address userAddress;
    uint userBalance;
    privilege userPrivilege;
  }
  
  mapping(uint => landDetails) public Lands;
  uint public landcount;
  mapping(address => userDetails) public Users;
  uint public usercount;
 
	event transferMoney(address ownerAddress, uint amount);

  constructor() public {
    owner = msg.sender;
    userDetails memory user = userDetails({
      userId: usercount,
          userName: "admin",
          userAddress: msg.sender,
          userPrivilege: privilege.admin,
          userBalance: 100 
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
  
  modifier isPrivileged {
    require (Users[msg.sender].userPrivilege == privilege.admin || Users[msg.sender].userPrivilege == privilege.registrar, "User cannot register Land");
    _;
  }
  
  function registerLand(uint _landTag, address payable _landOwnerAddress) public isPrivileged returns (bool) {
    userDetails memory user = Users[_landOwnerAddress];
    if (user.userAddress != address(0) && Lands[_landTag].landTag == 0) {
      landcount++;
      landDetails memory land = landDetails({
        landId: landcount,
            landTag: _landTag,
            landOwnerName: user.userName,
            landOwnerAddress: _landOwnerAddress,
            landPreviousOwners: new address[](0),
            landOwnerValidator: msg.sender,
            specifyLandTo: address(0),
						payableAmount: 0,
            landStatus: landState.owned
            });
      Lands[_landTag] = land;
      return true;
    }
    return false;
  }

  function chUserPrivilege(address _userAddress, privilege _Privilege) public isPrivileged returns (bool) {
    Users[_userAddress].userPrivilege = _Privilege;
    return true;
  }

  function chLandStatus(uint _landTag, landState _landState) public returns (bool) {
    if (Lands[_landTag].landOwnerAddress == msg.sender) {
      Lands[_landTag].landStatus = _landState;
      return true;
    }
    return false;
  }

  function chLandPayableAmount(uint _landTag, uint _payableAmount) public returns (bool) {
    if (Lands[_landTag].landOwnerAddress == msg.sender) {
      Lands[_landTag].payableAmount = _payableAmount;
      return true;
    }
    return false;
  }

  function registerUser(string memory _userName, address _userAddress) public isAdmin returns (bool) { 
    if (Users[_userAddress].userAddress == address(0)) {
      usercount++;
      userDetails memory user = userDetails({
        userId: usercount,
            userName: _userName,
            userAddress: _userAddress,
            userPrivilege: privilege.customer,
            userBalance: _userAddress.balance
            });
      Users[_userAddress] = user;
      return true;
    }
    return false;
  }
  
  function registerCustomer(string memory _userName) public returns (bool) { 
    if (Users[msg.sender].userAddress == address(0)) {
      usercount++;
      userDetails memory user = userDetails({
        userId: usercount,
            userName: _userName,
            userAddress: msg.sender,
            userPrivilege: privilege.customer,
            userBalance: msg.sender.balance
            });
      Users[msg.sender] = user;
      return true;
    }
    return false;
  }
  
  function transactLand(uint _landTag) public payable returns (bool) {
    
    userDetails storage userBuy = Users[msg.sender];
    landDetails storage land = Lands[_landTag];
    if (land.landStatus == landState.onSale) {// && 
				//userBuy.userAddress.balance >= land.payableAmount) {
  
      userDetails storage userSell = Users[land.landOwnerAddress];
       
			land.landOwnerAddress.send(land.payableAmount * 1 ether);
			
      land.landPreviousOwners.push(land.landOwnerAddress);
		  land.landOwnerAddress = msg.sender;
      land.landOwnerName = Users[msg.sender].userName;
      land.landStatus = landState.owned;
        
      landDetails memory landmem = land;
      Lands[_landTag] = landmem;
      return true;
    }
    return false;
  }  
}
