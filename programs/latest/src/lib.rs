use anchor_lang::prelude::*;

declare_id!("GDUYvFQJJ4UievNjB3uHA5m317UB2yReoU2canC3o7e2");

#[program]
pub mod latest {
    use super::*;

    // Transfers a given amount of lamports (SOL) from the sender to the receiver.
    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
        // Create the transfer instruction using the Solana system program.
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &ctx.accounts.receiver.key(),
            amount,
        );
        // Invoke the instruction
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.sender.to_account_info(),
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
    /// CHECK: The receiver account is unchecked because we only need its public key.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
