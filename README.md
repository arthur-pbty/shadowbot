# Shadowbot

Bot Discord ecrit en Rust (Serenity) avec une architecture modulaire, des commandes prefixees, des interactions (slash/components/modals) et une couche PostgreSQL pour la persistance.

Le projet couvre moderation, logs, tickets, suggestions, automatisations, gestion des roles/salons, jeux et configuration fine des permissions.

## Points forts

- 200+ commandes (prefixees + alias)
- Gestion de permissions par commande et par niveau
- Prefixe principal + prefixes par serveur
- Systeme de tickets avec claim/close/add/remove
- Suggestions avec workflow d approbation
- Automod (antispam, antilink, badwords, antimassmention)
- Logs (messages, moderation, vocal, roles, boosts, raids)
- Presence dynamique (play/listen/watch/compet/stream)
- Persistance PostgreSQL (schema cree automatiquement au demarrage)

## Stack technique

- Rust edition 2024
- [Serenity 0.12](https://github.com/serenity-rs/serenity)
- Tokio
- SQLx + PostgreSQL
- dotenv
- Docker / Docker Compose

## Structure du projet

```text
src/
  main.rs                  # bootstrap bot + DB
  permissions.rs           # ACL, prefixes, checks permissions
  db.rs                    # pool SQLx + schema + acces DB
  activity.rs              # rotation de presence/statut
  commands/                # commandes prefixees par domaine
  events/                  # handlers d evenements Discord
  utils/                   # services transverses (logs, automod, etc.)
```

## Prerequis

- Rust stable (toolchain recente compatible edition 2024)
- Cargo
- PostgreSQL (optionnel, mais recommande)
- Un bot Discord et son token

Important: le bot utilise `GatewayIntents::all()`. Pense a activer les intents necessaires dans le portail Discord Developer (dont Message Content intent).

## Configuration

1. Copier le fichier d exemple:

```bash
cp .env.example .env
```

2. Renseigner les variables dans `.env`:

```env
# Discord
BOT_TOKEN=change_me
FORCE_OWNER_IDS=671763971803447298

# PostgreSQL
POSTGRES_DB=shadowbot
POSTGRES_USER=shadowbot
POSTGRES_PASSWORD=change_me

# App database URL
DATABASE_URL=postgres://shadowbot:change_me@postgres:5432/shadowbot
```

### Variables importantes

- `BOT_TOKEN`: token du bot Discord (obligatoire)
- `FORCE_OWNER_IDS`: IDs utilisateurs consideres owners (CSV possible)
- `DATABASE_URL`: URL PostgreSQL de l application

Note: si `DATABASE_URL` pointe vers `@postgres:` mais que le bot tourne hors Docker, le code tente un fallback automatique vers `@localhost:`.

## Lancement en local (sans Docker pour le bot)

1. Demarrer PostgreSQL (local ou via Docker).

2. Si besoin, lancer seulement la base via Compose:

```bash
docker compose up -d postgres
```

3. Lancer le bot:

```bash
cargo run
```

Le schema SQL est initialise automatiquement au demarrage.

## Lancement full Docker (bot + base)

```bash
docker compose up --build -d
```

Voir les logs:

```bash
docker compose logs -f bot
```

Arreter:

```bash
docker compose down
```

## Fonctionnement des commandes

- Prefixe par defaut: `+`
- Prefixe principal modifiable: `+mainprefix <prefix>`
- Prefixe serveur modifiable: `+prefix <prefix>`
- Aide: `+help` ou `/help`

Exemples:

```text
+ping
+help moderation
+ticket
+suggestion
+warn @user raison
```

### Slash commands

Le projet supporte les interactions slash/components/modals. La commande `/help` est enregistree globalement au demarrage, et plusieurs modules gerent aussi des interactions slash specifiques (ticket, suggestions, tempvoc, autopublish, etc.).

## Base de donnees

Le schema est cree dans `src/db.rs` via `init_schema` avec plusieurs tables, notamment:

- `message_log`
- `bot_settings`, `bot_activities`
- `bot_command_permissions`, `bot_command_access`, `bot_perm_level_access`
- `bot_aliases`
- `bot_tickets`, `bot_ticket_members`, `bot_ticket_settings`
- `bot_suggestions`, `bot_suggestion_settings`
- `bot_autopublish_channels`, `bot_piconly_channels`
- `bot_moderation_settings`, `bot_badwords`, `bot_strike_rules`, `bot_punish_rules`
- `bot_game_sessions`

Si `DATABASE_URL` est absent ou invalide, le bot demarre quand meme, mais certaines fonctions persistantes sont desactivees (ex: snipe persistant).

## Verification rapide

```bash
cargo fmt
cargo check
```

## Contribution

1. Ajouter/modifier un module de commande dans `src/commands/...`
2. Declarer le module dans `src/commands/mod.rs`
3. Ajouter son `COMMAND_DESCRIPTOR` (metadata)
4. Router la commande dans `src/events/message.rs`
5. Si interaction: router explicitement dans `src/events/interaction_create.rs`
6. Verifier ACL/permissions et aliases

## Securite

- Ne jamais commit de secrets (`.env` est ignore)
- Utiliser `.env.example` pour partager la config type

## Licence

Le projet est distribue sous une licence personnalisee. Voir le fichier `LICENSE`.
