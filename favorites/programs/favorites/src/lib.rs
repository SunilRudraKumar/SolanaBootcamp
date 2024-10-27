use anchor_lang::prelude::*;

// This line declares the ID of the program (contract) on the Solana blockchain.
// You'd typically provide the program's unique ID here after deployment.
declare_id!("");

pub const ANCHOR_DISCRIMINAOR_SIZE: usize = 8;

// Define the main program module `favorites`
#[program]
pub mod favorites {
    use super::*;

    // Define a function `set_favorites` in the `favorites` program module
    // which will be filled in later.
    pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        let favorites = &mut ctx.accounts.favorites;
        
        favorites.number = number;
        favorites.color = color;
        favorites.hobbies = hobbies;

        Ok(())
    }
}

// Define an account struct for storing user preferences, including a number, color, and hobbies.
// This struct will map to an account on the blockchain.
#[account]
pub struct Favorites {
    pub number: u64,
    
    // The color field with a max length of 50 characters.
    pub color: String,

    // The hobbies field is a vector of strings, each with a max length of 50 characters.
    pub hobbies: Vec<String>,
}

// Define the context struct `SetFavorites` for the `set_favorites` function.
// Context structs are used to define account requirements and access permissions.
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    // This is the account that pays for transaction fees and is mutable.
    // The `Signer` ensures that only the user themselves can update the account.
    #[account(mut)]
    pub user: Signer<'info>,

    // This is the `favorites` account, which stores the data for this program.
    #[account(
        init_if_needed,                        // Initialize account if it doesn’t exist.
        payer = user,                          // The user will pay for account creation.
        space = ANCHOR_DISCRIMINAOR_SIZE + 8 + 50 + (50 * 50),  // Calculate account size for favorites.
        seeds = [b"favorites", user.key().as_ref()],  // Seeds for deriving a program address (PDA).
        bump,                                  // Automatically calculates and provides the bump value.
    )]
    pub favorites: Account<'info, Favorites>,
}
