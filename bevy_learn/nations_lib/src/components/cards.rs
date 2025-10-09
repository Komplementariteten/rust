use bevy::prelude::{Commands, Component};
use crate::components::component_ids::{CompId, CARD_ELFEN_KING};

#[derive(Component)]
pub struct Card;

pub fn spawn_cards(commands: &mut Commands) {
    commands.spawn((Card, CompId(CARD_ELFEN_KING)));
}
