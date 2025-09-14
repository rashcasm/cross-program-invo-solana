import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProgramA } from "../target/types/program_a";

describe("program-a", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.programA as Program<ProgramA>;

  let signer = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {


    let [pda_address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pda"), signer.publicKey.toBuffer()],
      program.programId
    );

  console.log("Signer pubkey:", signer.publicKey.toBase58());
  console.log("PDA pubkey:", pda_address.toBase58());
  console.log("Program ID:", program.programId.toBase58());
  console.log("System Program:", anchor.web3.SystemProgram.programId.toBase58());
    // Add your test here.
  await airdrop(program.provider.connection, signer.publicKey, 500_000_000_000); // give signer SOL
  await airdrop(program.provider.connection, pda_address, 500_000_000_000);

    const tx = await program.methods.initialize().accounts({
      pdaAccount: pda_address,
      signer: signer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([signer]).rpc();

    console.log("Your transaction signature", tx);
  });
});


export async function airdrop(
  connection: any,
  address: any,
  ammount: 500_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, ammount),
    'confirmed'
  );
}