mod event_system;
mod imgui_wrapper;
mod main_state;

#[macro_use]
extern crate lazy_static;

use event_system::{Event, EventBus, EventMetadata};
use ggez::conf;
use ggez::event;
use ggez::input::mouse::set_cursor_hidden;
use ggez::GameResult;
use main_state::MainState;
use std::sync::RwLock;
use std::time::Instant;

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
