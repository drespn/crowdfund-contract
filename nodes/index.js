const { Connection, Keypair, PublicKey, SystemProgram, Transaction,TransactionInstruction, sendAndConfirmTransaction, LAMPORTS_PER_SOL } = require("@solana/web3.js");
const { fs } = require("mz");

async function establishConnection(){
    //default port when doing local work
    const rpcUrl = 'http://localhost:8899';

    const connection = new Connection(rpcUrl,'confirmed');

    const version = await connection.getVersion();

    console.log('Connection to cluster established',rpcUrl,version);
}


async function createKeyPairFromFile(){
    //get secret key from computer in .json format into string
    const secretKeyString = await fs.readFile("/Users/diegoespinosa/.config/solana/id.json",{encoding:'utf8'});

    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));

    return Keypair.fromSecretKey(secretKey);
}

async function createAccount() {

    const rpcUrl = 'http://localhost:8899';
    let connection = new Connection(rpcUrl, 'confirmed');
    const signer = await createKeyPairFromFile();
    const newAccountPubkey = await PublicKey.createWithSeed(
      signer.publicKey,
      "campaign3",
      new PublicKey("EfyWUgrFnBHRcSZXo9v8UYEzVc26uHRFAbyB7A6pS1aE"),
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      1024,
    );
    const instruction = SystemProgram.createAccountWithSeed({
      fromPubkey: signer.publicKey,
      basePubkey: signer.publicKey,
      seed: "campaign3",
      newAccountPubkey,
      lamports,
      space: 1024,
      programId : new PublicKey("EfyWUgrFnBHRcSZXo9v8UYEzVc26uHRFAbyB7A6pS1aE"),
    });
    const transaction = new Transaction().add(
      instruction
    );
  
    console.log(`The address of campaign1 account is : ${newAccountPubkey.toBase58()}`);
  
    await sendAndConfirmTransaction(connection, transaction, [signer]);
  
  }

establishConnection();
createAccount();