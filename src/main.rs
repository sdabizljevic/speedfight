// Example 8: Input
// Respond to user keyboard and mouse input onscreen
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Color, Image, VectorFont},
    input::{Key, MouseButton},
    run, Graphics, Input, Result, Settings, Timer, Window,
};

fn main() {
    run(
        Settings {
            title: "Input Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    //loads all necessary images and fonts.
    let castle_image = Image::load(&gfx, "titlescreen_background.png").await?;
    let hero = Image::load(&gfx, "fancy_pants.png").await?;
    let arena1 = Image::load(&gfx, "sewer_background.png").await?;
    let enemy1 = Image::load(&gfx, "bokoblin.png").await?;
    let title_font = VectorFont::load("title_font.ttf").await?;
    let main_font = VectorFont::load("main_font.ttf").await?;
    let punch_image = Image::load(&gfx, "punch.png").await?;
    let kick_image = Image::load(&gfx, "kick.png").await?;

    //creates necessary variables, like the frame counter or title screen tracker.
    let mut frames = Timer::time_per_second(30.0);
    let mut on_title_screen: u8 = 0;

    //starts running the game, this allows me to exit the game on command (whenever someone presses ESC).
    let mut running = true;
    while running {
        //loads title screen only on open.
        if on_title_screen == 0 {
            title_screen(&window, &mut gfx, &castle_image, &title_font)?;
            on_title_screen = 1;
        }

        //constantly loops checking for inputs.
        while let Some(event) = input.next_event().await {
            //stops game whenever escape is pressed.
            if input.key_down(Key::Escape) { running = false; }

            //if on the title screen, space will start the game.
            if input.key_down(Key::Space) && on_title_screen == 1 { 
                //fades the screen to black.
                let mut alpha: f32 = 0.0;
                let rect = Rectangle::new(Vector::new(0.0, 0.0), Vector::new(1100.0, 800.0));
                while alpha <= 1.0 {
                    while frames.tick() {
                        alpha += 0.01;
                        gfx.fill_rect(&rect, Color::BLACK.with_alpha(alpha));
                        gfx.present(&window)?;
                    }
                }
                gfx.clear(Color::BLACK);
                gfx.present(&window)?;
                on_title_screen = 2;

                //loads the first fight
                load_fight(&window, &mut gfx, &arena1, &hero, &enemy1);
                fight_one(&window, &mut gfx, &main_font, &punch_image, &kick_image);
            }

        }
    }
    Ok(())
}

//puts up the title screen.
fn title_screen(window: &Window, mut gfx: &mut Graphics, castle_image: &Image, title_font: &VectorFont) -> Result<()> {
    gfx.clear(Color::WHITE);

    //puts the background image up.
    let background_image = Rectangle::new(Vector::new(0.0, 0.0), castle_image.size());
    gfx.draw_image(&castle_image, background_image);

    //puts up the game title.
    let title_string = title_font.to_renderer(&gfx, 150.0);
    title_string?.draw(
        &mut gfx, 
        "SPEEDFIGHT", 
        Color::RED, 
        Vector::new(200.0, 200.0)
    )?;
    let title_string2 = title_font.to_renderer(&gfx, 148.5);
    title_string2?.draw(
         &mut gfx, 
        "SPEEDFIGHT", 
        Color::BLACK, 
        Vector::new(205.0, 200.0)
    )?;

    //puts up the "press space to continue" and "escape at..." buttons.
    let start_button = Rectangle::new(Vector::new(315.0, 490.0), Vector::new(400.0, 50.0));
    gfx.fill_rect(&start_button, Color::BLACK);
    gfx.stroke_rect(&start_button, Color::RED);
    let start_button_string = title_font.to_renderer(&gfx, 40.0);
    start_button_string?.draw(
        &mut gfx, 
        "Press SPACE to start", 
        Color::RED, 
        Vector::new(350.0, 525.0)
    )?;
    let escape_button_tutorial = title_font.to_renderer(&gfx, 27.0);
    escape_button_tutorial?.draw(
        &mut gfx, 
        "Press ESC at any time to quit", 
        Color::RED, 
        Vector::new(25.0, 750.0)
    )?;

    //presents all of the changes i've made to gfx.
    gfx.present(&window)?;

    //returns Ok assuming everything loaded properly.
    Ok(())
}

//loads the general fightscene, aside from health bars and other varying graphics that will change frequently.
fn load_fight(window: &Window, mut gfx: &mut Graphics, background_image: &Image, hero_image: &Image, enemy_image: &Image) -> Result<()> {
    gfx.clear(Color::BLACK);

    //draws background with hero and enemy on top.
    let background = Rectangle::new(Vector::new(0.0, 0.0), background_image.size());
    gfx.draw_image(&background_image, background);
    let hero = Rectangle::new(Vector::new(75.0, 400.0), hero_image.size());
    gfx.draw_image(&hero_image, hero);
    let enemy = Rectangle::new(Vector::new(550.0, 425.0), enemy_image.size());
    gfx.draw_image(&enemy_image, enemy);

    Ok(())
}

//function to update health bar rectangles.
fn health_bars_update(window: &Window, mut gfx: &mut Graphics, hero_health: &f32, enemy_health: &f32, enemy_total_health: &f32) -> Result<()> {
    //create the widths for the red and green portion of the health bars.
    let hero_green_width: f32 = (hero_health).clone();
    let hero_red_width: f32 = (100.0 - hero_health).clone();
    let enemy_green_width: f32 = (enemy_health).clone();
    let enemy_red_width: f32 = (enemy_total_health - enemy_health).clone();
    
    //creates and fills in the rectangles.
    let hero_green_rectangle = Rectangle::new(Vector::new(100.0, 350.0), Vector::new(hero_green_width * 2.0, 25.0));
    gfx.fill_rect(&hero_green_rectangle, Color::GREEN);
    let hero_red_rectangle = Rectangle::new(Vector::new(100.0 + hero_green_width * 2.0, 350.0), Vector::new(hero_red_width * 2.0, 25.0));
    gfx.fill_rect(&hero_red_rectangle, Color::RED);
    let enemy_green_rectangle = Rectangle::new(Vector::new(550.0, 350.0), Vector::new(enemy_green_width, 25.0));
    gfx.fill_rect(&enemy_green_rectangle, Color::GREEN);
    let enemy_red_rectangle = Rectangle::new(Vector::new(550.0 + enemy_green_width, 350.0), Vector::new(enemy_red_width, 25.0));
    gfx.fill_rect(&enemy_red_rectangle, Color::RED);

    Ok(())
}

//function to update the turn count.
fn turn_counter_update(window: &Window, mut gfx: &mut Graphics, font: &VectorFont, turn_count: &i32) -> Result<()> {
    let turn_count_string = turn_count.to_string();
    let turn_label = font.to_renderer(&gfx, 50.0);
    turn_label?.draw(
        &mut gfx, 
        &format!("Turn {}", turn_count_string).as_str(), 
        Color::GREEN, 
        Vector::new(450.0, 50.0)
    )?;

    Ok(())
}

//function that loads the bulk of the first fight. (fight label, available move icons, turn counter, and health bars)
fn fight_one(window: &Window, mut gfx: &mut Graphics, font: &VectorFont, punch_image: &Image, kick_image: &Image) -> Result<()> {
    //labels fight 1 at the top.
    let fight_label = font.to_renderer(&gfx, 50.0);
    fight_label?.draw(
        &mut gfx, 
        "FIGHT 1", 
        Color::GREEN, 
        Vector::new(50.0, 50.0)
    )?;

    //presents the moves you have access to in this fight. (you start with 2, and gain more as the game progresses).
    let punch_rectangle = Rectangle::new(Vector::new(50.0, 110.0), Vector::new(120.0, 120.0));
    gfx.fill_rect(&punch_rectangle, Color::YELLOW);
    gfx.stroke_rect(&punch_rectangle, Color::BLACK);
    let punch_icon = Rectangle::new(Vector::new(50.0, 125.0), punch_image.size());
    gfx.draw_image(&punch_image, punch_icon);
    let punch_key = font.to_renderer(&gfx, 50.0);
    punch_key?.draw(
        &mut gfx, 
        "Q", 
        Color::BLACK, 
        Vector::new(130.0, 220.0)
    )?;
    let kick_rectangle = Rectangle::new(Vector::new(190.0, 110.0), Vector::new(120.0, 120.0));
    gfx.fill_rect(&kick_rectangle, Color::ORANGE);
    gfx.stroke_rect(&kick_rectangle, Color::BLACK);
    let kick_icon = Rectangle::new(Vector::new(205.0, 110.0), kick_image.size());
    gfx.draw_image(&kick_image, kick_icon);
    let kick_key = font.to_renderer(&gfx, 50.0);
    kick_key?.draw(
        &mut gfx, 
        "W", 
        Color::BLACK, 
        Vector::new(270.0, 220.0)
    )?;
    
    //loads turn counter.
    turn_counter_update(&window, &mut gfx, &font, &1);

    //loads health bars.
    health_bars_update(&window, &mut gfx, &50.0, &50.0, &100.0);

    gfx.present(&window)?;
    Ok(())
}