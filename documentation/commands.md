BEFORE DOING ANYTHING: npm install

In order to mint more tokens, update the args for the mint_to function in mint_to.ts and 
run the command below, wallet 1, 2, 3 are the multisig owners, and there's a minimum threshold
of 2 to push the transaction through.


Program address: ErCER1skeaeqzRzSaqpmAKEAZmGGdh9VRzNAXXWUXv8f
spl mint: GCBFs8zj6KuUAzAeNskF7fhqmGgd42uXD9FMcnQSGo7b
wallet 1: 8vhsmokj9MVCc1uV8dXbsxnFwDeDL8aymXL4TfVptcCE
wallet 2: J2LTTLBJ1DoSFQZdXyYCVmHPCrXxzVfgDCHQHqLDP921
wallet 3: 9g7Lvri1ViPwpfE4zTd6g5k88ecne7uQQnahQAnETSAF

//Config
solana config get

//start validator
solana-test-validator

//to mint new tokens:
npx ts-node mint_to.ts //inside deployment/src/

//to send tokens:
spl-token transfer <TOKEN_MINT_ADDRESS> <AMOUNT> <RECIPIENT_WALLET_ADDRESS> --fund-recipient --owner <SENDER_KEYPAIR_PATH> --fee-payer <SENDER_KEYPAIR_PATH>

//to burn tokens:
spl-token burn <TOKEN_ACCOUNT_ADDRESS> <AMOUNT>


