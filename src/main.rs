use discord::Discord;

mod discord;
mod model;

#[tokio::main]
async fn main() {
    let discord = Discord {
        token: "Your Token"
    };

    //if discord.kick_user("942802258528198666", "820757138590531635").await {
    //    println!("Aravan Kicked")
    //}

    //let channels = discord.get_channels().await;

    //let guilds = discord.get_guilds().await;

    //let firends = discord.get_friends().await;

    //let usr = discord.get_info().await;

    let is_mute = discord.option_server("942802258528198666", "820757138590531635", true, true).await;
    
    if discord.kick_user("942802258528198666", "820757138590531635").await {
        discord.send_message("1061930000581918720", "fityou").await;
    }
}