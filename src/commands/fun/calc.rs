use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::common::{send_embed, theme_color};

fn parse_linear_expression(expr: &str) -> Option<(f64, f64)> {
    let normalized = expr.replace(' ', "").replace('-', "+-");
    let mut a = 0.0f64;
    let mut b = 0.0f64;

    for raw in normalized.split('+') {
        let term = raw.trim();
        if term.is_empty() {
            continue;
        }

        if term.contains('x') {
            let coeff = term.replace('x', "");
            let c = if coeff.is_empty() || coeff == "+" {
                1.0
            } else if coeff == "-" {
                -1.0
            } else {
                coeff.parse::<f64>().ok()?
            };
            a += c;
        } else {
            b += term.parse::<f64>().ok()?;
        }
    }

    Some((a, b))
}

fn solve_linear_equation(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.split('=').collect();
    if parts.len() != 2 {
        return None;
    }

    let (a1, b1) = parse_linear_expression(parts[0])?;
    let (a2, b2) = parse_linear_expression(parts[1])?;

    let a = a1 - a2;
    let b = b2 - b1;

    if a.abs() < f64::EPSILON {
        if b.abs() < f64::EPSILON {
            return Some("Équation indéterminée (infinité de solutions).".to_string());
        }
        return Some("Équation impossible (aucune solution).".to_string());
    }

    let x = b / a;
    Some(format!("x = {}", x))
}

pub async fn handle_calc(ctx: &Context, msg: &Message, args: &[&str]) {
    let color = theme_color(ctx).await;
    if args.is_empty() {
        let embed = CreateEmbed::new()
            .title("Erreur")
            .description("Usage: `+calc <calcul>`")
            .color(0xED4245);
        send_embed(ctx, msg, embed).await;
        return;
    }

    let query = args.join(" ");

    let result = if query.contains('=') && query.contains('x') {
        solve_linear_equation(&query)
            .unwrap_or_else(|| "Impossible de résoudre cette équation.".to_string())
    } else {
        match meval::eval_str(&query) {
            Ok(value) => value.to_string(),
            Err(_) => "Expression invalide.".to_string(),
        }
    };

    let embed = CreateEmbed::new()
        .title("Calcul")
        .field("Entrée", query, false)
        .field("Résultat", result, false)
        .color(color);

    send_embed(ctx, msg, embed).await;
}

pub struct CalcCommand;
pub static COMMAND_DESCRIPTOR: CalcCommand = CalcCommand;

impl crate::commands::command_contract::CommandSpec for CalcCommand {
    fn metadata(&self) -> crate::commands::command_contract::CommandMetadata {
        crate::commands::command_contract::CommandMetadata {
            name: "calc",
            category: "fun",
            params: "<expression>",
            description: "Evalue une expression numerique simple et renvoie le resultat.",
            examples: &["+calc", "+cc", "+help calc"],
            default_aliases: &["clc"],
            allow_in_dm: true,
            default_permission: 0,
        }
    }
}
