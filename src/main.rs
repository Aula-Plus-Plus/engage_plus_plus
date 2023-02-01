use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spinoff::{spinners, Color, Spinner};
use std::{collections::HashMap, fs::read_to_string};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use ureq::AgentBuilder;

#[derive(Debug, Deserialize, Serialize)]
struct Emoji {
    short_name: String,
    #[serde(flatten)]
    rubbish: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PostReaction {
    reaction: PostEmoji,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PostEmoji {
    item_id: String,
    item_type: String,
    emoji_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    object_id: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(flatten)]
    rubbish: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    posts: Vec<Post>,
    #[serde(flatten)]
    rubbish: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    university: String,
    space_id: String,
    token: String,
}

fn main() -> Result<()> {
    // Read config
    let data = read_to_string("./config.json").expect("Unable to read config.json");
    let Config {
        university,
        space_id,
        token,
    }: Config = serde_json::from_str(&data).expect("Unable to parse config.json");

    println!("Engage++: Maximising your Aula engagement since 2023!");
    println!("License: AGPLv3 and onwards");
    println!();

    let agent = AgentBuilder::new().build();

    // Grab posts
    let mut ids: Vec<String> = vec![];
    let mut time = OffsetDateTime::now_utc().format(&Rfc3339)?;
    let spinner = Spinner::new(spinners::Dots, "Fetching Posts...", Color::Blue);
    loop {
        let url = &format!(
            "https://apiv2.{university}.aula.education/posts/feed?space={space_id}&until={time}"
        );
        let call = agent.get(url).set("x-session-token", &token).call()?;
        let response: Response = call.into_json()?;
        if response.posts.is_empty() {
            break;
        }
        let mut lowest_time = OffsetDateTime::now_utc();
        for post in response.posts {
            ids.push(post.object_id);
            if lowest_time > post.created_at {
                lowest_time = post.created_at;
            }
        }
        time = lowest_time.format(&Rfc3339)?;
    }
    let posts_count = ids.len();
    spinner.success(&format!("{posts_count} posts fetched!"));

    // Grab emojis that aula uses -- webpack://sls-web-app/app/utils/emoji.js
    let spinner = Spinner::new(spinners::Dots, "Fetching Latest Emoji...", Color::Blue);
    let emojis: Vec<Emoji> = agent
        .get("https://raw.githubusercontent.com/iamcal/emoji-data/master/emoji.json")
        .call()?
        .into_json()?;
    let emoji_count = emojis.len();
    spinner.success(&format!("{emoji_count} emoji retrieved!"));

    // React to posts
    let mut total_emoji = 0;
    let mut spinner = Spinner::new(spinners::Dots, "Starting...", Color::Blue);
    for (posts_done, id) in ids.into_iter().enumerate() {
        let posts_done = posts_done + 1;
        for (emoji_done, emoji) in emojis.iter().enumerate() {
            let emoji_done = emoji_done + 1;
            spinner.update_text(format!("Reacting to post {posts_done}/{posts_count} with emoji {emoji_done}/{emoji_count}..."));
            let body = PostReaction {
                reaction: PostEmoji {
                    item_id: id.clone(),
                    item_type: String::from("UBClassRoomPost"),
                    emoji_name: emoji.short_name.clone(),
                },
            };
            let _ = agent
                .post(&format!(
                    "https://apiv2.{university}.aula.education/reactions"
                ))
                .set("x-session-token", &token)
                .send_json(body)?;
            total_emoji += 1;
        }
    }
    spinner.success(&format!(
        "Engagement complete! Added a total {total_emoji} emoji across {posts_count} posts!"
    ));

    Ok(())
}
