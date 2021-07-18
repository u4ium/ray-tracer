pub mod camera;
pub mod image;
pub mod light;
pub mod object;
pub mod ppm;
pub mod ray;
pub mod scene;
pub mod vector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
