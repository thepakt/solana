use anchor_lang::prelude::*;

use crate::state::TaskEscrow;

#[derive(Accounts)]
pub struct InitializeTask<'info> {
    #[account(init, payer = maker, space = 8 + TaskEscrow::INIT_SPACE)]
    pub task_account: Account<'info, TaskEscrow>,
    #[account(mut)]
    pub maker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_task(ctx: Context<InitializeTask>, price: u64, duration: i64) -> Result<()> {
    let task_account = &mut ctx.accounts.task_account;
    task_account.maker = *ctx.accounts.maker.key;
    task_account.price = price;
    task_account.duration = duration;
    Ok(())
}
