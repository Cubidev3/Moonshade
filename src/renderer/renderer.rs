use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use scoped_pool::Pool;
use crate::math::color::Color;
use crate::renderer::pixel_shader::PixelShader;
use crate::renderer::image::Image;
use crate::renderer::lens_shader::LensShader;
use crate::renderer::ray_shader::RayShader;
use crate::math::ray::Ray;
use crate::surfaces::surface::{Surface, SurfacePoint};

const WORKER_COUNT: usize = 5;

pub struct Renderer<L: LensShader + Send + Sync, R: RayShader + Send + Sync, F: PixelShader + Send + Sync> {
    lens_shader: L,
    reflection_shader: R,
    fragment_shader: F,
    thread_pool: Pool
}

impl<L: LensShader + Send + Sync, R: RayShader + Send + Sync, F: PixelShader + Send + Sync> Renderer<L, R, F> {
    pub fn new(lens_shader: L, reflection_shader: R, fragment_shader: F) -> Renderer<L, R, F> {
        Renderer { lens_shader, reflection_shader, fragment_shader, thread_pool: Pool::new(WORKER_COUNT) }
    }

    pub fn render<S: Surface + Send + Sync>(&self, image: &mut Image, surface: &S) {
        let (tx, rx) = channel::<(usize, usize, Color)>();
        let (width, height) = image.resolution();

        self.thread_pool.scoped(|scope| {
            scope.execute(move || Self::wait_for_transmitted_pixels(image, rx));

            scope.zoom(|scope| {
                for py in 0..height {
                    let tx = tx.clone();
                    scope.execute(move || Self::render_line(py, width, height, &self.lens_shader, &self.reflection_shader, &self.fragment_shader, surface, tx))
                }

                drop(tx);
            });
        });
    }

    fn render_line<S: Surface + Send + Sync>(py: usize, width: usize, height: usize, lens_shader: &L, reflection_shader: &R, fragment_shader: &F, surface: &S, transmitter: Sender<(usize, usize, Color)>) {
        for px in 0..width {
            Renderer::render_pixel(px, py, width, height, lens_shader, reflection_shader, fragment_shader, surface, transmitter.clone());
        }
    }

    fn render_pixel<S: Surface + Send + Sync>(px: usize, py: usize, width: usize, height: usize, lens_shader: &L, reflection_shader: &R, fragment_shader: &F, surface: &S, transmitter: Sender<(usize, usize, Color)>) {
        let (nx, ny) = ((px as f64 + 0.5) / width as f64, (py as f64 + 0.5) / height as f64);

        let ray = lens_shader.ray_to_lens_point(nx, ny).unwrap();
        let reflections = Self::propagate_ray(reflection_shader, ray, surface);
        let final_color = fragment_shader.final_color(&reflections, surface);

        transmitter.send((px, py, final_color)).unwrap();
    }

    fn wait_for_transmitted_pixels(image: &mut Image, receiver: Receiver<(usize, usize, Color)>) {
        for (px, py, color) in receiver {
            image.paint(px, py, color);
        }
    }

    fn propagate_ray<S: Surface>(reflection_shader: &R, mut ray: Ray, surface: &S) -> VecDeque<SurfacePoint> {
        let mut reflection_stack: VecDeque<SurfacePoint> = VecDeque::with_capacity(reflection_shader.reflection_count_hint());

        loop {
            let reflection_point = match surface.intersect(ray) {
                Some(point) => point,
                _ => break
            };

            reflection_shader.on_intersection(ray, reflection_point, &mut reflection_stack);

            ray = match reflection_shader.next_ray(ray, reflection_point, &reflection_stack) {
                Some(ray) => ray,
                _ => break
            };
        }

        reflection_stack
    }
}