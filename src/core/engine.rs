use crate::core::*;
use crate::rendering::*;
use crate::input::*;
use crate::world::*;

use miniquad::*;

pub struct GameEngine {
    game: Box<dyn Game>,
    _settings: GameSettings,
    input_engine: InputEngine,
    rendering_engine: RenderingEngine,
    world_engine: WorldEngine,
}
impl GameEngine {
    pub fn new(mut game: Box<dyn Game>, settings: GameSettings, mut ctx: &mut Context) -> Self {
        let input_engine = InputEngine::new();
        let mut rendering_engine = RenderingEngine::new(&mut ctx, settings.render_settings.clone());
        let mut world_engine = WorldEngine::new();
        let base_world = world_engine.create_world();
        world_engine.push(base_world);

        let emd = Emerald::new(&mut ctx, &mut world_engine, &mut rendering_engine);

        game.initialize(emd);

        GameEngine {
            game,
            _settings: settings,
            input_engine,
            rendering_engine,
            world_engine,
        }
    }
}
impl EventHandler for GameEngine {
    fn update(&mut self, mut ctx: &mut Context) {
        let emd = Emerald::new(&mut ctx, &mut self.world_engine, &mut self.rendering_engine);

        self.game.update(emd);

        self.rendering_engine.update(&mut ctx, self.world_engine.world_mut());
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.input_engine.key_down(keycode);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.input_engine.key_up(keycode);
    }

    fn draw(&mut self, mut ctx: &mut Context) {
        self.rendering_engine.update(&mut ctx, self.world_engine.world_mut());
    }
}