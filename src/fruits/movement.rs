use super::DifficultyObject;

use std::cmp::Ordering;

const ABSOLUTE_PLAYER_POSITIONING_ERROR: f32 = 16.0;
const NORMALIZED_HITOBJECT_RADIUS: f32 = 41.0;
const POSITION_EPSILON: f32 = NORMALIZED_HITOBJECT_RADIUS - ABSOLUTE_PLAYER_POSITIONING_ERROR;
const DIRECTION_CHANGE_BONUS: f32 = 21.0;
const SKILL_MULTIPLIER: f32 = 900.0;
const STRAIN_DECAY_BASE: f32 = 0.2;
const DECAY_WEIGHT: f32 = 0.94;

pub(crate) struct Movement {
    pub(crate) half_catcher_width: f32,

    last_player_position: Option<f32>,
    last_distance_moved: f32,
    last_strain_time: f32,

    current_strain: f32,
    current_section_peak: f32,

    pub(crate) strain_peaks: Vec<f32>,
    prev_time: Option<f32>,
}

impl Movement {
    #[inline]
    pub(crate) fn new(cs: f32) -> Self {
        let mut half_catcher_width = super::calculate_catch_width(cs) * 0.5;
        half_catcher_width *= 1.0 - ((cs - 5.5).max(0.0) * 0.0625);

        Self {
            half_catcher_width,

            last_player_position: None,
            last_distance_moved: 0.0,
            last_strain_time: 0.0,

            current_strain: 1.0,
            current_section_peak: 1.0,

            strain_peaks: Vec::with_capacity(128),
            prev_time: None,
        }
    }

    #[inline]
    pub(crate) fn save_current_peak(&mut self) {
        self.strain_peaks.push(self.current_section_peak);
    }

    #[inline]
    pub(crate) fn start_new_section_from(&mut self, time: f32) {
        self.current_section_peak = self.peak_strain(time - self.prev_time.unwrap());
    }

    pub(crate) fn process(&mut self, current: &DifficultyObject) {
        self.current_strain *= strain_decay(current.delta);
        self.current_strain += self.strain_value_of(&current) * SKILL_MULTIPLIER;
        self.current_section_peak = self.current_strain.max(self.current_section_peak);
        self.prev_time.replace(current.start_time);
    }

    pub(crate) fn difficulty_value(&mut self) -> f32 {
        let mut difficulty = 0.0;
        let mut weight = 1.0;

        self.strain_peaks
            .sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

        for &strain in self.strain_peaks.iter() {
            difficulty += strain * weight;
            weight *= DECAY_WEIGHT;
        }

        difficulty
    }

    fn strain_value_of(&mut self, current: &DifficultyObject) -> f32 {
        let last_player_pos = self
            .last_player_position
            .unwrap_or(current.last_normalized_pos);

        let mut pos = last_player_pos
            .max(current.normalized_pos - POSITION_EPSILON)
            .min(current.normalized_pos + POSITION_EPSILON);

        let dist_moved = pos - last_player_pos;
        let weighted_strain_time = current.strain_time + 13.0 + (3.0 / current.clock_rate);

        let mut dist_addition = dist_moved.abs().powf(1.3) / 510.0;

        if dist_moved.abs() > 0.1 {
            if self.last_distance_moved.abs() > 0.1
                && dist_moved.signum() != self.last_distance_moved.signum()
            {
                let bonus_factor = dist_moved.abs().min(50.0) / 50.0;
                let anti_flow_factor = (self.last_distance_moved.abs().min(70.0) / 70.0).max(0.38);

                dist_addition += DIRECTION_CHANGE_BONUS / (self.last_strain_time + 16.0).sqrt()
                    * bonus_factor
                    * anti_flow_factor
                    * (1.0 - (weighted_strain_time / 1000.0).powi(3)).max(0.0);
            }

            dist_addition += 12.5 * dist_moved.abs().min(NORMALIZED_HITOBJECT_RADIUS * 2.0)
                / (NORMALIZED_HITOBJECT_RADIUS * 6.0)
                / weighted_strain_time.sqrt();
        }

        let mut edge_dash_bonus = 0.0;

        if current.last.hyper_dist <= 20.0 {
            if !current.last.hyper_dash {
                edge_dash_bonus += 5.7;
            } else {
                pos = current.normalized_pos;
            }

            dist_addition *= 1.0
                + edge_dash_bonus
                    * ((20.0 - current.last.hyper_dist) / 20.0)
                    * ((current.strain_time * current.clock_rate).min(265.0) / 265.0).powf(1.5);
        }

        self.last_player_position.replace(pos);
        self.last_distance_moved = dist_moved;
        self.last_strain_time = current.strain_time;

        dist_addition / weighted_strain_time
    }

    #[inline]
    fn peak_strain(&self, delta_time: f32) -> f32 {
        self.current_strain * strain_decay(delta_time)
    }
}

#[inline]
fn strain_decay(ms: f32) -> f32 {
    STRAIN_DECAY_BASE.powf(ms / 1000.0)
}
