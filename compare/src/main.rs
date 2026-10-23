//! Prints what each analyser decides for the same statements.
//!
//! Speed is the easy comparison and the less interesting one. This is the
//! other half: given identical input and an equivalent placement policy,
//! which statements does each project agree to run?

use lucent_compare::{
    Decision, STATEMENTS, lucent_decision, lucent_router, shahrah_decision, shahrah_policy,
};
use lucent_sql::AnalysisCache;
use shahrah_sql::cache::Cache;

fn main() {
    let router = lucent_router();
    let lucent_cache = AnalysisCache::default();
    let policy = shahrah_policy();
    let shahrah_cache = Cache::new(4_096);

    let rows: Vec<(&str, Decision, Decision)> = STATEMENTS
        .iter()
        .map(|(name, sql)| {
            (
                *name,
                lucent_decision(&router, &lucent_cache, sql),
                shahrah_decision(&shahrah_cache, &policy, sql),
            )
        })
        .collect();

    println!("| Statement | Lucent | shahrah |");
    println!("|---|---|---|");
    for (name, ours, theirs) in &rows {
        let mark = if ours == theirs { "" } else { "  **<-**" };
        println!(
            "| {name} | {} | {}{mark} |",
            ours.symbol(),
            theirs.symbol()
        );
    }

    let runs_here = rows.iter().filter(|(_, ours, _)| ours.runs()).count();
    let runs_there = rows.iter().filter(|(_, _, theirs)| theirs.runs()).count();
    println!();
    println!(
        "Lucent runs {runs_here} of {} statements; shahrah runs {runs_there}.",
        rows.len()
    );

    println!();
    println!("## Where they differ");
    println!();
    for (name, ours, theirs) in &rows {
        if ours == theirs {
            continue;
        }
        println!("**{name}**");
        println!("  - Lucent: {}{}", ours.symbol(), suffix(ours));
        println!("  - shahrah:  {}{}", theirs.symbol(), suffix(theirs));
    }
}

fn suffix(decision: &Decision) -> String {
    decision
        .reason()
        .map(|reason| format!(" — {reason}"))
        .unwrap_or_default()
}
