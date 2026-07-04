//! Application eframe : pilote le rendu progressif passe par passe et affiche
//! l'image en cours de convergence, mise à jour à chaque passe.

use crate::camera::Camera;
use crate::math::Color;
use crate::object::HittableList;

pub struct RaytracinApp {
    camera: Camera,
    world: HittableList,
    accum: Vec<Color>,
    passes: u32,
    saved: bool,
    texture: Option<egui::TextureHandle>,
}

impl RaytracinApp {
    fn new(camera: Camera, world: HittableList) -> Self {
        let pixel_count = (camera.image_width * camera.image_height) as usize;
        Self {
            camera,
            world,
            accum: vec![Color::zero(); pixel_count],
            passes: 0,
            saved: false,
            texture: None,
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
        let target = self.camera.sample_per_pixel() as u32;

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
            ui.add(
                egui::ProgressBar::new(self.passes as f32 / target as f32).show_percentage(),
            );
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
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Raytracin",
        native_options,
        Box::new(|_cc| Ok(Box::new(RaytracinApp::new(camera, world)))),
    )
}
