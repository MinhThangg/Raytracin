//! Application eframe : pilote le rendu progressif passe par passe et affiche
//! l'image en cours de convergence, mise à jour à chaque passe.

use crate::camera::Camera;
use crate::math::Color;
use crate::object::HittableList;

/// Valeurs des sliders, appliquées à la caméra à chaque frame via `apply_settings`.
struct Settings {
    width: i32,
    height: i32,
    spp: i32,
    depth: i32,
}

pub struct RaytracinApp {
    camera: Camera,
    world: HittableList,
    accum: Vec<Color>,
    passes: u32,
    saved: bool,
    texture: Option<egui::TextureHandle>,
    settings: Settings,
}

impl RaytracinApp {
    fn new(camera: Camera, world: HittableList) -> Self {
        let pixel_count = (camera.image_width * camera.image_height) as usize;
        let settings = Settings {
            width: camera.image_width,
            height: camera.image_height,
            spp: camera.sample_per_pixel,
            depth: camera.max_depth,
        };
        Self {
            camera,
            world,
            accum: vec![Color::zero(); pixel_count],
            passes: 0,
            saved: false,
            texture: None,
            settings,
        }
    }

    /// Remet l'accumulation à zéro : à appeler chaque fois que l'intégrande change
    /// (profondeur) ou que le buffer est réalloué (résolution), rendant les passes
    /// déjà accumulées incomparables aux futures.
    fn reset_accumulation(&mut self) {
        for c in &mut self.accum {
            *c = Color::zero();
        }
        self.passes = 0;
        self.saved = false;
    }

    /// Applique les changements de sliders à la caméra et à l'état d'accumulation.
    fn apply_settings(&mut self) {
        if self.settings.width != self.camera.image_width
            || self.settings.height != self.camera.image_height
        {
            self.camera = Camera::new(
                self.settings.width,
                self.settings.height,
                self.settings.spp,
                self.settings.depth,
            );
            self.accum = vec![Color::zero(); (self.settings.width * self.settings.height) as usize];
            self.reset_accumulation();
            self.texture = None;
            return;
        }

        if self.settings.depth != self.camera.max_depth {
            self.camera.max_depth = self.settings.depth;
            self.reset_accumulation();
        }

        if self.settings.spp != self.camera.sample_per_pixel {
            self.camera.sample_per_pixel = self.settings.spp;
            self.saved = false;
        }
    }

    /// Moyenne l'accumulation courante, tonemap en RGB8 et met à jour la texture
    /// affichée (créée à la première passe, mise à jour ensuite).
    fn update_texture(&mut self, ctx: &egui::Context) {
        let inv_passes = 1.0 / self.passes as f32;
        let averaged: Vec<Color> = self.accum.iter().map(|c| *c * inv_passes).collect();
        let bytes = crate::tonemap_rgb8(&averaged);
        let width = self.camera.image_width as usize;
        let height = self.camera.image_height as usize;
        let color_image = egui::ColorImage::from_rgb([width, height], &bytes);

        match &mut self.texture {
            Some(texture) => texture.set(color_image, egui::TextureOptions::LINEAR),
            None => {
                self.texture =
                    Some(ctx.load_texture("render", color_image, egui::TextureOptions::LINEAR));
            }
        }
    }

    /// Moyenne l'accumulation courante par le nombre de passes effectuées
    /// et écrit le PNG. Ne fait rien si aucune passe n'a encore eu lieu.
    fn save_current(&mut self) {
        if self.passes == 0 || self.saved {
            return;
        }
        let inv_passes = 1.0 / self.passes as f32;
        let averaged: Vec<Color> = self.accum.iter().map(|c| *c * inv_passes).collect();
        crate::save_png(
            &averaged,
            self.camera.image_width as u32,
            self.camera.image_height as u32,
        );
        self.saved = true;
    }
}

impl eframe::App for RaytracinApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("settings")
            .default_size(260.0)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.settings.width, 100..=2000).text("Largeur"));
                ui.add(egui::Slider::new(&mut self.settings.height, 100..=2000).text("Hauteur"));
                ui.add(
                    egui::Slider::new(&mut self.settings.spp, 1..=1000)
                        .logarithmic(true)
                        .text("Échantillons/pixel"),
                );
                ui.add(
                    egui::Slider::new(&mut self.settings.depth, 1..=200).text("Profondeur max"),
                );
            });
        self.apply_settings();

        let target = self.camera.sample_per_pixel as u32;

        if self.passes < target {
            self.camera
                .render_pass(&self.world, &mut self.accum, self.passes);
            self.passes += 1;
            self.update_texture(ui.ctx());
            // Une seule passe par frame : on redemande un repaint pour enchaîner.
            ui.ctx().request_repaint();
        } else if !self.saved {
            self.save_current();
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(format!("Passe {}/{}", self.passes, target));
            ui.add(egui::ProgressBar::new(self.passes as f32 / target as f32).show_percentage());
            if self.saved {
                ui.label("image.png enregistré");
            }
            // L'image reste affichée (texture conservée) même une fois le rendu terminé.
            if let Some(texture) = &self.texture {
                ui.add(egui::Image::new(texture).shrink_to_fit());
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Le PNG doit être écrit même si la fenêtre est fermée avant la fin.
        self.save_current();
    }
}

/// Lance la fenêtre eframe et pilote le rendu progressif jusqu'à son terme
/// (ou jusqu'à fermeture anticipée, auquel cas le PNG est écrit avec ce qui
/// a déjà été accumulé).
pub fn run(camera: Camera, world: HittableList) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1040.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Raytracin",
        native_options,
        Box::new(|_cc| Ok(Box::new(RaytracinApp::new(camera, world)))),
    )
}
