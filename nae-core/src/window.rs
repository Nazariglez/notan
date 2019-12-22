pub trait BaseWindow {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn fullscreen(&self) -> bool;
    fn title(&self) -> &str;
}
