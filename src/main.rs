use std::path::Path;

use macroquad::prelude::*;
use std::fmt::Write as _;
use wasmi::*;

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
    );
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
    linker
        .func_wrap(
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
        )
        .unwrap();
    linker
        .func_wrap(
            "fs",
            "load_file",
            |caller: Caller<'_, HostState>,
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
        )
        .unwrap();

    linker
        .func_wrap(
            "graphics",
            "clear_screen",
            |caller: Caller<'_, HostState>, color: i32| -> () {
                clear_background(caller.data().col(color));
            },
        )
        .unwrap();

    linker
        .func_wrap("net", "get_me", |_caller: Caller<'_, HostState>| -> u32 {
            0
        })
        .unwrap();

    linker
        .func_wrap(
            "net",
            "get_peers",
            |_caller: Caller<'_, HostState>| -> u32 { 0 },
        )
        .unwrap();

    linker
        .func_wrap(
            "input",
            "read_buttons",
            |_caller: Caller<'_, HostState>, arg0: u32| -> u32 { 0 },
        )
        .unwrap();

    linker
        .func_wrap(
            "input",
            "read_pad",
            |_caller: Caller<'_, HostState>, arg0: u32| -> u32 { 0 },
        )
        .unwrap();

    linker
        .func_wrap(
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
        )
        .unwrap();

    linker
        .func_wrap(
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
        )
        .unwrap();

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
            
            }}).unwrap();
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

    let h = screen_height() / screen_width() * 240.;
    let camera = Camera2D::from_display_rect(Rect::new(0., h, 240., -h));
    set_camera(&camera);

    instance
        .get_typed_func::<(), ()>(&store, "boot")
        .unwrap()
        .call(&mut store, ())
        .unwrap();

    loop {
        clear_background(GREEN);
        if let Ok(update) = instance.get_typed_func::<(), ()>(&store, "update") {
            update.call(&mut store, ()).unwrap();
        }

        instance
            .get_typed_func::<(), ()>(&store, "render")
            .unwrap()
            .call(&mut store, ())
            .unwrap();

        next_frame().await;
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
