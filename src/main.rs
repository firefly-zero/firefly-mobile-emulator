use macroquad::prelude::*;
use wasmi::*;

#[macroquad::main("fireflydroid")]
async fn main() {
    let wasm = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))
            (func (export "hello")
                (call $host_hello (i32.const 3))
            )
        )
    "#;
    // First step is to create the Wasm execution engine with some config.
    //
    // In this example we are using the default configuration.
    let engine = Engine::default();
    // Now we can compile the above Wasm module with the given Wasm source.
    let module = Module::new(&engine, wasm).unwrap();

    type HostState = ();
    let mut store = Store::new(&engine, ());

    // A linker can be used to instantiate Wasm modules.
    // The job of a linker is to satisfy the Wasm module's imports.
    let mut linker = <Linker<HostState>>::new(&engine);
    // We are required to define all imports before instantiating a Wasm module.
    linker
        .func_wrap(
            "host",
            "hello",
            |_caller: Caller<'_, HostState>, param: i32| {
                draw_text(
                    &format!("Got {param} from WebAssembly"),
                    0.,
                    100.,
                    50.,
                    LIME,
                );
            },
        )
        .unwrap();
    let instance = linker.instantiate_and_start(&mut store, &module).unwrap();
    // Now we can finally query the exported "hello" function and call it.

    // MacroQuad default font is pixelated, so we use Roboto Font!
    let roboto_bytes = std::fs::read("/system/fonts/Roboto-Regular.ttf").unwrap();
    let font = load_ttf_font_from_bytes(&roboto_bytes).unwrap();

    loop {
        clear_background(GREEN);
        instance
            .get_typed_func::<(), ()>(&store, "hello")
            .unwrap()
            .call(&mut store, ())
            .unwrap();

        draw_multiline_text_ex(
            format!(
                "Put your finger on the screen!\nFinger count: {}",
                touches().len()
            )
            .as_str(),
            20.,
            600.,
            None,
            TextParams {
                font: Some(&font),
                font_size: 80,
                ..Default::default()
            },
        );
        next_frame().await;
    }
}

// DON'T REMOVE, This function is the entrypoint on Android.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "C" fn quad_main() {
    main();
}
