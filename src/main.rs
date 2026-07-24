use eframe::egui;
use std::path::PathBuf;
use std::process::Command;

const GIT_ROOT: &str = "/SNS/VENUS/shared/software/git";
const LOGO_BYTES: &[u8] = include_bytes!("../logos/ImagingLogo.png");
const LOGO_MAX_HEIGHT: f32 = 64.0;
/// Screenshots shown in the preview panel: `<PREVIEW_DIR>/<preview>.png` (or `.jpg`).
const PREVIEW_DIR: &str =
    "/SNS/VENUS/shared/software/git/portal_to_all_rust_applications/previews";
const PREVIEW_PANEL_WIDTH: f32 = 360.0;

/// How an application is started from its repository.
enum Launcher {
    /// Shell script at the repo root that rebuilds if needed, then execs.
    Script(&'static str),
    /// Pre-built binary in `target/release/`.
    Binary(&'static str),
    /// Executable at an absolute path outside `GIT_ROOT`.
    Absolute(&'static str),
}

struct AppDef {
    name: &'static str,
    description: &'static str,
    repo: &'static str,
    launcher: Launcher,
    /// File stem of the screenshot in `PREVIEW_DIR`.
    preview: &'static str,
}

const APPS: &[AppDef] = &[
    AppDef {
        name: "TIFF Viewer",
        description: "Browse and inspect folders of TIFF images from a measurement.",
        repo: "rust_tiff_viewer",
        launcher: Launcher::Script("launch_rust_tiff_viewer.sh"),
        preview: "rust_tiff_viewer",
    },
    AppDef {
        name: "TOF Profile Viewer",
        description: "Browse TIFF stacks from a VENUS/Timepix measurement and plot TOF profiles.",
        repo: "rust_tof_profile_viewer",
        launcher: Launcher::Script("launch_tof_profile_viewer.sh"),
        preview: "rust_tof_profile_viewer",
    },
    AppDef {
        name: "ROI Selector",
        description: "Select regions of interest on an imaging data set and export a mask file.",
        repo: "rust_roi_selector",
        launcher: Launcher::Script("launch_roi_selector.sh"),
        preview: "rust_roi_selector",
    },
    AppDef {
        name: "Crop TIFF",
        description: "Select a rectangular crop region on a TIFF or .npy stack and export it.",
        repo: "rust_crop_tiff",
        launcher: Launcher::Script("launch_crop_tiff.sh"),
        preview: "rust_crop_tiff",
    },
    AppDef {
        name: "CT Reconstruction",
        description: "Neutron CT reconstruction workflows for VENUS and MARS.",
        repo: "rust_ct_reconstruction",
        launcher: Launcher::Script("launch_ct_reconstruction.sh"),
        preview: "rust_ct_reconstruction",
    },
    AppDef {
        name: "Hyperspectral Masker",
        description: "Create a copy of an input data set with selected pixels masked.",
        repo: "rust_hyperspectra_masker",
        launcher: Launcher::Script("launch_rust_hyperspectral_makser.sh"),
        preview: "rust_hyperspectra_masker",
    },
    AppDef {
        name: "Auto Normalization Monitor",
        description: "Monitor the state of the VENUS auto normalization pipeline.",
        repo: "rust_autonormalization_monitor",
        launcher: Launcher::Script("launch_autonormalization_monitor.sh"),
        preview: "rust_autonormalization_monitor",
    },
    AppDef {
        name: "BM3D ORNL GUI",
        description: "Denoise imaging data / sinograms with the BM3D ORNL algorithm.",
        repo: "rust_bm3dornl",
        launcher: Launcher::Script("launch_bm3dornl_gui.sh"),
        preview: "rust_bm3dornl",
    },
    AppDef {
        name: "NRAD Spot Cleaner",
        description: "Remove spots and hot pixels from radiographs with the NRAD algorithms.",
        repo: "",
        launcher: Launcher::Absolute("/SNS/VENUS/shared/software/bin/nrad-gui"),
        preview: "nrad_spot_cleaner",
    },
    AppDef {
        name: "Timepix3 Tool",
        description: "Process Timepix3 (TPX3) pixel detector data with the rustpix GUI.",
        repo: "rustpix",
        launcher: Launcher::Script("launch_rustpix.sh"),
        preview: "rustpix",
    },
    AppDef {
        name: "Jupyter Notebooks Portal",
        description: "Provision and launch the imaging Jupyter notebooks for an IPTS.",
        repo: "rust_jupyter_notebooks_portal",
        launcher: Launcher::Binary("rust_jupyter_notebooks_portal"),
        preview: "rust_jupyter_notebooks_portal",
    },
    AppDef {
        name: "Jupyter Portal (dedicated IPTS)",
        description: "Launch the JupyterLab notebooks of a dedicated IPTS.",
        repo: "rust_jupyter_portal_dedicated_ipts",
        launcher: Launcher::Binary("rust_jupyter_portal_dedicated_ipts"),
        preview: "rust_jupyter_portal_dedicated_ipts",
    },
    AppDef {
        name: "Marimo Portal (general tools)",
        description: "Launch the general-tools marimo notebooks.",
        repo: "rust_marimo_portal_general_tools",
        launcher: Launcher::Binary("rust_marimo_portal_general_tools"),
        preview: "rust_marimo_portal_general_tools",
    },
    AppDef {
        name: "Marimo Portal (dedicated IPTS)",
        description: "Launch the marimo notebooks of a dedicated IPTS.",
        repo: "rust_marimo_portal_dedicated_ipts",
        launcher: Launcher::Binary("rust_marimo_portal"),
        preview: "rust_marimo_portal_dedicated_ipts",
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
            Launcher::Absolute(path) => PathBuf::from(path),
        }
    }

    fn launch(&self) -> Result<String, String> {
        let path = self.launch_path();
        let work_dir = match self.launcher {
            Launcher::Absolute(_) => path
                .parent()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/")),
            _ => self.repo_dir(),
        };
        Command::new(&path)
            .current_dir(work_dir)
            .spawn()
            .map(|_| format!("Launched: {}", self.name))
            .map_err(|e| format!("Cannot launch {}: {e}", path.display()))
    }
}

enum Preview {
    NotLoaded,
    Missing,
    Loaded(egui::TextureHandle),
}

fn load_preview(ctx: &egui::Context, app: &AppDef) -> Preview {
    let dir = PathBuf::from(PREVIEW_DIR);
    let Some(bytes) = ["png", "jpg", "jpeg"]
        .iter()
        .find_map(|ext| std::fs::read(dir.join(format!("{}.{ext}", app.preview))).ok())
    else {
        return Preview::Missing;
    };
    let Ok(img) = image::load_from_memory(&bytes) else {
        return Preview::Missing;
    };
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Preview::Loaded(ctx.load_texture(
        format!("preview_{}", app.preview),
        color_image,
        egui::TextureOptions::LINEAR,
    ))
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
    /// One entry per app, loaded on first display in the preview panel.
    previews: Vec<Preview>,
    /// Index into APPS of the app shown in the preview panel (last hovered).
    selected: usize,
    status: Option<Result<String, String>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            logo: None,
            available: APPS.iter().map(|a| a.launch_path().exists()).collect(),
            previews: APPS.iter().map(|_| Preview::NotLoaded).collect(),
            selected: 0,
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

        if matches!(self.previews[self.selected], Preview::NotLoaded) {
            self.previews[self.selected] = load_preview(ctx, &APPS[self.selected]);
        }

        egui::SidePanel::right("preview_panel")
            .exact_width(PREVIEW_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                let app = &APPS[self.selected];
                ui.add_space(12.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(app.name).strong().size(16.0));
                });
                ui.add_space(8.0);
                match &self.previews[self.selected] {
                    Preview::Loaded(tex) => {
                        ui.vertical_centered(|ui| {
                            ui.add(
                                egui::Image::from_texture(tex)
                                    .max_width(ui.available_width())
                                    .max_height(ui.available_height() - 12.0),
                            );
                        });
                    }
                    _ => {
                        ui.vertical_centered(|ui| {
                            ui.add_space(24.0);
                            ui.label(
                                egui::RichText::new("No preview available")
                                    .weak()
                                    .italics(),
                            );
                        });
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, app) in APPS.iter().enumerate() {
                    let available = self.available[idx];
                    let group = egui::Frame::group(ui.style())
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
                    if ui.rect_contains_pointer(group.response.rect) {
                        self.selected = idx;
                    }
                    ui.add_space(6.0);
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([940.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Neutron Imaging Application Portal",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}
