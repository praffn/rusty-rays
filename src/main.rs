extern crate image;

mod film;
mod geom;
mod light;
mod linalg;

use film::Camera;
use geom::Shape;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let ambient_light = light::AmbientLight::new(film::Color::white(), 0.8);

    let mat_a = geom::mat::DiffuseMaterial::new(1.0, film::Color::blue(), 1.0, film::Color::blue());

    let mat_b = geom::mat::DiffuseMaterial::new(0.8, film::Color::red(), 1.0, film::Color::red());

    let sphere_a = geom::Sphere::new(linalg::Point3::new(0.0, 0.0, 0.0), 1.0, Box::new(mat_a));
    let sphere_b = geom::Sphere::new(linalg::Point3::new(0.0, -0.5, -1.5), 0.2, Box::new(mat_b));

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(sphere_a), Box::new(sphere_b)];

    let scene = geom::Scene {
        lights: vec![Box::new(light::PointLight::new(
            linalg::Point3::new(3.0, 3.0, -3.0),
            film::Color::from_rgb(255, 255, 255),
            1.0,
        ))],
        ambient_light: Box::new(ambient_light),
        shape: Box::new(shapes),
    };

    let camera = film::PinholeCamera::builder()
        .position(linalg::Point3::new(0.0, 0.0, -3.0))
        .look_at(linalg::Point3::new(0.0, 0.0, 0.0))
        .resolution(image_width, image_height)
        .harmonize_dimensions(1.0)
        .build();

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let mut color = film::Color::new(0.3882352941, 0.431372549, 0.4470588235);
            let rays = camera.get_rays_for_coordinate(x, y);
            for ray in &rays {
                color += scene.trace(ray);
            }
            color *= 1.0 / rays.len() as f32;
            *pixel = image::Rgb(color.to_rgb())
        }
    }

    imgbuf.save("sphere.png").unwrap();
}
