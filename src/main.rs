mod services_factory;
use fltk::{app, button::*, enums::Color, enums::Font, frame::Frame, prelude::*, window::Window};

use fltk_evented::Listener;

use fltk_theme::{color_themes, ColorTheme};
use services_factory::*;

fn main() {
    let msg = greet("Pat", "english");
    println!("{}", msg);

    // format!("{:?}",msg);
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let theme = ColorTheme::new(color_themes::SHAKE_THEME);
    theme.apply();
    let mut win = Window::new(100, 100, 340, 320, "Practice Run App");

    let but_decrease: Listener<_> = Button::default()
        .with_label("-")
        .with_pos(10, 240)
        .with_size(100, 30)
        .into();
    let but_increase: Listener<_> = Button::default()
        .with_label("+")
        .with_pos(230, 240)
        .with_size(100, 30)
        .into();

    let mut counter = Frame::default().with_size(230, 100).center_of(&win);

    let mut  header_label = Frame::default().with_size(100, 80);
    header_label.set_label("The Button Counter App");
    header_label.set_label_size(22);
    header_label.set_label_color(Color::from_hex(0x42A5F5));
    // header_label.with_pos_y(10,10);
    header_label.center_x(&win).set_label_font(Font::Times);
    
    
    win.make_resizable(false);
    win.end();
    win.show();
    counter.set_label_size(30);
    counter.set_label_font(Font::Times);
    counter.set_label_color(Color::from_hex(0x42A5F5));

    let mut val: i8 = 0;
    while app.wait() {
        if but_increase.triggered() {
            if val != 127{
            val += 1};
        }
        if but_decrease.triggered() {
            if val != 0{
            val -= 1;
        }
    }
        counter.set_label(&val.to_string());
    }

    app.run().unwrap();
}

// a;
