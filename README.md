 Direct Token Swap — Solana Program
This is a Solana smart contract (program) built with Anchor that enables users to swap USDT for NU tokens. The swap is conducted directly between user wallets and program-controlled pools, with events emitted for transparency.

📌 Features
🔁 Direct Token Swap: Users can buy NU tokens by paying in USDT at a 1:1 ratio.

💼 Token Vaults: Swaps are executed using token pool accounts for USDT and NU.

📡 Event Emission: Emits a BuyNuEvent after every successful swap.

🔐 Secure Transfers: Enforces balance checks and uses PDA-based authorities.

🛠️ Program ID
Copy
Edit
3At9UEz1bGW2ofW4twm4EBEmz6XRB22K19PubbmJGNP2
📂 Directory Structure
bash
Copy
Edit
src/
 └── lib.rs         # Main Anchor program logic
📦 Dependencies
anchor-lang

anchor-spl

🔄 Swap Flow (buy_nu)
USDT Transfer: Transfers amount_nu USDT from user to USDT pool.

NU Transfer: Transfers the same amount of NU from NU pool to the user.

Event Emitted: BuyNuEvent logs the user's address and the swap amounts.

📜 Accounts
BuyNU Context
Name	Type	Description
swapper	Signer	The user performing the swap
usdt_from	TokenAccount (mut)	User's USDT token account
usdt_pool	TokenAccount (mut)	Program’s USDT vault
nu_pool	TokenAccount (mut)	Program’s NU token vault
nu_to	TokenAccount (mut)	User's NU token account
pool_authority	AccountInfo	PDA authorized to transfer from NU pool
token_program	Program<Token>	SPL Token program

⚠️ Errors
InsufficientFunds: Triggered if the user does not have enough USDT to perform the swap.

📢 Events
BuyNuEvent
Emitted after a successful swap:

rust
Copy
Edit
pub struct BuyNuEvent {
    pub user: Pubkey,
    pub usdt_amount: u64,
    pub nu_amount: u64,
}
🚀 Usage (Client-Side)
You can use this program with any Anchor-compatible frontend or script. Ensure:

The pool token accounts and PDA authority are correctly initialized.

The PDA is seeded as expected (e.g., ["authority"]).

🧑‍💻 Contributing
Pull requests are welcome! If you find bugs or want to extend functionality (e.g., dynamic pricing), feel free to fork and contribute.

📄 License
MIT License
