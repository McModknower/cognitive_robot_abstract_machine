use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod rust_integration {
    use pyo3::prelude::*;
    use numpy::ndarray::array;
    use numpy::{PyArray, PyArray1, PyArray2, PyArray3, PyArrayMethods, PyReadonlyArray2, ToPyArray};

    #[pyfunction]
    fn is_supported_by(
        supported_shape: &Bound<'_,PyAny>,
	supported_frame_transform: PyReadonlyArray2<f64>,
	supported_transform: PyReadonlyArray2<f64>,
        supporting_shape: &Bound<'_,PyAny>,
	supporting_frame_transform: PyReadonlyArray2<f64>,
	supporting_transform: PyReadonlyArray2<f64>,
	max_intersection_height: f64,
    ) -> PyResult<bool> {
	// let height_diff = (self.aabb.lower().0[2] - obj.aabb.upper().0[2]).abs();
	
	Ok(false)
    }
}
