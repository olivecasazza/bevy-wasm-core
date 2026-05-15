//! Theme plugin - applies ui-theme styling to bevy_egui.

use bevy::prelude::*;
use bevy_egui::EguiContexts;

/// Plugin that applies the shared ui-theme styling to egui.
pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ThemeApplied>()
            .add_systems(Update, apply_theme_system);
    }
}

#[derive(Resource, Default)]
struct ThemeApplied(bool);

fn apply_theme_system(mut contexts: EguiContexts, mut applied: ResMut<ThemeApplied>) {
    if applied.0 {
        return;
    }
    let ctx = contexts.ctx_mut();
    ui_theme::apply_style(ctx);
    applied.0 = true;
}
