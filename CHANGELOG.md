## Upcoming

- No changes so far

# v0.4.0

- Add new feature `ppysb_edition` (Special changes for **RELAX** and **AUTOPILOT**)
- By [TUROU2004](https://github.com/TROU2004):
  - **High AR bonus relates to the number of beatmap objects**: Buff longer beatmaps, Nerf shorter beatmaps.
  - ++ **HD** bonus (Relax)
  - ++ **ACC weight**, penalty `50` (Relax)
  - ++ **Slider** bonus.
- By [Pure-Peace](https://github.com/Pure-Peace):
  - **Autopilot**: adjusted the proportion of `aim`, `spd`, and `acc` to the total pp.
- Rename feature `peace_edition` -> `score_v2_buff`
- score_v2_buff: `acc *= 1.25` (Only std, exclude relax and autopilot)

# v0.3.0

- sync with rosu-pp 2.3:
- Fixed a panic for some mania difficulty calculations on converts
- **Updated the difficulty & pp changes from 21.07.27**
- **Updated osu's clockrate bugfix for all modes**

# v0.2.10

- Default not enabled `score_v2_buff`.
- Fix `score_v2_buff`'s score v2 buff.

# v0.2.9

- Fix: Dead loop when reading an empty .osu file.

# v0.2.8

- When using async features, the `parse_sync` method is still provided, you can choose freely.
- Upgrade to tokio1.9

# v0.2.7

- bug fix

# v0.2.6

- `score_v2_buff`: no longer contains relax/ap nerf, instead use `relax_nerf`.
- `score_v2_buff`: score v2 - acc_value *= `1.20` (buff).
- `relax_nerf`: relax / ap - acc_value *= `0.7` -> `0.8`.

# v0.2.5

- sync with rosu-pp 2.2:
- Reduced amount of required features of `async_std` and `async_tokio`
- Fixed a panic for some mania difficulty calculations on converts
- osu & fruits:
  - Fixed specific slider patterns
  - Optimized Bezier, Catmull, and other small things

    Benchmarking for osu!standard showed a 25%+ improvement for performance aswell as accuracy

- fruits:
  - Fixed tick timing for reverse sliders

- taiko:
  - Micro optimizations

- parse & osu:
  - Cleanup and tiny optimizations

# v0.2.4

- Default use tokio
- Upgrade tokio to 1.6

# v0.2.3

- Now, we can get more info from pp result
- Add PpResult.raw (A struct included aim, spd, acc, str values)
- Add PpResult.mods
- Add PpResult.mode

# v0.2.2

- Now, it will be more comfortable when calculating multiple different ACC values.
- Add mutable reference methods: set_accuracy()
- calculate() now use mutable reference.

# v0.2.1

- Add mods Score v2.
- Buff Score v2 with ACC value bonus * 1.14 (Only STD).

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
