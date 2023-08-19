use super::vec::Vector2;

pub struct Point2<T>([T; 2]);
pub struct Point3<T>([T; 3]);

impl<T> From<Point3<T>> for Point2<T> {
    fn from(value: Point3<T>) -> Self {
        let [x, y, _] = value.0;
        Self([x, y])
    }
}

impl<T> From<Vector2<T>> for Point2<T> {
    fn from(vec: Vector2<T>) -> Self {
        Self(vec.take_inner())
    }
}
