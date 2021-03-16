extern crate reqwest;

use super::models::*; 

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
    utils::MessageBuilder,
};

#[command]
// Cat Pic
async fn cat(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("cat command handler called.");
    debug!("Message: {:?}", msg);
    debug!("Args: {:?}", args);

    let mut resp;

    if !args.is_empty() {
        resp = get_cat(args.single::<String>().unwrap());
    } else {
        resp = get_cat(String::from("search"));
    }

    let cat = resp.await.unwrap();

    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author)
        .push(" Meow: ")
        .push(cat.url)
        .build();

    if let Err(why) = msg.channel_id.say(&context.http, &response).await {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

async fn get_cat(id: std::string::String) -> Result<models::Cat, reqwest::Error> {

    let baseurl = "https://api.thecatapi.com/v1/images/";
    let callurl;
    let resp;

    if !id.eq("search") {
        callurl = format!("{}{}", baseurl, id);
        let root : models::Cat = reqwest::get(callurl.as_str()).await?.json().await?;
        resp = root;
        debug!("{:#?}", resp);
    } else {
        callurl = format!("{}{}", baseurl, id);
        let root : models::RootCat = reqwest::get(callurl.as_str()).await?.json().await?;
        resp = root.cat;
        debug!("{:#?}", resp);
    }

    debug!("{}", callurl);

    Ok(resp)
}