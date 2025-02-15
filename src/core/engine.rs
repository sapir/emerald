use crate::assets::*;
use crate::audio::*;
use crate::core::*;
use crate::input::*;
use crate::logging::*;
use crate::rendering::*;

use miniquad::*;
use std::collections::VecDeque;

pub struct GameEngine {
    game: Box<dyn Game>,
    _settings: GameSettings,
    audio_engine: AudioEngine,
    input_engine: InputEngine,
    logging_engine: LoggingEngine,
    rendering_engine: RenderingEngine,
    last_instant: f64,
    fps_tracker: VecDeque<f64>,
    asset_store: AssetStore,
    profile_cache: ProfileCache,
}
impl<'c> GameEngine {
    pub fn new(
        mut game: Box<dyn Game>,
        settings: GameSettings,
        mut ctx: &mut Context<'_, 'c>,
    ) -> Self {
        let mut asset_store = AssetStore::new(ctx, settings.title.clone()).unwrap();
        let mut logging_engine = LoggingEngine::new();
        let mut audio_engine = AudioEngine::new();
        let mut input_engine = InputEngine::new();
        let mut rendering_engine =
            RenderingEngine::new(&mut ctx, settings.render_settings.clone(), &mut asset_store);

        let mut profile_cache = ProfileCache::new(Default::default());

        let delta = 0.0;
        let starting_amount = 50;
        let mut fps_tracker = VecDeque::with_capacity(starting_amount);
        fps_tracker.resize(starting_amount, 1.0 / 60.0);
        let last_instant = miniquad::date::now();

        let emd = Emerald::new(
            delta,
            0.0,
            &mut ctx,
            &mut audio_engine,
            &mut input_engine,
            &mut logging_engine,
            &mut rendering_engine,
            &mut asset_store,
            &mut profile_cache,
        );

        game.initialize(emd);

        GameEngine {
            game,
            fps_tracker,
            _settings: settings,
            audio_engine,
            input_engine,
            logging_engine,
            rendering_engine,
            last_instant,
            asset_store,
            profile_cache,
        }
    }

    /// Return frame rate averaged out over last N frames
    /// https://github.com/17cupsofcoffee/tetra/blob/master/src/time.rs
    #[inline]
    pub fn get_fps(&self) -> f64 {
        1.0 / (self.fps_tracker.iter().sum::<f64>() / self.fps_tracker.len() as f64)
    }

    #[inline]
    fn update_fps_tracker(&mut self, delta: f64) {
        self.fps_tracker.pop_front();
        self.fps_tracker.push_back(delta);
    }
}
impl<'a, 'b> EventHandler for GameEngine {
    #[inline]
    fn update(&mut self, mut ctx: &mut Context<'_, '_>) {
        let start_of_frame = miniquad::date::now();
        let delta = start_of_frame - self.last_instant;
        self.last_instant = start_of_frame;

        self.update_fps_tracker(delta);

        let emd = Emerald::new(
            delta as f32,
            self.get_fps(),
            &mut ctx,
            &mut self.audio_engine,
            &mut self.input_engine,
            &mut self.logging_engine,
            &mut self.rendering_engine,
            &mut self.asset_store,
            &mut self.profile_cache,
        );

        self.game.update(emd);
        self.logging_engine.update().unwrap();
        self.input_engine.update_and_rollover().unwrap();
        self.audio_engine.post_update().unwrap();
    }

    #[inline]
    fn key_down_event(
        &mut self,
        _ctx: &mut Context<'_, '_>,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        self.input_engine.set_key_down(keycode, repeat);
    }

    #[inline]
    fn key_up_event(&mut self, _ctx: &mut Context<'_, '_>, keycode: KeyCode, _keymods: KeyMods) {
        self.input_engine.set_key_up(keycode);
    }

    #[inline]
    fn mouse_motion_event(&mut self, ctx: &mut Context<'_, '_>, x: f32, y: f32) {
        let y = ctx.screen_size().1 - y;
        self.input_engine.set_mouse_translation(x, y)
    }

    #[inline]
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context<'_, '_>,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let y = ctx.screen_size().1 - y;
        self.input_engine.set_mouse_down(button, x, y)
    }

    #[inline]
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context<'_, '_>,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let y = ctx.screen_size().1 - y;
        self.input_engine.set_mouse_up(button, x, y)
    }

    #[inline]
    fn touch_event(
        &mut self,
        ctx: &mut Context<'_, '_>,
        phase: TouchPhase,
        id: u64,
        x: f32,
        y: f32,
    ) {
        let y = ctx.screen_size().1 - y;
        self.input_engine.touch_event(phase, id, x, y)
    }

    #[inline]
    fn draw(&mut self, mut ctx: &mut Context<'_, '_>) {
        let start_of_frame = miniquad::date::now();
        let delta = start_of_frame - self.last_instant;

        self.rendering_engine
            .pre_draw(ctx, &mut self.asset_store)
            .unwrap();
        let emd = Emerald::new(
            delta as f32,
            self.get_fps(),
            &mut ctx,
            &mut self.audio_engine,
            &mut self.input_engine,
            &mut self.logging_engine,
            &mut self.rendering_engine,
            &mut self.asset_store,
            &mut self.profile_cache,
        );

        self.game.draw(emd);
        ctx.commit_frame();

        self.rendering_engine.post_draw(ctx, &mut self.asset_store);
    }
}
