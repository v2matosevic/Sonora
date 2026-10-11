//! Showcase captures for the public README.
//!
//! These are scenes rendered by the current Sonora UI and staged with the real facts of real
//! files: titles, artist, album, length, stream format and embedded cover art are all read by
//! Sonora's own decoder. Playback position and the crossfade state are staged, because a still
//! image cannot hold a running player. Nothing here plays audio or changes a file.
//!
//! Not run by default. Point `SONORA_SHOWCASE_MEDIA` at a folder of demonstration tracks (sorted
//! by file name) and `SONORA_SHOWCASE_OUT` at an output folder, then run:
//!
//! ```text
//! cargo test --test showcase -- --ignored
//! ```
use eframe::egui;
use egui_kittest::Harness;
use sonora::{
    decoder::{AudioDecoder, Metadata, StreamInfo},
    player::{DeviceInfo, State},
    queue::Track,
    settings::Settings,
    ui::Sonora,
};
use std::{path::PathBuf, sync::Arc};

struct Song {
    path: PathBuf,
    metadata: Metadata,
    info: StreamInfo,
}

fn songs() -> Vec<Song> {
    let dir = PathBuf::from(
        std::env::var_os("SONORA_SHOWCASE_MEDIA").expect("set SONORA_SHOWCASE_MEDIA to a folder"),
    );
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("media folder")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("mp3")))
        .collect();
    files.sort();
    assert!(files.len() >= 4, "need at least four demonstration tracks");
    files
        .into_iter()
        .map(|path| {
            let d = AudioDecoder::open(&path, true).expect("Sonora's decoder opens the file");
            Song {
                metadata: d.metadata.clone(),
                info: d.info.clone(),
                path,
            }
        })
        .collect()
}

fn playing(song: &Song, fading: Option<f64>) -> State {
    let rate = song.info.rate;
    State {
        generation: 0,
        playing: true,
        duration: song.metadata.duration,
        rate,
        channels: song.info.channels,
        info: Some(song.info.clone()),
        // A typical 48 kHz shared-mode output; the conversion is part of what Sonora reports.
        resampled: rate != 48_000,
        output: Some(Arc::new(DeviceInfo {
            name: "Speakers".into(),
            rate: 48_000,
            channels: 2,
            format: "32-bit float".into(),
        })),
        fading,
        next_ready: true,
        ..Default::default()
    }
}

struct Scene {
    name: &'static str,
    theme: egui::Theme,
    size: egui::Vec2,
    /// Which song is on the stage.
    current: usize,
    position: f64,
    fading: Option<f64>,
    crossfade_panel: bool,
    details: bool,
}

fn render(songs: &[Song], scene: &Scene, out: &std::path::Path) {
    let current = scene.current;
    let (position, fading) = (scene.position, scene.fading);
    let (panel, details) = (scene.crossfade_panel, scene.details);
    let theme = scene.theme;
    let paths: Vec<(PathBuf, String)> = songs
        .iter()
        .map(|s| (s.path.clone(), s.metadata.title.clone()))
        .collect();
    let facts = &songs[current];
    let (state, meta) = (playing(facts, fading), facts.metadata.clone());
    let next = songs.get(current + 1).map(|s| s.path.clone());
    let mut harness = Harness::builder()
        .with_size(scene.size)
        .with_pixels_per_point(2.0)
        .with_theme(theme)
        .with_max_steps(12)
        .build_ui_state(
            move |ui, app: &mut Option<Sonora>| {
                let context = ui.ctx().clone();
                let ctx = &context;
                if app.is_none() {
                    v2_ui::install_with(
                        ctx,
                        v2_ui::App::Sonora,
                        v2_ui::Options {
                            reduced_motion: Some(true),
                            text_scale: Some(1.0),
                            ..Default::default()
                        },
                    );
                    ctx.set_theme(theme);
                    let mut a =
                        Sonora::with_settings(ctx.clone(), vec![], Settings::default(), None);
                    a.queue.tracks = paths
                        .iter()
                        .map(|(path, title)| Track {
                            path: path.clone(),
                            title: title.clone(),
                        })
                        .collect();
                    a.metadata = meta.clone();
                    if let Some((w, h, rgba)) = meta.artwork.as_ref() {
                        a.visual_artwork(ctx, [*w, *h], rgba);
                    }
                    a.queue.current = Some(current);
                    a.visual_state(state.clone(), position);
                    a.visual_crossfade(6.0);
                    a.visual_next(next.clone());
                    if fading.is_some() && current > 0 {
                        a.visual_fading_from(Some(current - 1));
                    }
                    if panel {
                        a.visual_open_crossfade();
                    }
                    if details {
                        a.visual_open_details();
                    }
                    *app = Some(a);
                    ctx.request_repaint();
                    return;
                }
                let full = ctx.content_rect();
                ui.scope_builder(egui::UiBuilder::new().max_rect(full), |ui| {
                    app.as_mut().unwrap().draw(ui)
                });
            },
            None,
        );
    // A playing scene asks for a repaint every 250 ms, so step a fixed number of frames.
    harness.run_steps(4);
    std::fs::create_dir_all(out).unwrap();
    harness
        .render()
        .unwrap()
        .save(out.join(format!("{}.png", scene.name)))
        .unwrap();
}

#[test]
#[ignore = "writes README captures; needs SONORA_SHOWCASE_MEDIA and SONORA_SHOWCASE_OUT"]
fn showcase_captures() {
    let songs = songs();
    let out = PathBuf::from(
        std::env::var_os("SONORA_SHOWCASE_OUT").expect("set SONORA_SHOWCASE_OUT to a folder"),
    );
    let size = egui::vec2(1040.0, 720.0);
    for scene in [
        Scene {
            name: "now-playing-dark",
            theme: egui::Theme::Dark,
            size,
            current: 0,
            position: 127.0,
            fading: None,
            crossfade_panel: false,
            details: false,
        },
        Scene {
            name: "crossfade-panel-dark",
            theme: egui::Theme::Dark,
            size,
            current: 0,
            position: 127.0,
            fading: None,
            crossfade_panel: true,
            details: false,
        },
        Scene {
            name: "crossfading-dark",
            theme: egui::Theme::Dark,
            size,
            current: 1,
            position: 3.0,
            fading: Some(3.2),
            crossfade_panel: false,
            details: false,
        },
        Scene {
            name: "now-playing-light",
            theme: egui::Theme::Light,
            size,
            current: 2,
            position: 61.0,
            fading: None,
            crossfade_panel: false,
            details: false,
        },
    ] {
        render(&songs, &scene, &out);
    }
}
