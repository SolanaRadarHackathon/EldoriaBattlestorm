import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplCore } from '@metaplex-foundation/mpl-core'
import { generateSigner, publicKey } from '@metaplex-foundation/umi'
import { create, ruleSet } from '@metaplex-foundation/mpl-core'

import {
    Connection,
    Keypair,
    PublicKey,
    TransactionMessage,
    VersionedTransaction,
    SystemProgram,
    TransactionInstruction,
    SYSVAR_RENT_PUBKEY,
    Transaction,
    LAMPORTS_PER_SOL,
    SYSVAR_SLOT_HISTORY_PUBKEY,
    SYSVAR_SLOT_HASHES_PUBKEY,
  } from "@solana/web3.js";
import { keypairIdentity } from '@metaplex-foundation/umi';

const umi = createUmi('https://api.devnet.solana.com').use(mplCore())

const connection= new Connection("https://api.devnet.solana.com","confirmed");

const secretKey = 
[95,168,25,23,8,29,184,48,24,224,178,228,249,73,243,66,254,205,99,69,118,
127,28,4,237,252,228,240,83,0,28,210,158,98,151,229,149,244,142,17,58,
193,85,159,194,75,48,136,151,11,112,32,40,95,178,76,250,15,242,142,3,71,163,244]

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(secretKey))
const payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));

umi.use(keypairIdentity(keypair))

const programId = new PublicKey("Fkohtb7Y6k8QCt7d2SYJyGDNMYT3pu5GZYczDNXdTun2")
const mpl_core = new PublicKey('CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d')


const creator = publicKey('BfGcY8YBCdgp9eE5V8Dem3nKzU367YKa41vEeHp5xnkj')
const burn_auth = publicKey('CdaBE1S8ew2BpcsnqpMBezmWDu2QdD6AY8euovhVph4V')//254
const update_auth = publicKey('HQ8643M6MBaxwdJGPwZbMMJsx9nnLkzZMxxXS7oYVv2Q')//253



const assetSigner = generateSigner(umi)

const create_core_nft = async () => {
    
await create(umi, {
    asset: assetSigner,
    name: 'Dark Knight',
    uri: 'https://butterflylottery.github.io/metadata.json',
    plugins: [
      {
        type: 'Royalties',
        basisPoints: 500,
        creators: [
          {
            address: creator,
            percentage: 100,
          },

        ],
        ruleSet: ruleSet('None'), // Compatibility rule set
      },
      {
        type:'PermanentBurnDelegate',
        authority: { type: 'Address', address: burn_auth },
      },
      {
        type: 'UpdateDelegate',
        authority: { type: 'Address', address: update_auth },
        additionalDelegates: [],
      },
      {
        type: 'Attributes',
        attributeList: [
          { key: 'color', value: 'blue' },
          { key: 'health', value: '50' },
          { key: 'defence', value: '30' },
        ],
      },
    ],
  }).sendAndConfirm(umi)
  
  console.log(assetSigner.publicKey.toString())
}


const get_pda_address = async () => {
    
    const burn = PublicKey.findProgramAddressSync([Buffer.from("burn")],programId)
    const update = PublicKey.findProgramAddressSync([Buffer.from("update")],programId)

    console.log(burn[0].toString())
    console.log(burn[1])

}



const read_attributes = async () => {
    
    const burn = PublicKey.findProgramAddressSync([Buffer.from("burn")],programId)
    const update = PublicKey.findProgramAddressSync([Buffer.from("update")],programId)

    const nft = new PublicKey("5NV6EGD8pw3YrRmHvjV6mmidnubiv5gbeDw3hzNaLWYe")

    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
          {isSigner:false,isWritable:true,pubkey:nft},
        ],
        data:Buffer.from([1])});
    
    
        const message = new TransactionMessage({
          instructions: [ix],
            payerKey: payer.publicKey,
            recentBlockhash : (await connection.getLatestBlockhash()).blockhash
          }).compileToV0Message();
      
          const tx = new VersionedTransaction(message);
          tx.sign([payer]);
    
        const sig = await connection.sendTransaction(tx);

}
const burn_nft_pda = async () => {
    
    const burn = PublicKey.findProgramAddressSync([Buffer.from("burn")],programId)
    const update = PublicKey.findProgramAddressSync([Buffer.from("update")],programId)

    const nft = new PublicKey("5NV6EGD8pw3YrRmHvjV6mmidnubiv5gbeDw3hzNaLWYe")

    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
          {isSigner:false,isWritable:true,pubkey:burn[0]},
          {isSigner:false,isWritable:true,pubkey:nft},
          {isSigner:false,isWritable:true,pubkey:mpl_core},
        ],
        data:Buffer.from([3])});
    
    
        const message = new TransactionMessage({
          instructions: [ix],
            payerKey: payer.publicKey,
            recentBlockhash : (await connection.getLatestBlockhash()).blockhash
          }).compileToV0Message();
      
          const tx = new VersionedTransaction(message);
          tx.sign([payer]);
    
        const sig = await connection.sendTransaction(tx);

}

const upgrade_2 = async () => {
    
    const burn = PublicKey.findProgramAddressSync([Buffer.from("burn")],programId)
    const update = PublicKey.findProgramAddressSync([Buffer.from("update")],programId)

    const nft = new PublicKey("5NV6EGD8pw3YrRmHvjV6mmidnubiv5gbeDw3hzNaLWYe")
    const noop = new PublicKey("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV")
    

    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
          {isSigner:false,isWritable:true,pubkey:update[0]},
          {isSigner:false,isWritable:true,pubkey:payer.publicKey},
          {isSigner:false,isWritable:true,pubkey:nft},
          {isSigner:false,isWritable:true,pubkey:mpl_core},
          {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
          {isSigner:false,isWritable:true,pubkey:noop},
        ],
        data:Buffer.from([4])});
    
    
        const message = new TransactionMessage({
          instructions: [ix],
            payerKey: payer.publicKey,
            recentBlockhash : (await connection.getLatestBlockhash()).blockhash
          }).compileToV0Message();
      
          const tx = new VersionedTransaction(message);
          tx.sign([payer]);
    
        const sig = await connection.sendTransaction(tx);

}




