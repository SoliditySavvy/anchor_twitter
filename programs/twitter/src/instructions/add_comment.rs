use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    let comment = &mut ctx.accounts.comment;

    if comment_content.len() < COMMENT_LENGTH {
        return Err(TwitterError::CommentTooLong.into());
    }

    // Copy the comment content into the Comment account's content field. => into the bytearray.
    comment.content[..comment_content.len()].copy_from_slice(comment_content.as_bytes());

    // - comment_author
    comment.comment_author = *ctx.accounts.comment_author.to_account_info().key;
    // - parent_tweet
    comment.parent_tweet = *ctx.accounts.tweet.to_account_info().key;
    // - content_length
    comment.content_length = comment_content.len() as u16;
    // - bump
    comment.bump = *ctx.bumps.get("comment").unwrap();

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
