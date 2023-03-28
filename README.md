# Luxury-Token

Vishnu Sankar – 101440532 

## Summary:
PSP34 (Pseudo Standardized Product 34) is an ERC-721 equivalent token that can be used to represent unique and non-fungible products in a decentralized manner on a blockchain network. A potential use case for PSP34 tokens can be in the luxury fashion industry. 

In the luxury fashion industry, each product is unique, and customers are willing to pay a premium price for exclusive items. PSP34 tokens can be used to represent these unique products, and each token can be associated with a specific product in a decentralized manner. 


## Users:
Luxury fashion brands: The luxury fashion brands would be the ones creating the PSP34 tokens to represent their unique products. They would use the smart contract to mint the tokens and associate them with the specific products. 

Retailers: Retailers who sell luxury fashion products could use the smart contract to verify the authenticity of the PSP34 tokens associated with the products they are selling. 

Customers: Customers who purchase luxury fashion products could use the smart contract to verify the authenticity of the PSP34 token associated with the product they are buying. They could also use the smart contract to transfer ownership of the token to another person. 

Collectors: Collectors who are interested in owning unique and rare luxury fashion products could use the smart contract to buy and sell PSP34 tokens representing these products. 

Blockchain network participants: Finally, the users of the blockchain network itself would also interact with the smart contract to validate the transactions related to the PSP34 tokens. 

## Scenario 

Each player interacts with each other in the following manner:

Luxury fashion brands: The luxury fashion brands would use the smart contract to create new PSP34 tokens and associate them with unique products. They would specify the properties of the product, such as its colour, size, and material, along with its provenance and other relevant information. Once the PSP34 token is minted, it can be transferred to other users, such as retailers or collectors. 

Retailers: Retailers who sell luxury fashion products could use the smart contract to verify the authenticity of the PSP34 tokens associated with the products they are selling. They would scan the QR code or enter the token ID into the smart contract to verify that the PSP34 token is valid and associated with the product being sold. 

Customers: Customers who purchase luxury fashion products could use the smart contract to verify the authenticity of the PSP34 token associated with the product they are buying. They would scan the QR code or enter the token ID into the smart contract to verify that the PSP34 token is valid and associated with the product they are purchasing. They could also transfer ownership of the token to another person by using the smart contract to update the ownership record on the blockchain. 

Collectors: Collectors could use the smart contract to buy and sell PSP34 tokens representing unique and rare luxury fashion products. They could use the smart contract to verify the authenticity of the token and transfer ownership to other collectors or customers. The smart contract would verify the transaction and update the token's ownership records accordingly. 

Blockchain network participants: Participants in the blockchain network would interact with the smart contract to validate the transactions related to the PSP34 tokens. They would confirm that the transactions are valid and that the PSP34 tokens being transferred are authentic and associated with the correct products.

## Actions 

Here are some examples of the actions that each player can take when interacting with a smart contract: 

### Luxury fashion brands: 

1. Deploy the smart contract on the blockchain network. 

2. Mint PSP34 tokens and associate each token with a specific unique product by inputting the product information, such as the name, description, and image, into the smart contract. 

3. Transfer ownership of the PSP34 token to a retailer or collector by updating the ownership record on the blockchain through the smart contract. 

 
### Retailers: 

1. Use a scanning device to scan the QR code or enter the PSP34 token ID into the smart contract. 

2. Verify the authenticity of the PSP34 token and confirm that it is associated with the product being sold. 

3. Complete the transaction by transferring the ownership of the product to the customer. 

### Customers: 

1. Use a scanning device to scan the QR code or enter the PSP34 token ID into the smart contract. 

2. Verify the authenticity of the PSP34 token and confirm that it is associated with the product being purchased. 

3. Transfer ownership of the PSP34 token to another person by updating the ownership record on the blockchain through the smart contract. 

### Collectors: 

1. Use a scanning device to scan the QR code or enter the PSP34 token ID into the smart contract. 

2. Verify the authenticity of the PSP34 token and confirm that it is associated with the unique luxury product being traded. 

3. Transfer ownership of the PSP34 token to another person by updating the ownership record on the blockchain through the smart contract. 

### Blockchain network participants: 

1. Validate the transactions related to the PSP34 tokens by confirming that the transactions are valid and that the PSP34 tokens being transferred are authentic and associated with the correct products. 

## Code Description
- The code defines the smart contract using the ink_lang framework, which allows for easy development of smart contracts on the Polkadot platform using the Rust programming language.
- The smart contract defines a custom Token struct, which has four fields: name, symbol, total_supply, and balances.
- The name and symbol fields are of type String, while the total_supply field is of type u128.
- The balances field is a StorageHashMap that maps AccountId keys to u128 values, representing the balance of each account holding the token.
- The Token struct implements the Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, and PackedLayout traits, which are used for serialization and deserialization of the struct.
- The Token struct has a new constructor, which takes three arguments (name, symbol, and total_supply) and initializes a new Token instance with the specified values.
- The new constructor also initializes the balances mapping with a single entry for the caller of the constructor, with a balance equal to total_supply.
- The Token struct has a transfer message, which takes two arguments (to and value) and transfers value amount of tokens from the caller's account to the specified to account.
- The transfer message first checks if the caller has enough balance to transfer the requested value, and returns false if not.
- If the caller has enough balance, the transfer message updates the balances mapping to subtract value from the caller's balance and add value to the to account's balance, and returns true.
- The transfer message is marked with the ink attribute, which indicates that it is a callable message that can be invoked from outside the contract.
