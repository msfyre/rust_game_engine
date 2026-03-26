use crate::runtime::Runtime;

pub enum RuntimeEventTrigger {
    UPDATE,
    RENDER,
}

pub enum RuntimeEventCallback {
    Function(fn(f32) -> Result<(), sdl3::Error>),
    Method(Box<dyn FnMut(f32) -> Result<(), sdl3::Error>>),
    MethodWithRuntimeReference(Box<dyn FnMut(f32, &Runtime) -> Result<(), sdl3::Error>>),
    MethodWithMutableRuntimeReference(Box<dyn FnMut(f32, &mut Runtime) -> Result<(), sdl3::Error>>),
}

pub struct RuntimeEvent {
    pub trigger: RuntimeEventTrigger,
    pub callback: RuntimeEventCallback,
}
