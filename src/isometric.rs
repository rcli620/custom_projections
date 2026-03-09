use bevy::camera::{CameraProjection, SubCameraView};
use bevy::math::{Mat4, Vec3A};

#[derive(Debug, Clone)]
pub struct IsometricProjection {
    pub scale: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraProjection for IsometricProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        Mat4::orthographic_rh(
            -self.aspect_ratio * self.scale,
            self.aspect_ratio * self.scale,
            -self.scale,
            self.scale,
            self.near,
            self.far,
        )
    }

    fn get_clip_from_view_for_sub(&self, _sub_view: &SubCameraView) -> Mat4 {
        self.get_clip_from_view()
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    fn far(&self) -> f32 { self.far }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        let w = self.aspect_ratio * self.scale;
        let h = self.scale;
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