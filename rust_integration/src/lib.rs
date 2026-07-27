use pyo3::prelude::*;
use numpy::array;
use numpy::ndarray::{s, ArrayView2};

/// A Python module implemented in Rust.
#[pymodule]
mod rust_integration {
    use super::*;
    use pyo3::prelude::*;
    use numpy::PyReadonlyArray2;

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
	let supported_transform = supported_transform.try_as_matrix().expect("supported_transform");
	let supporting_transform = supporting_transform.try_as_matrix().expect("supporting_transform");
	let supported_frame_t_supporting_frame = supported_frame_t_supporting_frame.try_as_matrix().expect("supported_frame_t_supporting_frame");
	// transform both OBBs to reference frame of supported body
	let supported_shape = supported_shape.transform(&supported_transform);
	let supporting_shape = supported_shape.transform(&(supported_frame_t_supporting_frame * supporting_transform));
	
	Ok(false)
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
    fn transform(&self, self_t_new_pose: &ArrayView2<f64>) -> Self {
	// Get all 8 corners of the BB in link-local space
	let mut list_self_t_corner = Vec::with_capacity(8);
	for x in [self.min_x, self.max_x] {
	    for y in [self.min_y, self.max_y] {
		for z in [self.min_z, self.max_z] {
		    list_self_t_corner.push(
			array![[1., 0., 0., x],
			       [0., 1., 0., y],
			       [0., 0., 1., z],
			       [0., 0., 0., 1.]]
		    );
		}
	    }
	}; // shape (8, 3)
	
	let list_reference_t_corner =
	    list_self_t_corner.iter().map(
		|self_t_corner| self_t_new_pose * self_t_corner
	    ).collect::<Vec<_>>();
	
	let list_reference_p_corner =
	    list_reference_t_corner.iter().map(
		|reference_t_corner| reference_t_corner.slice(s![..3, 3..])
	    ).collect::<Vec<_>>();

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

fn minimum(arrays: &Vec<ArrayView2<f64>>, axis: usize) -> f64 {
    let mut min = f64::MAX;
    for array in arrays {
	min = min.min(*array.get([axis,0]).expect("array has no value on given axis"));
    }
    min
}

fn maximum(arrays: &Vec<ArrayView2<f64>>, axis: usize) -> f64 {
    let mut max = f64::MIN;
    for array in arrays {
	max = max.max(*array.get([axis,0]).expect("array has no value on given axis"));
    }
    max
}
