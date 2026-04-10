use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::logs_service;

pub async fn handle_channel_pins_update(ctx: &Context, pin: &ChannelPinsUpdateEvent) {
    logs_service::on_channel_pins_update(ctx, pin).await;
}
