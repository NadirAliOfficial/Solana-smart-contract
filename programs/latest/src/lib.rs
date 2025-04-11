use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("GDUYvFQJJ4UievNjB3uHA5m317UB2yReoU2canC3o7e2");

#[program]
pub mod latest {
    use super::*;

    // Transfers a given amount of lamports (SOL) from the sender to the fixed payment wallet.
    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
        // Parse the hard-coded payment wallet address into a Pubkey.
        let payment_wallet = Pubkey::from_str("8FpezRfPry8XbQMmgMHXANj5mSGn9bpHsEe6P6t9sQG")
            .expect("Invalid payment wallet address");
        
        // Create the transfer instruction using the Solana system program.
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &payment_wallet,
            amount,
        );
        // Invoke the instruction.
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.sender.to_account_info(),
                // Ensure that the provided receiver account matches the fixed payment wallet.
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: The receiver account is unchecked because it must match the payment wallet.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
