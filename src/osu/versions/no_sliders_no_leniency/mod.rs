//! In addtion to not considering the positional offset caused by stack leniency, slider paths are also ignored.
//! This means the travel distance of notes is completely omitted which may cause further inaccuracies.
//! Since the slider paths don't have to be computed though, it is generally faster than `no_leniency`.

#![cfg(feature = "no_sliders_no_leniency")]

use super::super::DifficultyAttributes;

mod difficulty_object;
mod osu_object;
mod skill;
mod skill_kind;
mod slider_state;

use difficulty_object::DifficultyObject;
use osu_object::OsuObject;
use skill::Skill;
use skill_kind::SkillKind;
use slider_state::SliderState;

use crate::{parse::HitObjectKind, Beatmap, Mods, StarResult, Strains};

const OBJECT_RADIUS: f32 = 64.0;
const SECTION_LEN: f32 = 400.0;
const DIFFICULTY_MULTIPLIER: f32 = 0.0675;
const NORMALIZED_RADIUS: f32 = 52.0;

/// Star calculation for osu!standard maps.
///
/// Sliders are considered as regular hitcircles and stack leniency is ignored.
/// Still very good results but the least precise version in general.
/// However, this is the most efficient one.
///
/// In case of a partial play, e.g. a fail, one can specify the amount of passed objects.
pub fn stars(map: &Beatmap, mods: impl Mods, passed_objects: Option<usize>) -> StarResult {
    let take = passed_objects.unwrap_or_else(|| map.hit_objects.len());

    let attributes = map.attributes().mods(mods);
    let hitwindow = super::difficulty_range(attributes.od).floor() / attributes.clock_rate;
    let od = (80.0 - hitwindow) / 6.0;

    if take < 2 {
        return StarResult::Osu(DifficultyAttributes {
            ar: attributes.ar,
            od,
            ..Default::default()
        });
    }

    let radius = OBJECT_RADIUS * (1.0 - 0.7 * (attributes.cs - 5.0) / 5.0) / 2.0;
    let mut scaling_factor = NORMALIZED_RADIUS / radius;

    if radius < 30.0 {
        let small_circle_bonus = (30.0 - radius).min(5.0) / 50.0;
        scaling_factor *= 1.0 + small_circle_bonus;
    }

    let clock_rate = attributes.clock_rate;

    let mut max_combo = 0;
    let mut state = SliderState::new(&map);

    let mut hit_objects = map
        .hit_objects
        .iter()
        .take(take)
        .filter_map(|h| match &h.kind {
            HitObjectKind::Circle => {
                max_combo += 1;

                Some(OsuObject::new(h.pos, h.start_time, false, clock_rate))
            }
            HitObjectKind::Slider {
                pixel_len, repeats, ..
            } => {
                max_combo += state.count_ticks(h.start_time, *pixel_len, *repeats, &map);

                Some(OsuObject::new(h.pos, h.start_time, false, clock_rate))
            }
            HitObjectKind::Spinner { .. } => {
                max_combo += 1;

                Some(OsuObject::new(h.pos, h.start_time, true, clock_rate))
            }
            HitObjectKind::Hold { .. } => None,
        });

    let mut aim = Skill::new(SkillKind::Aim);
    let mut speed = Skill::new(SkillKind::Speed);

    let mut prev_prev = None;
    let mut prev = hit_objects.next().unwrap();
    let mut prev_vals = None;

    // First object has no predecessor and thus no strain, handle distinctly
    let mut current_section_end = (prev.time / SECTION_LEN).ceil() * SECTION_LEN;

    // Handle second object separately to remove later if-branching
    let curr = hit_objects.next().unwrap();
    let h = DifficultyObject::new(&curr, &prev, prev_vals, prev_prev, scaling_factor);

    while h.base.time > current_section_end {
        current_section_end += SECTION_LEN;
    }

    aim.process(&h);
    speed.process(&h);

    prev_prev = Some(prev);
    prev_vals = Some((h.jump_dist, h.strain_time));
    prev = curr;

    // Handle all other objects
    for curr in hit_objects {
        let h = DifficultyObject::new(&curr, &prev, prev_vals, prev_prev, scaling_factor);

        while h.base.time > current_section_end {
            aim.save_current_peak();
            aim.start_new_section_from(current_section_end);
            speed.save_current_peak();
            speed.start_new_section_from(current_section_end);

            current_section_end += SECTION_LEN;
        }

        aim.process(&h);
        speed.process(&h);

        prev_prev = Some(prev);
        prev_vals = Some((h.jump_dist, h.strain_time));
        prev = curr;
    }

    aim.save_current_peak();
    speed.save_current_peak();

    let aim_strain = aim.difficulty_value().sqrt() * DIFFICULTY_MULTIPLIER;
    let speed_strain = speed.difficulty_value().sqrt() * DIFFICULTY_MULTIPLIER;

    let stars = aim_strain + speed_strain + (aim_strain - speed_strain).abs() / 2.0;

    StarResult::Osu(DifficultyAttributes {
        stars,
        ar: attributes.ar,
        od,
        speed_strain,
        aim_strain,
        max_combo,
        n_circles: map.n_circles as usize,
        n_spinners: map.n_spinners as usize,
    })
}

