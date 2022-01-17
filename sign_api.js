
const nearAPI = require("near-api-js");
const bs58 = require('bs58');


async function init() {
  
  let keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore("/home/bhc/.near-credentials");
  let account_id = "bhc8521.testnet"

  const keyPair = await keyStore.getKey("testnet", account_id);

  let args = {  //api获取的args
    haha: 1
  }
  const args_string = JSON.stringify(args)
  const data_buffer = Buffer.from(args_string);
  const { signature } = keyPair.sign(data_buffer);
  let sign = bs58.encode(signature)

  const public_key = keyPair.publicKey.data
  const bs58_public_key = bs58.encode(public_key)


  //最后返回sign和bs58_public_key
}


init()