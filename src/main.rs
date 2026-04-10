use dotenv::dotenv;
use serenity::prelude::*;
use std::env;

mod activity;
mod commands;
mod db;
mod events;
mod permissions;

use crate::db::{DbPoolKey, create_pool, init_schema};
use crate::events::handler::Handler;

async fn connect_database(database_url: &str) -> Option<sqlx::PgPool> {
    if let Ok(pool) = create_pool(database_url).await {
        return Some(pool);
    }

    if database_url.contains("@postgres:") {
        let local_url = database_url.replace("@postgres:", "@localhost:");
        if let Ok(pool) = create_pool(&local_url).await {
            println!(
                "DB: fallback appliqué vers localhost (DATABASE_URL utilisait l'hôte 'postgres')."
            );
            return Some(pool);
        }
    }

    None
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN non défini dans .env");
    let database_url = env::var("DATABASE_URL").ok();

    let intents = GatewayIntents::all();

    let mut client = Client::builder(bot_token, intents)
        .event_handler(Handler)
        .await
        .expect("Erreur client");

    if let Some(url) = database_url {
        match connect_database(&url).await {
            Some(pool) => {
                if let Err(err) = init_schema(&pool).await {
                    eprintln!("DB: init schema impossible: {err}");
                } else {
                    let mut data = client.data.write().await;
                    data.insert::<DbPoolKey>(pool);
                    println!("DB: connectée et prête.");
                }
            }
            None => {
                eprintln!(
                    "DB: connexion impossible, le bot démarre sans persistance (+snipe indisponible)."
                );
            }
        }
    } else {
        eprintln!("DB: DATABASE_URL non défini, démarrage sans persistance.");
    }

    if let Err(why) = client.start().await {
        println!("Erreur: {:?}", why);
    }
}
