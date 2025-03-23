const anchor = require('@coral-xyz/anchor');
const { SystemProgram, PublicKey } = anchor.web3;
const fs = require('fs');

// Load the IDL (Interface Definition Language) file
const idl = JSON.parse(fs.readFileSync('./target/idl/latest.json', 'utf8'));

// Replace with your deployed program ID
const programId = new PublicKey("GDUYvFQJJ4UievNjB3uHA5m317UB2yReoU2canC3o7e2");

// Set up the provider (use local test validator)
const provider = anchor.AnchorProvider.local("http://localhost:8899");
anchor.setProvider(provider);

// Initialize the program
const program = new anchor.Program(idl, programId, provider);

async function main() {
  console.log("Calling sendSol on the smart contract...");

  // For testing, send SOL from your wallet back to itself
  const recipient = provider.wallet.publicKey;
  const amount = new anchor.BN(1_000_000_000); // 1 SOL

  try {
    // Call the `sendSol` instruction on the smart contract
    const txSignature = await program.rpc.sendSol(amount, {
      accounts: {
        sender: provider.wallet.publicKey,
        receiver: recipient,
        systemProgram: SystemProgram.programId,
      },
    });

    console.log("Transaction successful! Signature:", txSignature);
  } catch (error) {
    console.error("Error during transaction:", error);
  }
}

// Run the test
main()
  .then(() => console.log("Test complete"))
  .catch((err) => console.error(err));