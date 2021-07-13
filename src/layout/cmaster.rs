use crate::layout::Layout;
use crate::stack::Stack;
use crate::x::{Connection, WindowGeometry, WindowId};
use crate::Viewport;

#[derive(Clone)]
pub struct CenterMaster {
    name: String,
    resized_width: i16,
    tile_resized_width: i16,
    outergaps: u32,
    innergaps: u32,
}

impl Layout for CenterMaster {
    fn name(&self) -> &str {
        &self.name
    }
    fn layout(
        &self,
        connection: &Connection,
        viewport: &Viewport,
        stack: &Stack<WindowId>,
        master: &Option<WindowId>,
    ) {
        if stack.is_empty() {
            return;
        }
        let master_id = if master.is_none() {
            stack.focused().unwrap()
        } else {
            master.as_ref().unwrap()
        };
        if stack.len() < 3{
            let mut tile_layout: super::tile::TileLayout = super::tile::TileLayout::new("tmp_tl_cmaster", self.innergaps, self.outergaps);
            tile_layout.resized_width = self.tile_resized_width;
            tile_layout.layout(connection, viewport, stack, &Some(*master_id));
        }else {
            self.c_master(connection, viewport, stack, master_id);
        }
    }

    fn decrease_master(&mut self, viewport: &Viewport, resize_amount: i16) {
        if !(self.resized_width > (viewport.width/8) as i16) {
            self.resized_width += resize_amount;
        }
        if self.tile_resized_width > -((viewport.width / 2) as i16 - (viewport.width / 8) as i16) {
            self.tile_resized_width -= resize_amount;
        }
    }

    fn increase_master(&mut self, viewport: &Viewport, resize_amount: i16) {
        if !(self.resized_width < -((viewport.width/14) as i16)) {
            self.resized_width -= resize_amount;
        }
        if self.tile_resized_width < ((viewport.width / 2) as i16 - (viewport.width / 8) as i16) {
            self.tile_resized_width += resize_amount;
        }
    }
}

impl CenterMaster {
    pub fn new<S: Into<String>>(name: S, innergaps: u32, outergaps: u32) -> CenterMaster {
        Self {
            name: name.into(),
            resized_width: 160,
            innergaps,
            outergaps,
            tile_resized_width: 160
        }
    }

    fn c_master(
        &self,
        connection: &Connection,
        viewport: &Viewport,
        stack: &Stack<WindowId>,
        focused_id: &WindowId,
    ) {
        self.configure_master_window(connection, viewport, focused_id);
        let mut accumulator = 0;
        for window_id in stack.iter() {
            if window_id != focused_id {
                self.configure_normal_window(accumulator, connection, stack, viewport, window_id);
                accumulator += 1;
            }
        }
    }

    fn configure_normal_window(
        &self,
        i: u32,
        connection: &Connection,
        stack: &Stack<WindowId>,
        viewport: &Viewport,
        window_id: &WindowId,
    ) {
        let normal_geometry = self.get_normal_geometry(i, stack, viewport);
        connection.configure_window(window_id, &normal_geometry);
    }

    fn configure_master_window(
        &self,
        connection: &Connection,
        viewport: &Viewport,
        window_id: &WindowId,
    ) {
        let master_geometry = self.get_master_geometry(viewport);
        connection.configure_window(window_id, &master_geometry);
    }

    fn get_normal_geometry(
        &self,
        i: u32,
        stack: &Stack<WindowId>,
        viewport: &Viewport,
    ) -> WindowGeometry {
        let master_width: u32 = viewport.width / 2 + viewport.width / 16;
        let mut width: u32 = (self.resized_width + ((viewport.width - master_width) / 2) as i16) as u32;
        let stack_length: u32 = stack.len() as u32;
        let height;
        let y : u32;
        let x : u32;
        if i % 2 == 0 {
            let left_stack_len: u32 = stack_length / 2;
            height = (viewport.height - self.outergaps*2 + self.innergaps) / left_stack_len;
            x = self.outergaps;
            if stack.len() % 2 == 0 {
                y = i * (viewport.height - self.outergaps * 2 + self.innergaps) / stack_length;
            } else {
                y = i * (viewport.height - self.outergaps * 2 + self.innergaps)  / (stack_length - 1);
            }
        } else {
            let right_stack_len: u32 = (stack_length - 1) / 2;
            x = (self.resized_width * -1
                + (master_width + (viewport.width - master_width) / 2) as i16)
                as u32
                + self.outergaps;
            height = (viewport.height - self.outergaps*2 + self.innergaps) / right_stack_len;
            width -= self.outergaps * 2;
            if stack.len() % 2 == 0 {
                y = if right_stack_len < 2 {
                    0
                } else {
                    (i - 1) * (viewport.height - self.outergaps *2 + self.innergaps)  / (stack_length - 2)
                };
            } else {
                y = (i - 1) * (viewport.height - self.outergaps * 2 + self.innergaps)  / (stack_length - 1);
            }
        }
        WindowGeometry {
            x,
            y: y + self.outergaps,
            width,
            height: height - self.innergaps,
        }
    }

    fn get_master_geometry(&self, viewport: &Viewport) -> WindowGeometry {
        let width = ((self.resized_width * -2) + (viewport.width / 2 + viewport.width / 16) as i16)
            as u32
            - self.innergaps * 2;
        let x = (viewport.width - width) / 2 + self.outergaps;
        WindowGeometry {
            x,
            y: self.outergaps,
            width,
            height: viewport.height - self.outergaps * 2,
        }
    }
}
