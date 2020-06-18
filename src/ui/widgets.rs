use std::hash::Hash;
use std::fmt::Display;
use std::ops::Index;
use std::collections::HashMap;
use std::collections::hash_map::{Values, Keys};
use crate::ui::Label;

pub trait WidgetId: Clone + PartialEq + Eq + Hash + Send + Sync + Display + 'static {
    fn generate(last: &Option<Self>) -> Self;
}

pub enum Widget {
    LabelW(Label),
}

impl WidgetId for u32 {
    fn generate(last: &Option<Self>) -> Self { last.map(|last| last + 1).unwrap_or(0) }
}

pub struct Widgets<I: WidgetId = u32> {
    items: HashMap<I, Widget>,
    last_key: Option<I>,
}

impl<I> Widgets<I> where I: WidgetId, {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            last_key: None,
        }
    }
    pub fn add(&mut self, widget: Widget) -> I {
        let id = I::generate(&self.last_key);
        self.items.insert(id.clone(), widget);
        self.last_key = Some(id.clone());
        id
    }

    pub fn get(&self, id: I) -> Option<&Widget> {
        self.items.get(&id)
    }

    pub fn get_mut(&mut self, id: I) -> Option<&mut Widget> {
        self.items.get_mut(&id)
    }

    pub fn widgets(&self) -> Values<'_, I, Widget> {
        self.items.values()
    }

    pub fn ids(&self) -> Keys<'_, I, Widget> {
        self.items.keys()
    }
}

impl<I> Index<I> for Widgets<I> where I: WidgetId,
{
    type Output = Widget;
    fn index(&self, id: I) -> &Self::Output { &self.items[&id] }
}
