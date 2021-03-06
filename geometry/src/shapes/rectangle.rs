use crate::{Point, Points, Size, BoundingBox};
use crate::utils::standard_aabb::get_bounding;
use crate::str_to_f32;

/// Rectangle data structure
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub name: String,
    pub points: Points,
    pub indices: Vec<i32>,
    pub origin: Point,
    pub aabb: BoundingBox
}

impl Rectangle {
    pub fn new (name: String, origin: Point, size: Size) -> Rectangle {
        let parts = get_bounding(&origin, &size);

        let mut points: Points = Vec::new();
        points.push(Point::new(parts.min_x, parts.min_y));
        points.push(Point::new(parts.max_x, parts.min_y));
        points.push(Point::new(parts.max_x, parts.max_y));
        points.push(Point::new(parts.min_x, parts.max_y));

        let indices = vec![0, 1, 2, 2, 3, 0];

        Rectangle {
            name,
            points,
            indices,
            origin,
            aabb: parts.aabb
        }
    }
}

/// Convert a String to Rectangle
/// Structure: "name,origin_x,origin_y,width,height"
/// Example: "rect,0,0,200,200"
impl From<String> for Rectangle {
    fn from(def: String) -> Rectangle {
        let parts: Vec<&str> = def.split(",").collect();

        let x = str_to_f32!(parts[1]);
        let y = str_to_f32!(parts[2]);
        let width = str_to_f32!(parts[3]);
        let height = str_to_f32!(parts[4]);

        return Rectangle::new("Rectangle 1".into(),Point::new(x, y), Size::new(width, height));
    }
}

/// Convert Rectangle to String
/// Example: let result: String = rectangle.into();
/// Result Structure: "name,origin_x,origin_y,width,height"
impl From<Rectangle> for String {
    fn from(rectangle: Rectangle) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.push(rectangle.name);
        parts.push(rectangle.origin.x.to_string());
        parts.push(rectangle.origin.y.to_string());
        parts.push(rectangle.aabb.width.to_string());
        parts.push(rectangle.aabb.height.to_string());
        return parts.join(",");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_rectangle() {
        let rectangle = Rectangle::new("Rectangle 1".into(),Point::new(0.0, 0.0), Size::new(200.0, 200.0));
        assert_rectangle(&rectangle);
    }

    #[test]
    fn rectangle_from() {
        let value = String::from("Rectangle 1,0,0,200,200");
        let rectangle: Rectangle = Rectangle::from(value);
        assert_rectangle(&rectangle);
    }

    #[test]
    fn rectangle_to_string() {
        let rectangle = Rectangle::new("Rectangle 1".into(),Point::new(0.0, 0.0), Size::new(200.0, 200.0));
        let result: String = rectangle.into();
        assert_eq!(result, "Rectangle 1,0,0,200,200");
    }

    fn assert_rectangle(rectangle: &Rectangle) {
        assert_eq!(rectangle.name, "Rectangle 1");

        assert_eq!(rectangle.origin.x, 0.0);
        assert_eq!(rectangle.origin.y, 0.0);
        assert_eq!(rectangle.points.len(), 6);

        assert_eq!(rectangle.points[0].x, -100.0);
        assert_eq!(rectangle.points[0].y, -100.0);
        assert_eq!(rectangle.points[1].x, 100.0);
        assert_eq!(rectangle.points[1].y, -100.0);
        assert_eq!(rectangle.points[2].x, 100.0);
        assert_eq!(rectangle.points[2].y, 100.0);
        assert_eq!(rectangle.points[4].x, -100.0);
        assert_eq!(rectangle.points[4].y, 100.0);

        assert_eq!(rectangle.indices.len(), 6);
        assert_eq!(rectangle.indices[0], 0);
        assert_eq!(rectangle.indices[1], 1);
        assert_eq!(rectangle.indices[2], 2);
        assert_eq!(rectangle.indices[3], 2);
        assert_eq!(rectangle.indices[4], 3);
        assert_eq!(rectangle.indices[5], 0);

        assert_eq!(rectangle.aabb.min_x, -100.0);
        assert_eq!(rectangle.aabb.max_x, 100.0);
        assert_eq!(rectangle.aabb.min_y, -100.0);
        assert_eq!(rectangle.aabb.max_y, 100.0);
    }
}