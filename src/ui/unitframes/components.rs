use bevy::prelude::*;

#[derive(Component)]
pub struct UiUnitFrameRoot<T: Component>(pub T);

#[derive(Component)]
pub struct UiName<T: Component>(pub T);

#[derive(Component)]
pub struct UiHealthValue<T: Component>(pub T);

#[derive(Component)]
pub struct UiHealthPercentage<T: Component>(pub T);

#[derive(Component)]
pub struct UiEnergyValue<T: Component>(pub T);

#[derive(Component)]
pub struct UiEnergyPercentage<T: Component>(pub T);
