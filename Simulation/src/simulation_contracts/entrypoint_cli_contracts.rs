use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
#[command(about = "URF controlled champion objective simulator")]
pub(crate) struct Cli {
    #[arg(
        long,
        help = "Scenario path or scenario name (resolved as Simulation/scenarios/<name>.json)"
    )]
    pub(crate) scenario: String,
    #[arg(long, value_enum, default_value_t = Mode::ControlledChampion)]
    pub(crate) mode: Mode,
    #[arg(long, default_value_t = 30)]
    pub(crate) ticks: usize,
    #[arg(long, default_value_t = 8)]
    pub(crate) top_x: usize,
    #[arg(long, default_value_t = 2)]
    pub(crate) min_item_diff: usize,
    #[arg(long, default_value_t = 5.0)]
    pub(crate) max_relative_gap_percent: f64,
    #[arg(long)]
    pub(crate) report_path: Option<String>,
    #[arg(long)]
    pub(crate) threads: Option<usize>,
    #[arg(long)]
    pub(crate) max_runtime_seconds: Option<f64>,
    #[arg(
        long,
        help = "Popcorn mode: continue running while significant objective improvements keep occurring within this window"
    )]
    pub(crate) popcorn_window_seconds: Option<f64>,
    #[arg(long, default_value_t = 1.0)]
    pub(crate) popcorn_min_relative_improvement_percent: f64,
    #[arg(long, default_value_t = 10.0)]
    pub(crate) status_every_seconds: f64,
    #[arg(long, value_enum, default_value_t = SearchQualityProfile::MaximumQuality)]
    pub(crate) search_quality_profile: SearchQualityProfile,
    #[arg(
        long,
        help = "Deterministic search seed override (default behavior is random)"
    )]
    pub(crate) seed: Option<u64>,
    #[arg(
        long,
        help = "Comma-separated fixed item names for controlled_champion_fixed_loadout mode"
    )]
    pub(crate) fixed_item_names: Option<String>,
    #[arg(
        long,
        help = "Optional comma-separated rune names override (six runes) for controlled_champion_fixed_loadout mode"
    )]
    pub(crate) fixed_rune_names: Option<String>,
    #[arg(
        long,
        help = "Optional comma-separated shard stat override (three shards) for controlled_champion_fixed_loadout mode"
    )]
    pub(crate) fixed_shard_stats: Option<String>,
    #[arg(
        long,
        help = "Optional report folder label for controlled_champion_fixed_loadout mode"
    )]
    pub(crate) fixed_eval_label: Option<String>,
    #[arg(
        long,
        default_value_t = 1,
        help = "Optional repeat count per keystone for controlled_champion_fixed_loadout_rune_sweep (enables multi-seed-ready aggregation)"
    )]
    pub(crate) fixed_sweep_seed_repeats: usize,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum Mode {
    #[value(name = "controlled_champion", alias = "vladimir")]
    ControlledChampion,
    #[value(name = "controlled_champion_fixed_loadout")]
    ControlledChampionFixedLoadout,
    #[value(name = "controlled_champion_fixed_loadout_rune_sweep")]
    ControlledChampionFixedLoadoutRuneSweep,
    #[value(name = "controlled_champion_step", alias = "vladimir_step")]
    ControlledChampionStep,
    #[value(name = "taric_as")]
    TaricAs,
    #[value(name = "hecarim_ms")]
    HecarimMs,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum SearchQualityProfile {
    #[value(name = "fast")]
    Fast,
    #[value(name = "balanced")]
    Balanced,
    #[value(name = "maximum_quality")]
    MaximumQuality,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ControlledChampionRunOptions<'a> {
    pub(crate) top_x: usize,
    pub(crate) min_item_diff: usize,
    pub(crate) max_relative_gap_percent: f64,
    pub(crate) report_path_override: Option<&'a str>,
    pub(crate) max_runtime_seconds: Option<f64>,
    pub(crate) popcorn_window_seconds: Option<f64>,
    pub(crate) popcorn_min_relative_improvement_percent: f64,
    pub(crate) status_every_seconds: f64,
    pub(crate) search_quality_profile: SearchQualityProfile,
    pub(crate) seed_override: Option<u64>,
}

#[derive(Debug, Clone)]
pub(crate) struct ControlledChampionFixedLoadoutOptions<'a> {
    pub(crate) report_path_override: Option<&'a str>,
    pub(crate) search_quality_profile: SearchQualityProfile,
    pub(crate) fixed_item_names: Vec<String>,
    pub(crate) fixed_rune_names: Option<Vec<String>>,
    pub(crate) fixed_shard_stats: Option<Vec<String>>,
    pub(crate) fixed_eval_label: Option<String>,
    pub(crate) fixed_sweep_seed_repeats: usize,
}
