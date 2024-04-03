use crate::{modules::character_controller::CharacterController, world3d::Player};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Event)]
pub struct JumpEvent(pub Entity);

#[derive(Debug, Component)]
pub struct JumpAction(pub Vec3);

pub fn keyboard_input_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_jump: EventWriter<JumpEvent>,
    mut q: Query<Entity, With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if let Ok(player_handle) = q.get_single_mut() {
            ev_jump.send(JumpEvent(player_handle));
        }
    }
}

pub fn handle_jump_event(
    mut ev_jump: EventReader<JumpEvent>,
    mut q: Query<(&mut CharacterController, &mut ExternalImpulse, &JumpAction)>,
) {
    for ev in ev_jump.read() {
        if let Ok((mut _ctr, mut _impulse, _jump_action)) = q.get_mut(ev.0) {
            todo!()
        }
    }
}

pub struct JumpPlugin;
impl Plugin for JumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JumpEvent>()
            .add_systems(Update, (handle_jump_event, keyboard_input_controller));
    }
}
