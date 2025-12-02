
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { SuperfanContracts } from "../target/types/superfan_contracts";

describe("superfan-contracts: market factory", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .SuperfanContracts as Program<SuperfanContracts>;

  let configPda: PublicKey;
  let sponsorPda: PublicKey;
  let marketCounterPda: PublicKey;

  // Simple helper to create a 32-byte hash from a string
  const hash32 = (input: string): number[] => {
    const bytes = anchor.utils.bytes.utf8.encode(input);
    const out = new Array(32).fill(0);
    for (let i = 0; i < Math.min(32, bytes.length); i++) {
      out[i] = bytes[i];
    }
    return out;
  };

  before(async () => {
    // Derive the global config PDA
    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("superfan_config")],
      program.programId
    );

    const usdcMint = Keypair.generate().publicKey;
    const admin = provider.wallet.publicKey;

    // Initialize global config
    await program.methods
      .initializeConfig(new anchor.BN(100), usdcMint, admin)
      .accounts({
        config: configPda,
        payer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const config = await program.account.superfanConfig.fetch(configPda);
    console.log("Initialized config:", config);

    // Derive sponsor + market counter PDAs
    [sponsorPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("sponsor"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    [marketCounterPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("market_counter"), sponsorPda.toBuffer()],
      program.programId
    );

    const nameHash = hash32("Test Label");

    await program.methods
      .registerSponsor(nameHash)
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
        sponsor: sponsorPda,
        marketCounter: marketCounterPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const sponsor = await program.account.sponsor.fetch(sponsorPda);
    const counter = await program.account.sponsorMarketCounter.fetch(
      marketCounterPda
    );

    console.log("Registered sponsor:", sponsor);
    console.log("Sponsor market counter:", counter);
  });

  it("creates a market successfully", async () => {
    const now = Math.floor(Date.now() / 1000);
    const tradingStartsAt = now + 10;
    const tradingEndsAt = now + 3600;
    const resolutionDeadline = tradingEndsAt + 86400; // +1 day

    const convictionThresholdBps = 2000; // 20%
    const maxPoolExposure = new anchor.BN(25_000_000_000); // 25k with 6 decimals (example)

    const marketId = new anchor.BN(1);
    const artistWallet = Keypair.generate().publicKey;
    const artistIdHash = hash32("artist-123");

    const liquidityPool = Keypair.generate().publicKey;
    const signalOracle = Keypair.generate().publicKey;

    const [marketPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("market"),
        sponsorPda.toBuffer(),
        marketId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createMarket(
        marketId,
        artistWallet,
        artistIdHash,
        new anchor.BN(tradingStartsAt),
        new anchor.BN(tradingEndsAt),
        new anchor.BN(resolutionDeadline),
        convictionThresholdBps,
        maxPoolExposure,
        liquidityPool,
        signalOracle
      )
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
        sponsor: sponsorPda,
        marketCounter: marketCounterPda,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const market = await program.account.market.fetch(marketPda);

    console.log("Created market:", market);

    // Basic assertions
    if (!market.sponsor.equals(sponsorPda)) {
      throw new Error("Market sponsor mismatch");
    }
    if (!market.artistWallet.equals(artistWallet)) {
      throw new Error("Market artist wallet mismatch");
    }
  });

  it("locks a market after trading window", async () => {
    const now = Math.floor(Date.now() / 1000);
    // Put trading window entirely in the past
    const tradingStartsAt = now - 3600;
    const tradingEndsAt = now - 10;
    const resolutionDeadline = tradingEndsAt + 86400;

    const convictionThresholdBps = 1500;
    const maxPoolExposure = new anchor.BN(10_000_000_000);

    const marketId = new anchor.BN(2);
    const artistWallet = Keypair.generate().publicKey;
    const artistIdHash = hash32("artist-456");
    const liquidityPool = Keypair.generate().publicKey;
    const signalOracle = Keypair.generate().publicKey;

    const [marketPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("market"),
        sponsorPda.toBuffer(),
        marketId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createMarket(
        marketId,
        artistWallet,
        artistIdHash,
        new anchor.BN(tradingStartsAt),
        new anchor.BN(tradingEndsAt),
        new anchor.BN(resolutionDeadline),
        convictionThresholdBps,
        maxPoolExposure,
        liquidityPool,
        signalOracle
      )
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
        sponsor: sponsorPda,
        marketCounter: marketCounterPda,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    // Now lock the market
    await program.methods
      .lockMarket()
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
        sponsor: sponsorPda,
        market: marketPda,
      })
      .rpc();

    const market = await program.account.market.fetch(marketPda);
    console.log("Locked market:", market);

    if (market.status !== 1) {
      throw new Error("Market should be in Locked status");
    }
  });

  it("cancels a market before trading starts", async () => {
    const now = Math.floor(Date.now() / 1000);
    const tradingStartsAt = now + 3600;
    const tradingEndsAt = tradingStartsAt + 3600;
    const resolutionDeadline = tradingEndsAt + 86400;

    const convictionThresholdBps = 1000;
    const maxPoolExposure = new anchor.BN(5_000_000_000);

    const marketId = new anchor.BN(3);
    const artistWallet = Keypair.generate().publicKey;
    const artistIdHash = hash32("artist-789");
    const liquidityPool = Keypair.generate().publicKey;
    const signalOracle = Keypair.generate().publicKey;

    const [marketPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("market"),
        sponsorPda.toBuffer(),
        marketId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createMarket(
        marketId,
        artistWallet,
        artistIdHash,
        new anchor.BN(tradingStartsAt),
        new anchor.BN(tradingEndsAt),
        new anchor.BN(resolutionDeadline),
        convictionThresholdBps,
        maxPoolExposure,
        liquidityPool,
        signalOracle
      )
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
        sponsor: sponsorPda,
        marketCounter: marketCounterPda,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    await program.methods
      .cancelMarket()
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
        sponsor: sponsorPda,
        market: marketPda,
      })
      .rpc();

    const market = await program.account.market.fetch(marketPda);
    console.log("Cancelled market:", market);

    if (market.status !== 3) {
      throw new Error("Market should be in Cancelled status");
    }
  });

  it("rejects locking a market too early", async () => {
    const now = Math.floor(Date.now() / 1000);
    const tradingStartsAt = now - 60;
    const tradingEndsAt = now + 3600;
    const resolutionDeadline = tradingEndsAt + 86400;

    const convictionThresholdBps = 1200;
    const maxPoolExposure = new anchor.BN(5_000_000_000);

    const marketId = new anchor.BN(4);
    const artistWallet = Keypair.generate().publicKey;
    const artistIdHash = hash32("artist-too-early");
    const liquidityPool = Keypair.generate().publicKey;
    const signalOracle = Keypair.generate().publicKey;

    const [marketPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("market"),
        sponsorPda.toBuffer(),
        marketId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createMarket(
        marketId,
        artistWallet,
        artistIdHash,
        new anchor.BN(tradingStartsAt),
        new anchor.BN(tradingEndsAt),
        new anchor.BN(resolutionDeadline),
        convictionThresholdBps,
        maxPoolExposure,
        liquidityPool,
        signalOracle
      )
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
        sponsor: sponsorPda,
        marketCounter: marketCounterPda,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let threw = false;
    try {
      await program.methods
        .lockMarket()
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
          sponsor: sponsorPda,
          market: marketPda,
        })
        .rpc();
    } catch (err: any) {
      threw = true;
      console.log("Expected error when locking too early:", err.toString());
    }

    if (!threw) {
      throw new Error("Expected lockMarket to fail when called too early");
    }
  });
});
