import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PdaAccount } from "../target/types/pda_account";
import { PublicKey } from "@solana/web3.js";

describe("pda-account", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PdaAccount as Program<PdaAccount>;
  const user = provider.wallet as anchor.Wallet;

  // Derive the PDA address using the seeds specified on the program
  const [PDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("data"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    const transactionSignature = await program.methods
      .initialize()
      .accounts({
        user: user.publicKey
      })
      .rpc();

    console.log("Transaction Signature:", transactionSignature);
  });

  it("Fetch Account", async () => {
    const pdaAccount = await program.account.dataAccount.fetch(PDA);
    console.log(JSON.stringify(pdaAccount, null, 2));
  });
});

/*import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Zinc } from "../target/types/zinc";

describe("zinc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.zinc as Program<Zinc>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
*/