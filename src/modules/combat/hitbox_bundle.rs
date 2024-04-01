use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::attack::{AttackEvent, HitEvent};

#[derive(Component, Debug)]
pub struct HitboxSource(pub AttackEvent);

#[derive(Component)]
pub struct Lifespan(Timer);

#[derive(Bundle)]
pub struct HitboxBundle {
    source: HitboxSource,
    transform: TransformBundle,
    collider: Collider,
    sensor: Sensor,
    active_events: ActiveEvents,
    lifespan: Lifespan,
}

impl HitboxBundle {
    pub fn new(
        transform: TransformBundle,
        collider: Collider,
        source: AttackEvent,
        lifespan_timer: Timer,
    ) -> Self {
        HitboxBundle {
            collider,
            transform,
            source: HitboxSource(source),
            lifespan: Lifespan(lifespan_timer),
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

pub fn handle_lifespan(
    mut commands: Commands,
    time: Res<Time>,
    mut lifespan_q: Query<(Entity, &mut Lifespan)>,
) {
    for (e, mut lifespan) in lifespan_q.iter_mut() {
        lifespan.0.tick(time.delta());
        if lifespan.0.finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}

pub fn handle_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut ev_hit: EventWriter<HitEvent>,
    hitbox_q: Query<&HitboxSource>,
) {
    for collision_event in collision_events.read() {
        match *collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok(hitbox) = hitbox_q.get(e1) {
                    ev_hit.send(HitEvent {
                        attack: hitbox.0.attack,
                        source: hitbox.0.source,
                        target: e2,
                    });
                };
                if let Ok(hitbox) = hitbox_q.get(e2) {
                    ev_hit.send(HitEvent {
                        attack: hitbox.0.attack,
                        source: hitbox.0.source,
                        target: e1,
                    });
                };
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
