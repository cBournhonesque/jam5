use avian2d::collision::Collider;
use avian2d::parry::shape::Polyline;
use avian2d::parry::transformation::vhacd::VHACDParameters;
use bevy::prelude::*;
use shared::player::trail::Trail;
use shared::player::zone::{Zone, ZoneBundle};
use crate::player::trail::SelfLoopContactEvent;



pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.observe(handle_self_loop_contact);
    }
}


/// Handling of self-loop contacts:
/// - spawn a zone
/// - reset the trail
fn handle_self_loop_contact(
    trigger: Trigger<SelfLoopContactEvent>,
    mut commands: Commands,
    mut trail: Query<(&Parent, &mut Trail, &mut Collider)>
) {
    if let Ok((parent, mut trail, mut collider)) = trail.get_mut(trigger.entity()) {
        info!("Self loop contact! Generate zone");
        // add the point to the trail, and update the collider
        trail.add_point(trigger.event().contact_point);

        // generate a zone from the trail
        let zone = trail.generate_zone();
        if let Some(mut parent) = commands.get_entity(parent.get()) {
            parent.with_children(|parent| {
                parent.spawn((
                     ZoneBundle::new(zone)
                ));
            });
        }

        // reset the line and collider
        trail.line = vec![trigger.event().contact_point];
        *collider = Collider::circle(0.0);
    }
}
