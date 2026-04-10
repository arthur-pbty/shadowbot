use chrono::{DateTime, Utc};
use serenity::model::prelude::{ChannelId, GuildId, Message, MessageId, RoleId, UserId};
use serenity::prelude::TypeMapKey;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct DbPoolKey;

impl TypeMapKey for DbPoolKey {
    type Value = PgPool;
}

#[derive(Debug, Clone)]
pub struct SnipedMessage {
    pub author_id: Option<i64>,
    pub content: String,
    pub deleted_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BlacklistEntry {
    pub user_id: i64,
    pub reason: String,
    pub added_by: Option<i64>,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuditLog {
    pub id: i64,
    pub bot_id: i64,
    pub guild_id: i64,
    pub log_type: String,
    pub user_id: Option<i64>,
    pub channel_id: Option<i64>,
    pub role_id: Option<i64>,
    pub message_id: Option<i64>,
    pub action: String,
    pub details: Option<sqlx::types::JsonValue>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SentMpEntry {
    pub entry_id: i64,
    pub sender_id: i64,
    pub recipient_id: i64,
    pub dm_channel_id: i64,
    pub message_id: i64,
    pub content: String,
    pub sent_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Ticket {
    pub id: i64,
    pub bot_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub creator_id: i64,
    pub claimer_id: Option<i64>,
    pub title: String,
    pub status: String,
    pub close_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct TicketSettings {
    pub bot_id: i64,
    pub guild_id: i64,
    pub category_id: Option<i64>,
    pub log_channel_id: Option<i64>,
    pub enabled: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Suggestion {
    pub id: i64,
    pub bot_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub author_id: i64,
    pub content: String,
    pub status: String,
    pub upvotes: i64,
    pub downvotes: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct SuggestionSettings {
    pub bot_id: i64,
    pub guild_id: i64,
    pub enabled: bool,
    pub channel_id: Option<i64>,
    pub approve_channel_id: Option<i64>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct AutopublishChannel {
    pub bot_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub enabled: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct TempvocSettings {
    pub bot_id: i64,
    pub guild_id: i64,
    pub trigger_channel_id: Option<i64>,
    pub category_id: Option<i64>,
    pub enabled: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct TempvocRoom {
    pub id: i64,
    pub bot_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub owner_id: i64,
    pub created_at: DateTime<Utc>,
}

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

pub async fn init_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS message_log (
            bot_id BIGINT NOT NULL,
            message_id BIGINT NOT NULL,
            guild_id BIGINT NULL,
            channel_id BIGINT NOT NULL,
            author_id BIGINT NULL,
            content TEXT NOT NULL,
            observed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            deleted_at TIMESTAMPTZ NULL,
            PRIMARY KEY (bot_id, message_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_message_log_channel_deleted
        ON message_log (bot_id, channel_id, deleted_at DESC);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_settings (
            bot_id BIGINT PRIMARY KEY,
            embed_color INTEGER NOT NULL DEFAULT 16711680,
            status TEXT NOT NULL DEFAULT 'online',
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_activities (
            bot_id BIGINT PRIMARY KEY,
            kind TEXT NOT NULL,
            messages TEXT NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_owners (
            bot_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, user_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_blacklist (
            bot_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            reason TEXT NOT NULL,
            added_by BIGINT NULL,
            added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, user_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_blacklist_bot
        ON bot_blacklist (bot_id);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        ALTER TABLE bot_settings
        ADD COLUMN IF NOT EXISTS main_prefix TEXT NOT NULL DEFAULT '+';
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        ALTER TABLE bot_settings
        ADD COLUMN IF NOT EXISTS help_type TEXT NOT NULL DEFAULT 'button';
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        ALTER TABLE bot_settings
        ADD COLUMN IF NOT EXISTS help_aliases BOOLEAN NOT NULL DEFAULT TRUE;
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        ALTER TABLE bot_settings
        ADD COLUMN IF NOT EXISTS mp_enabled BOOLEAN NOT NULL DEFAULT TRUE;
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        ALTER TABLE bot_settings
        ADD COLUMN IF NOT EXISTS help_perms BOOLEAN NOT NULL DEFAULT TRUE;
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_guild_prefixes (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            prefix TEXT NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_command_permissions (
            bot_id BIGINT NOT NULL,
            command_name TEXT NOT NULL,
            perm_level SMALLINT NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, command_name)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_perm_level_access (
            bot_id BIGINT NOT NULL,
            scope_type TEXT NOT NULL,
            scope_id BIGINT NOT NULL,
            perm_level SMALLINT NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, scope_type, scope_id, perm_level)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_command_access (
            bot_id BIGINT NOT NULL,
            scope_type TEXT NOT NULL,
            scope_id BIGINT NOT NULL,
            command_name TEXT NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, scope_type, scope_id, command_name)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_aliases (
            bot_id BIGINT NOT NULL,
            alias_name TEXT NOT NULL,
            command_name TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, alias_name)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_sent_mp_log (
            entry_id BIGSERIAL PRIMARY KEY,
            bot_id BIGINT NOT NULL,
            sender_id BIGINT NULL,
            recipient_id BIGINT NOT NULL,
            dm_channel_id BIGINT NOT NULL,
            message_id BIGINT NOT NULL,
            content TEXT NOT NULL,
            sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            deleted_at TIMESTAMPTZ NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        ALTER TABLE bot_sent_mp_log
        ADD COLUMN IF NOT EXISTS sender_id BIGINT NULL;
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_sent_mp_log_bot_sent
        ON bot_sent_mp_log (bot_id, sent_at DESC);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_backups (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            kind TEXT NOT NULL,
            backup_name TEXT NOT NULL,
            payload JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id, kind, backup_name)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_autobackups (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            kind TEXT NOT NULL,
            interval_days INTEGER NOT NULL,
            next_run_at TIMESTAMPTZ NOT NULL,
            last_run_at TIMESTAMPTZ NULL,
            PRIMARY KEY (bot_id, guild_id, kind)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_autoreacts (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            channel_id BIGINT NOT NULL,
            emoji TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id, channel_id, emoji)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_temproles (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            role_id BIGINT NOT NULL,
            expires_at TIMESTAMPTZ NOT NULL,
            active BOOLEAN NOT NULL DEFAULT TRUE,
            added_by BIGINT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id, user_id, role_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_temproles_due
        ON bot_temproles (bot_id, guild_id, active, expires_at);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_log_channels (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            log_type TEXT NOT NULL,
            channel_id BIGINT NULL,
            enabled BOOLEAN NOT NULL DEFAULT TRUE,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id, log_type)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_log_settings (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            modlog_events TEXT NOT NULL DEFAULT 'warn,mute,tempmute,unmute,cmute,tempcmute,uncmute,kick,ban,tempban,unban,lock,unlock,hide,unhide,addrole,delrole,derank,clear,sanctions',
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_nolog_channels (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            channel_id BIGINT NOT NULL,
            disable_message BOOLEAN NOT NULL DEFAULT FALSE,
            disable_voice BOOLEAN NOT NULL DEFAULT FALSE,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id, channel_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_join_leave_settings (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            kind TEXT NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT FALSE,
            channel_id BIGINT NULL,
            custom_message TEXT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id, kind)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_boost_embed (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT TRUE,
            title TEXT NULL,
            description TEXT NULL,
            color INTEGER NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_sanctions (
            id BIGSERIAL PRIMARY KEY,
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            moderator_id BIGINT NULL,
            kind TEXT NOT NULL,
            reason TEXT NOT NULL DEFAULT 'Aucune raison',
            channel_id BIGINT NULL,
            expires_at TIMESTAMPTZ NULL,
            active BOOLEAN NOT NULL DEFAULT TRUE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_sanctions_lookup
        ON bot_sanctions (bot_id, guild_id, user_id, created_at DESC);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_sanctions_expire
        ON bot_sanctions (bot_id, guild_id, active, expires_at);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_audit_logs (
            id BIGSERIAL PRIMARY KEY,
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            log_type TEXT NOT NULL,
            user_id BIGINT NULL,
            channel_id BIGINT NULL,
            role_id BIGINT NULL,
            message_id BIGINT NULL,
            action TEXT NOT NULL,
            details JSONB NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_audit_logs_lookup
        ON bot_audit_logs (bot_id, guild_id, log_type, created_at DESC);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_audit_logs_guild
        ON bot_audit_logs (bot_id, guild_id, created_at DESC);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_tickets (
            id BIGSERIAL PRIMARY KEY,
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            channel_id BIGINT NOT NULL,
            creator_id BIGINT NOT NULL,
            claimer_id BIGINT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'open',
            close_reason TEXT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            closed_at TIMESTAMPTZ NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_tickets_lookup
        ON bot_tickets (bot_id, guild_id, status, created_at DESC);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_ticket_members (
            ticket_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (ticket_id, user_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_ticket_settings (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            category_id BIGINT NULL,
            log_channel_id BIGINT NULL,
            enabled BOOLEAN NOT NULL DEFAULT TRUE,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_suggestions (
            id BIGSERIAL PRIMARY KEY,
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            channel_id BIGINT NOT NULL,
            message_id BIGINT NOT NULL,
            author_id BIGINT NOT NULL,
            content TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            upvotes BIGINT NOT NULL DEFAULT 0,
            downvotes BIGINT NOT NULL DEFAULT 0,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            UNIQUE (bot_id, guild_id, message_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_suggestions_lookup
        ON bot_suggestions (bot_id, guild_id, status, created_at DESC);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_suggestion_settings (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT FALSE,
            channel_id BIGINT NULL,
            approve_channel_id BIGINT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_autopublish_channels (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            channel_id BIGINT NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT TRUE,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id, channel_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_tempvoc_settings (
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            trigger_channel_id BIGINT NULL,
            category_id BIGINT NULL,
            enabled BOOLEAN NOT NULL DEFAULT FALSE,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (bot_id, guild_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bot_tempvoc_rooms (
            id BIGSERIAL PRIMARY KEY,
            bot_id BIGINT NOT NULL,
            guild_id BIGINT NOT NULL,
            channel_id BIGINT NOT NULL,
            owner_id BIGINT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bot_tempvoc_rooms_lookup
        ON bot_tempvoc_rooms (bot_id, guild_id, created_at DESC);
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_main_prefix(
    pool: &PgPool,
    bot_id: UserId,
    prefix: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_settings (bot_id, main_prefix, embed_color, status)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (bot_id)
        DO UPDATE SET main_prefix = EXCLUDED.main_prefix, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(prefix)
    .bind(0xFF0000_i32)
    .bind("online")
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_main_prefix(pool: &PgPool, bot_id: UserId) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT main_prefix
        FROM bot_settings
        WHERE bot_id = $1
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(p,)| p))
}

pub async fn set_guild_prefix(
    pool: &PgPool,
    bot_id: UserId,
    guild_id: GuildId,
    prefix: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_guild_prefixes (bot_id, guild_id, prefix)
        VALUES ($1, $2, $3)
        ON CONFLICT (bot_id, guild_id)
        DO UPDATE SET prefix = EXCLUDED.prefix, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(prefix)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_guild_prefix(
    pool: &PgPool,
    bot_id: UserId,
    guild_id: GuildId,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT prefix
        FROM bot_guild_prefixes
        WHERE bot_id = $1 AND guild_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(p,)| p))
}

pub async fn set_command_permission(
    pool: &PgPool,
    bot_id: UserId,
    command_name: &str,
    perm_level: u8,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_command_permissions (bot_id, command_name, perm_level)
        VALUES ($1, $2, $3)
        ON CONFLICT (bot_id, command_name)
        DO UPDATE SET perm_level = EXCLUDED.perm_level, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(command_name)
    .bind(i16::from(perm_level))
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_command_permission(
    pool: &PgPool,
    bot_id: UserId,
    command_name: &str,
) -> Result<Option<u8>, sqlx::Error> {
    let row = sqlx::query_as::<_, (i16,)>(
        r#"
        SELECT perm_level
        FROM bot_command_permissions
        WHERE bot_id = $1 AND command_name = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(command_name)
    .fetch_optional(pool)
    .await?;

    Ok(row.and_then(|(p,)| u8::try_from(p).ok()))
}

pub async fn reset_command_permissions(pool: &PgPool, bot_id: UserId) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        DELETE FROM bot_command_permissions
        WHERE bot_id = $1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn grant_perm_level(
    pool: &PgPool,
    bot_id: UserId,
    scope_type: &str,
    scope_id: u64,
    perm_level: u8,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_perm_level_access (bot_id, scope_type, scope_id, perm_level)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (bot_id, scope_type, scope_id, perm_level)
        DO UPDATE SET updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(scope_type)
    .bind(scope_id as i64)
    .bind(i16::from(perm_level))
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn grant_command_access(
    pool: &PgPool,
    bot_id: UserId,
    scope_type: &str,
    scope_id: u64,
    command_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_command_access (bot_id, scope_type, scope_id, command_name)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (bot_id, scope_type, scope_id, command_name)
        DO UPDATE SET updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(scope_type)
    .bind(scope_id as i64)
    .bind(command_name)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_scope_permissions(
    pool: &PgPool,
    bot_id: UserId,
    scope_type: &str,
    scope_id: u64,
) -> Result<u64, sqlx::Error> {
    let res1 = sqlx::query(
        r#"
        DELETE FROM bot_perm_level_access
        WHERE bot_id = $1 AND scope_type = $2 AND scope_id = $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(scope_type)
    .bind(scope_id as i64)
    .execute(pool)
    .await?;

    let res2 = sqlx::query(
        r#"
        DELETE FROM bot_command_access
        WHERE bot_id = $1 AND scope_type = $2 AND scope_id = $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(scope_type)
    .bind(scope_id as i64)
    .execute(pool)
    .await?;

    Ok(res1.rows_affected() + res2.rows_affected())
}

pub async fn clear_role_permissions(pool: &PgPool, bot_id: UserId) -> Result<u64, sqlx::Error> {
    let res1 = sqlx::query(
        r#"
        DELETE FROM bot_perm_level_access
        WHERE bot_id = $1 AND scope_type = 'role';
        "#,
    )
    .bind(bot_id.get() as i64)
    .execute(pool)
    .await?;

    let res2 = sqlx::query(
        r#"
        DELETE FROM bot_command_access
        WHERE bot_id = $1 AND scope_type = 'role';
        "#,
    )
    .bind(bot_id.get() as i64)
    .execute(pool)
    .await?;

    Ok(res1.rows_affected() + res2.rows_affected())
}

pub async fn list_role_scopes(pool: &PgPool, bot_id: UserId) -> Result<Vec<i64>, sqlx::Error> {
    let rows1 = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT DISTINCT scope_id
        FROM bot_perm_level_access
        WHERE bot_id = $1 AND scope_type = 'role';
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_all(pool)
    .await?;

    let rows2 = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT DISTINCT scope_id
        FROM bot_command_access
        WHERE bot_id = $1 AND scope_type = 'role';
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_all(pool)
    .await?;

    let mut out = rows1.into_iter().map(|(id,)| id).collect::<Vec<_>>();
    for (id,) in rows2 {
        if !out.contains(&id) {
            out.push(id);
        }
    }
    out.sort_unstable();
    Ok(out)
}

pub async fn list_role_perm_levels(
    pool: &PgPool,
    bot_id: UserId,
    role_id: u64,
) -> Result<Vec<u8>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (i16,)>(
        r#"
        SELECT perm_level
        FROM bot_perm_level_access
        WHERE bot_id = $1 AND scope_type = 'role' AND scope_id = $2
        ORDER BY perm_level DESC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(role_id as i64)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .filter_map(|(p,)| u8::try_from(p).ok())
        .collect())
}

pub async fn list_role_command_access(
    pool: &PgPool,
    bot_id: UserId,
    role_id: u64,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT command_name
        FROM bot_command_access
        WHERE bot_id = $1 AND scope_type = 'role' AND scope_id = $2
        ORDER BY command_name ASC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(role_id as i64)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(c,)| c).collect())
}

pub async fn has_perm_level_access(
    pool: &PgPool,
    bot_id: UserId,
    user_id: UserId,
    role_ids: &[RoleId],
    perm_level: u8,
) -> Result<bool, sqlx::Error> {
    let mut scopes = vec![("user".to_string(), user_id.get() as i64)];
    for role_id in role_ids {
        scopes.push(("role".to_string(), role_id.get() as i64));
    }

    for (scope_type, scope_id) in scopes {
        let row = sqlx::query_as::<_, (i64,)>(
            r#"
            SELECT 1
            FROM bot_perm_level_access
            WHERE bot_id = $1
              AND scope_type = $2
              AND scope_id = $3
              AND perm_level = $4
            LIMIT 1;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(scope_type)
        .bind(scope_id)
        .bind(i16::from(perm_level))
        .fetch_optional(pool)
        .await?;

        if row.is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}

pub async fn has_command_access(
    pool: &PgPool,
    bot_id: UserId,
    user_id: UserId,
    role_ids: &[RoleId],
    command_name: &str,
) -> Result<bool, sqlx::Error> {
    let mut scopes = vec![("user".to_string(), user_id.get() as i64)];
    for role_id in role_ids {
        scopes.push(("role".to_string(), role_id.get() as i64));
    }

    for (scope_type, scope_id) in scopes {
        let row = sqlx::query_as::<_, (i64,)>(
            r#"
            SELECT 1
            FROM bot_command_access
            WHERE bot_id = $1
              AND scope_type = $2
              AND scope_id = $3
              AND command_name = $4
            LIMIT 1;
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(scope_type)
        .bind(scope_id)
        .bind(command_name)
        .fetch_optional(pool)
        .await?;

        if row.is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}

pub async fn set_bot_theme(pool: &PgPool, bot_id: UserId, color: u32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_settings (bot_id, embed_color)
        VALUES ($1, $2)
        ON CONFLICT (bot_id)
        DO UPDATE SET embed_color = EXCLUDED.embed_color, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(color as i32)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_bot_theme(pool: &PgPool, bot_id: UserId) -> Result<Option<u32>, sqlx::Error> {
    let row = sqlx::query_as::<_, (i32,)>(
        r#"
        SELECT embed_color
        FROM bot_settings
        WHERE bot_id = $1
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(c,)| c.max(0) as u32))
}

pub async fn set_bot_status(
    pool: &PgPool,
    bot_id: UserId,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_settings (bot_id, status, embed_color)
        VALUES ($1, $2, $3)
        ON CONFLICT (bot_id)
        DO UPDATE SET status = EXCLUDED.status, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(status)
    .bind(0xFF0000_i32)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_bot_status(pool: &PgPool, bot_id: UserId) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT status
        FROM bot_settings
        WHERE bot_id = $1
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(s,)| s))
}

pub async fn set_help_type(
    pool: &PgPool,
    bot_id: UserId,
    help_type: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_settings (bot_id, help_type, embed_color, status, help_aliases, mp_enabled)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (bot_id)
        DO UPDATE SET help_type = EXCLUDED.help_type, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(help_type)
    .bind(0xFF0000_i32)
    .bind("online")
    .bind(true)
    .bind(true)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_help_type(pool: &PgPool, bot_id: UserId) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT help_type
        FROM bot_settings
        WHERE bot_id = $1
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(value,)| value))
}

pub async fn set_help_aliases_enabled(
    pool: &PgPool,
    bot_id: UserId,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_settings (bot_id, help_aliases, embed_color, status, mp_enabled)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (bot_id)
        DO UPDATE SET help_aliases = EXCLUDED.help_aliases, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(enabled)
    .bind(0xFF0000_i32)
    .bind("online")
    .bind(true)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_help_aliases_enabled(
    pool: &PgPool,
    bot_id: UserId,
) -> Result<Option<bool>, sqlx::Error> {
    let row = sqlx::query_as::<_, (bool,)>(
        r#"
        SELECT help_aliases
        FROM bot_settings
        WHERE bot_id = $1
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(value,)| value))
}

pub async fn set_help_perms_enabled(
    pool: &PgPool,
    bot_id: UserId,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_settings (bot_id, help_perms, embed_color, status, help_aliases)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (bot_id)
        DO UPDATE SET help_perms = EXCLUDED.help_perms, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(enabled)
    .bind(0xFF0000_i32)
    .bind("online")
    .bind(true)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_help_perms_enabled(
    pool: &PgPool,
    bot_id: UserId,
) -> Result<Option<bool>, sqlx::Error> {
    let row = sqlx::query_as::<_, (bool,)>(
        r#"
        SELECT help_perms
        FROM bot_settings
        WHERE bot_id = $1
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(value,)| value))
}

pub async fn set_mp_enabled(
    pool: &PgPool,
    bot_id: UserId,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_settings (bot_id, mp_enabled, embed_color, status, help_aliases)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (bot_id)
        DO UPDATE SET mp_enabled = EXCLUDED.mp_enabled, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(enabled)
    .bind(0xFF0000_i32)
    .bind("online")
    .bind(true)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_mp_enabled(pool: &PgPool, bot_id: UserId) -> Result<Option<bool>, sqlx::Error> {
    let row = sqlx::query_as::<_, (bool,)>(
        r#"
        SELECT mp_enabled
        FROM bot_settings
        WHERE bot_id = $1
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(value,)| value))
}

pub async fn set_command_alias(
    pool: &PgPool,
    bot_id: UserId,
    alias_name: &str,
    command_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_aliases (bot_id, alias_name, command_name)
        VALUES ($1, $2, $3)
        ON CONFLICT (bot_id, alias_name)
        DO UPDATE SET command_name = EXCLUDED.command_name, created_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(alias_name)
    .bind(command_name)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_command_alias(
    pool: &PgPool,
    bot_id: UserId,
    alias_name: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT command_name
        FROM bot_aliases
        WHERE bot_id = $1 AND alias_name = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(alias_name)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(value,)| value))
}

pub async fn remove_command_alias(
    pool: &PgPool,
    bot_id: UserId,
    alias_name: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        DELETE FROM bot_aliases
        WHERE bot_id = $1 AND alias_name = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(alias_name)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn list_command_aliases(
    pool: &PgPool,
    bot_id: UserId,
) -> Result<Vec<(String, String)>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT alias_name, command_name
        FROM bot_aliases
        WHERE bot_id = $1
        ORDER BY alias_name ASC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn log_sent_mp_message(
    pool: &PgPool,
    bot_id: UserId,
    sender_id: UserId,
    recipient_id: UserId,
    dm_channel_id: ChannelId,
    message_id: MessageId,
    content: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_sent_mp_log (bot_id, sender_id, recipient_id, dm_channel_id, message_id, content)
        VALUES ($1, $2, $3, $4, $5, $6);
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(sender_id.get() as i64)
    .bind(recipient_id.get() as i64)
    .bind(dm_channel_id.get() as i64)
    .bind(message_id.get() as i64)
    .bind(content)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn count_sent_mp_messages(pool: &PgPool, bot_id: UserId) -> Result<i64, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM bot_sent_mp_log
        WHERE bot_id = $1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

pub async fn list_sent_mp_messages(
    pool: &PgPool,
    bot_id: UserId,
    limit: i64,
    offset: i64,
) -> Result<Vec<SentMpEntry>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (i64, Option<i64>, i64, i64, i64, String, DateTime<Utc>, Option<DateTime<Utc>>)>(
        r#"
        SELECT entry_id, sender_id, recipient_id, dm_channel_id, message_id, content, sent_at, deleted_at
        FROM bot_sent_mp_log
        WHERE bot_id = $1
        ORDER BY sent_at DESC, entry_id DESC
        LIMIT $2 OFFSET $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(
                entry_id,
                sender_id,
                recipient_id,
                dm_channel_id,
                message_id,
                content,
                sent_at,
                deleted_at,
            )| SentMpEntry {
                entry_id,
                sender_id: sender_id.unwrap_or_default(),
                recipient_id,
                dm_channel_id,
                message_id,
                content,
                sent_at,
                deleted_at,
            },
        )
        .collect())
}

pub async fn get_sent_mp_message(
    pool: &PgPool,
    bot_id: UserId,
    entry_id: i64,
) -> Result<Option<SentMpEntry>, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64, Option<i64>, i64, i64, i64, String, DateTime<Utc>, Option<DateTime<Utc>>)>(
        r#"
        SELECT entry_id, sender_id, recipient_id, dm_channel_id, message_id, content, sent_at, deleted_at
        FROM bot_sent_mp_log
        WHERE bot_id = $1 AND entry_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(entry_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(
        |(
            entry_id,
            sender_id,
            recipient_id,
            dm_channel_id,
            message_id,
            content,
            sent_at,
            deleted_at,
        )| SentMpEntry {
            entry_id,
            sender_id: sender_id.unwrap_or_default(),
            recipient_id,
            dm_channel_id,
            message_id,
            content,
            sent_at,
            deleted_at,
        },
    ))
}

pub async fn mark_sent_mp_deleted(
    pool: &PgPool,
    bot_id: UserId,
    entry_id: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        UPDATE bot_sent_mp_log
        SET deleted_at = NOW()
        WHERE bot_id = $1 AND entry_id = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(entry_id)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn mark_sent_mp_deleted_by_message(
    pool: &PgPool,
    bot_id: UserId,
    dm_channel_id: ChannelId,
    message_id: MessageId,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        UPDATE bot_sent_mp_log
        SET deleted_at = NOW()
        WHERE bot_id = $1 AND dm_channel_id = $2 AND message_id = $3;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(dm_channel_id.get() as i64)
    .bind(message_id.get() as i64)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn set_bot_activity(
    pool: &PgPool,
    bot_id: UserId,
    kind: &str,
    messages: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_activities (bot_id, kind, messages)
        VALUES ($1, $2, $3)
        ON CONFLICT (bot_id)
        DO UPDATE SET kind = EXCLUDED.kind, messages = EXCLUDED.messages, updated_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(kind)
    .bind(messages)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_bot_activity(
    pool: &PgPool,
    bot_id: UserId,
) -> Result<Option<(String, String)>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT kind, messages
        FROM bot_activities
        WHERE bot_id = $1
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn clear_bot_activity(pool: &PgPool, bot_id: UserId) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM bot_activities
        WHERE bot_id = $1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_bot_owner(
    pool: &PgPool,
    bot_id: UserId,
    user_id: UserId,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        DELETE FROM bot_owners
        WHERE bot_id = $1 AND user_id = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(user_id.get() as i64)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn clear_bot_owners(pool: &PgPool, bot_id: UserId) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        DELETE FROM bot_owners
        WHERE bot_id = $1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn list_bot_owners(pool: &PgPool, bot_id: UserId) -> Result<Vec<i64>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT user_id
        FROM bot_owners
        WHERE bot_id = $1
        ORDER BY added_at ASC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(id,)| id).collect())
}

pub async fn is_bot_owner(
    pool: &PgPool,
    bot_id: UserId,
    user_id: UserId,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT 1
        FROM bot_owners
        WHERE bot_id = $1 AND user_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(user_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}

pub async fn add_to_blacklist(
    pool: &PgPool,
    bot_id: UserId,
    user_id: UserId,
    reason: &str,
    added_by: Option<UserId>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_blacklist (bot_id, user_id, reason, added_by)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (bot_id, user_id)
        DO UPDATE SET reason = EXCLUDED.reason, added_by = EXCLUDED.added_by, added_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(user_id.get() as i64)
    .bind(reason)
    .bind(added_by.map(|u| u.get() as i64))
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_from_blacklist(
    pool: &PgPool,
    bot_id: UserId,
    user_id: UserId,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        DELETE FROM bot_blacklist
        WHERE bot_id = $1 AND user_id = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(user_id.get() as i64)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn clear_blacklist(pool: &PgPool, bot_id: UserId) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        DELETE FROM bot_blacklist
        WHERE bot_id = $1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn is_blacklisted(
    pool: &PgPool,
    bot_id: UserId,
    user_id: UserId,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT 1
        FROM bot_blacklist
        WHERE bot_id = $1 AND user_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(user_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}

pub async fn list_blacklist(
    pool: &PgPool,
    bot_id: UserId,
) -> Result<Vec<BlacklistEntry>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (i64, String, Option<i64>, DateTime<Utc>)>(
        r#"
        SELECT user_id, reason, added_by, added_at
        FROM bot_blacklist
        WHERE bot_id = $1
        ORDER BY added_at DESC;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(user_id, reason, added_by, added_at)| BlacklistEntry {
            user_id,
            reason,
            added_by,
            added_at,
        })
        .collect())
}

pub async fn get_blacklist_info(
    pool: &PgPool,
    bot_id: UserId,
    user_id: UserId,
) -> Result<Option<BlacklistEntry>, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64, String, Option<i64>, DateTime<Utc>)>(
        r#"
        SELECT user_id, reason, added_by, added_at
        FROM bot_blacklist
        WHERE bot_id = $1 AND user_id = $2
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(user_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(uid, reason, added_by, added_at)| BlacklistEntry {
        user_id: uid,
        reason,
        added_by,
        added_at,
    }))
}

pub async fn list_blacklisted_ids(
    pool: &PgPool,
    bot_id: UserId,
) -> Result<Vec<UserId>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT user_id
        FROM bot_blacklist
        WHERE bot_id = $1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .filter_map(|(id,)| u64::try_from(id).ok().map(UserId::new))
        .collect())
}

pub async fn upsert_message_observed(
    pool: &PgPool,
    bot_id: UserId,
    msg: &Message,
) -> Result<(), sqlx::Error> {
    let guild_id = msg.guild_id.map(|id| id.get() as i64);

    sqlx::query(
        r#"
        INSERT INTO message_log (bot_id, message_id, guild_id, channel_id, author_id, content)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (bot_id, message_id)
        DO UPDATE SET
            guild_id = EXCLUDED.guild_id,
            channel_id = EXCLUDED.channel_id,
            author_id = EXCLUDED.author_id,
            content = EXCLUDED.content,
            observed_at = NOW();
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(msg.id.get() as i64)
    .bind(guild_id)
    .bind(msg.channel_id.get() as i64)
    .bind(msg.author.id.get() as i64)
    .bind(msg.content.clone())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn mark_message_deleted(
    pool: &PgPool,
    bot_id: UserId,
    guild_id: Option<GuildId>,
    channel_id: ChannelId,
    message_id: MessageId,
    fallback_author_id: Option<UserId>,
    fallback_content: Option<String>,
) -> Result<(), sqlx::Error> {
    let updated = sqlx::query(
        r#"
        UPDATE message_log
        SET deleted_at = NOW()
        WHERE bot_id = $1 AND message_id = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(message_id.get() as i64)
    .execute(pool)
    .await?;

    if updated.rows_affected() == 0 {
        sqlx::query(
            r#"
            INSERT INTO message_log (
                bot_id, message_id, guild_id, channel_id, author_id, content, observed_at, deleted_at
            ) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            ON CONFLICT (bot_id, message_id)
            DO UPDATE SET deleted_at = NOW();
            "#,
        )
        .bind(bot_id.get() as i64)
        .bind(message_id.get() as i64)
        .bind(guild_id.map(|id| id.get() as i64))
        .bind(channel_id.get() as i64)
        .bind(fallback_author_id.map(|id| id.get() as i64))
        .bind(fallback_content.unwrap_or_else(|| "[contenu indisponible]".to_string()))
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get_last_deleted_in_channel(
    pool: &PgPool,
    bot_id: UserId,
    channel_id: ChannelId,
) -> Result<Option<SnipedMessage>, sqlx::Error> {
    let row = sqlx::query_as::<_, (Option<i64>, String, DateTime<Utc>)>(
        r#"
        SELECT author_id, content, deleted_at
        FROM message_log
        WHERE bot_id = $1
          AND channel_id = $2
          AND deleted_at IS NOT NULL
        ORDER BY deleted_at DESC
        LIMIT 1;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(channel_id.get() as i64)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(author_id, content, deleted_at)| SnipedMessage {
        author_id,
        content,
        deleted_at,
    }))
}

pub async fn insert_audit_log(
    pool: &PgPool,
    bot_id: UserId,
    guild_id: GuildId,
    log_type: &str,
    user_id: Option<UserId>,
    channel_id: Option<ChannelId>,
    role_id: Option<RoleId>,
    message_id: Option<MessageId>,
    action: &str,
    details: Option<sqlx::types::JsonValue>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO bot_audit_logs (
            bot_id, guild_id, log_type, user_id, channel_id, 
            role_id, message_id, action, details, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())
        RETURNING id;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(log_type)
    .bind(user_id.map(|u| u.get() as i64))
    .bind(channel_id.map(|c| c.get() as i64))
    .bind(role_id.map(|r| r.get() as i64))
    .bind(message_id.map(|m| m.get() as i64))
    .bind(action)
    .bind(details)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn get_audit_logs(
    pool: &PgPool,
    bot_id: UserId,
    guild_id: GuildId,
    limit: i64,
    offset: i64,
) -> Result<Vec<AuditLog>, sqlx::Error> {
    let rows = sqlx::query_as::<
        _,
        (
            i64,
            i64,
            i64,
            String,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            String,
            Option<sqlx::types::JsonValue>,
            DateTime<Utc>,
        ),
    >(
        r#"
        SELECT id, bot_id, guild_id, log_type, user_id, channel_id, 
               role_id, message_id, action, details, created_at
        FROM bot_audit_logs
        WHERE bot_id = $1 AND guild_id = $2
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                bot_id,
                guild_id,
                log_type,
                user_id,
                channel_id,
                role_id,
                message_id,
                action,
                details,
                created_at,
            )| AuditLog {
                id,
                bot_id,
                guild_id,
                log_type,
                user_id,
                channel_id,
                role_id,
                message_id,
                action,
                details,
                created_at,
            },
        )
        .collect())
}

pub async fn count_audit_logs(
    pool: &PgPool,
    bot_id: UserId,
    guild_id: GuildId,
) -> Result<i64, sqlx::Error> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) FROM bot_audit_logs
        WHERE bot_id = $1 AND guild_id = $2;
        "#,
    )
    .bind(bot_id.get() as i64)
    .bind(guild_id.get() as i64)
    .fetch_one(pool)
    .await?;

    Ok(count)
}

// ========== TICKET FUNCTIONS ==========

pub async fn create_ticket(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    channel_id: i64,
    creator_id: i64,
    title: String,
) -> Result<Ticket, sqlx::Error> {
    let ticket = sqlx::query_as::<_, Ticket>(
        r#"
        INSERT INTO bot_tickets (bot_id, guild_id, channel_id, creator_id, title, status)
        VALUES ($1, $2, $3, $4, $5, 'open')
        RETURNING *;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .bind(channel_id)
    .bind(creator_id)
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(ticket)
}

#[allow(dead_code)]
pub async fn get_ticket(pool: &PgPool, ticket_id: i64) -> Result<Option<Ticket>, sqlx::Error> {
    let ticket = sqlx::query_as::<_, Ticket>(
        r#"
        SELECT * FROM bot_tickets WHERE id = $1;
        "#,
    )
    .bind(ticket_id)
    .fetch_optional(pool)
    .await?;

    Ok(ticket)
}

pub async fn claim_ticket(
    pool: &PgPool,
    ticket_id: i64,
    claimer_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE bot_tickets SET claimer_id = $1 WHERE id = $2;
        "#,
    )
    .bind(claimer_id)
    .bind(ticket_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn close_ticket(
    pool: &PgPool,
    ticket_id: i64,
    close_reason: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE bot_tickets SET status = 'closed', close_reason = $1, closed_at = NOW() WHERE id = $2;
        "#,
    )
    .bind(close_reason)
    .bind(ticket_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_ticket_by_channel(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    channel_id: i64,
) -> Result<Option<Ticket>, sqlx::Error> {
    let ticket = sqlx::query_as::<_, Ticket>(
        r#"
        SELECT * FROM bot_tickets
        WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3
        LIMIT 1;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .bind(channel_id)
    .fetch_optional(pool)
    .await?;

    Ok(ticket)
}

pub async fn update_ticket_title(
    pool: &PgPool,
    ticket_id: i64,
    title: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE bot_tickets SET title = $1 WHERE id = $2;
        "#,
    )
    .bind(title)
    .bind(ticket_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn add_ticket_member(
    pool: &PgPool,
    ticket_id: i64,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_ticket_members (ticket_id, user_id)
        VALUES ($1, $2)
        ON CONFLICT (ticket_id, user_id) DO NOTHING;
        "#,
    )
    .bind(ticket_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_ticket_member(
    pool: &PgPool,
    ticket_id: i64,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM bot_ticket_members WHERE ticket_id = $1 AND user_id = $2;
        "#,
    )
    .bind(ticket_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_guild_tickets(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Ticket>, sqlx::Error> {
    let tickets = sqlx::query_as::<_, Ticket>(
        r#"
        SELECT * FROM bot_tickets 
        WHERE bot_id = $1 AND guild_id = $2
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(tickets)
}

pub async fn get_or_create_ticket_settings(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
) -> Result<TicketSettings, sqlx::Error> {
    let settings = sqlx::query_as::<_, TicketSettings>(
        r#"
        INSERT INTO bot_ticket_settings (bot_id, guild_id)
        VALUES ($1, $2)
        ON CONFLICT (bot_id, guild_id) DO UPDATE SET updated_at = NOW()
        RETURNING *;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .fetch_one(pool)
    .await?;

    Ok(settings)
}

pub async fn update_ticket_settings(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    category_id: Option<i64>,
    log_channel_id: Option<i64>,
    enabled: bool,
) -> Result<TicketSettings, sqlx::Error> {
    let settings = sqlx::query_as::<_, TicketSettings>(
        r#"
        UPDATE bot_ticket_settings 
        SET category_id = $1, log_channel_id = $2, enabled = $3, updated_at = NOW()
        WHERE bot_id = $4 AND guild_id = $5
        RETURNING *;
        "#,
    )
    .bind(category_id)
    .bind(log_channel_id)
    .bind(enabled)
    .bind(bot_id)
    .bind(guild_id)
    .fetch_one(pool)
    .await?;

    Ok(settings)
}

// ========== SUGGESTION FUNCTIONS ==========

pub async fn create_suggestion(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    channel_id: i64,
    message_id: i64,
    author_id: i64,
    content: String,
) -> Result<Suggestion, sqlx::Error> {
    let suggestion = sqlx::query_as::<_, Suggestion>(
        r#"
        INSERT INTO bot_suggestions (bot_id, guild_id, channel_id, message_id, author_id, content, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'pending')
        RETURNING *;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .bind(channel_id)
    .bind(message_id)
    .bind(author_id)
    .bind(content)
    .fetch_one(pool)
    .await?;

    Ok(suggestion)
}

#[allow(dead_code)]
pub async fn get_suggestion(
    pool: &PgPool,
    suggestion_id: i64,
) -> Result<Option<Suggestion>, sqlx::Error> {
    let suggestion = sqlx::query_as::<_, Suggestion>(
        r#"
        SELECT * FROM bot_suggestions WHERE id = $1;
        "#,
    )
    .bind(suggestion_id)
    .fetch_optional(pool)
    .await?;

    Ok(suggestion)
}

#[allow(dead_code)]
pub async fn update_suggestion_votes(
    pool: &PgPool,
    suggestion_id: i64,
    upvotes: i64,
    downvotes: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE bot_suggestions SET upvotes = $1, downvotes = $2 WHERE id = $3;
        "#,
    )
    .bind(upvotes)
    .bind(downvotes)
    .bind(suggestion_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn update_suggestion_status(
    pool: &PgPool,
    suggestion_id: i64,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE bot_suggestions SET status = $1 WHERE id = $2;
        "#,
    )
    .bind(status)
    .bind(suggestion_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn get_guild_suggestions(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Suggestion>, sqlx::Error> {
    let suggestions = sqlx::query_as::<_, Suggestion>(
        r#"
        SELECT * FROM bot_suggestions 
        WHERE bot_id = $1 AND guild_id = $2
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(suggestions)
}

pub async fn get_or_create_suggestion_settings(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
) -> Result<SuggestionSettings, sqlx::Error> {
    let settings = sqlx::query_as::<_, SuggestionSettings>(
        r#"
        INSERT INTO bot_suggestion_settings (bot_id, guild_id)
        VALUES ($1, $2)
        ON CONFLICT (bot_id, guild_id) DO UPDATE SET updated_at = NOW()
        RETURNING *;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .fetch_one(pool)
    .await?;

    Ok(settings)
}

pub async fn update_suggestion_settings(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    enabled: bool,
    channel_id: Option<i64>,
    approve_channel_id: Option<i64>,
) -> Result<SuggestionSettings, sqlx::Error> {
    let settings = sqlx::query_as::<_, SuggestionSettings>(
        r#"
        UPDATE bot_suggestion_settings 
        SET enabled = $1, channel_id = $2, approve_channel_id = $3, updated_at = NOW()
        WHERE bot_id = $4 AND guild_id = $5
        RETURNING *;
        "#,
    )
    .bind(enabled)
    .bind(channel_id)
    .bind(approve_channel_id)
    .bind(bot_id)
    .bind(guild_id)
    .fetch_one(pool)
    .await?;

    Ok(settings)
}

pub async fn add_autopublish_channel(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    channel_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bot_autopublish_channels (bot_id, guild_id, channel_id, enabled)
        VALUES ($1, $2, $3, TRUE)
        ON CONFLICT (bot_id, guild_id, channel_id) DO UPDATE SET enabled = TRUE;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .bind(channel_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_autopublish_channel(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    channel_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM bot_autopublish_channels 
        WHERE bot_id = $1 AND guild_id = $2 AND channel_id = $3;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .bind(channel_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_autopublish_channels(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
) -> Result<Vec<AutopublishChannel>, sqlx::Error> {
    let channels = sqlx::query_as::<_, AutopublishChannel>(
        r#"
        SELECT * FROM bot_autopublish_channels 
        WHERE bot_id = $1 AND guild_id = $2 AND enabled = TRUE;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .fetch_all(pool)
    .await?;

    Ok(channels)
}

// ========== TEMPVOC FUNCTIONS ==========

pub async fn get_or_create_tempvoc_settings(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
) -> Result<TempvocSettings, sqlx::Error> {
    let settings = sqlx::query_as::<_, TempvocSettings>(
        r#"
        INSERT INTO bot_tempvoc_settings (bot_id, guild_id)
        VALUES ($1, $2)
        ON CONFLICT (bot_id, guild_id) DO UPDATE SET updated_at = NOW()
        RETURNING *;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .fetch_one(pool)
    .await?;

    Ok(settings)
}

pub async fn update_tempvoc_settings(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    trigger_channel_id: Option<i64>,
    category_id: Option<i64>,
    enabled: bool,
) -> Result<TempvocSettings, sqlx::Error> {
    let settings = sqlx::query_as::<_, TempvocSettings>(
        r#"
        UPDATE bot_tempvoc_settings 
        SET trigger_channel_id = $1, category_id = $2, enabled = $3, updated_at = NOW()
        WHERE bot_id = $4 AND guild_id = $5
        RETURNING *;
        "#,
    )
    .bind(trigger_channel_id)
    .bind(category_id)
    .bind(enabled)
    .bind(bot_id)
    .bind(guild_id)
    .fetch_one(pool)
    .await?;

    Ok(settings)
}

pub async fn create_tempvoc_room(
    pool: &PgPool,
    bot_id: i64,
    guild_id: i64,
    channel_id: i64,
    owner_id: i64,
) -> Result<TempvocRoom, sqlx::Error> {
    let room = sqlx::query_as::<_, TempvocRoom>(
        r#"
        INSERT INTO bot_tempvoc_rooms (bot_id, guild_id, channel_id, owner_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *;
        "#,
    )
    .bind(bot_id)
    .bind(guild_id)
    .bind(channel_id)
    .bind(owner_id)
    .fetch_one(pool)
    .await?;

    Ok(room)
}

pub async fn get_tempvoc_room_by_channel(
    pool: &PgPool,
    channel_id: i64,
) -> Result<Option<TempvocRoom>, sqlx::Error> {
    let room = sqlx::query_as::<_, TempvocRoom>(
        r#"
        SELECT * FROM bot_tempvoc_rooms WHERE channel_id = $1;
        "#,
    )
    .bind(channel_id)
    .fetch_optional(pool)
    .await?;

    Ok(room)
}

pub async fn delete_tempvoc_room(pool: &PgPool, channel_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM bot_tempvoc_rooms WHERE channel_id = $1;
        "#,
    )
    .bind(channel_id)
    .execute(pool)
    .await?;

    Ok(())
}

