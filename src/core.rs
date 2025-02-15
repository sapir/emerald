pub mod components;
pub mod engine;
pub mod error;
pub mod game;
pub mod game_settings;

pub use components::transform::*;
pub use components::*;
pub use engine::GameEngine;
pub use error::*;
pub use game::*;
pub use game_settings::*;

use crate::assets::*;
use crate::audio::*;
use crate::input::*;
use crate::logging::*;
use crate::profiling::profile_cache::ProfileCache;
use crate::profiling::profiler::Profiler;
use crate::rendering::*;

pub struct Emerald<'a, 'b, 'c> {
    delta: f32,
    fps: f64,
    audio_engine: &'c mut AudioEngine,
    quad_ctx: &'c mut miniquad::Context<'a, 'b>,
    rendering_engine: &'c mut RenderingEngine,
    logging_engine: &'c mut LoggingEngine,
    input_engine: &'c mut InputEngine,
    pub(crate) asset_store: &'c mut AssetStore,
    profile_cache: &'c mut ProfileCache,
}
impl<'a, 'b, 'c> Emerald<'a, 'b, 'c> {
    #[inline]
    pub(crate) fn new(
        delta: f32,
        fps: f64,
        quad_ctx: &'c mut miniquad::Context<'a, 'b>,
        audio_engine: &'c mut AudioEngine,
        input_engine: &'c mut InputEngine,
        logging_engine: &'c mut LoggingEngine,
        rendering_engine: &'c mut RenderingEngine,
        asset_store: &'c mut AssetStore,
        profile_cache: &'c mut ProfileCache,
    ) -> Self {
        Emerald {
            delta,
            fps,
            audio_engine,
            quad_ctx,
            rendering_engine,
            input_engine,
            logging_engine,
            asset_store,
            profile_cache,
        }
    }

    pub fn set_asset_folder_root(&mut self, root: String) {
        self.asset_store.set_asset_folder_root(root);
    }

    pub fn set_user_data_folder_root(&mut self, root: String) {
        self.asset_store.set_user_data_folder_root(root);
    }

    pub fn get_asset_folder_root(&mut self) -> String {
        self.asset_store.get_asset_folder_root()
    }

    pub fn get_user_data_folder_root(&mut self) -> String {
        self.asset_store.get_user_data_folder_root()
    }

    // ************* General API ***************
    #[inline]
    pub fn delta(&self) -> f32 {
        self.delta
    }

    /// WARNING: This overrides the delta value set by the emerald engine.
    #[inline]
    pub fn set_delta(&mut self, delta: f32) {
        self.delta = delta;
    }

    /// Time since Epoch
    #[inline]
    pub fn now(&self) -> f64 {
        miniquad::date::now()
    }

    #[inline]
    pub fn screen_size(&self) -> (f32, f32) {
        let s = self.quad_ctx.screen_size();
        let dpi = self.quad_ctx.dpi_scale();
        (s.0 * dpi, s.1 * dpi)
    }

    #[inline]
    pub fn fps(&self) -> f64 {
        self.fps
    }

    pub fn quit(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.audio_engine.clear().ok();
        }

        self.quad_ctx.quit()
    }
    // *****************************************

    pub fn graphics(&mut self) -> GraphicsHandler<'a, 'b, '_> {
        GraphicsHandler::new(
            &mut self.quad_ctx,
            &mut self.asset_store,
            &mut self.rendering_engine,
        )
    }

    pub fn profiler<T: Into<String>>(&mut self, profile_name: T) -> Profiler<'_> {
        let now = self.now();
        
        Profiler::new(&mut self.profile_cache, profile_name, now)
    }

    // ************* Asset API ************* //
    #[inline]
    pub fn loader(&mut self) -> AssetLoader<'a, 'b, '_> {
        AssetLoader::new(
            &mut self.quad_ctx,
            &mut self.asset_store,
            &mut self.rendering_engine,
            &mut self.audio_engine,
        )
    }

    #[inline]
    pub fn writer(&mut self) -> Writer {
        Writer::new(self.get_user_data_folder_root())
    }
    // ************************************* //

    // ************* Audio API ************* //
    #[inline]
    pub fn audio(&mut self) -> AudioHandler<'_> {
        AudioHandler::new(&mut self.audio_engine, &mut self.asset_store)
    }
    // ************************************* //

    /// Logging
    // ************* Logging API ************* //
    #[inline]
    pub fn logger(&mut self) -> &mut LoggingEngine {
        &mut self.logging_engine
    }
    // ************************************* //

    // ************* Input API ************* //
    #[inline]
    pub fn input(&mut self) -> InputHandler {
        InputHandler::new(self.input_engine)
    }

    /// Makes all touches also be registered as mouse events.
    #[inline]
    pub fn touches_to_mouse(&mut self, enabled: bool) {
        self.input_engine.touches_to_mouse = enabled;
    }

    /// Makes mouse clicks treated as touch event.
    #[inline]
    pub fn mouse_to_touch(&mut self, enabled: bool) {
        self.input_engine.mouse_to_touch = enabled;
    }

    #[inline]
    pub fn set_key_pressed(&mut self, keycode: KeyCode, is_pressed: bool) {
        if is_pressed {
            self.input_engine.set_key_down(keycode, false);
        } else {
            self.input_engine.set_key_up(keycode);
        }
    }
    // ************************************* //
}
