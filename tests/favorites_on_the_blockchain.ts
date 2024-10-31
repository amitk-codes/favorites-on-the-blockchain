import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FavoritesOnTheBlockchain } from "../target/types/favorites_on_the_blockchain";
import * as assert from "assert";

describe("favorites_on_the_blockchain", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.FavoritesOnTheBlockchain as Program<FavoritesOnTheBlockchain>;
  const web3 = anchor.web3;
  const testUser = web3.Keypair.generate();

  // Constants for test data
  const FAVORITE_NUMBER = new anchor.BN(42);
  const FAVORITE_COLOR = "blue";
  const FAVORITE_HOBBIES = ["reading", "coding", "gaming"];

  // Helper function for deriving PDA
  const deriveFavoritesPda = async (userPublicKey: anchor.web3.PublicKey) => {
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), userPublicKey.toBuffer()],
      program.programId
    );
  };

  // Helper function for funding a test wallet
  const fundWallet = async (wallet: anchor.web3.Keypair, amount: number) => {
    const tx = await program.provider.connection.requestAirdrop(
      wallet.publicKey,
      amount
    );

    const blockHashInfo = await program.provider.connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction({
      blockhash: blockHashInfo.blockhash,
      lastValidBlockHeight: blockHashInfo.lastValidBlockHeight,
      signature: tx
    });
  };

  it("Sets the user's favorites", async () => {
    const [favoritesPda] = await deriveFavoritesPda(testUser.publicKey);
    await fundWallet(testUser, web3.LAMPORTS_PER_SOL);

    const accounts = {
      user: testUser.publicKey,
      favorites: favoritesPda,
      systemProgram: web3.SystemProgram.programId,
    }

    await program.methods
      .setFavorites(FAVORITE_NUMBER, FAVORITE_COLOR, FAVORITE_HOBBIES)
      .accounts(accounts)
      .signers([testUser])
      .rpc();

    const account = await program.account.favorites.fetch(favoritesPda);
    assert.equal(account.number.toString(), FAVORITE_NUMBER.toString());
    assert.equal(account.color, FAVORITE_COLOR);
    assert.deepEqual(account.hobbies, FAVORITE_HOBBIES);
  });
});
