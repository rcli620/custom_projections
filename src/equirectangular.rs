use bevy::camera::{CameraProjection, SubCameraView};
use bevy::math::{Mat4, Vec3A};

#[derive(Debug, Clone)]
pub struct EquirectangularProjection {
    pub near: f32,
    pub far: f32,
    pub aspect_ratio: f32,
}

impl Default for EquirectangularProjection {
    fn default() -> Self {
        Self {
            near: 0.1,
            far: 1000.0,
            aspect_ratio: 2.0, // panoramas are typically 2:1
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
        Mat4::perspective_infinite_reverse_rh(
            std::f32::consts::PI,   // 180° vertical FOV
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
        let w = self.aspect_ratio;
        let h = 1.0;
        [
            Vec3A::new( w, -h, z_near),
            Vec3A::new( w,  h, z_near),
            Vec3A::new(-w,  h, z_near),
            Vec3A::new(-w, -h, z_near),
            Vec3A::new( w, -h, z_far),
            Vec3A::new( w,  h, z_far),
            Vec3A::new(-w,  h, z_far),
            Vec3A::new(-w, -h, z_far),
        ]
    }
}