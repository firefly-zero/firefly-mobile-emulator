use firefly_runtime::FullID;
use firefly_types::Encode as _;
use kaolin::prelude::*;
use macroquad::prelude::*;
use miniserde::Deserialize;

use crate::{
    dir,
    ui::{self},
};

const BASE_URL: &str = "https://catalog.fireflyzero.com/";
const LIST_URL: &str = "https://catalog.fireflyzero.com/apps.json";

#[derive(Deserialize)]
struct ShortApp {
    id: String,
    name: String,
    short: String,
}

#[derive(Deserialize)]
struct App {
    name: String,
    download: String,
    desc: String,
    categories: Vec<String>,
}

const TITLE_FONT_SIZE: f32 = if cfg!(target_os = "android") {
    150.
} else {
    80.
};
const BUTTON_FONT_SIZE: f32 = if cfg!(target_os = "android") {
    120.
} else {
    50.
};
const DESCR_FONT_SIZE: f32 = if cfg!(target_os = "android") {
    80.
} else {
    30.
};

pub async fn list() {
    let resp = match ureq::get(LIST_URL).call() {
        Ok(r) => r,
        Err(e) => {
            while !is_key_down(KeyCode::Back) {
                draw_text(&format!("{e:?}"), 0., 0., 50., RED);
                next_frame().await;
            }
            return;
        }
    };
    let body = match resp.into_body().read_to_string() {
        Ok(body) => body,
        Err(_) => todo!(),
    };
    let apps: Vec<ShortApp> = match miniserde::json::from_str(&body) {
        Ok(apps) => apps,
        Err(_) => todo!(),
    };
    let mut ui = ui::Renderer::new();
    let name_width = apps
        .iter()
        .map(|app| measure_text(&app.name, None, DESCR_FONT_SIZE as _, 1.).width as u32)
        .max()
        .unwrap()
        .min(screen_width() as u32 / 3) as f64;
    while !is_key_pressed(KeyCode::Escape) {
        clear_background(GRAY);
        ui.draw(|k| {
            let style = TextStyle::new()
                .font_size(DESCR_FONT_SIZE)
                .color(BLACK.into());
            k.styled(
                FlexStyle::new()
                    .background_color(GRAY.into())
                    .layout(Layout::new().direction(Direction::TopToBottom))
                    .sizing(sizing!(grow!())),
                |k| {
                    k.styled(
                        FlexStyle::new()
                            .background_color(WHITE.into())
                            .layout(
                                Layout::new()
                                    .alignment(Alignment::Center)
                                    .justification(Justification::Center),
                            )
                            .sizing(sizing!(grow!(), fit!())),
                        |k| {
                            k.text(
                                "Catalog",
                                TextStyle::new()
                                    .font_size(TITLE_FONT_SIZE)
                                    .color(BLACK.into()),
                            )
                        },
                    )
                    .styled(
                        FlexStyle::new()
                            .background_color(GRAY.into())
                            .layout(Layout::new().direction(Direction::TopToBottom))
                            .sizing(sizing!(grow!())),
                        |mut k| {
                            for app in &apps {
                                k = k.styled(
                                    FlexStyle::new()
                                        .custom(app.id.as_str())
                                        .background_color(GRAY.into())
                                        .border(Border {
                                            width: 3.,
                                            color: DARKGRAY.into(),
                                        })
                                        .layout(Layout::new().direction(Direction::LeftToRight))
                                        .sizing(sizing!(grow!(), fit!())),
                                    |k| {
                                        k.styled(
                                            FlexStyle::new()
                                                .border(Border {
                                                    width: 3.,
                                                    color: DARKGRAY.into(),
                                                })
                                                .sizing(sizing!(fixed!(name_width), grow!())),
                                            |k| k.text(&app.name, style),
                                        )
                                        .styled(
                                            FlexStyle::new()
                                                .border(Border {
                                                    width: 3.,
                                                    color: DARKGRAY.into(),
                                                })
                                                .background_color(GRAY.into())
                                                .sizing(sizing!(grow!())),
                                            |k| k.text(&app.short, style),
                                        )
                                    },
                                );
                            }
                            k
                        },
                    )
                },
            )
        });

        if let Some(id) = ui.clicked.iter().next() {
            // clear clicks
            next_frame().await;
            // render app info
            app(id).await;
            if is_key_down(KeyCode::Escape) {
                return;
            }
        }

        next_frame().await
    }
}

