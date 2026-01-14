use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use firefly_hal::{DeviceConfig, DeviceImpl};
use firefly_runtime::{FullID, NetHandler};
use kaolin::{prelude::*, style::TextStyle};
use macroquad::prelude::*;

mod catalog;
mod drawing;
mod input;
mod ui;

struct HostState {
    screen: Image,
}

#[macroquad::main("fireflydroid")]
async fn main() {
    set_panic_handler(|msg, backtrace| async move {
        let mut ui = ui::Renderer::new(screen_width() as i32, screen_height() as i32);

        loop {
            clear_background(RED);
            ui.draw(|k| {
                k.styled(
                    FlexStyle::new()
                        .background_color(GRAY.into())
                        .layout(Layout::new().direction(Direction::TopToBottom))
                        .sizing(sizing!(grow!())),
                    |k| {
                        let style = TextStyle::new().font_size(80.0).color(BLACK.into());
                        k.text(&msg, style).text(&backtrace, style)
                    },
                )
            });
            next_frame().await;
        }
    });

    catalog::list().await;
}

async fn play(id: &FullID) -> Result<(), firefly_runtime::Error> {
    let project_path = dir();

    let state = HostState {
        screen: Image {
            width: 240,
            height: 160,
            bytes: vec![0; 240 * 160 * 4],
        },
    };

    let device = DeviceConfig {
        root: project_path.to_owned(),
        ..DeviceConfig::default()
    };

    let device = DeviceImpl::new(device);
    let config = firefly_runtime::RuntimeConfig {
        id: Some(id.clone()),
        device,
        display: state,
        net_handler: NetHandler::None,
    };
    let mut runtime = firefly_runtime::Runtime::new(config)?;
    runtime.start()?;
    loop {
        clear_background(GRAY);

        let exit = runtime.update()?;
        // Exit requested. Finalize runtime and get ownership of the device back.
        if exit {
            let _config = runtime.finalize()?;
            return Ok(());
        }

        let ui = calc_ui_pos();
        let input = input::input(&ui);
        runtime.device_mut().update_input(input);

        let screen = Texture2D::from_image(&runtime.display_mut().screen);
        draw_texture_ex(
            &screen,
            ui.x,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(ui.size),
                source: None,
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        draw_circle_lines(ui.pad.p.x, ui.pad.p.y, ui.pad.r, 5., BLACK);

        for (label, button) in ["S", "E", "W", "N"].iter().zip(&ui.buttons) {
            draw_circle(button.p.x, button.p.y, button.r, BLACK);
            let size = get_text_center(label, None, button.r as u16, 1.0, 0.);
            draw_text(
                label,
                button.p.x - size.x,
                button.p.y - size.y,
                button.r,
                WHITE,
            );
        }

        next_frame().await;
    }
}

struct UiPos {
    x: f32,
    size: Vec2,
    pad: Button,
    buttons: [Button; 4],
}

#[derive(Default)]
struct Button {
    p: Vec2,
    r: f32,
}

impl From<(Vec2, f32)> for Button {
    fn from((p, r): (Vec2, f32)) -> Self {
        Button { p, r }
    }
}

fn calc_ui_pos() -> UiPos {
    let portrait = screen_width() < screen_height();

    let (x, w, h) = if portrait {
        let w = screen_width();
        let h = w / 240. * 160.;
        (0., w, h)
    } else {
        let h = screen_height();
        let w = h / 160. * 240.;
        let x = screen_width() / 2. - w / 2.;
        (x, w, h)
    };
    let size = vec2(w, h);
    let pad: Button = if portrait {
        let x = screen_width() / 3.;
        let y = screen_height() - x;
        (vec2(x, y), x)
    } else {
        (vec2(x / 2., x), x / 2.)
    }
    .into();

    let buttons = if portrait {
        let center = vec2(pad.p.x * 2.2, pad.p.y - pad.p.x * 1.3);
        let offset = pad.p.x / 2.;
        let r = offset * 0.66;
        [
            Button::from((center + vec2(0., offset), r)),
            Button::from((center + vec2(offset, 0.), r)),
            Button::from((center - vec2(offset, 0.), r)),
            Button::from((center - vec2(0., offset), r)),
        ]
    } else {
        Default::default()
    };
    UiPos {
        x,
        size,
        pad,
        buttons,
    }
}

fn dir() -> PathBuf {
    let Some(dirs) = directories::ProjectDirs::from("de", "oliobk", "fireflydroid") else {
        return PathBuf::from("/data/data/de.oliobk.fireflydroid");
    };

    dirs.cache_dir().to_owned()
}

// DON'T REMOVE, This function is the entrypoint on Android.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "C" fn quad_main() {
    main();
}
