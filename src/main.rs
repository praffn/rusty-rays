extern crate image;

mod film;
mod geom;
mod linalg;

use film::Camera;
use geom::Shape;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let camera = film::PinholeCamera::builder()
        .position(linalg::Point3::new(0.0, 0.0, -3.0))
        .look_at(linalg::Point3::new(0.0, 0.0, 0.0))
        .resolution(image_width, image_height)
        .harmonize_dimensions(1.0)
        .build();

    let sphere_a = geom::Sphere::new(linalg::Point3::new(0.0, 0.0, 0.0), 1.0);
    let sphere_b = geom::Sphere::new(linalg::Point3::new(1.5, 0.0, 0.0), 0.2);

    let shapes = vec![sphere_a, sphere_b];

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let mut color = film::Color::new(0.3882352941, 0.431372549, 0.4470588235);
            let rays = camera.get_rays_for_coordinate(x, y);
            let len = rays.len();
            for ray in rays {
                match shapes.hit(&ray) {
                    None => {}
                    Some(_) => {
                        color = color + film::Color::new(0.9921568627, 0.4745098039, 0.6588235294)
                    }
                }
            }
            color = color * (1.0 / len as f32);
            *pixel = image::Rgb(color.to_rgb())
        }
    }

    imgbuf.save("sphere.png").unwrap();
}
