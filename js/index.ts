import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction
} from "@solana/web3.js"
import BN from "bn.js"


const main = async () => {
    var args = process.argv.slice(2)
    const programId = new PublicKey(args[0])

    const connection = new Connection("http:://api.devnet.solana.com/")
    // Thats me
    const feePayer = new Keypair()
    await connection.requestAirdrop(feePayer.publicKey,2e9)

    const account = new Keypair()
    const accountKey = account.publicKey
    const tx = new Transaction()
    let signers = [feePayer]

    let createIx = SystemProgram.createAccount({
        fromPubkey: feePayer.publicKey,
        newAccountPubkey: accountKey,
        lamports: await connection.getMinimumBalanceForRentExemption(8),
        space: 8,
        programId: programId,
    })
    signers.push(account)
    tx.add(createIx)

    const idx = Buffer.from(new Uint8Array([0]))
    let incrIx = new TransactionInstruction({
        keys:[
            {
                pubkey:accountKey,
                isSigner:false,
                isWritable: true,
            }
        ],
        programId:programId,
        data: idx
    })
    tx.add(incrIx)


    let txid = await sendAndConfirmTransaction(connection,tx,signers,{
        skipPreflight:true,
        preflightCommitment:"confirmed",
    })
    console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);

    const data = (await connection.getAccountInfo(accountKey)).data
    const count = new BN(data,"le")
    console.log("counter key: ",accountKey.toBase58())
    console.log("count: ",count.toNumber()) 
}

main()
.then(() => {
    console.log("success")
}).catch(e =>{
    console.error(e)
})