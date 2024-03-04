use bevy::{prelude::*, scene::SceneBundle};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationStates {
    Idle,
    Walk,
    Backpedal,
    Run,
    Attack,
    Flinch,
}

#[derive(Component, Debug)]
pub struct AnimationState(pub AnimationStates);

#[derive(Component)]
pub struct ModelAnimations(pub Vec<Handle<AnimationClip>>);
impl ModelAnimations {
    pub fn from_vec(source: &Vec<Handle<AnimationClip>>) -> ModelAnimations {
        ModelAnimations(source.iter().map(|a| a.clone_weak()).collect::<Vec<_>>())
    }
    fn get_handle(&self, index: usize) -> Handle<AnimationClip> {
        self.0[index].clone_weak()
    }
    pub fn attack(&self) -> Handle<AnimationClip> {
        self.get_handle(0)
    }
    pub fn backpedal(&self) -> Handle<AnimationClip> {
        self.get_handle(1)
    }
    pub fn flinch(&self) -> Handle<AnimationClip> {
        self.get_handle(2)
    }
    pub fn idle(&self) -> Handle<AnimationClip> {
        self.get_handle(3)
    }
    pub fn run(&self) -> Handle<AnimationClip> {
        self.get_handle(4)
    }
    pub fn walk(&self) -> Handle<AnimationClip> {
        self.get_handle(5)
    }
    pub fn match_animation_state(&self, animation_state: &AnimationState) -> Handle<AnimationClip> {
        match animation_state.0 {
            AnimationStates::Idle => self.idle(),
            AnimationStates::Walk => self.walk(),
            AnimationStates::Backpedal => self.backpedal(),
            AnimationStates::Flinch => self.flinch(),
            AnimationStates::Run => self.run(),
            AnimationStates::Attack => self.attack(),
        }
    }
}

#[derive(Bundle)]
pub struct AnimatedModelBundle {
    pub animation_state: AnimationState,
    pub scene: SceneBundle,
    pub animations: ModelAnimations,
}
