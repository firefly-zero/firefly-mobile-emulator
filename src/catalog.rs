use kaolin::prelude::*;
use macroquad::prelude::*;
use miniserde::Deserialize;
use std::collections::HashMap;

use crate::ui;

const BASE_URL: &str = "https://catalog.fireflyzero.com/";
const LIST_URL: &str = "https://catalog.fireflyzero.com/apps.json";

#[derive(Deserialize)]
struct ShortApp {
    id: String,
    name: String,
    author: String,
    short: String,
    added: String,
}

#[derive(Deserialize)]
struct App {
    name: String,
    author: Author,
    short: String,
    added: String,
    download: String,
    desc: String,
    links: Option<HashMap<String, String>>,
    categories: Vec<String>,
}

#[derive(Deserialize)]
struct Author {
    name: String,
    pronouns: Option<String>,
    links: HashMap<String, String>,
    short: String,
    about: Option<String>,
}

pub async fn list() {
    let resp = match ureq::get(LIST_URL).call() {
        Ok(r) => r,
        Err(e) => {
            while !is_key_down(KeyCode::Back) {
                draw_text(&format!("{e:?}"), 0., 0., 50., RED);
                next_frame().await;
            }
            pop_camera_state();
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
    let mut ui = ui::Renderer::new(screen_width() as i32, screen_height() as i32);
    let name_width = apps
        .iter()
        .map(|app| measure_text(&app.name, None, 80, 1.).width as u32)
        .max()
        .unwrap()
        .min(screen_width() as u32 / 3) as f64;
    while !is_key_pressed(KeyCode::Back) {
        clear_background(GRAY);
        ui.draw(|k| {
            let style = TextStyle::new().font_size(80.0).color(BLACK.into());
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
                                TextStyle::new().font_size(150.0).color(BLACK.into()),
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

        next_frame().await
    }
    pop_camera_state();
}

/*
pub fn cmd_catalog_show(args: &CatalogShowArgs) -> Result<()> {
    if args.id.contains('.') {
        show_app(args)
    } else {
        show_author(args)
    }
}

pub fn show_app(args: &CatalogShowArgs) -> Result<()> {
    let url = format!("{BASE_URL}{}.json", args.id);
    let resp = ureq::get(&url).call().context("send request")?;
    let mut body = resp.into_body().into_reader();
    let app: App = serde_json::from_reader(&mut body).context("parse JSON")?;
    println!("{} {}", col("title"), app.name);
    println!("{} {}", col("author"), app.author.name);
    println!("{} {}", col("added"), app.added);
    println!("{} {}", col("short"), app.short);
    println!("{}", col("categories"));
    for category in app.categories {
        println!("  {category}");
    }
    if let Some(links) = app.links {
        println!("{}", col("links"));
        for (name, url) in links {
            println!("  {}: {}", name.cyan(), url);
        }
    }
    println!("{} {}", col("download"), app.download);
    println!("{}\n{}", col("description"), app.desc);
    Ok(())
}

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
