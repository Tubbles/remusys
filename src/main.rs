extern crate ggez;

mod imgui_wrapper;

use crate::imgui_wrapper::ImGuiWrapper;

use ggez::conf;
use ggez::event::{self, Axis, Button, EventHandler, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Mesh, Text};
use ggez::input::mouse::set_cursor_hidden;
use ggez::timer;
use ggez::{Context, GameResult};
use glam::*;

mod event_system;
use event_system::{Event, EventBus, EventMetadata};
use std::sync::RwLock;

use std::time::Instant;

#[macro_use]
extern crate lazy_static;

struct MainState {
    has_imgui: bool,
    pos_x: f32,
    pos_y: f32,
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    width: f32,
    height: f32,
    start: Instant,
    first_loop: bool,
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32, start: Instant) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let this = MainState {
            has_imgui: false,
            pos_x: 0.0,
            pos_y: 0.0,
            imgui_wrapper,
            hidpi_factor,
            width: 0.0,
            height: 0.0,
            start,
            first_loop: true,
        };
        Ok(this)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let now = Instant::now();
        if !self.has_imgui {
            // self.pos_x = self.pos_x % 800.0 + 1.0;
        }
        // println!("{:?}", _ctx.timer_context);
        if self.first_loop {
            println!(
                "Time to first loop: {} s",
                now.duration_since(self.start).as_secs_f32()
            );
            self.first_loop = false;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // Render game stuff
        {
            let x = (self.pos_x + 1.0f32) * self.width / 2.0f32;
            let y = (self.pos_y + 1.0f32) * self.height / 2.0f32;
            // println!("circle x: {}, y: {}", x, y);
            let circle = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(
                    x, //
                    y, //
                ),
                0.10f32 * self.height,
                0.25f32,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &circle, DrawParam::new())?;
        }
        {
            let fps_counter = Text::new(format!("{:.0}", timer::fps(ctx)));
            // .set_bounds(Vec2::new(0.0, 0.0), graphics::Align::Left);
            graphics::draw(ctx, &fps_counter, DrawParam::new())?;
        }

        // Render game ui
        if self.has_imgui {
            self.imgui_wrapper.render(ctx, self.hidpi_factor);
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        // println!("mouse x: {}, y: {}", x, y);
        if self.has_imgui {
            self.imgui_wrapper.update_mouse_pos(x, y);
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if self.has_imgui {
            self.imgui_wrapper.update_mouse_down(button);
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        if self.has_imgui {
            self.imgui_wrapper.update_mouse_up(button);
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        if keycode == KeyCode::Escape && !repeat {
            self.has_imgui ^= true;
        }
        if keycode == KeyCode::Q && keymods.contains(KeyMods::CTRL) {
            ggez::event::quit(ctx);
        }
        if self.has_imgui {
            self.imgui_wrapper.update_key_down(keycode, keymods);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        if self.has_imgui {
            self.imgui_wrapper.update_key_up(keycode, keymods);
        }
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        if self.has_imgui {
            self.imgui_wrapper.update_text(val);
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
        self.width = width;
        self.height = height;
        // println!("{:?}", graphics::screen_coordinates(ctx));
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        if self.has_imgui {
            self.imgui_wrapper.update_scroll(x, y);
        }
    }

    fn gamepad_button_down_event(&mut self, ctx: &mut Context, button: Button, _id: GamepadId) {
        // println!("button: {:?}, _id: {:?}", button, _id);
        if button == Button::Mode {
            ggez::event::quit(ctx);
        }
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, _id: GamepadId) {
        // println!("axis: {:?}, value: {:?}, _id: {:?}", axis, value, _id);
        if axis == Axis::RightStickX {
            self.pos_x = value;
        }
        if axis == Axis::RightStickY {
            self.pos_y = -value;
        }
    }
}

pub fn main() -> GameResult {
    let start = Instant::now();
    main2();
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_setup(
        conf::WindowSetup::default()
            .title("super_simple with imgui")
            .vsync(true),
    );
    let (mut ctx, event_loop) = cb.build()?;
    set_cursor_hidden(&mut ctx, true);
    let state = MainState::new(&mut ctx, 1.0, start)?;
    event::run(ctx, event_loop, state)
}

struct NoEvent {
    i: i32,
}

lazy_static! {
    static ref NOEVENT_METADATA: RwLock<EventMetadata<NoEvent>> = RwLock::new(EventMetadata::new());
    static ref EVENT_BUS: EventBus = EventBus::new();
}

impl Event for NoEvent {
    fn event_metadata<F, R>(f: F) -> R
    where
        F: FnOnce(&EventMetadata<Self>) -> R,
    {
        f(&*NOEVENT_METADATA.read().unwrap())
    }

    fn mut_metadata<F, R>(f: F) -> R
    where
        F: FnOnce(&mut EventMetadata<Self>) -> R,
    {
        f(&mut *NOEVENT_METADATA.write().unwrap())
    }
}

fn test(e: &mut NoEvent) {
    println!("test {}", e.i);
    e.i += 1;
}

fn test2(e: &mut NoEvent) {
    println!("test2 {}", e.i);
}

fn main2() {
    let test_id = EVENT_BUS.register(test, 0);
    let mut event = NoEvent { i: 3 };
    EVENT_BUS.post(&mut event);
    EVENT_BUS.register(test2, 1);
    EVENT_BUS.post(&mut event);
    EVENT_BUS.unregister(test_id);
    EVENT_BUS.post(&mut event);
}
