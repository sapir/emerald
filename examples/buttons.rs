use emerald::*;

pub fn main() {
    let mut settings = GameSettings::default();
    let render_settings = RenderSettings {
        resolution: (320 * 3, 180 * 3),
        ..Default::default()
    };
    settings.render_settings = render_settings;
    emerald::start(
        Box::new(MyGame {
            world: World::new(),
        }),
        settings,
    )
}

pub struct MyGame {
    world: World,
}
impl Game for MyGame {
    fn initialize(&mut self, mut emd: Emerald) {
        emd.set_asset_folder_root(String::from("./examples/assets/"));
        let unpressed = emd.loader().texture("button_unpressed.png").unwrap();
        let pressed = emd.loader().texture("button_pressed.png").unwrap();
        emd.touches_to_mouse(true);

        self.world.spawn((
            Transform::default(),
            UIButton::new(pressed.clone(), unpressed.clone()),
        ));
        self.world.spawn((
            Transform::from_translation((320.0, 180.0)),
            UIButton::new(pressed.clone(), unpressed.clone()),
        ));
        self.world.spawn((
            Transform::from_translation((-320.0, -180.0)),
            UIButton::new(pressed.clone(), unpressed.clone()),
        ));
    }

    fn update(&mut self, mut emd: Emerald) {
        ui_button_system(&mut emd, &mut self.world);
    }

    fn draw(&mut self, mut emd: Emerald) {
        emd.graphics().begin().ok();
        emd.graphics().draw_world(&mut self.world).ok();
        emd.graphics().render().ok();
    }
}
