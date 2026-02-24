#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

tmp_source=$(mktemp)
tmp_covered=$(mktemp)
missing=$(mktemp)

for f in 'From Online/champions'/*.json; do
  basename "$f" .json
done | sort -u > "$tmp_source"

for f in Characters/*.json; do
  b=$(basename "$f" .json)
  if [ "$b" = "ChampionDefaults" ]; then
    continue
  fi
  printf '%s\n' "$b"
done | sort -u > "$tmp_covered"

comm -23 "$tmp_source" "$tmp_covered" > "$missing"

source_count=$(wc -l < "$tmp_source" | tr -d ' ')
covered_count=$(wc -l < "$tmp_covered" | tr -d ' ')
missing_count=$(wc -l < "$missing" | tr -d ' ')
coverage_percent=$(awk -v c="$covered_count" -v s="$source_count" 'BEGIN { if (s==0) {print "0.00"} else { printf "%.2f", (c/s)*100 } }')

source_json=$(jq -R -s 'split("\n") | map(select(length>0))' "$tmp_source")
covered_json=$(jq -R -s 'split("\n") | map(select(length>0))' "$tmp_covered")
missing_json=$(jq -R -s 'split("\n") | map(select(length>0))' "$missing")

jq -n \
  --arg generated_at "2026-02-24" \
  --arg source_glob "From Online/champions/*.json" \
  --arg canonical_glob "Characters/*.json" \
  --argjson source_keys "$source_json" \
  --argjson covered_keys "$covered_json" \
  --argjson missing_keys "$missing_json" \
  --argjson source_count "$source_count" \
  --argjson covered_count "$covered_count" \
  --argjson missing_count "$missing_count" \
  --arg coverage_percent "$coverage_percent" \
  '{
    schema_version: "1.0.0",
    generated_at: $generated_at,
    source_of_truth: {
      source_champion_corpus: $source_glob,
      canonical_characters_folder: $canonical_glob
    },
    totals: {
      from_online_champions: $source_count,
      canonical_character_files: $covered_count,
      missing_canonical_character_files: $missing_count,
      coverage_percent: ($coverage_percent | tonumber)
    },
    covered_champion_keys: $covered_keys,
    missing_champion_keys: $missing_keys,
    notes: [
      "ChampionDefaults.json is excluded from canonical champion-key coverage counts.",
      "Canonical champion keys are derived from Characters file basenames for parity tracking.",
      "This inventory is data-only and does not imply runtime script coverage parity.",
      "Target state is 1 canonical Characters/<Champion>.json per From Online champion key with Vladimir-level schema parity and sourced ability execution metadata."
    ],
    sources: [
      {
        name: "From Online champion corpus",
        path: "From Online/champions/*.json",
        accessed: "2026-02-24",
        used_for: "source champion key denominator"
      },
      {
        name: "Canonical character data",
        path: "Characters/*.json",
        accessed: "2026-02-24",
        used_for: "covered canonical champion key numerator"
      }
    ]
  }' > Simulation/champion_data_coverage_inventory.json

rm -f "$tmp_source" "$tmp_covered" "$missing"

echo "Wrote Simulation/champion_data_coverage_inventory.json"
