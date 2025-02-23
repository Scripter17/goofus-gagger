use std::str::FromStr;

use crate::types::*;

pub fn is_default<T: Default + Eq>(x: &T) -> bool {x == &T::default()}

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
