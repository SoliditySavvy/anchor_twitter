use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, _comment_content: String) -> Result<()> {
    let _comment = &mut ctx.accounts.comment;

    require!(
        _comment_content.as_bytes().len() <= COMMENT_LENGTH,
        TwitterError::CommentTooLong
    );

    let mut comment_data = [0u8; COMMENT_LENGTH];
    comment_data[.._comment_content.as_bytes().len()].copy_from_slice(_comment_content.as_bytes());
    _comment.content = comment_data;

    _comment.comment_author = *ctx.accounts.comment_author.key;
    _comment.parent_tweet = ctx.accounts.tweet.key();
    _comment.content_length = _comment_content.as_bytes().len() as u16;
    _comment.bump = *ctx.bumps.get("comment").unwrap();

    Ok(())
}
#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    #[account(
        init,
        payer = comment_author,
        space = 8 + Comment::LEN,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            anchor_lang::solana_program::hash::hash(comment_content.as_bytes()).to_bytes().as_ref(),
            tweet.key().as_ref(),
            ],
        bump)]
    pub comment: Account<'info, Comment>,
    #[account(
        mut,
        seeds = [
            tweet.topic[..tweet.topic_length as usize].as_ref(),
            TWEET_SEED.as_bytes(),
            tweet.tweet_author.key().as_ref()
        ],
        bump = tweet.bump
    )]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
