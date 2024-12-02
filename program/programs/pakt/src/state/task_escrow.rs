use anchor_lang::prelude::*;

// account that stores all the data
// can be updated continously
#[account]
#[derive(InitSpace)]
pub struct TaskEscrow {
    pub maker: Pubkey,            // Maker's public key
    pub investor: Option<Pubkey>, // Investor's public key (optional until invested)
    pub mediator: Pubkey,         // Mediator's public key (optional)
    pub price: u64,               // Price in lamports
    pub duration: i64,            // Duration in seconds (e.g., 5 days)
    pub start_time: i64,          // Timestamp when the task started
    pub is_active: bool,          // Whether the task is active
    pub is_disputed: bool,        // Whether there's a dispute
    pub is_completed: bool,       // Whether the task is completed
}
