use macroquad::prelude::*;
use std::path::PathBuf;

use wasmi::*;

use crate::{HostState, mem, read_str};

pub fn setup(linker: &mut Linker<HostState>, project_path: PathBuf) -> Result<(), Error> {
    linker.func_wrap(
        "graphics",
        "draw_text",
        |caller: Caller<'_, HostState>,
         text_ptr: u32,
         text_len: u32,
         font_ptr: u32,
         font_len: u32,
         x: i32,
         y: i32,
         color: i32| {
            draw_text(
                &read_str(&caller, text_ptr, text_len),
                x as f32,
                y as f32,
                15.,
                caller.data().col(color),
            );
        },
    )?;

    linker.func_wrap(
        "fs",
        "load_file",
        move |caller: Caller<'_, HostState>,
              path: u32,
              path_len: u32,
              buf: u32,
              buf_len: u32|
              -> u32 {
            let p = read_str(&caller, path, path_len);
            let file = std::fs::read(project_path.join(p)).unwrap();
            mem(&caller)
                .write(caller, buf as usize, &file[..buf_len as usize])
                .unwrap();
            0
        },
    )?;

    linker.func_wrap(
        "graphics",
        "clear_screen",
        |caller: Caller<'_, HostState>, color: i32| -> () {
            clear_background(caller.data().col(color));
        },
    )?;

    linker.func_wrap("net", "get_me", |_caller: Caller<'_, HostState>| -> u32 {
        0
    })?;

    linker.func_wrap(
        "net",
        "get_peers",
        |_caller: Caller<'_, HostState>| -> u32 { 0 },
    )?;

    linker.func_wrap(
        "input",
        "read_buttons",
        |_caller: Caller<'_, HostState>, arg0: u32| -> u32 { 0 },
    )?;

    linker.func_wrap(
        "input",
        "read_pad",
        |_caller: Caller<'_, HostState>, arg0: u32| -> u32 { 0 },
    )?;

    linker.func_wrap(
        "graphics",
        "draw_circle",
        |caller: Caller<'_, HostState>,
         x: i32,
         y: i32,
         diameter: u32,
         fill_color: i32,
         stroke_color: i32,
         stroke_width: i32|
         -> () {
            let r = diameter as f32 / 2.;
            draw_circle(x as f32 + r, y as f32 + r, r, caller.data().col(fill_color));
            if stroke_width > 0 {
                draw_circle_lines(
                    x as f32 + r,
                    y as f32 + r,
                    r,
                    stroke_width as f32,
                    caller.data().col(stroke_color),
                )
            }
        },
    )?;

    linker.func_wrap(
        "graphics",
        "draw_rect",
        |caller: Caller<'_, HostState>,
         x: i32,
         y: i32,
         w: i32,
         h: i32,
         fill_color: i32,
         stroke_color: i32,
         stroke_width: i32|
         -> () {
            draw_rectangle(
                x as f32,
                y as f32,
                w as f32,
                h as f32,
                caller.data().col(fill_color),
            );
            if stroke_width > 0 {
                draw_rectangle_lines(
                    x as f32,
                    y as f32,
                    w as f32,
                    h as f32,
                    stroke_width as f32,
                    caller.data().col(stroke_color),
                );
            }
        },
    )?;
    Ok(())
}
