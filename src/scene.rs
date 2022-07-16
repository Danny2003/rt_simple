use crate::aarect::*;
use crate::bvh::BVHNode;
use crate::constant_medium::ConstantMedium;
use crate::cornell_box::CornellBox;
pub use crate::hit::*;
use crate::material::*;
pub use crate::rt_weekend::*;
use crate::sphere::*;
use crate::texture::*;
pub use crate::vec3::Vec3;
use std::sync::Arc;

pub fn random_scene() -> HitList {
    let mut world = HitList::new();

    let checker = Arc::new(CheckerTexture::new_rgb(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_texture(checker)),
    )));

    // let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    // world.add(Arc::new(Sphere::new(
    //     Vec3::new(0., -1000., 0.),
    //     1000.,
    //     ground_material,
    // )));

    for a in -11..11 {
        for b in -11..11 {
            // choose material
            let choose_mat = random_double();
            let center = Vec3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            // 1.2^2 -0.8^2 < 0.9^2 to prevent being too close to the right sphere
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse: 80%
                    let albedo = Vec3::elemul(Vec3::random(), Vec3::random());
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal: 15%
                    let albedo = Vec3::random_in_range(0.5, 1.);
                    let fuzz = random_double_in_range(0., 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass: 5%
                    Arc::new(Dielectric::new(1.5))
                };
                if choose_mat < 0.8 {
                    let center2 = center + Vec3::new(0., random_double_in_range(0., 0.5), 0.);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.,
                        1.,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    // centre
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vec3::new(0., 1., 0.), 1., material1)));
    // left
    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., material2)));
    // right
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.));
    world.add(Arc::new(Sphere::new(Vec3::new(4., 1., 0.), 1., material3)));

    world
}
pub fn two_spheres() -> HitList {
    let mut world = HitList::new();

    let checker = Arc::new(CheckerTexture::new_rgb(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., -10., 0.),
        10.,
        Arc::new(Lambertian::new_texture(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., 10., 0.),
        10.,
        Arc::new(Lambertian::new_texture(checker)),
    )));
    world
}
pub fn two_perlin_spheres() -> HitList {
    let mut world = HitList::new();
    let pertext = Arc::new(NoiseTexture::new(4.));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::new_texture(pertext)),
    )));
    world
}
pub fn earth() -> HitList {
    let earth_texture = Arc::new(ImageTexture::new("input/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Vec3::new(0., 0., 0.), 2., earth_surface));
    let mut world = HitList::new();
    world.add(globe);
    world
}
pub fn simple_light() -> HitList {
    let mut world = HitList::new();
    let pertext = Arc::new(NoiseTexture::new(4.));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::new_texture(pertext)),
    )));
    let diff_light = Arc::new(DiffuseLight::new(Vec3::new(4., 4., 4.)));
    world.add(Arc::new(XYRectangle::new(3., 5., 1., 3., -2., diff_light)));
    world
}
pub fn cornell_box() -> HitList {
    let mut world = HitList::new();
    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(15., 15., 15.)));

    world.add(Arc::new(YZRectangle::new(0., 555., 0., 555., 555., green)));
    world.add(Arc::new(YZRectangle::new(0., 555., 0., 555., 0., red)));
    world.add(Arc::new(XZRectangle::new(
        213., 343., 227., 332., 554., light,
    )));
    world.add(Arc::new(XZRectangle::new(
        0.,
        555.,
        0.,
        555.,
        0.,
        white.clone(),
    )));
    world.add(Arc::new(XZRectangle::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.add(Arc::new(XYRectangle::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(CornellBox::new(
                Vec3::new(0., 0., 0.),
                Vec3::new(165., 330., 165.),
                white.clone(),
            )),
            15.,
        )),
        Vec3::new(265., 0., 295.),
    )));
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(CornellBox::new(
                Vec3::new(0., 0., 0.),
                Vec3::new(165., 165., 165.),
                white,
            )),
            -18.,
        )),
        Vec3::new(130., 0., 65.),
    )));
    world
}
/// We replace the two blocks with smoke and fog (dark and light particles),
/// and make the light bigger (and dimmer so it doesn’t blow out the scene) for faster convergence
pub fn cornell_smoke() -> HitList {
    let mut world = HitList::new();
    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(7., 7., 7.)));
    world.add(Arc::new(YZRectangle::new(0., 555., 0., 555., 555., green)));
    world.add(Arc::new(YZRectangle::new(0., 555., 0., 555., 0., red)));
    world.add(Arc::new(XZRectangle::new(
        113., 443., 127., 432., 554., light,
    )));
    world.add(Arc::new(XZRectangle::new(
        0.,
        555.,
        0.,
        555.,
        0.,
        white.clone(),
    )));
    world.add(Arc::new(XZRectangle::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.add(Arc::new(XYRectangle::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.add(Arc::new(ConstantMedium::new(
        Arc::new(Translate::new(
            Arc::new(RotateY::new(
                Arc::new(CornellBox::new(
                    Vec3::new(0., 0., 0.),
                    Vec3::new(165., 330., 165.),
                    white.clone(),
                )),
                15.,
            )),
            Vec3::new(265., 0., 295.),
        )),
        0.01,
        Vec3::zero(),
    )));
    world.add(Arc::new(ConstantMedium::new(
        Arc::new(Translate::new(
            Arc::new(RotateY::new(
                Arc::new(CornellBox::new(
                    Vec3::new(0., 0., 0.),
                    Vec3::new(165., 165., 165.),
                    white,
                )),
                -18.,
            )),
            Vec3::new(130., 0., 65.),
        )),
        0.01,
        Vec3::ones(),
    )));
    world
}
pub fn final_scene() -> HitList {
    let mut boxes1 = HitList::new();
    let ground = Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + w * i as f64;
            let z0 = -1000.0 + w * j as f64;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_in_range(1., 101.);
            let z1 = z0 + w;

            boxes1.add(Arc::new(CornellBox::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HitList::new();
    world.add(Arc::new(BVHNode::new(boxes1.list, 0., 1.)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(7., 7., 7.)));
    world.add(Arc::new(XZRectangle::new(
        123., 423., 147., 412., 554., light,
    )));
    let center1 = Vec3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = Arc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.,
        1.,
        50.,
        moving_sphere_material,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(260., 150., 45.),
        50.,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., 150., 145.),
        50.,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    )));
    let mut boundary = Arc::new(Sphere::new(
        Vec3::new(360., 150., 145.),
        70.,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new(
        boundary.clone(),
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(
        Vec3::new(0., 0., 0.),
        5000.,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Vec3::new(1., 1., 1.),
    )));

    let emat = Arc::new(Lambertian::new_texture(Arc::new(ImageTexture::new(
        "input/earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        Vec3::new(400., 200., 400.),
        100.,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    world.add(Arc::new(Sphere::new(
        Vec3::new(220., 280., 300.),
        80.,
        Arc::new(Lambertian::new_texture(pertext)),
    )));
    let mut boxes2 = HitList::new();
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_in_range(0., 165.),
            10.,
            white.clone(),
        )));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::new(boxes2.list, 0., 1.)),
            15.,
        )),
        Vec3::new(-100., 270., 395.),
    )));
    world
}
