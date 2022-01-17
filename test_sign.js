
const nearAPI = require("near-api-js");
var request = require('request');
const tweetnacl = require("tweetnacl");
const bs58 = require('bs58');


async function init() {
  
  const keyStore = new nearAPI.keyStores.InMemoryKeyStore();
  const PRIVATE_KEY =
    "5tzMa75ppoMcLb8i7yyX2EfZsvnnyfSmss9ReiKVSZu4oXat5tQ1M5U6VVFaehqWnmT9gdK7Y5ygerEvXB8qbR2t";
  // creates a public / private key pair using the provided private key
  const keyPair = nearAPI.KeyPair.fromString(PRIVATE_KEY);
  // adds the keyPair you created to keyStore
  await keyStore.setKey("testnet", "lzs.testnet", keyPair);
    
  // const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore(KEY_PATH);
  // const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore("/home/bhc/.near-credentials");
  // await keyStore.setKey("testnet", "bhc3.testnet", keyPair);

  const near = await nearAPI.connect({
    keyStore: keyStore,
    // keyStore: new nearAPI.keyStores.UnencryptedFileSystemKeyStore("~/.near-credentials/testnet/bhc3.testnet.json"),
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org',
  });
  let account_id = "lzs.testnet"

  //const keyPair = await keyStore.getKey("testnet", account_id);

  let args = "abc"
  const args_string = JSON.stringify(args)
  const data_buffer = Buffer.from(args_string);
  const { signature } = keyPair.sign(data_buffer);
  let sign = bs58.encode(signature)

  const public_key = keyPair.publicKey.data
  const bs58_public_key = bs58.encode(public_key)
  // 验证
  const decode_sign = bs58.decode(sign)
  let valid = tweetnacl.sign.detached.verify(new Uint8Array(data_buffer), new Uint8Array(decode_sign), public_key);
  console.log(valid)

  let account = await near.account(account_id);
  let contract = await new nearAPI.Contract(account, "discord-roles.bhc8521.testnet", {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: [],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ['del_role'],
  })

  await contract.del_role({args: args_string, sign: sign, pk: bs58_public_key}, "300000000000000")
}


init()