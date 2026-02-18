
use super::*;

#[test]
fn report_path_uses_normalized_champion_key() {
    let path = default_report_path_for_champion("Dr. Mundo");
    let path_text = path.to_string_lossy();
    assert!(path_text.ends_with("output/drmundo_run_report.md"));
}
