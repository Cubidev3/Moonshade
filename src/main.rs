use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use moonshade_raytracer::math::color::Color;
use moonshade_raytracer::math::matrix::Matrix;
use moonshade_raytracer::math::point::Point;
use moonshade_raytracer::math::ray::Ray;
use moonshade_raytracer::math::transformation::scale::Scale;
use moonshade_raytracer::math::transformation::transform::Transform;
use moonshade_raytracer::math::vector::Vector;
use moonshade_raytracer::renderer::pixel_shader::default_pixel_shader::DefaultPixelShader;
use moonshade_raytracer::renderer::ray_shader::default_ray_shader::DefaultRayShader;
use moonshade_raytracer::renderer::image::Image;
use moonshade_raytracer::renderer::lens_shader::panorama_orthographic_lens_shader::PanoramaOrthographicLensShader;
use moonshade_raytracer::renderer::lens_shader::panorama_perspective_lens_shader::PanoramaPerspectiveLensShader;
use moonshade_raytracer::renderer::lens_shader::plane_perspective_lens_shader::PlanePerspectiveLensShader;
use moonshade_raytracer::renderer::lens_shader::sphere_lens_shader::SphereLensShader;
use moonshade_raytracer::renderer::renderer::Renderer;
use moonshade_raytracer::surfaces::material::Material;
use moonshade_raytracer::surfaces::multiple_surfaces::MultipleSurfaces;
use moonshade_raytracer::surfaces::sphere::Sphere;
use moonshade_raytracer::surfaces::surface::Surface;
use moonshade_raytracer::surfaces::transformed_surface::TransformedSurface;

fn main() {
    let lens = PlanePerspectiveLensShader::new(2.0, Vector::new(16.0, 9.0, 0.0));
    let spherical_lens = SphereLensShader::full_sphere(5.0);
    let panorama_lens = PanoramaOrthographicLensShader::full_amplitude(9.0, 2.0);

    let ray_shader = DefaultRayShader::new(5);
    let pixel_shader = DefaultPixelShader;

    let renderer = Renderer::new(lens, ray_shader, pixel_shader);

    let sphere = TransformedSurface::new(
        Transform::translation(Vector::FORWARD * 5.0),
        Sphere::new(
            3.0,
            Material::new(Color::GREEN)
        )
    );

    let sphere2 = TransformedSurface::new(
        Transform::translation(Vector::FORWARD * -5.0),
        Sphere::new(
            1.0,
            Material::new(Color::new(0.25, 0.5, 1.0, 1.0))
        )
    );

    let world = MultipleSurfaces::new(vec![Box::new(sphere), Box::new(sphere2)]);

    let mut image = Image::new(1920, 1080);
    renderer.render(&mut image, &world);

    let mut file = File::create("raytraced.pbm").unwrap();
    let _ = file.write(image.pbm().as_bytes());
}
