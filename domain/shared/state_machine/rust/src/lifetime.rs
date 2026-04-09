pub trait SystemInit<C> {
    fn init_system(config: C) -> anyhow::Result<()>;
}

pub trait SystemDispose {
    fn dispose_system() -> anyhow::Result<()>;
}
