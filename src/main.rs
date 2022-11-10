mod tg;

fn main() {

    let cols = tg::term_width();
    let rows = tg::term_height();
    println!("width: {cols}, height: {rows}");

    let mut old_settings = tg::game_settings();

    loop
    {
        println!("{}", tg::get_key());
    }

    tg::restore_settings(&mut old_settings);
}
