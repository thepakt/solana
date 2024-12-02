// use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token_interface::{
//         close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
//         TransferChecked,
//     },
// };

// use crate::state::task_escrow::TaskEscrow;

// // Confirm task completion (called by either maker or investor)
// pub fn confirm_completion(ctx: Context<ConfirmCompletion>, is_completed: bool) -> Result<()> {
//     let task_account = &mut ctx.accounts.task_account;
//     let signer = ctx.accounts.signer.key;

//     if *signer == task_account.maker {
//         task_account.is_completed = is_completed;
//     } else if *signer == task_account.investor {
//         task_account.is_completed = is_completed;
//     } else {
//         return err!(ErrorCode::Unauthorized);
//     }

//     // If both agree it's completed, pay the maker
//     if task_account.is_completed {
//         let bump = *ctx.bumps.get("escrow_account").unwrap();
//         let seeds = &[b"escrow", task_account.key().as_ref(), &[bump]];
//         let signer_seeds = &[&seeds[..]];

//         let cpi_accounts = Transfer {
//             from: ctx.accounts.escrow_account.to_account_info(),
//             to: ctx.accounts.maker.to_account_info(),
//         };
//         let cpi_ctx = CpiContext::new_with_signer(
//             ctx.accounts.system_program.to_account_info(),
//             cpi_accounts,
//             signer_seeds,
//         );
//         anchor_lang::system_program::transfer(cpi_ctx, task_account.price)?;
//         task_account.is_active = false;
//     }

//     Ok(())
// }
