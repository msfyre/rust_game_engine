pub struct VectorFloat {
    pub x: f32,
    pub y: f32,
}

pub struct VectorUnsigned {
    pub x: u32,
    pub y: u32,
}

// Runtime Types
trait Callback<Args> {
    fn call(&self, args: Args);
}

impl<Func, Args> Callback<Args> for Func
where
    Func: Fn(Args),
{
    fn call(&self, args: Args) {
        self(args);
    }
}
