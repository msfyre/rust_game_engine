pub enum RuntimeEventTrigger {
    UPDATE,
    RENDER,
}

pub enum RuntimeEventCallback {
    Function(fn(f32)),
    Method(Box<dyn FnMut(f32) -> Result<(), sdl3::Error>>),
}

pub struct RuntimeEvent {
    pub trigger: RuntimeEventTrigger,
    pub callback: RuntimeEventCallback,
}
