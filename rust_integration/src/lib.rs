use numpy::nalgebra::{Dyn, Matrix4, MatrixView4, VectorView4};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod rust_integration {
    use super::*;
    use numpy::PyReadonlyArray2;
    use numpy::nalgebra::{Dyn, U4};

    #[pyfunction]
    fn is_supported_by(
        supported_shape: BoundingBox,
        supported_transform: PyReadonlyArray2<f64>,
        supporting_shape: BoundingBox,
        supporting_transform: PyReadonlyArray2<f64>,
        supported_frame_t_supporting_frame: PyReadonlyArray2<f64>,
        max_intersection_height: f64,
    ) -> PyResult<bool> {
        // migrate types to ones I can use in rust
        let supported_transform = supported_transform
            .try_as_matrix::<U4, U4, Dyn, Dyn>()
            .expect("supported_transform");
        let supporting_transform = supporting_transform
            .try_as_matrix::<U4, U4, Dyn, Dyn>()
            .expect("supporting_transform");
        let supported_frame_t_supporting_frame = supported_frame_t_supporting_frame
            .try_as_matrix::<U4, U4, Dyn, Dyn>()
            .expect("supported_frame_t_supporting_frame");

        // transform both OBBs to reference frame of supported body
        let supported_shape = supported_shape.transform(&supported_transform);
        let supporting_shape = supporting_shape
            .transform(&(supported_frame_t_supporting_frame * supporting_transform).as_view());

        // // check if the bottom of supported shape is close to top of supporting shape
        // let height_diff = (supported_shape.min_z - supporting_shape.max_z).abs();
        // if height_diff > max_intersection_height {
        //     return Ok(false);
        // }

        // check if supported shape is inside x/y column of supporting shape.
        let result = supporting_shape.min_x <= supported_shape.min_x
            && supported_shape.max_x <= supporting_shape.max_x
            && supporting_shape.min_y <= supported_shape.min_y
            && supported_shape.max_y <= supporting_shape.max_y;

        Ok(result)
    }
}

#[derive(FromPyObject)]
struct BoundingBox {
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

impl BoundingBox {
    fn transform(&self, self_t_new_pose: &MatrixView4<f64, Dyn, Dyn>) -> Self {
        // Get all 8 corners of the BB in link-local space
        let mut list_self_t_corner = Vec::with_capacity(8);
        for x in [self.min_x, self.max_x] {
            for y in [self.min_y, self.max_y] {
                for z in [self.min_z, self.max_z] {
                    list_self_t_corner.push(Matrix4::from_row_slice(&[
                        1., 0., 0., x, 0., 1., 0., y, 0., 0., 1., z, 0., 0., 0., 1.,
                    ]));
                }
            }
        } // shape (8, 3)

        let list_reference_t_corner = list_self_t_corner
            .iter()
            .map(|self_t_corner| self_t_new_pose * self_t_corner)
            .collect::<Vec<_>>();

        let list_reference_p_corner = list_reference_t_corner
            .iter()
            .map(|reference_t_corner| reference_t_corner.fixed_view(0, 3))
            .collect::<Vec<_>>();

        //Compute new corner points

        Self {
            min_x: minimum(&list_reference_p_corner, 0),
            min_y: minimum(&list_reference_p_corner, 1),
            min_z: minimum(&list_reference_p_corner, 2),
            max_x: maximum(&list_reference_p_corner, 0),
            max_y: maximum(&list_reference_p_corner, 1),
            max_z: maximum(&list_reference_p_corner, 2),
        }
    }
}

fn minimum(arrays: &Vec<VectorView4<f64>>, axis: usize) -> f64 {
    let mut min = f64::MAX;
    for array in arrays {
        min = min.min(array[axis]);
    }
    min
}

fn maximum(arrays: &Vec<VectorView4<f64>>, axis: usize) -> f64 {
    let mut max = f64::MIN;
    for array in arrays {
        max = max.max(array[axis]);
    }
    max
}
