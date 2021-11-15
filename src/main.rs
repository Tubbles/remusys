//! The simplest possible example that does something.
// #![allow(clippy::unnecessary_wraps)]

extern crate ggez;

mod imgui_wrapper;

use crate::imgui_wrapper::ImGuiWrapper;

use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Mesh, Text};
use ggez::timer;
use ggez::{Context, GameResult};
use glam::*;

struct MainState {
    has_imgui: bool,
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let s = MainState {
            has_imgui: false,
            pos_x: 0.0,
            imgui_wrapper,
            hidpi_factor,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.has_imgui {
            self.pos_x = self.pos_x % 800.0 + 1.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // Render game stuff
        {
            let circle = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(self.pos_x, 380.0),
                100.0,
                2.0,
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
        //println!("{:?}", graphics::screen_coordinates(ctx));
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        if self.has_imgui {
            self.imgui_wrapper.update_scroll(x, y);
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_setup(
        conf::WindowSetup::default()
            .title("super_simple with imgui")
            .vsync(false),
    );
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx, 1.0)?;
    event::run(ctx, event_loop, state)
}
