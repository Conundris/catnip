use rand::{Rng, thread_rng};
use std::collections::HashSet;
use serenity::{
    prelude::*,
    client::Context,
    framework::standard::{help_commands, macros::help, macros::command, Args, CommandGroup, CommandResult, HelpOptions},
    model::prelude::{Message, UserId},
    utils::MessageBuilder,
};

#[command]
/// Roll 1d20 and send a response with the result.
async fn roll20(context: &Context, msg: &Message) -> CommandResult {
    debug!("roll20 command handler called");

    // The RNG function produces values in the range 0 to 20-1, so
    // add 1 to bring the range into the expected range.
    let rolled_value: i32 = thread_rng().gen_range(0, 20) + 1;

    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author)
        .push(" rolls 1d20 with the result: ")
        .push_bold(rolled_value)
        .build();

    if let Err(why) = msg.channel_id.say(&context.http, &response).await {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[help]
#[max_levenshtein_distance(3)]
#[no_help_available_text(
"**Error**: I was unable to find any information on this command, \
    usually indicating that this command does not exist or does not have \
    any help available for said command. Please try again later, or try \
    searching for a different command instead."
)]
async fn help(
    context: &Context,
    message: &Message,
    arguments: Args,
    options: &'static HelpOptions,
    command_groups: &[&'static CommandGroup],
    bot_owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::plain(context, message, arguments, &options, command_groups, bot_owners).await;
    Ok(())
}
