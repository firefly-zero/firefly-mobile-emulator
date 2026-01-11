use std::path::Path;

use macroquad::prelude::*;
use std::fmt::Write as _;
use wasmi::*;

mod imports;

struct HostState {
    colors: [Color; 17],
}

impl HostState {
    fn col(&self, i: i32) -> Color {
        self.colors[i as usize]
    }
}

#[macroquad::main("fireflydroid")]
async fn main() {
    let project_path = Path::new(
        "/data/user/0/com.mohammedkhc.ide.rust/files/home/.local/share/firefly/roms/sys/input-test/",
    ).to_owned();
    let wasm = std::fs::read(project_path.join("_bin")).unwrap();
    // First step is to create the Wasm execution engine with some config.
    //
    // In this example we are using the default configuration.
    let engine = Engine::default();
    // Now we can compile the above Wasm module with the given Wasm source.
    let module = Module::new(&engine, wasm).unwrap();

    let mut store = Store::new(
        &engine,
        HostState {
            colors: [
                Color::from_rgba(0, 0, 0, 0),
                Color::from_hex(0x1A1C2C),
                Color::from_hex(0x5D275D),
                Color::from_hex(0xB13E53),
                Color::from_hex(0xEF7D57),
                Color::from_hex(0xFFCD75),
                Color::from_hex(0xA7F070),
                Color::from_hex(0x38B764),
                Color::from_hex(0x257179),
                Color::from_hex(0x29366F),
                Color::from_hex(0x3B5DC9),
                Color::from_hex(0x41A6F6),
                Color::from_hex(0x73EFF7),
                Color::from_hex(0xF4F4F4),
                Color::from_hex(0x94B0C2),
                Color::from_hex(0x566C86),
                Color::from_hex(0x333C57),
            ],
        },
    );

    // A linker can be used to instantiate Wasm modules.
    // The job of a linker is to satisfy the Wasm module's imports.
    let mut linker = <Linker<HostState>>::new(&engine);
    // We are required to define all imports before instantiating a Wasm module.
    imports::setup(&mut linker, project_path).unwrap();

    #[expect(unused_variables)]
    let instance = match linker.instantiate_and_start(&mut store, &module) {
        Ok(instance) => instance,
        Err(e) => match e.kind() {
            errors::ErrorKind::TrapCode(trap_code) => todo!(),
            errors::ErrorKind::Message(_) => todo!(),
            errors::ErrorKind::I32ExitStatus(_) => todo!(),
            errors::ErrorKind::Host(host_error) => todo!(),
            errors::ErrorKind::Global(global_error) => todo!(),
            errors::ErrorKind::Memory(memory_error) => todo!(),
            errors::ErrorKind::Table(table_error) => todo!(),
            errors::ErrorKind::Linker(linker_error) => match linker_error {
                errors::LinkerError::DuplicateDefinition { import_name } => todo!(),
                errors::LinkerError::MissingDefinition { name, ty } => {
                    let mut args = String::new();
                    for (i, arg) in ty.func().unwrap().params().iter().enumerate() {
                        let arg = val_to_ty(arg);
                        writeln!(args, "arg{i}: {},", arg).unwrap();
                    }
                    let ret = match ty.func().unwrap().results() {
                        [] => "()",
                        [ret] => val_to_ty(ret),
                        _ => panic!(),
                    };

                    std::fs::write(
                        Path::new("/data/data/com.mohammedkhc.ide.rust/files/home/fireflydroid")
                            .join("missing_def.rs"),
                        format!(
                            r##"
    linker
        .func_wrap(
            "{}",
            "{}",
            |_caller: Caller<'_, HostState>,
            {args}
            | -> {ret}
            {{
            
            }})?;
"##,
                            name.module(),
                            name.name()
                        ),
                    )
                    .unwrap();
                    return;
                }
                errors::LinkerError::InvalidTypeDefinition {
                    name,
                    expected,
                    found,
                } => todo!(),
            },
            errors::ErrorKind::Instantiation(instantiation_error) => todo!(),
            errors::ErrorKind::Fuel(fuel_error) => todo!(),
            errors::ErrorKind::Func(func_error) => todo!(),
            errors::ErrorKind::Read(read_error) => todo!(),
            errors::ErrorKind::Wasm(binary_reader_error) => todo!(),
            errors::ErrorKind::Translation(translation_error) => todo!(),
            errors::ErrorKind::Limits(enforced_limits_error) => todo!(),
            errors::ErrorKind::Ir(error) => todo!(),
            errors::ErrorKind::Wat(error) => todo!(),
            _ => todo!(),
        },
    };

    let target = render_target(240, 160);
    let mut camera = Camera2D::from_display_rect(Rect::new(0., 160., 240., -160.));
    let screen = target.texture.clone();
    screen.set_filter(FilterMode::Nearest);
    camera.render_target = Some(target);

    instance
        .get_typed_func::<(), ()>(&store, "boot")
        .unwrap()
        .call(&mut store, ())
        .unwrap();

    loop {
        push_camera_state();
        set_camera(&camera);
        if let Ok(update) = instance.get_typed_func::<(), ()>(&store, "update") {
            update.call(&mut store, ()).unwrap();
        }

        instance
            .get_typed_func::<(), ()>(&store, "render")
            .unwrap()
            .call(&mut store, ())
            .unwrap();
        pop_camera_state();
        clear_background(GRAY);

        let ui = calc_ui_pos();

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

fn read_str(caller: &Caller<'_, HostState>, addr: u32, len: u32) -> String {
    let mut p = vec![0; len as usize];
    mem(caller).read(caller, addr as usize, &mut p).unwrap();
    String::from_utf8(p).unwrap()
}

fn mem(caller: &Caller<'_, HostState>) -> Memory {
    caller.get_export("memory").unwrap().into_memory().unwrap()
}

fn val_to_ty(arg: &ValType) -> &'static str {
    match arg {
        ValType::I32 => "u32",
        ValType::I64 => "u64",
        ValType::F32 => "f32",
        ValType::F64 => "f64",
        ValType::V128 => todo!(),
        ValType::FuncRef => todo!(),
        ValType::ExternRef => todo!(),
    }
}

// DON'T REMOVE, This function is the entrypoint on Android.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "C" fn quad_main() {
    main();
}
