# 💸 Solana SOL Transfer Program (Anchor)

This project is a Solana smart contract written using the Anchor framework. It allows a user to send SOL (lamports) from their wallet to another wallet by invoking a `send_sol` instruction.

---

## 📦 Project Structure

```bash
latest/
├── Anchor.toml
├── programs/
│   └── latest/
│       └── src/
│           └── lib.rs      # Main smart contract code
├── target/
│   └── idl/
│       └── latest.json     # Auto-generated IDL after build
├── migrations/
│   └── deploy.ts
├── tests/
│   └── latest.ts           # (Optional) Mocha/JS tests
└── README.md               # This file
```

---

## 🚀 Smart Contract: `send_sol`

### 🔧 Function
```rust
pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()>
```

### 📥 Inputs
- `amount: u64` — amount in **lamports** to be transferred.

### 👛 Accounts
- `sender` — Must sign the transaction and be mutable.
- `receiver` — Mutable recipient account.
- `system_program` — System program to invoke transfer.

---

## 🔨 Build the Program

```bash
anchor build
```

- This will generate the IDL in `target/idl/latest.json` and build the `.so` file needed for deployment.

---

## 📤 Deploy to Devnet

Generate a new keypair (optional):

```bash
solana-keygen new --outfile target/deploy/latest-keypair.json
```

Then deploy:

```bash
solana config set --url https://api.devnet.solana.com
solana program deploy target/deploy/latest.so
```

Copy the **program ID** and update it in `lib.rs`:

```rust
declare_id!("YOUR_PROGRAM_ID_HERE");
```

---

## 🧪 Run JS Test Script (Manual)

Create a `test.js` file in your root directory:

```bash
node test.js
```

Make sure you install dependencies first:

```bash
npm install @coral-xyz/anchor
```

---

## 💡 Notes

- Use `anchor.AnchorProvider.env()` to use the Solana CLI config wallet.
- All SOL amounts are in **lamports** (1 SOL = 1_000_000_000 lamports).
- This contract uses the system instruction to perform the transfer.

---

## 📚 Resources

- [Solana Docs](https://docs.solana.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Solana Explorer (Devnet)](https://explorer.solana.com/?cluster=devnet)
