import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { sentinel } from "../target/types/sentinel";
import { expect } from "chai";

describe("sentinel", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.sentinel as Program<sentinel>;

  it("registers proposal and updates risk", async () => {

    const proposalKey = anchor.web3.Keypair.generate().publicKey;

    // PDA derivations
    const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("proposal"), proposalKey.toBuffer()],
      program.programId
    );

    const [metricsPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("metrics"), proposalKey.toBuffer()],
      program.programId
    );

    const [riskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("risk"), proposalKey.toBuffer()],
      program.programId
    );

    const now = Math.floor(Date.now() / 1000);

    // 1️⃣ Register Proposal
    await program.methods
    .registerProposal(
      proposalKey,
      new anchor.BN(now),
      new anchor.BN(now + 3600)
    )
    .accounts({
      signer: provider.wallet.publicKey,
      proposalAnalysis: proposalPda,
      participationMetrics: metricsPda,
      riskSignal: riskPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    // 2️⃣ LOW risk
    await program.methods
    .updateMetrics(new anchor.BN(100), new anchor.BN(10))
    .accounts({
      signer: provider.wallet.publicKey,
      participationMetrics: metricsPda,
      riskSignal: riskPda,
    })
    .rpc();
    
    let riskAccount = await program.account.riskSignal.fetch(riskPda);
    expect(riskAccount.riskLevel).to.equal(0);

    // 3️⃣ MEDIUM risk
    await program.methods
      .updateMetrics(new anchor.BN(100), new anchor.BN(40))
      .accounts({
        signer: provider.wallet.publicKey,
        participationMetrics: metricsPda,
        riskSignal: riskPda,
      })
      .rpc();

    riskAccount = await program.account.riskSignal.fetch(riskPda);
    expect(riskAccount.riskLevel).to.equal(1);

    // 4️⃣ HIGH risk
    await program.methods
      .updateMetrics(new anchor.BN(100), new anchor.BN(80))
      .accounts({
        signer: provider.wallet.publicKey,
        participationMetrics: metricsPda,
        riskSignal: riskPda,
      })
      .rpc();

    riskAccount = await program.account.riskSignal.fetch(riskPda);
    expect(riskAccount.riskLevel).to.equal(2);
  });

});