#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs::File;

use eframe::egui::{self, Align2, Color32, ComboBox, Context, DragValue, FontId, Id, LayerId, Order, Style, TextStyle, Visuals};
use rom::{circuit_arena_name, enemy_type::EnemyType, LevelData, Rom, Wave};

mod rom;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 510.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    use egui::FontFamily::{Proportional, Monospace};

    eframe::run_native(
        "Smash TV",
        options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals {
                    override_text_color: Some(Color32::from_rgb(220, 220, 220)),
                    ..Visuals::dark()
                },

                text_styles: [
                    (TextStyle::Heading, FontId::new(18.0, Proportional)),
                    (TextStyle::Body, FontId::new(13.5, Proportional)),
                    (TextStyle::Monospace, FontId::new(12.0, Monospace)),
                    (TextStyle::Button, FontId::new(13.5, Proportional)),
                    (TextStyle::Small, FontId::new(9.0, Proportional)),
                ].into(),

                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::new(Editor{center_text: true, ..Editor::default()})
        }),
    )
}

#[derive(Default)]
struct Editor {
    dropped_file: Option<egui::DroppedFile>,
    selected_level: u8,

    rom: Option<Rom>,
    level_data: Vec<LevelData>,
    center_text: bool,
}

impl Editor {
    fn create_ui(&mut self, ctx: &Context) {
        self.side_panel(ctx);
        self.bottom_panel(ctx);
        self.central_panel(ctx);
    }

