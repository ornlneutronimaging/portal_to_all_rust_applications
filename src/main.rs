use eframe::egui;
use std::path::PathBuf;
use std::process::Command;

const GIT_ROOT: &str = "/SNS/VENUS/shared/software/git";
const LOGO_BYTES: &[u8] = include_bytes!("../logos/ImagingLogo.png");
const LOGO_MAX_HEIGHT: f32 = 64.0;

/// How an application is started from its repository.
enum Launcher {
    /// Shell script at the repo root that rebuilds if needed, then execs.
    Script(&'static str),
    /// Pre-built binary in `target/release/`.
    Binary(&'static str),
}

struct AppDef {
    name: &'static str,
    description: &'static str,
    repo: &'static str,
    launcher: Launcher,
}

const APPS: &[AppDef] = &[
    AppDef {
        name: "TIFF Viewer",
        description: "Browse and inspect folders of TIFF images from a measurement.",
        repo: "rust_tiff_viewer",
        launcher: Launcher::Script("launch_rust_tiff_viewer.sh"),
    },
    AppDef {
        name: "TOF Profile Viewer",
        description: "Browse TIFF stacks from a VENUS/Timepix measurement and plot TOF profiles.",
        repo: "rust_tof_profile_viewer",
        launcher: Launcher::Script("launch_tof_profile_viewer.sh"),
    },
    AppDef {
        name: "ROI Selector",
        description: "Select regions of interest on an imaging data set and export a mask file.",
        repo: "rust_roi_selector",
        launcher: Launcher::Script("launch_roi_selector.sh"),
    },
    AppDef {
        name: "CT Reconstruction",
        description: "Neutron CT reconstruction workflows for VENUS and MARS.",
        repo: "rust_ct_reconstruction",
        launcher: Launcher::Script("launch_ct_reconstruction.sh"),
    },
    AppDef {
        name: "Hyperspectral Masker",
        description: "Create a copy of an input data set with selected pixels masked.",
        repo: "rust_hyperspectra_masker",
        launcher: Launcher::Script("launch_rust_hyperspectral_makser.sh"),
    },
    AppDef {
        name: "Auto Normalization Monitor",
        description: "Monitor the state of the VENUS auto normalization pipeline.",
        repo: "rust_autonormalization_monitor",
        launcher: Launcher::Script("launch_autonormalization_monitor.sh"),
    },
    AppDef {
        name: "Jupyter Notebooks Portal",
        description: "Provision and launch the imaging Jupyter notebooks for an IPTS.",
        repo: "rust_jupyter_notebooks_portal",
        launcher: Launcher::Binary("rust_jupyter_notebooks_portal"),
    },
    AppDef {
        name: "Jupyter Portal (dedicated IPTS)",
        description: "Launch the JupyterLab notebooks of a dedicated IPTS.",
        repo: "rust_jupyter_portal_dedicated_ipts",
        launcher: Launcher::Binary("rust_jupyter_portal_dedicated_ipts"),
    },
    AppDef {
        name: "Marimo Portal (general tools)",
        description: "Launch the general-tools marimo notebooks.",
        repo: "rust_marimo_portal_general_tools",
        launcher: Launcher::Binary("rust_marimo_portal_general_tools"),
    },
    AppDef {
        name: "Marimo Portal (dedicated IPTS)",
        description: "Launch the marimo notebooks of a dedicated IPTS.",
        repo: "rust_marimo_portal_dedicated_ipts",
        launcher: Launcher::Binary("rust_marimo_portal"),
    },
];

impl AppDef {
    fn repo_dir(&self) -> PathBuf {
        PathBuf::from(GIT_ROOT).join(self.repo)
    }

    fn launch_path(&self) -> PathBuf {
        match self.launcher {
            Launcher::Script(script) => self.repo_dir().join(script),
            Launcher::Binary(binary) => {
                self.repo_dir().join("target").join("release").join(binary)
            }
        }
    }

    fn launch(&self) -> Result<String, String> {
        let path = self.launch_path();
        Command::new(&path)
            .current_dir(self.repo_dir())
            .spawn()
            .map(|_| format!("Launched: {}", self.name))
            .map_err(|e| format!("Cannot launch {}: {e}", path.display()))
    }
}

fn load_logo(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let img = image::load_from_memory(LOGO_BYTES).ok()?;
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture("imaging_logo", color_image, egui::TextureOptions::LINEAR))
}

struct App {
    logo: Option<egui::TextureHandle>,
    /// One flag per entry in APPS: the launch script / binary exists.
    available: Vec<bool>,
    status: Option<Result<String, String>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            logo: None,
            available: APPS.iter().map(|a| a.launch_path().exists()).collect(),
            status: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.logo.is_none() {
            self.logo = load_logo(ctx);
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Neutron Imaging Applications")
                            .strong()
                            .size(22.0),
                    );
                    ui.label("Select the application you want to launch");
                });
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if let Some(tex) = &self.logo {
                            ui.add(
                                egui::Image::from_texture(tex).max_height(LOGO_MAX_HEIGHT),
                            );
                        }
                    },
                );
            });
            ui.add_space(8.0);
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.add_space(4.0);
            match &self.status {
                Some(Ok(msg)) => {
                    ui.colored_label(egui::Color32::from_rgb(46, 160, 67), msg);
                }
                Some(Err(msg)) => {
                    ui.colored_label(egui::Color32::RED, msg);
                }
                None => {
                    ui.label("Ready");
                }
            }
            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, app) in APPS.iter().enumerate() {
                    let available = self.available[idx];
                    egui::Frame::group(ui.style())
                        .rounding(egui::Rounding::same(6.0))
                        .inner_margin(egui::Margin::same(10.0))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.set_width(ui.available_width() - 110.0);
                                    ui.label(
                                        egui::RichText::new(app.name).strong().size(16.0),
                                    );
                                    ui.label(app.description);
                                });
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.add_enabled_ui(available, |ui| {
                                            let mut button = egui::Button::new(
                                                egui::RichText::new("Launch")
                                                    .color(egui::Color32::WHITE),
                                            )
                                            .rounding(egui::Rounding::same(6.0))
                                            .min_size(egui::vec2(90.0, 28.0));
                                            if available {
                                                button = button
                                                    .fill(egui::Color32::from_rgb(46, 160, 67));
                                            }
                                            let resp = ui
                                                .add(button)
                                                .on_hover_text(
                                                    app.launch_path().display().to_string(),
                                                )
                                                .on_disabled_hover_text(format!(
                                                    "Not found: {}",
                                                    app.launch_path().display()
                                                ));
                                            if resp.clicked() {
                                                self.status = Some(app.launch());
                                            }
                                        });
                                    },
                                );
                            });
                        });
                    ui.add_space(6.0);
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([560.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Neutron Imaging Application Portal",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}
