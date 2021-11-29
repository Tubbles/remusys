use std::collections::HashMap;
use std::marker::PhantomData;
extern crate uuid;
use uuid::Uuid;

// Note: This doesn't support Copy or Clone for safety reasons.
// More specifically, it should be impossible to unregister the same handler more than once.
pub struct EventHandlerId<T: Event + ?Sized> {
    id: Uuid,
    _t: PhantomData<T>,
}
impl<T: Event + ?Sized> Eq for EventHandlerId<T> {}
impl<T: Event + ?Sized> PartialEq for EventHandlerId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self._t == other._t
    }
}

struct EventHandler<T: Event + ?Sized> {
    priority: i32,
    f: fn(&mut T),
    id: EventHandlerId<T>,
}

pub struct EventMetadata<T: Event + ?Sized> {
    handlers: HashMap<&'static EventBus, Vec<EventHandler<T>>>,
}

impl<T: Event + ?Sized> EventMetadata<T> {
    pub fn new() -> EventMetadata<T> {
        EventMetadata {
            handlers: HashMap::new(),
        }
    }

    fn put(&mut self, bus: &'static EventBus, f: fn(&mut T), priority: i32) -> EventHandlerId<T> {
        let vec = self.handlers.entry(bus).or_insert_with(Vec::new);
        let pos = vec
            .binary_search_by(|a| a.priority.cmp(&priority))
            .unwrap_or_else(|e| e);
        let id = Uuid::new_v4();
        vec.insert(
            pos,
            EventHandler {
                f: f,
                priority: priority,
                id: EventHandlerId {
                    id: id,
                    _t: PhantomData,
                },
            },
        );
        EventHandlerId {
            id: id,
            _t: PhantomData,
        }
    }

    fn remove(&mut self, bus: &EventBus, f: EventHandlerId<T>) {
        let flag = self.handlers.get_mut(bus).iter_mut().any(|v| {
            v.retain(|x| x.id != f);
            v.is_empty()
        });
        if flag {
            self.handlers.remove(bus);
        }
    }

    #[inline]
    fn post(&self, bus: &EventBus, event: &mut T) -> bool {
        self.handlers
            .get(bus)
            .iter()
            .flat_map(|x| x.iter())
            .any(|h| {
                (h.f)(event);
                event.cancelled()
            })
    }
}

pub trait Event {
    // type properties
    fn event_metadata<F, R>(f: F) -> R
    where
        F: FnOnce(&EventMetadata<Self>) -> R;

    fn mut_metadata<F, R>(f: F) -> R
    where
        F: FnOnce(&mut EventMetadata<Self>) -> R;

    fn cancellable() -> bool {
        false
    }

    // instance properties
    fn cancelled(&self) -> bool {
        false
    }

    fn cancel(&self, _: bool) {
        panic!()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct EventBus {
    uuid: Uuid,
}

impl EventBus {
    pub fn new() -> EventBus {
        EventBus {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn register<T>(&'static self, f: fn(&mut T), priority: i32) -> EventHandlerId<T>
    where
        T: Event,
    {
        T::mut_metadata(|x| x.put(self, f, priority))
    }

    pub fn unregister<T>(&self, f: EventHandlerId<T>)
    where
        T: Event,
    {
        T::mut_metadata(|x| x.remove(self, f))
    }

    pub fn post<T>(&self, event: &mut T) -> bool
    where
        T: Event,
    {
        T::event_metadata(|x| x.post(self, event))
    }
}
