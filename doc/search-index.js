var searchIndex = JSON.parse('{\
"flix_ns":{"doc":"OmniFlix NameService","t":[13,13,13,13,13,17,13,13,3,13,4,6,13,6,6,8,4,6,13,3,13,13,13,17,17,13,13,13,13,4,4,13,13,13,17,13,13,13,13,13,13,13,12,12,10,10,10,10,11,11,0,0,10,0,5,11,11,11,11,11,0,5,11,0,10,10,10,10,12,12,5,12,12,11,0,11,10,12,12,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,17,17,17,17,5,5,5,13,4,13,13,13,13,13,13,13,13,13,13,12,12,12,12,12,12,12,12,12,12,12,5,5,5,5,5,13,13,13,13,13,13,13,4,13,3,13,4,4,13,13,13,13,13,13,13,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,3,11,12,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,12,11,11,11],"n":["Approvals","Approve","Balance","Balance","Burn","CONFIG","ChangeAdmin","ClosedSaleWindow","Config","Config","ContractError","Cw721Contract","Cw721ContractError","Cw721ExecuteMsg","Cw721MintMsg","Cw721Query","ExecuteMsg","Extension","GetSaleFlag","InstantiateMsg","InsufficientFundsSent","InvalidCharacter","InvalidToAddress","MAX_NAME_LENGTH","MIN_NAME_LENGTH","NameRecord","NameTaken","NameTooLong","NameTooShort","QueryMsg","QueryResponse","Register","ResolveRecord","Revoke","SUFFIX","SaleFlag","SendTokens","SetSale","Std","TransferName","Unauthorized","UnregisteredName","admin","admin","all_nft_info","all_tokens","approval","approvals","borrow","borrow_mut","config","contract","contract_info","error","execute","fmt","fmt","from","from","from","helpers","instantiate","into","msg","nft_info","num_tokens","operators","owner_of","purchase_price","purchase_price","query","sale_flag","sale_flag","source","state","to_string","tokens","transfer_price","transfer_price","try_from","try_into","type_id","0","0","c","flag","length","length","max_length","min_length","name","name","to_address","admin","amount","expires","flag","name","name","name","name","name","name","spender","spender","to","include_expired","name","name","name","address","amount","flag","name","name","CONFIG","MAX_NAME_LENGTH","MIN_NAME_LENGTH","SUFFIX","execute","instantiate","query","ClosedSaleWindow","ContractError","Cw721ContractError","InsufficientFundsSent","InvalidCharacter","InvalidToAddress","NameTaken","NameTooLong","NameTooShort","Std","Unauthorized","UnregisteredName","0","0","c","flag","length","length","max_length","min_length","name","name","to_address","assert_sent_sufficient_coin","assert_sent_sufficient_coins","sanitize_name","validate_name","verified_name_owner","Approvals","Approve","Balance","Balance","Burn","ChangeAdmin","Config","ExecuteMsg","GetSaleFlag","InstantiateMsg","NameRecord","QueryMsg","QueryResponse","Register","ResolveRecord","Revoke","SaleFlag","SendTokens","SetSale","TransferName","__clone_box","__clone_box","__clone_box","__clone_box","admin","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","deserialize","deserialize","deserialize","deserialize","eq","eq","eq","eq","fmt","fmt","fmt","fmt","from","from","from","from","into","into","into","into","json_schema","json_schema","json_schema","json_schema","ne","ne","ne","ne","purchase_price","sale_flag","schema_name","schema_name","schema_name","schema_name","serialize","serialize","serialize","serialize","to_owned","to_owned","to_owned","to_owned","transfer_price","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","admin","amount","expires","flag","name","name","name","name","name","name","spender","spender","to","include_expired","name","name","name","address","amount","flag","name","name","Config","__clone_box","admin","borrow","borrow_mut","clone","clone_into","deserialize","eq","fmt","from","into","json_schema","ne","new","purchase_price","sale_flag","schema_name","serialize","to_owned","transfer_price","try_from","try_into","type_id"],"q":["flix_ns","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","flix_ns::ContractError","","","","","","","","","","","flix_ns::ExecuteMsg","","","","","","","","","","","","","flix_ns::QueryMsg","","","","flix_ns::QueryResponse","","","","","flix_ns::config","","","","flix_ns::contract","","","flix_ns::error","","","","","","","","","","","","flix_ns::error::ContractError","","","","","","","","","","","flix_ns::helpers","","","","","flix_ns::msg","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","flix_ns::msg::ExecuteMsg","","","","","","","","","","","","","flix_ns::msg::QueryMsg","","","","flix_ns::msg::QueryResponse","","","","","flix_ns::state","","","","","","","","","","","","","","","","","","","","","","",""],"d":["For querying Approvals of a Name NFT","A name owner can temporarily or permanantly give …","Querying for the balance of the owner of name","Response for when QueryMsg::Balance is invoked","For deleting an NFT or name. The record will be deleted …","Config state instance where the configuration info is …","Only admin can call this endpoint. He can only do it once. …","When sale_flag is false, the Sale Window is considered to …","Configuration State","For querying for the current config state ","Contract Errors","cw721 contract initialised using generics","For convering Cw721ContractError variants to NameSerivce …","cw721 ExecuteMsg with generic over <code>Extension</code>","cw721 base provides MetaData extension for MintMsg as well","CW721 Contract Intigratation Variables","ExecuteMsg is the main JSON serializable enum. Based on …","Cw721Contract allows metadata to be stored as an Extension.","For querying the current value of sale flag","Example Json Payload for InstantiateMsg :","Raised when a wallet tries to execute an action with …","Raised when an invalid character is found in name during …","Raised when an attempt to transfer a name to a …","Maximum length of the name in u64 characters","Minimum length of name in u64 characters","Response for when <code>QueryMsg::ResolveRecord</code>","Raised when an attempt to register a previously taken name …","Raised when the name for registration is longer than …","Raised when the name for registration is too short or less …","Query enum variants are for distinguishing various kinds …","Various Query Responses","Register a name for a wallet","ResolveRecord returns the current address that the name …","Remove previously granted Approval","The suffix for the name","Response for when QueryMsg::GetSaleFlag is requested","Send tokens to aa wallet based on their name","The Config’s <code>sale_flag</code> needs to be true, if the accounts …","For converting error from <code>cosmwasm_std::StdError</code> Into Name …","Transfer the ownership to a different wallet","Raised when a wallet is not permitted to excute various …","Raised when an action encounters access to unregistered or …","The admin will be able to toggle the sale flag","Admin can change the sale_flag after the instantiation …","","","","","","","Contract Config Info","Contract Entry Points","","Contract Errors","The main Entry Point for the contract execution. It is …","","","Returns the argument unchanged.","","","Helpers","","Calls <code>U::from(self)</code>.","Messages","","","","","The price for registering the name","The amount that must be sent to the contract during …","","When the sale flag is true, people will be able to …","If sale flag is true, name can be registered. Otherwise, …","","States","","","The price for transfering the name to a different wallet","The amount that must be sent to the contract during …","","","","","","The first invalid character that is found","the current value of the sale_flag","Length of the current name that was responsible for this …","Length of the current name that was responsible for this …","Maximum length allowed for the name Also Refer to: …","Minimum length allowed for the name Also Refer to: …","Name which was responsible for this error","Name which was responsible for this error","The invalid address that caused this error","the wallet address of the new admin ","The amount that the message sender wants to send to the …","until","Config’s <code>sale_flag</code> will be set to whatever this value is.","Name to be registered. This name resolves to its owners …","A name that is owned by the message sender that needs to …","The owner of this name who will receive these tokens.","Name that needs to be burned or deleted.","the name","the name of the NFT on which the the spender until now …","Spender’s wallet address (or contract address) who needs …","The spender’s wallet address or contract address","The address of the new owner to whom the aforementioned …","The option to whether or not include expiration info","Queries for the owner of this name","Based on the name, the owner’s address will be resolved","the name that we want query approvals for","and the address that the record belongs","the balance in various denominations","current value of the flag","name ","name, whose owners balance is requested","Config state instance where the configuration info is …","Maximum length of the name in u64 characters","Minimum length of name in u64 characters","The suffix for the name","The main Entry Point for the contract execution. It is …","","","When sale_flag is false, the Sale Window is considered to …","Contract Errors","For convering Cw721ContractError variants to NameSerivce …","Raised when a wallet tries to execute an action with …","Raised when an invalid character is found in name during …","Raised when an attempt to transfer a name to a …","Raised when an attempt to register a previously taken name …","Raised when the name for registration is longer than …","Raised when the name for registration is too short or less …","For converting error from <code>cosmwasm_std::StdError</code> Into Name …","Raised when a wallet is not permitted to excute various …","Raised when an action encounters access to unregistered or …","","","The first invalid character that is found","the current value of the sale_flag","Length of the current name that was responsible for this …","Length of the current name that was responsible for this …","Maximum length allowed for the name Also Refer to: …","Minimum length allowed for the name Also Refer to: …","Name which was responsible for this error","Name which was responsible for this error","The invalid address that caused this error","A function to check if sufficient coin for a given action …","To check if the sender sent sufficient coins to the …","Removes “.flix” suffix if it ends with it and …","validate_name returns an error if the name is invalid (we …","When given a name, the function returns the address of …","For querying Approvals of a Name NFT","A name owner can temporarily or permanantly give …","Querying for the balance of the owner of name","Response for when QueryMsg::Balance is invoked","For deleting an NFT or name. The record will be deleted …","Only admin can call this endpoint. He can only do it once. …","For querying for the current config state ","ExecuteMsg is the main JSON serializable enum. Based on …","For querying the current value of sale flag","Example Json Payload for InstantiateMsg :","Response for when <code>QueryMsg::ResolveRecord</code>","Query enum variants are for distinguishing various kinds …","Various Query Responses","Register a name for a wallet","ResolveRecord returns the current address that the name …","Remove previously granted Approval","Response for when QueryMsg::GetSaleFlag is requested","Send tokens to aa wallet based on their name","The Config’s <code>sale_flag</code> needs to be true, if the accounts …","Transfer the ownership to a different wallet","","","","","The admin will be able to toggle the sale flag","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","The price for registering the name","When the sale flag is true, people will be able to …","","","","","","","","","","","","","The price for transfering the name to a different wallet","","","","","","","","","","","","","the wallet address of the new admin ","The amount that the message sender wants to send to the …","until","Config’s <code>sale_flag</code> will be set to whatever this value is.","Name to be registered. This name resolves to its owners …","A name that is owned by the message sender that needs to …","The owner of this name who will receive these tokens.","Name that needs to be burned or deleted.","the name","the name of the NFT on which the the spender until now …","Spender’s wallet address (or contract address) who needs …","The spender’s wallet address or contract address","The address of the new owner to whom the aforementioned …","The option to whether or not include expiration info","Queries for the owner of this name","Based on the name, the owner’s address will be resolved","the name that we want query approvals for","and the address that the record belongs","the balance in various denominations","current value of the flag","name ","name, whose owners balance is requested","Configuration State","","Admin can change the sale_flag after the instantiation …","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","Easy method to validate the InstantiateMsg and create a …","The amount that must be sent to the contract during …","If sale flag is true, name can be registered. Otherwise, …","","","","The amount that must be sent to the contract during …","","",""],"i":[1,2,1,3,2,0,2,4,0,1,0,0,4,0,0,0,0,0,1,0,4,4,4,0,0,3,4,4,4,0,0,2,1,2,0,3,2,2,4,2,4,4,5,6,7,7,7,7,4,4,0,0,7,0,0,4,4,4,4,4,0,0,4,0,7,7,7,7,5,6,0,5,6,4,0,4,7,5,6,4,4,4,8,9,10,11,12,13,13,12,14,15,16,17,18,19,20,21,22,18,23,19,24,19,24,22,25,26,27,25,28,29,30,28,29,0,0,0,0,0,0,0,4,0,4,4,4,4,4,4,4,4,4,4,8,9,10,11,12,13,13,12,14,15,16,0,0,0,0,0,1,2,1,3,2,2,1,0,1,0,3,0,0,2,1,2,3,2,2,2,2,5,1,3,5,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,5,5,2,5,1,3,2,5,1,3,2,5,1,3,5,2,5,1,3,2,5,1,3,2,5,1,3,17,18,19,20,21,22,18,23,19,24,19,24,22,25,26,27,25,28,29,30,28,29,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0],["deps",3,[["empty",3]]],["env",3],["string",3],["bool",0]],["result",4,[["allnftinforesponse",3],["stderror",4]]]],[[["",0],["deps",3,[["empty",3]]],["option",4,[["string",3]]],["option",4,[["u32",0]]]],["result",4,[["tokensresponse",3],["stderror",4]]]],[[["",0],["deps",3,[["empty",3]]],["env",3],["string",3],["string",3],["bool",0]],["result",4,[["approvalresponse",3],["stderror",4]]]],[[["",0],["deps",3,[["empty",3]]],["env",3],["string",3],["bool",0]],["result",4,[["approvalsresponse",3],["stderror",4]]]],[[["",0]],["",0]],[[["",0]],["",0]],null,null,[[["",0],["deps",3,[["empty",3]]]],["result",4,[["contractinforesponse",3],["stderror",4]]]],null,[[["depsmut",3],["env",3],["messageinfo",3],["executemsg",4]],["result",4,[["response",3],["contracterror",4]]]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[["cw721contracterror",4]]],[[["stderror",4]]],null,[[["depsmut",3],["env",3],["messageinfo",3],["instantiatemsg",3]],["result",4,[["response",3],["contracterror",4]]]],[[]],null,[[["",0],["deps",3,[["empty",3]]],["string",3]],["result",4,[["nftinforesponse",3],["stderror",4]]]],[[["",0],["deps",3,[["empty",3]]]],["result",4,[["numtokensresponse",3],["stderror",4]]]],[[["",0],["deps",3,[["empty",3]]],["env",3],["string",3],["bool",0],["option",4,[["string",3]]],["option",4,[["u32",0]]]],["result",4,[["operatorsresponse",3],["stderror",4]]]],[[["",0],["deps",3,[["empty",3]]],["env",3],["string",3],["bool",0]],["result",4,[["ownerofresponse",3],["stderror",4]]]],null,null,[[["deps",3],["env",3],["querymsg",4]],["stdresult",6,[["binary",3]]]],null,null,[[["",0]],["option",4,[["error",8]]]],null,[[["",0]],["string",3]],[[["",0],["deps",3,[["empty",3]]],["string",3],["option",4,[["string",3]]],["option",4,[["u32",0]]]],["result",4,[["tokensresponse",3],["stderror",4]]]],null,null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["depsmut",3],["env",3],["messageinfo",3],["executemsg",4]],["result",4,[["response",3],["contracterror",4]]]],[[["depsmut",3],["env",3],["messageinfo",3],["instantiatemsg",3]],["result",4,[["response",3],["contracterror",4]]]],[[["deps",3],["env",3],["querymsg",4]],["stdresult",6,[["binary",3]]]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["option",4,[["coin",3]]]],["result",4,[["contracterror",4]]]],[[],["result",4,[["contracterror",4]]]],[[["string",3]],["string",3]],[[["str",0]],["result",4,[["contracterror",4]]]],[[["depsmut",3],["env",3],["string",3]],["result",4,[["addr",3],["contracterror",4]]]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0],["private",3]]],[[["",0],["private",3]]],[[["",0],["private",3]]],[[["",0],["private",3]]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["executemsg",4]],[[["",0]],["instantiatemsg",3]],[[["",0]],["querymsg",4]],[[["",0]],["queryresponse",4]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0],["executemsg",4]],["bool",0]],[[["",0],["instantiatemsg",3]],["bool",0]],[[["",0],["querymsg",4]],["bool",0]],[[["",0],["queryresponse",4]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["schemagenerator",3]],["schema",4]],[[["schemagenerator",3]],["schema",4]],[[["schemagenerator",3]],["schema",4]],[[["schemagenerator",3]],["schema",4]],[[["",0],["executemsg",4]],["bool",0]],[[["",0],["instantiatemsg",3]],["bool",0]],[[["",0],["querymsg",4]],["bool",0]],[[["",0],["queryresponse",4]],["bool",0]],null,null,[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0],["private",3]]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["config",3]],[[["",0],["",0]]],[[],["result",4]],[[["",0],["config",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["schemagenerator",3]],["schema",4]],[[["",0],["config",3]],["bool",0]],[[["depsmut",3],["messageinfo",3],["instantiatemsg",3]]],null,null,[[],["string",3]],[[["",0]],["result",4]],[[["",0]]],null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]]],"p":[[4,"QueryMsg"],[4,"ExecuteMsg"],[4,"QueryResponse"],[4,"ContractError"],[3,"InstantiateMsg"],[3,"Config"],[8,"Cw721Query"],[13,"Std"],[13,"Cw721ContractError"],[13,"InvalidCharacter"],[13,"ClosedSaleWindow"],[13,"NameTooShort"],[13,"NameTooLong"],[13,"UnregisteredName"],[13,"NameTaken"],[13,"InvalidToAddress"],[13,"ChangeAdmin"],[13,"SendTokens"],[13,"Approve"],[13,"SetSale"],[13,"Register"],[13,"TransferName"],[13,"Burn"],[13,"Revoke"],[13,"Approvals"],[13,"ResolveRecord"],[13,"Balance"],[13,"NameRecord"],[13,"Balance"],[13,"SaleFlag"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};