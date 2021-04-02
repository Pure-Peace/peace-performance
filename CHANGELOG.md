## Upcoming

- No changes so far

# v0.2.0

- Sync with rosu-pp 0.2.0
- Async beatmap parsing through features `async_tokio` or `async_std`
- Hide various parsing related types further inwards, i.e. `peace_performance::parse::some_type` instead of `peace_performance::some_type`
  - Affected types: `DifficultyPoint`, `HitObject`, `Pos2`, `TimingPoint`, `HitObjectKind`, `PathType`, `HitSound`

## v0.1.1

- Sync with rosu-pp 0.1.x
- parse:
  - Efficiently handle huge amounts of curvepoints

- osu:
  - Fixed panic on unwrapping unavailable hit results
  - Fixed occasional underflow when calculating pp with passed_objects

- taiko:
  - Fixed missing flooring of hitwindow for pp calculation

- fruits:
  - Fixed passed objects in star calculation

- mania:
  - Fixed pp calculation on HR
