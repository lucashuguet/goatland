pub mod mesh;
pub mod orientation;

#[cfg(test)]
mod goatland {
    use std::f32::consts::PI;

    use bevy_math::{Quat, Vec2, Vec3};

    use crate::{mesh, orientation};

    #[test]
    fn test_deg2rand() {
        assert_eq!(orientation::deg2rand(180), PI);
    }

    #[test]
    fn test_parse_orientation() {
        let top = (Vec3::new(0.5, 1.5, -0.5), Quat::from_rotation_x(-PI / 2.));
        let bottom = (Vec3::new(0.5, -0.5, 0.5), Quat::from_rotation_x(PI / 2.));
        let left = (Vec3::new(-0.5, 0.5, 0.5), Quat::from_rotation_y(-PI / 2.));
        let right = (Vec3::new(0.5, 0.5, -0.5), Quat::from_rotation_y(PI / 2.));
        let front = (Vec3::new(0.5, 0.5, 0.5), Quat::IDENTITY);
        let back = (Vec3::new(-0.5, 0.5, -0.5), Quat::from_rotation_y(PI));

        assert_eq!(
            orientation::parse_orientation(
                Vec3::ZERO,
                Vec2::ONE * 2.,
                &orientation::Orientation::Top
            ),
            top
        );

        assert_eq!(
            orientation::parse_orientation(
                Vec3::ZERO,
                Vec2::ONE * 2.,
                &orientation::Orientation::Bottom
            ),
            bottom
        );

        assert_eq!(
            orientation::parse_orientation(
                Vec3::ZERO,
                Vec2::ONE * 2.,
                &orientation::Orientation::Left
            ),
            left
        );

        assert_eq!(
            orientation::parse_orientation(
                Vec3::ZERO,
                Vec2::ONE * 2.,
                &orientation::Orientation::Right
            ),
            right
        );

        assert_eq!(
            orientation::parse_orientation(
                Vec3::ZERO,
                Vec2::ONE * 2.,
                &orientation::Orientation::Front
            ),
            front
        );

        assert_eq!(
            orientation::parse_orientation(
                Vec3::ZERO,
                Vec2::ONE * 2.,
                &orientation::Orientation::Back
            ),
            back
        );
    }

    #[test]
    fn test_parse_normal() {
        assert_eq!(orientation::parse_normal(0), orientation::Orientation::Left);
        assert_eq!(
            orientation::parse_normal(1),
            orientation::Orientation::Bottom
        );
        assert_eq!(orientation::parse_normal(2), orientation::Orientation::Back);
        assert_eq!(
            orientation::parse_normal(3),
            orientation::Orientation::Right
        );
        assert_eq!(orientation::parse_normal(4), orientation::Orientation::Top);
        assert_eq!(
            orientation::parse_normal(5),
            orientation::Orientation::Front
        );
    }

    #[test]
    fn test_genchunk() {
        assert_eq!(mesh::genchunk(0, 0, 0).num_quads(), 375);
    }
}
