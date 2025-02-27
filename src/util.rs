//! Common and generic utility stuff.

use std::str::FromStr;
use std::sync::LazyLock;
use std::collections::HashSet;

use regex::Regex;
use serenity::model::user::User;

use crate::types::*;

/// Returns [`true`] if `x` is [`T::default`].
pub fn is_default<T: Default + Eq>(x: &T) -> bool {x == &T::default()}

/// Returns auto-...continuations?... for a comma separated list of [`GagModeName`]s.
pub async fn csv_gag_mode_name_autocomplete<'a>(_: poise::Context<'_, crate::types::State, serenity::Error>, value: &'a str) -> Box<dyn Iterator<Item = String> + 'a + Send> {
    let mut rets = GagModeName::all();
    for x in value.split(',') {if let Ok(x) = FromStr::from_str(x) {rets.remove(&x);}}
    if value.ends_with(',') {
        Box::new(rets.into_iter().map(move |x| format!("{value}{x}")))
    } else if value.is_empty() {
        Box::new(rets.into_iter().map(move |x| x.to_string()))
    } else {
        let last = value.rsplit(',').next().expect("Some value");
        let not_last = value.strip_suffix(last).expect("The value to end with its last bit");
        let rets2 = rets.clone().into_iter().filter(move |x| x.to_string().starts_with(last) && x.to_string() != last).map(move |x| format!("{not_last}{x}")).collect::<Vec<_>>();
        if rets2.is_empty() {
            Box::new(rets.into_iter().map(move |x| format!("{value},{x}")))
        } else {
            Box::new(rets2.into_iter())
        }
    }
}

/// The [`Regex`] of message starts to keep at the start.
static PREFIXES: LazyLock<Regex> = LazyLock::new(|| Regex::new("^(-#|#{1,3}) ").expect("The PREFIXES regex to be valid"));

/// Convenience function to gag and format a message.
pub fn to_gagged_message(text: &str, mode: GagModeName, author: &User) -> String {
    let prefix = PREFIXES.find(text).filter(|x| x.start() == 0).map(|x| x.as_str()).unwrap_or_default();
    format!("{prefix}{author} ({}): {}",
        mode.icon(),
        mode.get().rewrite(text.strip_prefix(prefix).expect("The message to always start with its prefix")).expect("The GagMode to be valid")
    )
}

/// Parses a comma separated list of [`GagModeName`]s into a [`HashSet`]/
/// # Errors
/// If a call to [`GagModeName::from_str`] returns an error, the substring that caused the error is returned.
pub fn parse_csv_gag_modes(modes: Option<&str>) -> Result<HashSet<GagModeName>, &str> {
    match modes {
        Some(modes) => modes.split(',').map(|x| GagModeName::from_str(x).map_err(|_| x)).collect(),
        None => Ok(Default::default())
    }
}
