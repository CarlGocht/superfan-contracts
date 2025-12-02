import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import {
  deriveConfigPda,
  deriveMarketCounterPda,
  deriveSponsorPda,
  hash32,
} from "../sdk/client";

module.exports = async function (provider: anchor.AnchorProvider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  const program = anchor.workspace.SuperfanContracts;

  const admin = new PublicKey(
    process.env.ADMIN ?? provider.wallet.publicKey.toBase58()
  );
  const usdcMintEnv = process.env.USDC_MINT;
  if (!usdcMintEnv) {
    throw new Error("USDC_MINT env var required for deploy migration");
  }
  const usdcMint = new PublicKey(usdcMintEnv);
  const maxSponsors = parseInt(process.env.MAX_SPONSORS ?? "100", 10);
  const sponsorName = process.env.SPONSOR_NAME ?? "Default Sponsor";

  const [configPda] = deriveConfigPda(program.programId);
  const existingConfig = await program.account.superfanConfig.fetchNullable(
    configPda
  );

  if (!existingConfig) {
    console.log("Initializing config", {
      maxSponsors,
      usdcMint: usdcMint.toBase58(),
      admin: admin.toBase58(),
    });
    await program.methods
      .initializeConfig(new anchor.BN(maxSponsors), usdcMint, admin)
      .accounts({
        config: configPda,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  } else {
    console.log("Config already initialized at", configPda.toBase58());
  }

  const [sponsorPda] = deriveSponsorPda(admin, program.programId);
  const sponsorAccount = await program.account.sponsor.fetchNullable(sponsorPda);
  const [marketCounterPda] = deriveMarketCounterPda(
    sponsorPda,
    program.programId
  );

  if (!sponsorAccount) {
    console.log("Registering sponsor", sponsorPda.toBase58());
    await program.methods
      .registerSponsor(hash32(sponsorName))
      .accounts({
        config: configPda,
        authority: admin,
        sponsor: sponsorPda,
        marketCounter: marketCounterPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  } else {
    console.log("Sponsor already registered at", sponsorPda.toBase58());
  }
};
