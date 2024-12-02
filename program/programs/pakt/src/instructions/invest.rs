// use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token_interface::{
//         close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
//         TransferChecked,
//     },
// };

// use crate::state::task_escrow::TaskEscrow;

// // Investor funds the task
// pub fn invest_in_task(ctx: Context<InvestInTask>) -> Result<()> {
//     let task_account = &mut ctx.accounts.task_account;
//     require!(task_account.is_active, ErrorCode::TaskNotActive);
//     require!(
//         task_account.investor == Pubkey::default(),
//         ErrorCode::AlreadyInvested
//     );

//     // Transfer funds to escrow
//     let cpi_accounts = Transfer {
//         from: ctx.accounts.investor.to_account_info(),
//         to: ctx.accounts.escrow_account.to_account_info(),
//     };
//     let cpi_program = ctx.accounts.system_program.to_account_info();
//     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//     anchor_lang::system_program::transfer(cpi_ctx, task_account.price)?;

//     task_account.investor = *ctx.accounts.investor.key;
//     task_account.start_time = Clock::get()?.unix_timestamp;
//     Ok(())
// }
