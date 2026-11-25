use std::time::Instant;

use sdl3::event::{Event, WindowEvent};

use crate::{Assets, Audio, FixedTimestep, Input, Instance};

/// Window and loop settings for [`run`].
#[derive(Clone, Debug)]
pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
    /// Render the world at this resolution and scale it up to the window.
    pub logical_size: Option<(u32, u32)>,
    /// Only scale the logical image by whole numbers.
    pub integer_scale: bool,
    /// Fixed updates per second.
    pub updates_per_second: u32,
    pub vsync: bool,
    /// Simultaneous sound effects. 0 disables audio.
    pub audio_voices: usize,
}

impl Config {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            logical_size: None,
            integer_scale: true,
            updates_per_second: 60,
            vsync: true,
            audio_voices: 16,
        }
    }

    pub fn logical_size(mut self, width: u32, height: u32) -> Self {
        self.logical_size = Some((width, height));
        self
    }

    pub fn integer_scale(mut self, on: bool) -> Self {
        self.integer_scale = on;
        self
    }

    pub fn updates_per_second(mut self, ups: u32) -> Self {
        self.updates_per_second = ups;
        self
    }

    pub fn vsync(mut self, on: bool) -> Self {
        self.vsync = on;
        self
    }
}

/// Everything a game gets access to each update/draw.
pub struct Context {
    pub instance: Instance,
    pub input: Input,
    pub assets: Assets,
    /// `None` if no audio device could be opened.
    pub audio: Option<Audio>,
    /// Number of fixed updates run so far.
    pub tick: u64,
    quit: bool,
}

impl Context {
    /// Stop the loop after the current frame.
    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }
}

/// A game driven by [`run`].
pub trait Game {
    /// Called at a fixed rate (`Config::updates_per_second`).
    fn update(&mut self, ctx: &mut Context);

    /// Called once per rendered frame, after `clear` and before `present`.
    /// Draw the world first, call `ctx.instance.begin_gui()`, then draw the HUD.
    fn draw(&mut self, ctx: &mut Context);

    /// Every SDL event, before input state is updated. Default: nothing.
    fn event(&mut self, _ctx: &mut Context, _event: &Event) {}

    /// Background colour for the world each frame.
    fn clear_color(&self) -> crate::Color {
        crate::Color::RGB(0, 0, 0)
    }
}

/// Create the window and run `G` until it quits or the window is closed.
/// `init` runs once with the context so the game can load assets.
pub fn run<G, F>(config: Config, init: F)
where
    G: Game,
    F: FnOnce(&mut Context) -> G,
{
    let mut instance = Instance::new(&config.title, config.width, config.height);
    instance.set_vsync(config.vsync);
    if let Some((w, h)) = config.logical_size {
        instance.set_logical_size(w, h, config.integer_scale);
    }
    let assets = Assets::new(instance.texture_creator());
    let audio = match (config.audio_voices > 0)
        .then(|| Audio::new(instance.sdl(), config.audio_voices))
        .unwrap_or_else(|| Err("no voices requested".into()))
    {
        Ok(a) => Some(a),
        Err(e) => {
            eprintln!("frug: audio disabled: {e}");
            None
        }
    };
    let mut ctx = Context {
        instance,
        input: Input::new(),
        assets,
        audio,
        tick: 0,
        quit: false,
    };
    let mut game = init(&mut ctx);

    let mut timestep = FixedTimestep::new(config.updates_per_second);
    let mut last = Instant::now();

    while !ctx.quit {
        for event in ctx.instance.get_events() {
            game.event(&mut ctx, &event);
            match &event {
                Event::Quit { .. } => ctx.quit = true,
                Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } => ctx.input.reset(),
                _ => {}
            }
            ctx.input.handle_event(&event);
        }

        let now = Instant::now();
        let steps = timestep.advance((now - last).as_secs_f64());
        last = now;
        for _ in 0..steps {
            game.update(&mut ctx);
            ctx.input.clear_transitions();
            ctx.tick += 1;
            if ctx.quit {
                break;
            }
        }

        if let Some(audio) = ctx.audio.as_mut() {
            audio.update();
        }

        let color = game.clear_color();
        ctx.instance.clear(color);
        game.draw(&mut ctx);
        ctx.instance.present();

        if !config.vsync {
            // Avoid spinning a core when vsync is off.
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}
