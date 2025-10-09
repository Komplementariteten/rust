pub mod components;

#[cfg(test)]
mod tests {
    use bevy::prelude::{Query, With};
    use crate::components::cards::Card;
    use crate::components::component_ids::CompId;

    pub fn print_all_cards(query: Query<&CompId, With<Card>>) {
        for cardId in &query {
            println!("Card Id:{:?}!", cardId.0);
        }
    }
}