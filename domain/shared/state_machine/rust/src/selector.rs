use std::fmt::Debug;

use crate::domain::Domain;

pub trait DataSink<T>: Send + Sync {
    fn send(&self, value: T) -> anyhow::Result<()>;
}

pub trait UntypedSelector<S>: Send + Sync {
    fn update(&mut self, state: &S) -> anyhow::Result<()>;
}

pub struct TypedSelector<S, T>
where
    T: PartialEq + Clone + Send + Sync + Debug,
{
    pub mapper: Box<dyn Fn(&S) -> T + Send + Sync>,
    pub last_value: Option<T>,
    pub sink: Box<dyn DataSink<T>>,
}

impl<S, T> UntypedSelector<S> for TypedSelector<S, T>
where
    T: PartialEq + Clone + Send + Sync + Debug,
{
    fn update(&mut self, state: &S) -> anyhow::Result<()> {
        let new_value = (self.mapper)(state);
        if Some(&new_value) != self.last_value.as_ref() {
            self.sink.send(new_value.clone())?;
            self.last_value = Some(new_value);
        } else {
        }
        Ok(())
    }
}

pub trait Selectable<D: Domain> {
    fn from_state(state: &D::State) -> Self;
}
