use pyo3::prelude::*;

/// An immutable array of Polygon geometries in WebAssembly memory using GeoArrow's in-memory
/// representation.
#[pyclass]
pub struct PolygonArray(pub(crate) geoarrow::array::PolygonArray<i32>);

impl From<geoarrow::array::PolygonArray<i32>> for PolygonArray {
    fn from(value: geoarrow::array::PolygonArray<i32>) -> Self {
        Self(value)
    }
}

impl From<PolygonArray> for geoarrow::array::PolygonArray<i32> {
    fn from(value: PolygonArray) -> Self {
        value.0
    }
}