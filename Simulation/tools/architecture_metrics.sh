#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIM_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
SRC_DIR="${SIM_DIR}/src"

TARGET_PER_FILE=700

FILES=(
  "core.rs"
  "search.rs"
  "reporting.rs"
  "data.rs"
  "defaults.rs"
  "engine.rs"
  "scenario_runner.rs"
)

baseline_for() {
  case "$1" in
    "core.rs") echo 933 ;;
    "search.rs") echo 2244 ;;
    "reporting.rs") echo 1075 ;;
    "data.rs") echo 2008 ;;
    "defaults.rs") echo 2455 ;;
    "engine.rs") echo 3579 ;;
    "scenario_runner.rs") echo 4284 ;;
    *)
      echo "0"
      ;;
  esac
}

current_total=0
baseline_total=0
remaining_gap_total=0

echo "Architecture metrics (facade line budgets)"
echo
echo "| File | Baseline | Current | Target | Remaining Gap |"
echo "|---|---:|---:|---:|---:|"

for file in "${FILES[@]}"; do
  baseline="$(baseline_for "${file}")"
  current="$(wc -l < "${SRC_DIR}/${file}")"
  current="${current//[[:space:]]/}"

  gap=0
  if (( current > TARGET_PER_FILE )); then
    gap=$((current - TARGET_PER_FILE))
  fi

  baseline_total=$((baseline_total + baseline))
  current_total=$((current_total + current))
  remaining_gap_total=$((remaining_gap_total + gap))

  echo "| ${file} | ${baseline} | ${current} | <= ${TARGET_PER_FILE} | ${gap} |"
done

target_total=$(( ${#FILES[@]} * TARGET_PER_FILE ))
completed_reduction=$((baseline_total - current_total))
required_reduction=$((baseline_total - target_total))

size_progress_percent="0.00"
if (( required_reduction > 0 )); then
  size_progress_percent="$(awk -v done="${completed_reduction}" -v need="${required_reduction}" 'BEGIN { printf "%.2f", (done / need) * 100.0 }')"
fi

echo
echo "Totals"
echo "- baseline_total_lines: ${baseline_total}"
echo "- current_total_lines: ${current_total}"
echo "- target_total_lines: ${target_total}"
echo "- size_based_progress_percent: ${size_progress_percent}%"
echo "- remaining_gap_to_target_lines: ${remaining_gap_total}"
