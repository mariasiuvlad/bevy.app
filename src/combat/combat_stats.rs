use bevy::prelude::*;

#[derive(Component)]
pub struct Stats {
    pub health: i32,
    pub max_health: i32,
    pub energy: i32,
    pub max_energy: i32,
    pub move_speed: f32,
    pub move_speed_modifier: f32,
}

#[derive(Bundle)]
pub struct StatsBundle {
    combat_stats: Stats,
}

impl Default for StatsBundle {
    fn default() -> Self {
        StatsBundle {
            combat_stats: Stats {
                max_health: 20,
                health: 20,
                max_energy: 100,
                energy: 100,
                move_speed: 3.,
                move_speed_modifier: 1.,
            },
        }
    }
}

impl Stats {
    pub fn computed_move_speed(&self) -> f32 {
        self.move_speed * self.move_speed_modifier
    }
    pub fn health_percentage(&self) -> f32 {
        (self.health * 100) as f32 / self.max_health as f32
    }
    pub fn energy_percentage(&self) -> f32 {
        (self.energy * 100) as f32 / self.max_energy as f32
    }
}
