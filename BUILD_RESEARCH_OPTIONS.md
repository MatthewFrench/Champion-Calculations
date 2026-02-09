# Vladimir URF Build Research Options (No Meta Sites)

Goal: derive a "perfect" build without relying on external win-rate or recommended-build sites.

This document ranks options by effectiveness (how likely to produce the best build under a clearly defined model), the kind of new information each can reveal, and how they overlap.

## Ranking (Most Effective to Least Effective)

1. Full Mathematical Optimization (Mechanics-Only)
2. Exhaustive Search Over Item Space
3. Monte-Carlo Fight Simulation
4. Multi-Objective Pareto Frontier
5. Worst-Case Optimization (Min-Max)
6. Constraint-Driven "Proof" Build
7. Item Efficiency Optimization

Rationale for ranking:
- Options 1 and 2 directly optimize or exhaustively search a formal objective, which gives the strongest claim of optimality under a defined model.
- Monte-Carlo adds robustness and surprises (edge cases), but is only as good as the simulated distributions.
- Pareto frontier is great for visualizing tradeoffs, but needs a scoring model and doesn't pick a single "best" by itself.
- Worst-case and constraint-based approaches are powerful but can be overly conservative and may sacrifice damage.
- Item efficiency is useful but can miss synergy and interaction effects.

---

## Option Details

### 1) Full Mathematical Optimization (Mechanics-Only)
Effectiveness: Highest (if model is correct).
New information: Gives a provable best build for a formal objective (e.g., maximize time-alive plus pool DPS).
Overlap:
- Uses the same formulas as Item Efficiency.
- Can be combined with Exhaustive Search for validation.
- Can generate inputs for Pareto Frontier.
Notes:
- Requires explicit assumptions: enemy damage mix, fight duration, CC patterns, and pool uptime model.

### 2) Exhaustive Search Over Item Space
Effectiveness: Very high (global optimum over item combinations for a fixed scoring function).
New information: Reveals unexpected or non-intuitive item combinations that maximize the objective.
Overlap:
- Depends on the same scoring model as Option 1.
- Can output a Pareto set (Option 4) directly.
- Can be used to validate results from Item Efficiency or Min-Max.
Notes:
- Computationally heavy but feasible with pruning.

### 3) Monte-Carlo Fight Simulation
Effectiveness: High (robustness across varied fight conditions).
New information: Identifies builds that remain strong under many scenarios; surfaces fragile builds.
Overlap:
- Uses the same base model as Options 1 and 2, but adds randomized scenarios.
- Can help pick a single build from a Pareto frontier.
Notes:
- Requires a simulation engine and random distribution design.

### 4) Multi-Objective Pareto Frontier
Effectiveness: Medium-high (excellent at exposing tradeoffs, not a single winner).
New information: Shows "no-tradeoff" builds and the true damage/survivability frontier.
Overlap:
- Built on the same evaluation function as Options 1 and 2.
- Monte-Carlo can rank or stress-test frontier builds.
Notes:
- Does not define "best" without a tie-breaker.

### 5) Worst-Case Optimization (Min-Max)
Effectiveness: Medium (strong defensively, can be suboptimal in typical fights).
New information: Reveals the build that survives the harshest plausible burst/CC sequences.
Overlap:
- Can be implemented with Options 1 or 2 by changing the objective.
- Complements Monte-Carlo by anchoring the worst end.
Notes:
- Tends to favor tank-heavy builds and may reduce damage.

### 6) Constraint-Driven "Proof" Build
Effectiveness: Medium (guarantees thresholds, but not necessarily maximum performance).
New information: Establishes minimum survivability/damage guarantees that other builds can be compared against.
Overlap:
- Can be solved with Options 1 or 2 as feasibility filters.
- Often used to narrow the search space.
Notes:
- Requires well-chosen constraints.

### 7) Item Efficiency Optimization
Effectiveness: Medium-low (good heuristics, misses synergy).
New information: Identifies high-value items per gold for health, resistances, or AoE damage.
Overlap:
- Provides strong priors or starting points for Options 1 and 2.
- Useful to prune the exhaustive search space.
Notes:
- Alone, it may not capture interactions between items and kit.

---

## Overlap Matrix (High-Level)

- Option 1 and Option 2: Strong overlap. Both rely on the same formal objective; Option 2 can validate Option 1's optimality.
- Option 2 and Option 4: Option 2 can generate a Pareto frontier as a byproduct.
- Option 3 and Option 4: Monte-Carlo can select a robust winner from the frontier.
- Option 5 is a specialized objective that can be computed using the machinery of Options 1 or 2.
- Option 6 is a constraint layer that can be combined with Options 1, 2, or 4.
- Option 7 is a heuristic feeder for Options 1 and 2, not a standalone proof.

---

## Recommended Path (If You Want the Strongest Claim)

1) Define the model assumptions precisely (enemy damage mix, fight duration, pool uptime).
2) Run an exhaustive search to find the global best build for that model.
3) Use Monte-Carlo to stress-test the top few builds against varied scenarios.
4) Publish the final winner plus a Pareto frontier summary for transparency.
