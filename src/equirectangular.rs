use bevy::camera::{CameraProjection, Projection, SubCameraView};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct EquirectangularProjection {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect_ratio: f32,
}

impl Default for EquirectangularProjection {
    fn default() -> Self {
        Self {
            fov: std::f32::consts::PI * 0.75, // 135°, wide but finite
            near: 0.1,
            far: 1000.0,
            aspect_ratio: 1.0, // panoramas are typically 2:1
        }
    }
}

impl CameraProjection for EquirectangularProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        // Equirectangular maps:
        //   horizontal → longitude (-π to π)
        //   vertical   → latitude  (-π/2 to π/2)
        // Approximate with a very wide perspective as base —
        // full equirectangular requires a custom shader pass.
        let fov = (std::f32::consts::PI * 0.75_f32).min(self.fov); // 135° max
        Mat4::perspective_infinite_reverse_rh(
            fov,
            self.aspect_ratio,
            self.near,
        )
    }

    fn get_clip_from_view_for_sub(&self, _sub: &SubCameraView) -> Mat4 {
        self.get_clip_from_view()
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    fn far(&self) -> f32 { self.far }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        let half_h_near = self.near * ops::tan(self.fov / 2.0);
        let half_w_near = half_h_near * self.aspect_ratio;
        let half_h_far  = z_far.abs() * ops::tan(self.fov / 2.0);
        let half_w_far  = half_h_far * self.aspect_ratio;

        // z is negative — forward is -Z in RH view space
        [
            Vec3A::new( half_w_near, -half_h_near, -z_near), // bottom right
            Vec3A::new( half_w_near,  half_h_near, -z_near), // top right
            Vec3A::new(-half_w_near,  half_h_near, -z_near), // top left
            Vec3A::new(-half_w_near, -half_h_near, -z_near), // bottom left
            Vec3A::new( half_w_far,  -half_h_far,  -z_far),  // bottom right
            Vec3A::new( half_w_far,   half_h_far,  -z_far),  // top right
            Vec3A::new(-half_w_far,   half_h_far,  -z_far),  // top left
            Vec3A::new(-half_w_far,  -half_h_far,  -z_far),  // bottom left
        ]
    }
}

impl EquirectangularProjection {
    pub fn projection() -> Projection {
        Projection::custom(Self {
            ..default()
        })
    }
}