/// Essentially the same as the `stars` function but instead of
/// evaluating the final strains, it just returns them as is.
///
/// Suitable to plot the difficulty of a map over time.
pub fn strains(map: &Beatmap, mods: impl Mods) -> Strains {
    let attributes = map.attributes().mods(mods);

    if map.hit_objects.len() < 2 {
        return Strains::default();
    }

    let radius = OBJECT_RADIUS * (1.0 - 0.7 * (attributes.cs - 5.0) / 5.0) / 2.0;
    let mut scaling_factor = NORMALIZED_RADIUS / radius;

    if radius < 30.0 {
        let small_circle_bonus = (30.0 - radius).min(5.0) / 50.0;
        scaling_factor *= 1.0 + small_circle_bonus;
    }

    let clock_rate = attributes.clock_rate;

    let mut hit_objects = map.hit_objects.iter().filter_map(|h| match &h.kind {
        HitObjectKind::Circle | HitObjectKind::Slider { .. } => {
            Some(OsuObject::new(h.pos, h.start_time, false, clock_rate))
        }
        HitObjectKind::Spinner { .. } => {
            Some(OsuObject::new(h.pos, h.start_time, true, clock_rate))
        }
        HitObjectKind::Hold { .. } => None,
    });

    let mut aim = Skill::new(SkillKind::Aim);
    let mut speed = Skill::new(SkillKind::Speed);

    let mut prev_prev = None;
    let mut prev = hit_objects.next().unwrap();
    let mut prev_vals = None;

    // First object has no predecessor and thus no strain, handle distinctly
    let mut current_section_end = (prev.time / SECTION_LEN).ceil() * SECTION_LEN;

    // Handle second object separately to remove later if-branching
    let curr = hit_objects.next().unwrap();
    let h = DifficultyObject::new(&curr, &prev, prev_vals, prev_prev, scaling_factor);

    while h.base.time > current_section_end {
        current_section_end += SECTION_LEN;
    }

    aim.process(&h);
    speed.process(&h);

    prev_prev = Some(prev);
    prev_vals = Some((h.jump_dist, h.strain_time));
    prev = curr;

    // Handle all other objects
    for curr in hit_objects {
        let h = DifficultyObject::new(&curr, &prev, prev_vals, prev_prev, scaling_factor);

        while h.base.time > current_section_end {
            aim.save_current_peak();
            aim.start_new_section_from(current_section_end);
            speed.save_current_peak();
            speed.start_new_section_from(current_section_end);

            current_section_end += SECTION_LEN;
        }

        aim.process(&h);
        speed.process(&h);

        prev_prev = Some(prev);
        prev_vals = Some((h.jump_dist, h.strain_time));
        prev = curr;
    }

    aim.save_current_peak();
    speed.save_current_peak();

    let strains = aim
        .strain_peaks
        .into_iter()
        .zip(speed.strain_peaks.into_iter())
        .map(|(aim, speed)| aim + speed)
        .collect();

    Strains {
        section_length: SECTION_LEN,
        strains,
    }
}
