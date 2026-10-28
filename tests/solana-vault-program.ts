import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("solana-vault-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaVaultProgram as Program;
  const authority = provider.wallet.publicKey;
  const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), authority.toBuffer()],
    program.programId
  );

  it("initializes the authority vault", async () => {
    await program.methods.initialize().accounts({ vault, authority }).rpc();
    const account: any = await program.account.vault.fetch(vault);
    expect(account.authority.toBase58()).to.equal(authority.toBase58());
    expect(account.totalDeposited.toNumber()).to.equal(0);
  });

  it("deposits and withdraws SOL", async () => {
    await program.methods.deposit(new anchor.BN(1_000_000)).accounts({ vault, authority }).rpc();
    let account: any = await program.account.vault.fetch(vault);
    expect(account.totalDeposited.toNumber()).to.equal(1_000_000);

    await program.methods.withdraw(new anchor.BN(400_000)).accounts({ vault, authority }).rpc();
    account = await program.account.vault.fetch(vault);
    expect(account.totalWithdrawn.toNumber()).to.equal(400_000);
  });
});
