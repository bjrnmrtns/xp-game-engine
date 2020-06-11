use std::hash::Hash;
use std::fmt::Display;
use std::ops::Index;
use std::collections::HashMap;

pub trait WidgetId: Clone + PartialEq + Eq + Hash + Send + Sync + Display + 'static {
    fn generate(last: &Option<Self>) -> Self;
}

pub trait Widget {}

impl WidgetId for u32 {
    fn generate(last: &Option<Self>) -> Self { last.map(|last| last + 1).unwrap_or(0) }
}

pub struct Widgets<T: Widget, I: WidgetId = u32> {
    items: HashMap<I, T>,
    last_key: Option<I>,
}

impl<T, I> Widgets<T, I> where T: Widget, I: WidgetId, {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            last_key: None,
        }
    }
    pub fn add(&mut self, widget: T) -> I {
        let id = I::generate(&self.last_key);
        self.items.insert(id.clone(), widget);
        id
    }
}

impl<T, I> Index<I> for Widgets<T, I>
    where T: Widget, I: WidgetId,
{
    type Output = T;
    fn index(&self, id: I) -> &Self::Output { &self.items[&id] }
}
