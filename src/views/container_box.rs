use crate::{
    app::AppContext,
    id::Id,
    view::{ChangeFlags, View},
};

pub struct ContainerBox {
    id: Id,
    child: Box<dyn View>,
}

pub fn container_box(
    cx: AppContext,
    child: impl FnOnce(AppContext) -> Box<dyn View>,
) -> ContainerBox {
    let id = cx.new_id();
    let mut child_cx = cx;
    child_cx.id = id;
    let child = child(child_cx);
    ContainerBox { id, child }
}

impl View for ContainerBox {
    fn id(&self) -> Id {
        self.id
    }

    fn child(&mut self, id: Id) -> Option<&mut dyn View> {
        if self.child.id() == id {
            Some(&mut *self.child)
        } else {
            None
        }
    }

    fn update(
        &mut self,
        cx: &mut crate::context::UpdateCx,
        state: Box<dyn std::any::Any>,
    ) -> crate::view::ChangeFlags {
        ChangeFlags::empty()
    }

    fn layout(&mut self, cx: &mut crate::context::LayoutCx) -> taffy::prelude::Node {
        cx.layout_node(self.id, true, |cx| vec![self.child.layout(cx)])
    }

    fn compute_layout(&mut self, cx: &mut crate::context::LayoutCx) {
        self.child.compute_layout_main(cx);
    }

    fn event(
        &mut self,
        cx: &mut crate::context::EventCx,
        id_path: Option<&[Id]>,
        event: crate::event::Event,
    ) -> bool {
        self.child.event_main(cx, id_path, event)
    }

    fn paint(&mut self, cx: &mut crate::context::PaintCx) {
        self.child.paint_main(cx);
    }
}
