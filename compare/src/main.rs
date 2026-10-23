//! Prints what each analyser decides for the same statements.
//!
//! Speed is the easy comparison and the less interesting one. This is the
//! other half: given identical input and an equivalent placement policy,
//! which statements does each project agree to run?

use lucent_compare::{
    Decision, STATEMENTS, lucent_decision, lucent_router, shahrah_broadcast_policy,
    shahrah_decision, shahrah_policy,
};
use lucent_sql::AnalysisCache;
use shahrah_sql::cache::Cache;

fn main() {
    let router = lucent_router();
    let lucent_cache = AnalysisCache::default();
    let policy = shahrah_policy();
    let broadcast = shahrah_broadcast_policy();
    let shahrah_cache = Cache::new(4_096);
    let broadcast_cache = Cache::new(4_096);

    let rows: Vec<(&str, Decision, Decision, Decision)> = STATEMENTS
        .iter()
        .map(|(name, sql)| {
            (
                *name,
                lucent_decision(&router, &lucent_cache, sql),
                shahrah_decision(&shahrah_cache, &policy, sql),
                shahrah_decision(&broadcast_cache, &broadcast, sql),
            )
        })
        .collect();

    println!("| Statement | Lucent | shahrah | shahrah, broadcast on |");
    println!("|---|---|---|---|");
    for (name, ours, theirs, wider) in &rows {
        println!(
            "| {name} | {} | {} | {} |",
            ours.symbol(),
            theirs.symbol(),
            wider.symbol()
        );
    }

    let lucent_runs = rows.iter().filter(|row| row.1.runs()).count();
    let shahrah_runs = rows.iter().filter(|row| row.2.runs()).count();
    let broadcast_runs = rows.iter().filter(|row| row.3.runs()).count();
    println!();
    println!(
        "Of {} statements, Lucent runs {lucent_runs}; shahrah runs {shahrah_runs}, or \
         {broadcast_runs} with broadcast on.",
        rows.len()
    );

    println!();
    println!("## Where they differ");
    println!();
    for (name, ours, theirs, wider) in &rows {
        if ours == theirs && ours == wider {
            continue;
        }
        println!("**{name}**");
        println!("  - Lucent: {}{}", ours.symbol(), suffix(ours));
        println!("  - shahrah:  {}{}", theirs.symbol(), suffix(theirs));
        println!(
            "  - shahrah, broadcast on: {}{}",
            wider.symbol(),
            suffix(wider)
        );
    }
}

fn suffix(decision: &Decision) -> String {
    decision
        .reason()
        .map(|reason| format!(" — {reason}"))
        .unwrap_or_default()
}
