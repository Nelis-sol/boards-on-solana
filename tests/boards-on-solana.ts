import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BoardsOnSolana } from "../target/types/boards_on_solana";
import { Keypair, 
  PublicKey, 
} from "@solana/web3.js";

describe("boards-on-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BoardsOnSolana as Program<BoardsOnSolana>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const connection = anchor.getProvider().connection;


  const log_tx = async (signature: string): Promise<string> => {
    console.log(
      `Your tx: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };



  let randomInt = Math.floor(Math.random() * 100) + 1;
  const board_seed = new anchor.BN(randomInt);

  const boardPDA = PublicKey.findProgramAddressSync(
    [
      Buffer.from("board"),
      board_seed.toBuffer('le', 8),
    ],
    program.programId,
  )[0];


  // CREATE NEW FULLY DEFINED TOKEN POLICY
  it("Create new board", async () => {

    const tx = await program.methods.newBoard(
      board_seed,
      "example.com",
    )
    .accounts({
      payer: wallet.payer.publicKey,
      board: boardPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc()
    .then(log_tx);

  });


  // it("Add list to board", async () => {

  //   const tx = await program.methods.addList(
  //     board_seed,
  //     "Doing",
  //     10,
  //   )
  //   .accounts({
  //     payer: wallet.payer.publicKey,
  //     board: boardPDA,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .rpc()
  //   .then(log_tx);

  // });



  // it("Add card to board", async () => {

  //   const tx = await program.methods.addCard(
  //     board_seed,
  //     1,
  //     new anchor.BN(1000000),
  //   )
  //   .accounts({
  //     payer: wallet.payer.publicKey,
  //     board: boardPDA,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .rpc()
  //   .then(log_tx);

  // });


  // it("Move card to list", async () => {

  //   const tx = await program.methods.moveCardToList(
  //     board_seed,
  //     1,
  //     2,
  //   )
  //   .accounts({
  //     payer: wallet.payer.publicKey,
  //     board: boardPDA,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .rpc()
  //   .then(log_tx);

  // });


  // it("Delete board", async () => {

  //   const tx = await program.methods.deleteBoard(
  //     board_seed
  //   )
  //   .accounts({
  //     payer: wallet.payer.publicKey,
  //     board: boardPDA,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .rpc()
  //   .then(log_tx);

  // });


});
