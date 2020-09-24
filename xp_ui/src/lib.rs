use std::collections::HashMap;

mod action;
mod label;
mod layout;
mod text;
mod widgets;

pub use self::{action::*, label::*, layout::*, text::*, widgets::*};
use crate::Widget::LabelW;

pub struct UI<'closure_lifetime, ClosureContext, I: WidgetId = u32> {
    cursor_position: (f32, f32),
    pub window_size: (f32, f32),
    widgets: Widgets<I>,
    actions: HashMap<(I, ActionType), Box<dyn Fn(&mut ClosureContext) + 'closure_lifetime>>,
}

impl<'closure_lifetime, ClosureContext, I> UI<'closure_lifetime, ClosureContext, I>
where
    I: WidgetId,
{
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            cursor_position: (width / 2.0, height / 2.0),
            window_size: (width, height),
            widgets: Widgets::new(),
            actions: HashMap::new(),
        }
    }

    pub fn add(&mut self, widget: Widget) -> I {
        self.widgets.add(widget)
    }

    pub fn try_get(&mut self, id: I) -> Option<&Widget> {
        self.widgets.get(id)
    }

    pub fn try_get_mut_label(&mut self, id: I) -> Option<&mut Label> {
        if let Some(LabelW(_, data)) = self.widgets.get_mut(id) {
            return Some(data);
        }
        None
    }

    pub fn update_window_size(&mut self, width: f32, height: f32) {
        self.window_size = (width, height);
        self.layout();
    }

    pub fn update_cursor_position(&mut self, x: f32, y: f32) {
        self.cursor_position = (x, self.window_size.1 - y);
    }

    pub fn layout(&mut self) {
        layout::layout_basic(self.widgets.widgets_mut(), self.window_size);
    }

    pub fn add_action_for_id<Closure: Fn(&mut ClosureContext) + 'closure_lifetime>(
        &mut self,
        id: I,
        action_type: ActionType,
        action: Closure,
    ) {
        self.actions.insert((id, action_type), Box::new(action));
    }

    fn inside(&self, layout: &Layout) -> bool {
        let left = layout.position.x;
        let right = layout.position.x + layout.size.width;
        let up = layout.position.y;
        let down = layout.position.y - layout.size.height;

        left < self.cursor_position.0
            && self.cursor_position.0 < right
            && down < self.cursor_position.1
            && self.cursor_position.1 < up
    }

    pub fn click(&self, context: &mut ClosureContext) {
        for ((id, action_type), action) in &self.actions {
            match ((id, action_type), action) {
                ((id, ActionType::OnClick), action) => match &self.widgets.widgets()[id] {
                    Widget::LabelW(layout, _) => {
                        if self.inside(layout) {
                            action(context);
                        }
                    }
                },
            }
        }
    }
    pub fn widgets(&self) -> &HashMap<I, Widget> {
        self.widgets.widgets()
    }
}
