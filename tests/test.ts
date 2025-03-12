import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Drvx } from "../target/types/drvx";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import adminArr from './keys/admin.json'
import feeWalletArr from './keys/feeWallet.json'
import key1 from './keys/user1.json';
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, createAssociatedTokenAccount, createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo, NATIVE_MINT, getAssociatedTokenAddressSync, createAssociatedTokenAccountIdempotent, createAssociatedTokenAccountIdempotentInstruction } from "@solana/spl-token";

const GLOBAL_SEED = "global-seed"
const DECIMALS = 9;

describe("drvx", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const connection = new Connection("https://devnet.helius-rpc.com/?api-key=36fe5fc9-8598-4302-a28f-a93d9cc441b7", { commitment: 'confirmed' });
  // const connection = new Connection("http://localhost:8899", { commitment: 'confirmed' });
  const admin = Keypair.fromSecretKey(new Uint8Array(adminArr))
  const feeWallet = Keypair.fromSecretKey(new Uint8Array(feeWalletArr))
  const user1 = Keypair.fromSecretKey(bs58.decode(key1.key));
  const program = anchor.workspace.Drvx as Program<Drvx>;


  let usdtMint: PublicKey = new PublicKey("WUZaKnih6xjdvfrBsxzME7SPs8ja33UxUCxtsNyJiAa")
  let drvxMint: PublicKey = new PublicKey("MfkjE6B3wF6u65uFBsZdt9t2rE2Bn96peNjjFcLRFvX")
  let userUsdtTokenAta: PublicKey
  let userDrvxTokenAta: PublicKey
  // let userUsdtTokenAta: PublicKey = (await getOrCreateAssociatedTokenAccount(connection, admin, usdtMint, admin.publicKey)).address
  // let userDrvxTokenAta: PublicKey = (await getOrCreateAssociatedTokenAccount(connection, admin, drvxMint, admin.publicKey)).address
  let globalState: PublicKey;
  let scUsdtAta: PublicKey;
  let scDrvxAta: PublicKey;

  const tokenDecimal = 9

  const adminWallet = admin.publicKey;

  const amount = new BN(1000000000).mul(new BN(10 ** tokenDecimal))

  console.log("Admin's wallet address is : ", admin.publicKey.toBase58(), '\n');

  it(" Admin wallet's state", async () => {
    console.log("Admin's wallet balance : ", ((await connection.getBalance(adminWallet)) / 10 ** 9).toFixed(3), "SOL"), '\n'
  }
  );


  // it("Airdrop to admin wallet", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  Trying to Airdrop to admin wallet  ==============================", '\n')
  //   console.log(`Requesting airdrop to admin for 1SOL : ${admin.publicKey}`, '\n')
  //   // 1 - Request Airdrop
  //   const signature = await connection.requestAirdrop(
  //     admin.publicKey,
  //     10 ** 9
  //   );
  //   // 2 - Fetch the latest blockhash
  //   const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
  //   // 3 - Confirm transaction success

  //   await connection.confirmTransaction({
  //     blockhash,
  //     lastValidBlockHeight,
  //     signature
  //   }, 'confirmed');
  //   console.log("Admin's wallet balance : ", (await connection.getBalance(admin.publicKey)) / 10 ** 9, "SOL", '\n')
  // })

  // it("Airdrop to user1 wallet", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  Trying to Airdrop to user1 wallet  ==============================", '\n')
  //   console.log(`Requesting airdrop to user1 for 1SOL : ${admin.publicKey}`, '\n')
  //   // 1 - Request Airdrop
  //   const signature = await connection.requestAirdrop(
  //     user1.publicKey,
  //     10 ** 9
  //   );
  //   // 2 - Fetch the latest blockhash
  //   const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
  //   // 3 - Confirm transaction success
  //   await connection.confirmTransaction({
  //     blockhash,
  //     lastValidBlockHeight,
  //     signature
  //   }, 'confirmed');
  //   console.log("user1 wallet balance : ", (await connection.getBalance(user1.publicKey)) / 10 ** 9, "SOL", '\n')
  // })

  // it("Mint usdt token to admin wallet", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  Trying to create and usdt mint token to admin's wallet  ==============================", '\n')
  //   console.log("Admin's wallet balance : ", await connection.getBalance(admin.publicKey) / LAMPORTS_PER_SOL, '\n')
  //   // console.log(await connection.getBalance(user1.publicKey) / LAMPORTS_PER_SOL)
  //   //create usdtMint
  //   try {
  //     usdtMint = await createMint(connection, admin, admin.publicKey, admin.publicKey, tokenDecimal)
  //     console.log('usdt mint address: ' + usdtMint.toBase58(), '\n');

  //     userUsdtTokenAta = (await getOrCreateAssociatedTokenAccount(connection, admin, usdtMint, admin.publicKey)).address

  //     console.log('Admin wallet usdt ata account address: ' + userUsdtTokenAta.toBase58(), '\n');

  //     //minting 100 new tokens to the token address we just created
  //     await mintTo(connection, admin, usdtMint, userUsdtTokenAta, admin.publicKey, BigInt(amount.toString()))
  //     const tokenBalance = await connection.getTokenAccountBalance(userUsdtTokenAta)
  //     console.log("Admin wallet usdt tokenBalance :", tokenBalance.value.uiAmount, '\n')
  //     console.log('Admin wallet usdt token successfully minted', '\n');
  //   } catch (error) {
  //     console.log("Admin Wallet Usdt Token creation error \n", error)
  //   }
  // })

  // it("Mint drvx token to admin wallet", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  Trying to create and mint drvx token to admin's wallet  ==============================  ", '\n')
  //   console.log("Admin's wallet balance : ", await connection.getBalance(admin.publicKey) / LAMPORTS_PER_SOL, '\n')
  //   // console.log(await connection.getBalance(user1.publicKey) / LAMPORTS_PER_SOL)
  //   //create usdtMint
  //   try {
  //     drvxMint = await createMint(connection, admin, admin.publicKey, admin.publicKey, tokenDecimal)
  //     console.log('drvx mint address: ' + drvxMint.toBase58(), '\n');

  //     userDrvxTokenAta = (await getOrCreateAssociatedTokenAccount(connection, admin, drvxMint, admin.publicKey)).address
  //     console.log('Admin wallet token account address: ' + userDrvxTokenAta.toBase58(), '\n');

  //     //minting 100 new tokens to the token address we just created
  //     await mintTo(connection, admin, drvxMint, userDrvxTokenAta, admin.publicKey, BigInt(amount.toString()))
  //     const tokenBalance = await connection.getTokenAccountBalance(userDrvxTokenAta)
  //     // tokenBalance.value.uiAmount
  //     console.log("Admin wallet drvx tokenBalance in user:", tokenBalance.value.uiAmount, '\n')
  //     console.log('Admin wallet drvx token successfully minted', '\n');
  //   } catch (error) {
  //     console.log("Admin wallet Drvx Token creation error \n", error)
  //   }
  // })



  it("Is initialized!", async () => {
    let userUsdtTokenAta: PublicKey = (await getOrCreateAssociatedTokenAccount(connection, admin, usdtMint, admin.publicKey)).address
    let userDrvxTokenAta: PublicKey = (await getOrCreateAssociatedTokenAccount(connection, admin, drvxMint, admin.publicKey)).address

    console.log("\n\n")
    console.log("==============================  Admin initializes the smart contract  ==============================  ", '\n')
    try {
      const [fetchedGlobalState] = PublicKey.findProgramAddressSync(
        [Buffer.from(GLOBAL_SEED)],
        program.programId
      )
      globalState = fetchedGlobalState;
      scDrvxAta = getAssociatedTokenAddressSync(drvxMint, fetchedGlobalState, true)
      scUsdtAta = getAssociatedTokenAddressSync(usdtMint, fetchedGlobalState, true)

      console.log("globalState:", globalState.toBase58())
      console.log("🚀 ~ it ~ feeWallet:", feeWallet.publicKey.toBase58())

      const tx = new Transaction().add(
        await program.methods.initialize()
          .accounts({
            admin: admin.publicKey,
            drvxMint,
            usdtMint,
            feeWallet: feeWallet.publicKey,
          })
          .signers([admin])
          .instruction()
      )

      tx.feePayer = admin.publicKey
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
      console.log(await connection.simulateTransaction(tx))
      await sendAndConfirmTransaction(connection, tx, [admin])
      console.log("Below is global state value")
      const globalStateValue = await program.account.globalState.fetch(globalState)
      console.log("globalStateValue: \n", globalStateValue)


    } catch (error) {
      console.log("error in initialization :", error, '\n')
    }
  })


  // it("Admin deposit 700000000 usdt tokens", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  Trying to deposit 700000000 usdt tokens  ==============================  ", '\n')
  //   try {
  //     // const [globalState] = PublicKey.findProgramAddressSync(
  //     //   [Buffer.from(GLOBAL_SEED)],
  //     //   program.programId
  //     // )
  //     const depositUsdtAmount = new BN(700000000).mul(new BN(10 ** tokenDecimal)); // Deposit 500 tokens
  //     console.log("globalState:", globalState.toBase58(), '\n')
  //     console.log("Deposit Usdt Token Amount:", depositUsdtAmount, '\n')
  //     const transaction = new Transaction()
  //     const scUsdtAta = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smare Contract UsdtAta:", scUsdtAta.toBase58(), '\n')

  //     if (await connection.getAccountInfo(scUsdtAta) == null) {
  //       console.log("Admin create globalUsdtAta", '\n')
  //       transaction.add(createAssociatedTokenAccountInstruction(
  //         admin.publicKey,
  //         scUsdtAta,
  //         globalState,
  //         usdtMint
  //       ))
  //     }
  //     const instruction = await program.methods.depositTokens(depositUsdtAmount)
  //       .accounts({
  //         admin: adminWallet,
  //         adminTokenAccount: userUsdtTokenAta,
  //         globalStateTokenAccount: scUsdtAta,
  //         tokenMint: usdtMint,
  //       }).instruction()

  //     transaction.add(instruction)

  //     transaction.feePayer = admin.publicKey
  //     transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  //     console.log(await connection.simulateTransaction(transaction))
  //     const sig = await sendAndConfirmTransaction(connection, transaction, [admin])
  //     console.log({ sig })
  //     // globalata = globalAta;
  //     console.log("======  trying to get the usdt token balance of admin wallet  ======", '\n')
  //     const balInfo = await connection.getTokenAccountBalance(userUsdtTokenAta)
  //     console.log("Admin wallet usdt token balance is : ", balInfo.value.uiAmount, '\n')

  //     const globalUsdtAtaBalInfo = await connection.getTokenAccountBalance(scUsdtAta)
  //     console.log("Smart Contract Usdt token balance is : ", globalUsdtAtaBalInfo.value.uiAmount, '\n')
  //   } catch (error) {
  //     console.log("error in usdt Token deposit  :", error)
  //   }
  // })

  // it("Admin withdraw 100000000 usdt tokens", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  Admin withdraw 10000000 usdt tokens  ==============================  ")
  //   try {
  //     // const [globalState] = PublicKey.findProgramAddressSync(
  //     //   [Buffer.from(GLOBAL_SEED)],
  //     //   program.programId
  //     // )
  //     const withdrawUsdtAmount = new BN(10000000).mul(new BN(10 ** tokenDecimal)); // withdraw  tokens
  //     console.log("globalState:", globalState.toBase58(), '\n')
  //     console.log("Withdraw Usdt Token Amount:", withdrawUsdtAmount, '\n')
  //     const scUsdtAta = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smare Contract UsdtAta:", scUsdtAta.toBase58(), '\n')
  //     const scUsdtAtaBalInfo = await connection.getTokenAccountBalance(scUsdtAta)
  //     console.log("Smare Contract UsdtAta token balance is : ", scUsdtAtaBalInfo.value.uiAmount, '\n')

  //     const adminUsdtAta = await getAssociatedTokenAddress(usdtMint, admin.publicKey)
  //     const transaction = new Transaction()

  //     if (await connection.getAccountInfo(adminUsdtAta) == null) {
  //       console.log("Admin create adminUsdtAta", '\n')
  //       transaction.add(createAssociatedTokenAccountInstruction(
  //         admin.publicKey,
  //         adminUsdtAta,
  //         admin.publicKey,
  //         usdtMint
  //       ))
  //     }

  //     const instruction = await program.methods.withdrawTokens(withdrawUsdtAmount)
  //       .accounts({
  //         admin: admin.publicKey,
  //         adminTokenAccount: adminUsdtAta,
  //         globalStateTokenAccount: scUsdtAta,
  //         tokenMint: usdtMint,
  //       }).instruction()

  //     transaction.add(instruction)
  //     transaction.feePayer = admin.publicKey
  //     transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  //     console.log(await connection.simulateTransaction(transaction))
  //     const sig = await sendAndConfirmTransaction(connection, transaction, [admin])
  //     console.log({ sig })
  //     console.log("trying to get the usdt token balance of admin wallet", '\n')
  //     const adminInfo = await connection.getTokenAccountBalance(adminUsdtAta)
  //     console.log("adminUsdtAta token balance is : ", adminInfo.value.uiAmount, '\n')
  //     // console.log("user1 wallet token balance is : ", user1Info.value.uiAmount)

  //     const globalAtaInfo = await connection.getTokenAccountBalance(scUsdtAta)
  //     console.log("Smart Contract UsdtAta token balance is : ", globalAtaInfo.value.uiAmount, '\n')


  //   } catch (error) {
  //     console.log("error in withdraw usdt Token :", error)
  //   }
  // })

  // it("Admin deposit 500000000 drvx tokens", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  Trying to deposit 500000000 drvx tokens  ==============================  ", '\n')
  //   try {
  //     // const [globalState] = PublicKey.findProgramAddressSync(
  //     //   [Buffer.from(GLOBAL_SEED)],
  //     //   program.programId
  //     // )
  //     const depositdrvxAmount = new BN(500000000).mul(new BN(10 ** tokenDecimal)); // Deposit 500 tokens
  //     console.log("globalState:", globalState.toBase58(), '\n')
  //     console.log("Deposit Drvx Token Amount:", depositdrvxAmount, '\n')
  //     const transaction = new Transaction()
  //     const scdrvxAta = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("scdrvxAta:", scdrvxAta.toBase58())

  //     if (await connection.getAccountInfo(scdrvxAta) == null) {
  //       console.log("Admin create globalDrvxAta", '\n')
  //       transaction.add(createAssociatedTokenAccountInstruction(
  //         admin.publicKey,
  //         scdrvxAta,
  //         globalState,
  //         drvxMint
  //       ))
  //     }
  //     const instruction = await program.methods.depositTokens(depositdrvxAmount)
  //       .accounts({
  //         admin: adminWallet,
  //         adminTokenAccount: userDrvxTokenAta,
  //         globalStateTokenAccount: scdrvxAta,
  //         tokenMint: drvxMint,
  //       }).instruction()

  //     transaction.add(instruction)

  //     transaction.feePayer = admin.publicKey
  //     transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  //     console.log(await connection.simulateTransaction(transaction))
  //     const sig = await sendAndConfirmTransaction(connection, transaction, [admin])
  //     console.log({ sig })
  //     // globalata = globalAta;
  //     console.log("======  trying to get the drvx token balance of admin wallet  ======", '\n')
  //     const balInfo = await connection.getTokenAccountBalance(userDrvxTokenAta)
  //     console.log("Admin wallet Drvx token balance is : ", balInfo.value.uiAmount)

  //     const globalDrvxAtaBalInfo = await connection.getTokenAccountBalance(scdrvxAta)
  //     console.log("Smart Contract Drvx token balance is : ", globalDrvxAtaBalInfo.value.uiAmount, '\n')
  //   } catch (error) {
  //     console.log("error in drvx Token deposit  :", error)
  //   }
  // })

  // it("Admin withdraw 300000000 drvx tokens", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  Admin withdraw 30000000 drvx tokens  ==============================  ")
  //   try {
  //     // const [globalState] = PublicKey.findProgramAddressSync(
  //     //   [Buffer.from(GLOBAL_SEED)],
  //     //   program.programId
  //     // )
  //     const withdrawdrvxAmount = new BN(30000000).mul(new BN(10 ** tokenDecimal)); // withdraw  tokens
  //     console.log("globalState:", globalState.toBase58(), '\n')
  //     console.log("Withdraw Drvx Token Amount:", withdrawdrvxAmount, '\n')
  //     const scdrvxAta = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smare Contract DrvxAta:", scdrvxAta.toBase58(), '\n')
  //     const scdrvxAtaBalInfo = await connection.getTokenAccountBalance(scdrvxAta)
  //     console.log("Smare Contract DrvxAta token balance is :", scdrvxAtaBalInfo.value.uiAmount, '\n')

  //     const admindrvxAta = await getAssociatedTokenAddress(drvxMint, admin.publicKey)
  //     const transaction = new Transaction()

  //     if (await connection.getAccountInfo(admindrvxAta) == null) {
  //       console.log("Admin create adminDrvxAta", '\n')
  //       transaction.add(createAssociatedTokenAccountInstruction(
  //         admin.publicKey,
  //         admindrvxAta,
  //         admin.publicKey,
  //         drvxMint
  //       ))
  //     }

  //     const instruction = await program.methods.withdrawTokens(withdrawdrvxAmount)
  //       .accounts({
  //         admin: admin.publicKey,
  //         adminTokenAccount: admindrvxAta,
  //         globalStateTokenAccount: scdrvxAta,
  //         tokenMint: drvxMint,
  //       }).instruction()

  //     transaction.add(instruction)
  //     transaction.feePayer = admin.publicKey
  //     transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  //     console.log(await connection.simulateTransaction(transaction))
  //     const sig = await sendAndConfirmTransaction(connection, transaction, [admin])
  //     console.log({ sig })
  //     console.log("trying to get the drvx token balance of admin wallet", '\n')
  //     const adminInfo = await connection.getTokenAccountBalance(admindrvxAta)
  //     console.log("admindrvxAta token balance is : ", adminInfo.value.uiAmount, '\n')
  //     // console.log("user1 wallet token balance is : ", user1Info.value.uiAmount)

  //     const globalAtaInfo = await connection.getTokenAccountBalance(scdrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is : ", globalAtaInfo.value.uiAmount, '\n')


  //   } catch (error) {
  //     console.log("error in withdraw drvx Token :", error)
  //   }
  // })


  // it("User swap 10_000 Usdt token to Drvx tokens", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  User swap 10000 Usdt token to Drvx tokens  ==============================  ")
  //   try {

  //     const swapUsdtToDrvxAmount = new BN(10000).mul(new BN(10 ** tokenDecimal)); // withdraw  tokens
  //     console.log("globalState:", globalState.toBase58(), '\n')
  //     console.log("Swap Amount:", swapUsdtToDrvxAmount, '\n')

  //     const scUsdtAta = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smart Contract UsdtAta:", scUsdtAta.toBase58(), '\n')
  //     const scUsdtAtaBalInfo = await connection.getTokenAccountBalance(scUsdtAta)
  //     console.log("Smart Contract UsdtAta token balance is :", scUsdtAtaBalInfo.value.uiAmount, '\n')

  //     const scDrvxAta = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", scDrvxAta.toBase58(), '\n')
  //     const scDrvxAtaBalInfo = await connection.getTokenAccountBalance(scDrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is :", scDrvxAtaBalInfo.value.uiAmount, '\n')

  //     const userDrvxAta = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", userDrvxAta.toBase58(), '\n')
  //     const userDrvxAtaBalInfo = await connection.getTokenAccountBalance(userDrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is :", userDrvxAtaBalInfo.value.uiAmount, '\n')


  //     const userUsdtAta = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", userUsdtAta.toBase58(), '\n')
  //     const userUsdtAtaBalInfo = await connection.getTokenAccountBalance(userUsdtAta)
  //     console.log("Smart Contract DrvxAta token balance is :", scDrvxAtaBalInfo.value.uiAmount, '\n')


  //     const feeWalletAta = getAssociatedTokenAddressSync(usdtMint, feeWallet.publicKey)
  //     console.log("FeeWalletAta address:", feeWalletAta.toBase58())
  //     const feeAtaInfo = await connection.getAccountInfo(feeWalletAta)
  //     if (feeAtaInfo) {
  //       const feeBalance = await connection.getTokenAccountBalance(feeWalletAta)
  //       console.log("Fee Balance:", feeBalance.value.uiAmount)
  //     } else {
  //       console.log("Fee wallet ata still not created")
  //     }

  //     const transaction = new Transaction().add(
  //       createAssociatedTokenAccountIdempotentInstruction(
  //         admin.publicKey, feeWalletAta, feeWallet.publicKey, usdtMint
  //       ),
  //       await program.methods
  //         .swapUsdtToDrvx(new BN(1_000_000).mul(new BN(10 ** 6)))
  //         .accounts({
  //           drvxMint,
  //           usdtMint,
  //           feeWallet: feeWallet.publicKey,
  //           user: admin.publicKey
  //         })
  //         .instruction()
  //     )
  //     transaction.feePayer = admin.publicKey
  //     transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  //     console.log(await connection.simulateTransaction(transaction))
  //     const sig = await sendAndConfirmTransaction(connection, transaction, [admin])
  //     console.log({ sig })





  //     const scUsdtAtaAfterSwap = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smart Contract UsdtAta:", scUsdtAtaAfterSwap.toBase58(), '\n')
  //     const scUsdtAtaBalInfoAfterSwap = await connection.getTokenAccountBalance(scUsdtAta)
  //     console.log("Smart Contract UsdtAta token balance is :", scUsdtAtaBalInfoAfterSwap.value.uiAmount, '\n')

  //     const scDrvxAtaAfterSwap = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", scDrvxAtaAfterSwap.toBase58(), '\n')
  //     const scDrvxAtaBalInfoAfterSwap = await connection.getTokenAccountBalance(scDrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is :", scDrvxAtaBalInfoAfterSwap.value.uiAmount, '\n')

  //     const userDrvxAtaAfterSwap = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", userDrvxAtaAfterSwap.toBase58(), '\n')
  //     const userDrvxAtaBalInfoAfterSwap = await connection.getTokenAccountBalance(userDrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is :", userDrvxAtaBalInfoAfterSwap.value.uiAmount, '\n')


  //     const userUsdtAtaAfterSwap = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", userUsdtAtaAfterSwap.toBase58(), '\n')
  //     const userUsdtAtaBalInfoAfterSwap = await connection.getTokenAccountBalance(userUsdtAta)
  //     console.log("Smart Contract DrvxAta token balance is :", userUsdtAtaBalInfoAfterSwap.value.uiAmount, '\n')


  //     const feeWalletAtaAfterSwap = getAssociatedTokenAddressSync(usdtMint, feeWallet.publicKey)
  //     console.log("FeeWalletAta address:", feeWalletAtaAfterSwap.toBase58())
  //     const feeAtaInfoAfterSwap = await connection.getAccountInfo(feeWalletAta)
  //     if (feeAtaInfoAfterSwap) {
  //       const feeBalance = await connection.getTokenAccountBalance(feeWalletAta)
  //       console.log("Fee Balance:", feeBalance.value.uiAmount)
  //     } else {
  //       console.log("Fee wallet ata still not created")
  //     }
  //   } catch (error) {
  //     console.log("error in withdraw drvx Token :", error)
  //   }
  // })

  // it("User swap 70_000 Usdt token to Drvx tokens", async () => {
  //   console.log("\n\n")
  //   console.log("==============================  User swap 70000 Usdt token to Drvx tokens  ==============================  ", '\n')
  //   try {

  //     const swapUsdtToDrvxAmount = new BN(70000).mul(new BN(10 ** tokenDecimal)); // withdraw  tokens
  //     console.log("globalState:", globalState.toBase58(), '\n')
  //     console.log("Swap Amount:", swapUsdtToDrvxAmount, '\n')

  //     const scUsdtAta = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smart Contract UsdtAta:", scUsdtAta.toBase58(), '\n')
  //     const scUsdtAtaBalInfo = await connection.getTokenAccountBalance(scUsdtAta)
  //     console.log("Smart Contract UsdtAta token balance is :", scUsdtAtaBalInfo.value.uiAmount, '\n')

  //     const scDrvxAta = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", scDrvxAta.toBase58(), '\n')
  //     const scDrvxAtaBalInfo = await connection.getTokenAccountBalance(scDrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is :", scDrvxAtaBalInfo.value.uiAmount, '\n')

  //     const userDrvxAta = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", userDrvxAta.toBase58(), '\n')
  //     const userDrvxAtaBalInfo = await connection.getTokenAccountBalance(userDrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is :", userDrvxAtaBalInfo.value.uiAmount, '\n')


  //     const userUsdtAta = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", userUsdtAta.toBase58(), '\n')
  //     const userUsdtAtaBalInfo = await connection.getTokenAccountBalance(userUsdtAta)
  //     console.log("Smart Contract DrvxAta token balance is :", scDrvxAtaBalInfo.value.uiAmount, '\n')


  //     const feeWalletAta = getAssociatedTokenAddressSync(usdtMint, feeWallet.publicKey)
  //     console.log("FeeWalletAta address:", feeWalletAta.toBase58())
  //     const feeAtaInfo = await connection.getAccountInfo(feeWalletAta)
  //     if (feeAtaInfo) {
  //       const feeBalance = await connection.getTokenAccountBalance(feeWalletAta)
  //       console.log("Fee Balance:", feeBalance.value.uiAmount)
  //     } else {
  //       console.log("Fee wallet ata still not created")
  //     }

  //     const transaction = new Transaction().add(
  //       createAssociatedTokenAccountIdempotentInstruction(
  //         admin.publicKey, feeWalletAta, feeWallet.publicKey, usdtMint
  //       ),
  //       await program.methods
  //         .swapDrvxToUsdt(new BN(70_000).mul(new BN(10 ** 6)))
  //         .accounts({

  //           // admin: admin.publicKey,
  //           // feeWallet: feeWallet.publicKey,
  //           // drvxMint,
  //           // usdtMint,
  //           // user: admin.publicKey

  //           drvxMint: drvxMint,
  //           feeWallet: feeWallet.publicKey,
  //           usdtMint: usdtMint,
  //           user: admin.publicKey
  //         })
  //         .instruction()
  //     )
  //     transaction.feePayer = admin.publicKey
  //     transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  //     console.log(await connection.simulateTransaction(transaction))
  //     const sig = await sendAndConfirmTransaction(connection, transaction, [admin])
  //     console.log({ sig })





  //     const scUsdtAtaAfterSwap = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smart Contract UsdtAta:", scUsdtAtaAfterSwap.toBase58(), '\n')
  //     const scUsdtAtaBalInfoAfterSwap = await connection.getTokenAccountBalance(scUsdtAta)
  //     console.log("Smart Contract UsdtAta token balance is :", scUsdtAtaBalInfoAfterSwap.value.uiAmount, '\n')

  //     const scDrvxAtaAfterSwap = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", scDrvxAtaAfterSwap.toBase58(), '\n')
  //     const scDrvxAtaBalInfoAfterSwap = await connection.getTokenAccountBalance(scDrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is :", scDrvxAtaBalInfoAfterSwap.value.uiAmount, '\n')

  //     const userDrvxAtaAfterSwap = await getAssociatedTokenAddress(drvxMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", userDrvxAtaAfterSwap.toBase58(), '\n')
  //     const userDrvxAtaBalInfoAfterSwap = await connection.getTokenAccountBalance(userDrvxAta)
  //     console.log("Smart Contract DrvxAta token balance is :", userDrvxAtaBalInfoAfterSwap.value.uiAmount, '\n')


  //     const userUsdtAtaAfterSwap = await getAssociatedTokenAddress(usdtMint, globalState, true)
  //     console.log("Smart Contract DrvxAta:", userUsdtAtaAfterSwap.toBase58(), '\n')
  //     const userUsdtAtaBalInfoAfterSwap = await connection.getTokenAccountBalance(userUsdtAta)
  //     console.log("Smart Contract DrvxAta token balance is :", userUsdtAtaBalInfoAfterSwap.value.uiAmount, '\n')


  //     const feeWalletAtaAfterSwap = getAssociatedTokenAddressSync(usdtMint, feeWallet.publicKey)
  //     console.log("FeeWalletAta address:", feeWalletAtaAfterSwap.toBase58())
  //     const feeAtaInfoAfterSwap = await connection.getAccountInfo(feeWalletAta)
  //     if (feeAtaInfoAfterSwap) {
  //       const feeBalance = await connection.getTokenAccountBalance(feeWalletAta)
  //       console.log("Fee Balance:", feeBalance.value.uiAmount)
  //     } else {
  //       console.log("Fee wallet ata still not created")
  //     }
  //   } catch (error) {
  //     console.log("error in withdraw drvx Token :", error)
  //   }
  // })

});