pub async fn app(id: &str) {
    let url = format!("{BASE_URL}{id}.json");
    let resp = match ureq::get(&url).call() {
        Ok(r) => r,
        Err(_) => todo!(),
    };
    let body = match resp.into_body().read_to_string() {
        Ok(body) => body,
        Err(_) => todo!(),
    };
    let app: App = match miniserde::json::from_str(&body) {
        Ok(app) => app,
        Err(_) => todo!(),
    };

    let id = FullID::try_from(id).unwrap_or_else(|e| panic!("{}", e));

    let cache = dir().join("roms").join(id.author()).join(id.app());

    let mut ui = ui::Renderer::new();
    while !is_key_pressed(KeyCode::Back) && !is_key_pressed(KeyCode::Escape) {
        clear_background(GRAY);
        ui.draw(|k| {
            let style = TextStyle::new()
                .font_size(DESCR_FONT_SIZE)
                .color(BLACK.into());
            k.styled(
                FlexStyle::new()
                    .background_color(GRAY.into())
                    .layout(Layout::new().direction(Direction::TopToBottom).gap(20.))
                    .sizing(sizing!(grow!())),
                |mut k| {
                    let action = if cache.exists() { "Run" } else { "Download" };
                    k = k.text(
                        &app.name,
                        TextStyle::new()
                            .font_size(TITLE_FONT_SIZE)
                            .color(BLACK.into()),
                    );
                    k = k.text(&app.desc, style);
                    k = k.styled(
                        FlexStyle::new()
                            .border(Border {
                                width: 10.,
                                color: DARKGREEN.into(),
                            })
                            .layout(Layout::new().justification(Justification::Center))
                            .sizing(sizing!(grow!(), fit!()))
                            .custom(action),
                        |k| {
                            k.text(
                                action,
                                style.font_size(BUTTON_FONT_SIZE).color(GREEN.into()),
                            )
                        },
                    );
                    for cat in &app.categories {
                        k = k.text(cat, style);
                    }
                    k
                },
            )
        });
        match ui.clicked.iter().next().map(|s| s.as_str()) {
            Some("Download") => {
                let resp = match ureq::get(&app.download).call() {
                    Ok(r) => r,
                    Err(_) => todo!(),
                };
                let body = match resp.into_body().read_to_vec() {
                    Ok(body) => body,
                    Err(_) => todo!(),
                };

                std::fs::create_dir_all(&cache).unwrap();
                let data = dir().join("data").join(id.author()).join(id.app());
                std::fs::create_dir_all(&data).unwrap();
                let today = (1, 2, 3);
                let stats = firefly_types::Stats {
                    minutes: [0; 4],
                    longest_play: [0; 4],
                    launches: [0; 4],
                    installed_on: today,
                    updated_on: today,
                    launched_on: (0, 0, 0),
                    xp: 0,
                    badges: Box::new([]),
                    scores: Box::new([]),
                };
                let raw = stats.encode_vec().unwrap();
                std::fs::write(data.join("stats"), raw).unwrap();

                let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&body[..])).unwrap();
                archive.extract(&cache).unwrap();
            }
            Some("Run") => match crate::play(&id).await {
                Ok(()) => {
                    if is_key_pressed(KeyCode::Escape) {
                        return;
                    }
                }
                Err(e) => loop {
                    clear_background(WHITE);
                    draw_text(&e.to_string(), 0., 100., 30., BLACK);
                    next_frame().await
                },
            },
            Some(other) => panic!("{other}"),
            None => {}
        }
        next_frame().await;
    }
}

/*
pub fn show_author(args: &CatalogShowArgs) -> Result<()> {
    let url = format!("{BASE_URL}{}.json", args.id);
    let resp = ureq::get(&url).call().context("send request")?;
    let mut body = resp.into_body().into_reader();
    let aut: Author = serde_json::from_reader(&mut body).context("parse JSON")?;
    println!("{} {}", col("name"), aut.name);
    if let Some(pronouns) = aut.pronouns {
        println!("{} {}", col("pronouns"), pronouns);
    }
    println!("{} {}", col("short"), aut.short);
    if !aut.links.is_empty() {
        println!("{}", col("links"));
        for (name, url) in aut.links {
            println!("  {}: {}", name.cyan(), url);
        }
    }
    if let Some(about) = aut.about {
        println!("{}\n{}", col("about"), about);
    }
    Ok(())
}

fn col(name: &str) -> String {
    format!("{name:11}").blue().to_string()
}
*/
