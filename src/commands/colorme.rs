use crate::SerenityError;
use serenity::{
    http::Http,
    model::prelude::{GuildId, Role},
};

pub async fn set_color(color: u64, http: impl AsRef<Http>) -> Result<Role, SerenityError> {
    let guild_id = GuildId(127133903550021632);
    // let color = Colour::from(arg);
    let color_name = color.to_string();

    let newrole = guild_id
        .create_role(&http, |r| r.name(color_name).colour(color))
        .await;
    newrole

    // let roles = guild_id.roles(&http).await.unwrap();

    // let role = &roles.into_values().find(|r| r.name == color_name).unwrap();
    // guild_id
    //     .edit_role(&http, &role.id, |r| r.colour(color).name(color_name))
    //     .await
    //     .unwrap();

    // Ok(())
}
