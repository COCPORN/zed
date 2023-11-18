use crate::prelude::*;
use gpui::{prelude::*, AnyElement};
use smallvec::SmallVec;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ToastOrigin {
    #[default]
    Bottom,
    BottomRight,
}

/// Don't use toast directly:
///
/// - For messages with a required action, use a `NotificationToast`.
/// - For messages that convey information, use a `StatusToast`.
///
/// A toast is a small, temporary window that appears to show a message to the user
/// or indicate a required action.
///
/// Toasts should not persist on the screen for more than a few seconds unless
/// they are actively showing the a process in progress.
///
/// Only one toast may be visible at a time.
pub struct Toast<V: 'static> {
    origin: ToastOrigin,
    children: SmallVec<[AnyElement<V>; 2]>,
}

impl<V: 'static> Element<V> for Toast<V> {
    type State = Option<AnyElement<V>>;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        _element_state: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (gpui::LayoutId, Self::State) {
        let mut element = self.render(view_state, cx).into_any();
        let layout_id = element.layout(view_state, cx);
        (layout_id, Some(element))
    }

    fn paint(
        self,
        bounds: gpui::Bounds<gpui::Pixels>,
        view_state: &mut V,
        element: &mut Self::State,
        cx: &mut ViewContext<V>,
    ) {
        let element = element.take().unwrap();
        element.paint(view_state, cx);
    }
}

impl<V: 'static> Toast<V> {
    pub fn new(origin: ToastOrigin) -> Self {
        Self {
            origin,
            children: SmallVec::new(),
        }
    }

    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let mut div = div();

        if self.origin == ToastOrigin::Bottom {
            div = div.right_1_2();
        } else {
            div = div.right_2();
        }

        div.z_index(5)
            .absolute()
            .bottom_9()
            .flex()
            .py_1()
            .px_1p5()
            .rounded_lg()
            .shadow_md()
            .overflow_hidden()
            .bg(cx.theme().colors().elevated_surface_background)
            .children(self.children)
    }
}

impl<V: 'static> ParentComponent<V> for Toast<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui::{Div, Render};

    use crate::{Label, Story};

    use super::*;

    pub struct ToastStory;

    impl Render for ToastStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Toast<Self>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Toast::new(ToastOrigin::Bottom).child(Label::new("label")))
        }
    }
}
