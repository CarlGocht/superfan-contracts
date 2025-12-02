import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { SuperfanContracts } from "../target/types/superfan_contracts";

// Simple 32-byte hash padded/truncated from UTF-8 string
export const hash32 = (input: string): number[] => {
  const bytes = anchor.utils.bytes.utf8.encode(input);
  const out = new Array(32).fill(0);
  for (let i = 0; i < Math.min(32, bytes.length); i++) {
    out[i] = bytes[i];
  }
  return out;
};

export const deriveConfigPda = (programId: PublicKey): [PublicKey, number] =>
  PublicKey.findProgramAddressSync([Buffer.from("superfan_config")], programId);

export const deriveSponsorPda = (
  authority: PublicKey,
  programId: PublicKey
): [PublicKey, number] =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("sponsor"), authority.toBuffer()],
    programId
  );

export const deriveMarketCounterPda = (
  sponsor: PublicKey,
  programId: PublicKey
): [PublicKey, number] =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("market_counter"), sponsor.toBuffer()],
    programId
  );

export const deriveMarketPda = (
  sponsor: PublicKey,
  marketId: anchor.BN,
  programId: PublicKey
): [PublicKey, number] =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("market"), sponsor.toBuffer(), marketId.toArrayLike(Buffer, "le", 8)],
    programId
  );

export class SuperfanClient {
  readonly provider: anchor.AnchorProvider;
  readonly program: anchor.Program<SuperfanContracts>;

  constructor(provider: anchor.AnchorProvider) {
    this.provider = provider;
    anchor.setProvider(provider);
    this.program = anchor.workspace
      .SuperfanContracts as anchor.Program<SuperfanContracts>;
  }

  async initializeConfig(maxSponsors: number, usdcMint: PublicKey, admin: PublicKey) {
    const [config] = deriveConfigPda(this.program.programId);
    await this.program.methods
      .initializeConfig(new anchor.BN(maxSponsors), usdcMint, admin)
      .accounts({
        config,
        payer: this.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    return config;
  }

  async registerSponsor(authority: PublicKey, name: string) {
    const [sponsor] = deriveSponsorPda(authority, this.program.programId);
    const [marketCounter] = deriveMarketCounterPda(
      sponsor,
      this.program.programId
    );
    const [config] = deriveConfigPda(this.program.programId);

    await this.program.methods
      .registerSponsor(hash32(name))
      .accounts({
        config,
        authority,
        sponsor,
        marketCounter,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    return { sponsor, marketCounter };
  }

  async createMarket(params: {
    authority: PublicKey;
    marketId: anchor.BN;
    artistWallet: PublicKey;
    artistId: string;
    tradingStartsAt: number;
    tradingEndsAt: number;
    resolutionDeadline: number;
    convictionThresholdBps: number;
    maxPoolExposure: anchor.BN;
    liquidityPool: PublicKey;
    signalOracle: PublicKey;
  }) {
    const sponsor = deriveSponsorPda(params.authority, this.program.programId)[0];
    const [market] = deriveMarketPda(
      sponsor,
      params.marketId,
      this.program.programId
    );
    const marketCounter = deriveMarketCounterPda(
      sponsor,
      this.program.programId
    )[0];
    const [config] = deriveConfigPda(this.program.programId);

    await this.program.methods
      .createMarket(
        params.marketId,
        params.artistWallet,
        hash32(params.artistId),
        new anchor.BN(params.tradingStartsAt),
        new anchor.BN(params.tradingEndsAt),
        new anchor.BN(params.resolutionDeadline),
        params.convictionThresholdBps,
        params.maxPoolExposure,
        params.liquidityPool,
        params.signalOracle
      )
      .accounts({
        config,
        authority: params.authority,
        sponsor,
        marketCounter,
        market,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    return market;
  }

  async lockMarket(authority: PublicKey, market: PublicKey) {
    const [config] = deriveConfigPda(this.program.programId);
    const sponsor = deriveSponsorPda(authority, this.program.programId)[0];
    await this.program.methods
      .lockMarket()
      .accounts({
        config,
        authority,
        sponsor,
        market,
      })
      .rpc();
  }

  async cancelMarket(authority: PublicKey, market: PublicKey) {
    const [config] = deriveConfigPda(this.program.programId);
    const sponsor = deriveSponsorPda(authority, this.program.programId)[0];
    await this.program.methods
      .cancelMarket()
      .accounts({
        config,
        authority,
        sponsor,
        market,
      })
      .rpc();
  }

  async resolveMarket(authority: PublicKey, market: PublicKey, outcomeYes: boolean) {
    const [config] = deriveConfigPda(this.program.programId);
    const sponsor = deriveSponsorPda(authority, this.program.programId)[0];
    await this.program.methods
      .resolveMarket(outcomeYes)
      .accounts({
        config,
        authority,
        sponsor,
        market,
      })
      .rpc();
  }
}
