/// Utility plugin to display a text label next to an entity.
///
/// Label will track parent position, ignoring rotation.
use avian2d::prelude::Position;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy::prelude::*;
use lightyear::prelude::client::VisualInterpolateStatus;

pub struct EntityLabelPlugin;

impl Plugin for EntityLabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (label_added, label_changed));

        app.add_systems(
            PostUpdate,
            // update the `GlobalTransform` of the label after TransformPropagate (and VisualInterpolation)
            update_entity_label_positions.after(TransformPropagate),
        );
    }
}

/// Component to add a label to an entity
#[derive(Component)]
pub struct EntityLabel {
    pub text: String,
    pub sub_text: String,
    pub offset: Vec2,
    pub inherit_rotation: bool,
    pub z: f32,
    pub size: f32,
    pub color: Color,
}

impl Default for EntityLabel {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            sub_text: "".to_owned(),
            offset: Vec2::new(0.0, 10.0),
            inherit_rotation: false,
            z: 10.0,
            size: 17.0,
            color: bevy::color::palettes::css::ANTIQUE_WHITE.into(),
        }
    }
}

/// Marker for labels that are children (with TextBundles) of entities with EntityLabel
#[derive(Component)]
pub struct EntityLabelChild;

/// Add the child entity containing the Text2dBundle
fn label_added(q: Query<(Entity, &EntityLabel), Added<EntityLabel>>, mut commands: Commands) {
    let font: Handle<Font> = Default::default();
    let mut ts = TextStyle::default();
    let mut ts_sub = TextStyle::default();
    for (e, label) in q.iter() {
        ts.font_size = label.size;
        ts_sub.font_size = label.size * 0.85;
        ts.color = label.color;
        ts_sub.color = label.color.with_alpha(0.6);
        commands
            .spawn((
                EntityLabelChild,
                Text2dBundle {
                    text: Text::from_sections([
                        TextSection::new(label.text.clone(), ts.clone()),
                        TextSection::new("\n", ts.clone()),
                        TextSection::new(label.sub_text.clone(), ts_sub.clone()),
                    ])
                    .with_no_wrap()
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_translation(Vec3::new(
                        label.offset.x,
                        label.offset.y,
                        label.z,
                    )),
                    ..default()
                },
                Name::from("Label"),
            ))
            .set_parent(e);
    }
}

/// modify text when EntityLabel changes
fn label_changed(
    q_parents: Query<(&EntityLabel, &Children), Changed<EntityLabel>>,
    mut q_children: Query<
        (&mut Text, &mut Transform),
        (With<EntityLabelChild>, Without<EntityLabel>),
    >,
) {
    for (label, children) in q_parents.iter() {
        for child in children.iter() {
            if let Ok((mut text, mut transform)) = q_children.get_mut(*child) {
                assert_eq!(text.sections.len(), 3);

                if label.text != text.sections[0].value {
                    text.sections[0].value.clone_from(&label.text);
                }
                text.sections[0].style.font_size = label.size;
                text.sections[0].style.color = label.color;

                if label.sub_text != text.sections[2].value {
                    text.sections[2].value.clone_from(&label.sub_text);
                }
                text.sections[2].style.font_size = label.size * 0.6;
                text.sections[2].style.color = label.color.with_alpha(0.5);

                *transform =
                    Transform::from_translation(Vec3::new(label.offset.x, label.offset.y, label.z));
            }
        }
    }
}

/// The bikes don't have GlobalTransform so there is no TransformPropagation
/// Instead, manually update the position of the text entities by setting it
/// to `bike_position + offset`
fn update_entity_label_positions(
    // we query the Position and not the Transform, because only the position is visually interpolated
    // so that's what we should use
    q_parents: Query<
        (&VisualInterpolateStatus<Position>, &EntityLabel),
        (Changed<Position>, Without<EntityLabelChild>),
    >,
    mut q_text: Query<(&Parent, &mut GlobalTransform), With<EntityLabelChild>>,
) {
    for (parent, mut transform) in q_text.iter_mut() {
        if let Ok((parent_pos, fl)) = q_parents.get(parent.get()) {
            if let Some(pos) = parent_pos.current_value {
                *transform =
                    GlobalTransform::from_translation(Vec3::from((pos.0 + fl.offset, fl.z)));
            }
        }
    }
}
