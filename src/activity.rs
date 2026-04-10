use serenity::gateway::ActivityData;
use serenity::model::prelude::OnlineStatus;
use serenity::prelude::{Context, TypeMapKey};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

pub struct ActivityTaskKey;

impl TypeMapKey for ActivityTaskKey {
    type Value = Arc<Mutex<Option<JoinHandle<()>>>>;
}

#[derive(Clone, Copy, Debug)]
pub enum RotatingActivityKind {
    Playing,
    Listening,
    Watching,
    Competing,
    Streaming,
}

impl RotatingActivityKind {
    pub fn from_command(command: &str) -> Option<Self> {
        match command {
            "+playto" => Some(Self::Playing),
            "+listen" => Some(Self::Listening),
            "+watch" => Some(Self::Watching),
            "+compet" => Some(Self::Competing),
            "+stream" => Some(Self::Streaming),
            _ => None,
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "playing" => Some(Self::Playing),
            "listening" => Some(Self::Listening),
            "watching" => Some(Self::Watching),
            "competing" => Some(Self::Competing),
            "streaming" => Some(Self::Streaming),
            _ => None,
        }
    }

    pub fn as_db(&self) -> &'static str {
        match self {
            Self::Playing => "playing",
            Self::Listening => "listening",
            Self::Watching => "watching",
            Self::Competing => "competing",
            Self::Streaming => "streaming",
        }
    }

    fn to_activity(self, message: &str) -> ActivityData {
        match self {
            Self::Playing => ActivityData::playing(message),
            Self::Listening => ActivityData::listening(message),
            Self::Watching => ActivityData::watching(message),
            Self::Competing => ActivityData::competing(message),
            Self::Streaming => ActivityData::streaming(message, "https://twitch.tv/discord")
                .unwrap_or_else(|_| ActivityData::playing(message)),
        }
    }
}

pub fn parse_status(value: &str) -> OnlineStatus {
    match value {
        "idle" => OnlineStatus::Idle,
        "dnd" => OnlineStatus::DoNotDisturb,
        "invisible" => OnlineStatus::Invisible,
        _ => OnlineStatus::Online,
    }
}

pub async fn stop_rotation(ctx: &Context) {
    let task_slot = {
        let mut data = ctx.data.write().await;
        if !data.contains_key::<ActivityTaskKey>() {
            data.insert::<ActivityTaskKey>(Arc::new(Mutex::new(None)));
        }
        data.get::<ActivityTaskKey>().cloned()
    };

    if let Some(slot) = task_slot {
        let mut guard = slot.lock().await;
        if let Some(handle) = guard.take() {
            handle.abort();
        }
    }
}

pub async fn start_rotation(
    ctx: &Context,
    kind: RotatingActivityKind,
    messages: Vec<String>,
    status: OnlineStatus,
) {
    if messages.is_empty() {
        return;
    }

    stop_rotation(ctx).await;

    let task_slot = {
        let mut data = ctx.data.write().await;
        if !data.contains_key::<ActivityTaskKey>() {
            data.insert::<ActivityTaskKey>(Arc::new(Mutex::new(None)));
        }
        data.get::<ActivityTaskKey>().cloned()
    };

    let Some(slot) = task_slot else {
        return;
    };

    let cloned_ctx = ctx.clone();
    let handle = tokio::spawn(async move {
        let mut index = 0usize;
        loop {
            let msg = &messages[index % messages.len()];
            let activity = kind.to_activity(msg);
            cloned_ctx.set_presence(Some(activity), status);
            index = (index + 1) % messages.len();
            sleep(Duration::from_secs(30)).await;
        }
    });

    let mut guard = slot.lock().await;
    *guard = Some(handle);
}
