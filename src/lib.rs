mod commands;

use commands::colorme;
use log::{error, info};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use shuttle_service::error::CustomError;
use shuttle_service::SecretStore;
use sqlx::PgPool;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(127133903550021632);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("hello").description("Say hello")
                })
                .create_application_command(|command| {
                    command
                        .name("colorme")
                        .description("give yourself a color!")
                        .create_option(|option| {
                            option
                                .name("color")
                                .description("hex code of your chosen color")
                                .kind(CommandOptionType::String)
                                .required(true)
                                .min_length(6)
                                .max_length(6)
                        })
                })
        })
        .await
        .unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => "hewwo".to_owned(),
                "colorme" => {
                    let arguement = command
                        .data
                        .options
                        .iter()
                        .find(|opt| opt.name == "color")
                        .cloned();
                    let value = arguement.unwrap().value.unwrap();
                    let color = value.as_u64().unwrap();
                    let result = colorme::set_color(color, &ctx.http).await;
                    match result {
                        Ok(role) => {
                            format!("it worked i guess, role: {}", role)
                        }
                        Err(err) => {
                            format!("Err: {}", err)
                        }
                    }
                }
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[shuttle_service::main]
async fn serenity(#[shared::Postgres] pool: PgPool) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml` from the shared Postgres database
    let token = pool
        .get_secret("DISCORD_TOKEN")
        .await
        .map_err(CustomError::new)?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client)
}
