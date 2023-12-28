use bevy::{
	prelude::{
		Color, Component, Gizmos, Input, KeyCode, Plugin, PreUpdate, Query, Res,
		ResMut, Resource, Transform, Update, Vec3, With,
	},
	reflect::Reflect,
	time::Time,
};

use crate::pose_gizmo;

const SPEED: f32 = 4.;

pub struct KeyboardControllerPlugin;

impl Plugin for KeyboardControllerPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.register_type::<Direction>()
			.init_resource::<Direction>()
			.add_systems(PreUpdate, direction_from_keys)
			.add_systems(Update, move_controlled_entities);
	}
}

/// Any entity with this attached gets its transform controlled by the keyboard.
#[derive(Component, Debug, Default)]
pub struct KeyboardController;

/// Direction of movement. Controlled by keyboard.
#[derive(Resource, Debug, PartialEq, Eq, Clone, Default, Reflect)]
struct Direction {
	pub forward: bool,
	pub back: bool,
	pub left: bool,
	pub right: bool,
	pub up: bool,
	pub down: bool,
}

impl Direction {
	pub fn clear(&mut self) {
		self.forward = false;
		self.back = false;
		self.left = false;
		self.right = false;
		self.up = false;
		self.down = false;
	}

	fn velocity(&self) -> Vec3 {
		let mut v = Vec3::ZERO;
		fn f(dir_bool: bool, dir_vec: Vec3) -> Vec3 {
			dir_bool as u8 as f32 * dir_vec
		}

		v += f(self.forward, Vec3::NEG_Z);
		v += f(self.back, Vec3::Z);
		v += f(self.left, Vec3::NEG_X);
		v += f(self.right, Vec3::X);
		v += f(self.up, Vec3::Y);
		v += f(self.down, Vec3::NEG_Y);

		v
	}
}

fn direction_from_keys(
	mut direction: ResMut<Direction>,
	keypress: Res<Input<KeyCode>>,
) {
	direction.clear();
	if keypress.pressed(KeyCode::W) || keypress.pressed(KeyCode::Up) {
		direction.forward = true;
	}
	if keypress.pressed(KeyCode::S) || keypress.pressed(KeyCode::Down) {
		direction.back = true;
	}
	if keypress.pressed(KeyCode::A) || keypress.pressed(KeyCode::Left) {
		direction.left = true;
	}
	if keypress.pressed(KeyCode::D) || keypress.pressed(KeyCode::Right) {
		direction.right = true;
	}
	if keypress.pressed(KeyCode::Space) {
		direction.up = true;
	}
	if keypress.pressed(KeyCode::ShiftLeft) {
		direction.down = true;
	}
}

fn move_controlled_entities(
	direction: Res<Direction>,
	time: Res<Time>,
	mut trans: Query<&mut Transform, With<KeyboardController>>,
	mut gizmos: Gizmos,
) {
	for mut t in trans.iter_mut() {
		let local_vel = direction.velocity() * SPEED * time.delta_seconds();
		let parent_vel = (t.local_x() * local_vel.x)
			+ (t.local_y() * local_vel.y)
			+ (t.local_z() * local_vel.z);
		t.translation += parent_vel;
		pose_gizmo(&mut gizmos, &t, Color::GRAY);
	}
}
