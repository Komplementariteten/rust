use druid::{
    AppLauncher, PlatformError, Widget, WindowDesc, ImageBuf, 
    widget::{Label, Image, FillStrat, Flex}, 
    piet::{InterpolationMode}};

fn main() -> Result<(), PlatformError> {
    let main_win = WindowDesc::new(ui_builder());
    let data = 0_u32;
    AppLauncher::with_window(main_win).launch(data)

}

fn ui_builder() -> impl Widget<u32> {
    let img_data = ImageBuf::empty();
    let mut img_view = Image::new(img_data);
    let label = Label::new("test").center();
    img_view.set_fill_mode(FillStrat::FitWidth);
    img_view.set_interpolation_mode(InterpolationMode::Bilinear);
    Flex::column().with_child(img_view).with_child(label)
} 