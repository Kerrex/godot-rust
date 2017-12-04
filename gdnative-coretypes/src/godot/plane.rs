use godot::*;
use gdnative_sys::*;
use std::ops::Neg;

fn new_plane() -> godot_plane {
    godot_plane {
        _dont_touch_that: [0; 16usize],
    }
}

impl GDPlane {
    pub fn new_from_f32(a: f32, b: f32, c: f32, d: f32) -> GDPlane {
        unsafe {
            let mut new_plane = new_plane();
            godot_plane_new_with_reals(&mut new_plane, a, b, c, d);
            GDPlane { _plane: new_plane }
        }
    }

    pub fn new_from_vectors(v1: &GDVector3, v2: &GDVector3, v3: &GDVector3) -> GDPlane {
        unsafe {
            let mut new_plane = new_plane();
            godot_plane_new_with_vectors(&mut new_plane, &v1._vector, &v2._vector, &v3._vector);
            GDPlane { _plane: new_plane }
        }
    }

    pub fn new_from_normal(normal: &GDVector3, d: f32) -> GDPlane {
        unsafe {
            let mut new_plane = new_plane();
            godot_plane_new_with_normal(&mut new_plane, &normal._vector, d);
            GDPlane { _plane: new_plane }
        }
    }

    pub fn normalized(&self) -> GDPlane {
        unsafe {
            let new_plane = godot_plane_normalized(&self._plane);
            GDPlane { _plane: new_plane }
        }
    }

    pub fn center(&self) -> GDVector3 {
        unsafe {
            let center = godot_plane_center(&self._plane);
            GDVector3 { _vector: center }
        }
    }

    // No idea what's that tbh
    pub fn get_any_point(&self) -> GDVector3 {
        unsafe {
            let any_point = godot_plane_get_any_point(&self._plane);
            GDVector3 { _vector: any_point }
        }
    }

    pub fn is_point_over(&self, point: &GDVector3) -> bool {
        unsafe { godot_plane_is_point_over(&self._plane, &point._vector) }
    }

    pub fn has_point(&self, point: &GDVector3, epsilon: f32) -> bool {
        unsafe { godot_plane_has_point(&self._plane, &point._vector, epsilon) }
    }

    pub fn distance_to(&self, point: &GDVector3) -> f32 {
        unsafe { godot_plane_distance_to(&self._plane, &point._vector) }
    }

    pub fn project(&self, point: &GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_plane_project(&self._plane, &point._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn intersect_three_planes(
        &self,
        b: &GDPlane,
        c: &GDPlane,
        destination: &mut GDVector3,
    ) -> bool {
        unsafe {
            godot_plane_intersect_3(&self._plane, &mut destination._vector, &b._plane, &c._plane)
        }
    }

    pub fn insersects_ray(
        &self,
        destination: &mut GDVector3,
        from: &GDVector3,
        direction: &GDVector3,
    ) -> bool {
        unsafe {
            godot_plane_intersects_ray(
                &self._plane,
                &mut destination._vector,
                &from._vector,
                &direction._vector,
            )
        }
    }

    pub fn intersects_segment(
        &self,
        destination: &mut GDVector3,
        begin: &GDVector3,
        end: &GDVector3,
    ) -> bool {
        unsafe {
            godot_plane_intersects_segment(
                &self._plane,
                &mut destination._vector,
                &begin._vector,
                &end._vector,
            )
        }
    }

    pub fn get_normal(&self) -> GDVector3 {
        unsafe {
            let vector = godot_plane_get_normal(&self._plane);
            GDVector3 { _vector: vector }
        }
    }

    pub fn set_normal(&mut self, new_normal: &GDVector3) {
        unsafe {
            godot_plane_set_normal(&mut self._plane, &new_normal._vector)
        }
    }

    pub fn get_d(&self) -> f32 {
        unsafe {
            godot_plane_get_d(&self._plane)
        }
    }

    pub fn set_d(&mut self, new_d: f32) {
        unsafe {
            godot_plane_set_d(&mut self._plane, new_d)
        }
    }
}

impl Neg for GDPlane {
    type Output = GDPlane;

    fn neg(self) -> GDPlane {
        unsafe {
            let plane = godot_plane_operator_neg(&self._plane);
            GDPlane { _plane: plane }
        }
    }
}

impl PartialEq for GDPlane {
    fn eq(&self, other: &GDPlane) -> bool {
        unsafe {
            godot_plane_operator_equal(&self._plane, &other._plane)
        }
    }
}

impl Eq for GDPlane {}
