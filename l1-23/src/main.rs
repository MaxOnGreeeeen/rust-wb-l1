fn main() {
    let point_a = Point::new(10.0, 20.0);
    let point_b = Point::new(20.0, 30.0);

    assert_eq!(point_a.distance(&point_b), 14.142136);
    assert_eq!(point_a.distance(&point_b), point_b.distance(&point_a));
}

// Структура Point с инкапсулированными параметрами x,y
pub struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub trait DistanceTrait {
    fn distance(&self, point: &Self) -> f32;
}
// Трейт доступен только для чисел с плавающей точкой
impl<T> DistanceTrait for Point<T>
where
    T: Copy + Into<f32>,
{
    // Нахождение расстояния между точками
    fn distance(&self, point: &Point<T>) -> f32 {
        let dx = self.x.into() - point.x.into();
        let dy = self.y.into() - point.y.into();
        let sum_of_squares = (dx * dx) + (dy * dy);

        (f32::from(sum_of_squares)).sqrt()
    }
}
