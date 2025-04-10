import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmRawTransaction,PublicKey, sendAndConfirmTransaction } from "@solana/web3.js";
import wallet from "./dev-wallet.json"

const from = Keypair.fromSecretKey(new Uint8Array(wallet))

const to = new PublicKey("9VXye98Kkcpk9evGKSZfXjFrexk3phXmfdim8dvcUrrz")

const connection = new Connection("https://api.devnet.solana.com");


(async()=>{
    try{
        const balance = await connection.getBalance(from.publicKey)

        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey:from.publicKey,
                toPubkey:to,
                lamports:LAMPORTS_PER_SOL/100
            })
        );
        transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;

         const fee = (await connection.getFeeForMessage(transaction.compileMessage(),'confirmed')).value||0;

         transaction.instructions.pop()

         transaction.add(
            SystemProgram.transfer({
                fromPubkey:from.publicKey,
                toPubkey:to,
                lamports:balance-fee,
            })
         )
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        )
        console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${signature}?cluster=devnet`);

    }catch(e){
        console.error(`Oops, something went wrong: ${e}`)
    }
})();