    fn side_panel(&mut self, ctx: &Context) {
        if self.rom.is_some() {
            egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                .max_height(500.0)
                .drag_to_scroll(false)
                .show(ui, |ui| {
                    for (idx, &name) in circuit_arena_name().iter().enumerate() {
                        if idx == 11 || idx == 29 {
                            ui.separator();
                        }

                        if ui.add(egui::SelectableLabel::new(self.selected_level == idx as u8, name)).clicked() {
                            self.selected_level = idx as u8;
                        }
                    }
                });
            });
        }
    }

    fn bottom_panel(&mut self, ctx: &Context) {
        if let Some(rom) = &mut self.rom {
            egui::TopBottomPanel::bottom("bottom_panel")
            .show(ctx, |ui|{
                ui.add_space(7.0);
                if ui.button("Save changes").clicked() {
                    rom.save_level_data(&self.level_data);
                }
                ui.add_space(3.0);
            });
        }
    }

    fn central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(rom) = &mut self.rom {
                let level_data = &mut self.level_data[self.selected_level as usize];

                ui.horizontal(|ui| {
                    ui.label("Arena name:")
                    .on_hover_cursor(egui::CursorIcon::Help)
                    .on_hover_text("Up to 26 characters (automatically truncated).");
                    if ui.text_edit_singleline(&mut level_data.name).lost_focus() {
                        if self.center_text {
                            let trimmed = level_data.name.trim();

                            // center to 27 chars to put the extra space from uneven names on the left side.
                            // most, but not all, default names follow this convention.
                            level_data.name = format!("{:^27}", trimmed);
                            level_data.name.pop();
                        } else {
                            level_data.name.truncate(26);
                        }
                    };
                    ui.checkbox(&mut self.center_text, "Automatically center text")
                    .on_hover_cursor(egui::CursorIcon::Help)
                    .on_hover_text("Adds space around the inputted text to center it.");
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Waves allowed remaining:")
                    .on_hover_cursor(egui::CursorIcon::Help)
                    .on_hover_text(concat!(
                        "Enemy waves allowed to remain for the arena to be considered beaten.\n",
                        "For example, setting this to 1 makes an arena finish when mines are still present.",
                    ));
                    ui.add(DragValue::new(&mut level_data.waves_remaining));
                });

                ui.add_space(10.0);

                // show and edit room connections
                ui.label("Connects to:")
                .on_hover_cursor(egui::CursorIcon::Help)
                .on_hover_text("The Up, Right and Down connections of this arena.");

                let direction = ["U", "R", "D"];
                for (idx, con) in level_data.connections.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", direction[idx]));

                        let text = match con {
                            0 => "-",
                            0xFF => "Goal",
                            _ => {
                                let offset = [0, 11, 29];
                                circuit_arena_name()[*con as usize - 1 + offset[level_data.circuit as usize]]
                            }
                        };

                        ComboBox::from_id_source(idx)
                        .selected_text(text)
                        .width(155.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value( con, 0, "-");

                            let asd = [1..=11, 12..=29, 30..=52];
                            for arena in asd[level_data.circuit as usize].clone() {
                                ui.selectable_value( con, arena, circuit_arena_name()[arena as usize - 1]);
                            }

                            ui.selectable_value( con, 0xFF, "Goal");
                        });
                    });
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                //enemy waves
                egui::Grid::new("enemy_grid")
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.label("Enemy type");
                    ui.label("Count");
                    ui.label("Spawn limit");
                    ui.label("Unknown")
                    .on_hover_cursor(egui::CursorIcon::Help)
                    .on_hover_text(concat!(
                        "I think this is a \"variation / mode\" control of sorts.\n",
                        "For example, this can be used to change grunts to purple.",
                    ));
                    ui.label("Cooldown timer")
                    .on_hover_cursor(egui::CursorIcon::Help)
                    .on_hover_text("Setting this 0 essentially stops further spawns.");
                    ui.label("Pre-spawned");
                    ui.label("Spawn timer");
                    ui.end_row();

                    for _ in 0 .. 7 {
                        ui.separator();
                    }
                    ui.end_row();

                    for (idx, wave) in level_data.waves.iter_mut().enumerate() {
                        ComboBox::from_id_source(idx)
                        .selected_text(wave.enemy.name())
                        .show_ui(ui, |ui| {
                            for enemy in EnemyType::enemy_list().iter() {
                                ui.selectable_value(&mut wave.enemy, enemy.clone(), enemy.name());
                            }
                        });

                        ui.add(DragValue::new(&mut wave.count));
                        ui.add(DragValue::new(&mut wave.spawn_limit));
                        ui.add(DragValue::new(&mut wave.unknown));
                        ui.horizontal(|ui|{ ui.add(DragValue::new(&mut wave.cooldown_timer)); ui.label(format!("{:.02}s", wave.cooldown_timer as f32 / 60.0));});
                        ui.add(DragValue::new(&mut wave.pre_spawned));
                        ui.horizontal(|ui|{ ui.add(DragValue::new(&mut wave.spawn_timer)); ui.label(format!("{:.02}s", wave.spawn_timer as f32 / 60.0));});

                        ui.end_row();
                    }
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("+").clicked() {
                        level_data.waves.push(Wave{
                            enemy: EnemyType::Grunt,
                            count: 1,
                            spawn_limit: 1,
                            unknown: 0,
                            cooldown_timer: 1,
                            pre_spawned: 0,
                            spawn_timer: 0,
                        });
                    }
                    
                    if ui.button("-")
                    .on_hover_cursor(egui::CursorIcon::Help)
                    .on_hover_text("Remove wave.\nThe minimum is 1 wave.").clicked() {
                        if level_data.waves.len() > 1 {
                            level_data.waves.pop();
                        }
                    }
                });
            } else {
                ui.label("Drag-and-drop a Super Smash T.V. (USA) rom onto the window!");
            }
        });
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.create_ui(ctx);

        preview_files_being_dropped(ctx);

        ctx.input(|i| { // Collect dropped files:
            if !i.raw.dropped_files.is_empty() {
                self.dropped_file = Some(i.raw.dropped_files[0].clone());

                let path = self.dropped_file.as_ref().unwrap().path.as_deref().unwrap();
                let f = match File::open(path) {
                    Ok(o) => o,
                    Err(e) => todo!("{}", e),
                };

                let file_size = f.metadata().unwrap().len();

                if file_size < 524288 {
                    println!("file too small!");
                } else if file_size > 524288 * 4 { // original size * 4 in case it's been extended already
                    println!("file too big! probably not a smash tv rom");
                } else {
                    let rom = std::fs::read(path).expect("Super Smash T.V. (USA).sfc not found!");
                    if rom[0] == 0x78 && rom[1] == 0x9C && rom[2] == 0x00 { // super bootleg rom check
                        self.rom = Some(Rom{rom: rom});

                        self.level_data = self.rom.as_ref().unwrap().get_level_data();
                    } else {
                        println!("doesn't seem to be a smash tv rom!");
                    }
                }
            }
        });
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) { // Preview hovering files:
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
