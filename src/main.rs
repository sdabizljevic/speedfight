// Example 8: Input
// Respond to user keyboard and mouse input onscreen
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Color, Image, VectorFont},
    input::{Key, MouseButton},
    run, Graphics, Input, Result, Settings, Timer, Window,
};
use rand::Rng;
use std::{thread, time};

//struct for any entity in the game, including the hero and enemies.
struct Entity {
    max_health: f32,
    current_health: f32,
    moves_unlocked: u8,
}

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
    let hero_image = Image::load(&gfx, "fancy_pants.png").await?;
    let arena1_image = Image::load(&gfx, "sewer_background.png").await?;
    let enemy1_image = Image::load(&gfx, "bokoblin.png").await?;
    let title_font = VectorFont::load("title_font.ttf").await?;
    let main_font = VectorFont::load("main_font.ttf").await?;
    let punch_image = Image::load(&gfx, "punch.png").await?;
    let kick_image = Image::load(&gfx, "kick.png").await?;

    //creates necessary variables, like the frame counter or title screen tracker.
    let mut frames = Timer::time_per_second(30.0);
    let mut screen_tracker: u8 = 0;
    let mut turn: f32 = 0.0;

    //creats hero and enemy in advance.
    let mut hero = Entity {
        max_health: 100.0,
        current_health: 100.0,
        moves_unlocked: 2,
    };
    let mut enemy = Entity {
        max_health: 100.0,
        current_health: 100.0,
        moves_unlocked: 2,
    };

    //starts running the game, this allows me to exit the game on command (whenever someone presses ESC).
    let mut running = true;
    while running {
        //loads title screen only on open.
        if screen_tracker == 0 {
            title_screen(&window, &mut gfx, &castle_image, &title_font)?;
            screen_tracker = 1;
        }

        //constantly loops checking for inputs.
        while let Some(event) = input.next_event().await {
            //stops game whenever escape is pressed.
            if input.key_down(Key::Escape) { running = false; }

            //if on the title screen, space will start the game.
            if input.key_down(Key::Space) && screen_tracker == 1 { 
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
                screen_tracker = 2;

                //loads the visuals for the first fight.
                load_fight(&mut gfx, &arena1_image, &hero_image, &enemy1_image);
                fight_one(&mut gfx, &main_font, &punch_image, &kick_image, );

                //loads the initial turn counter and health bars.
                health_bars_update(&mut gfx, &hero, &enemy);
                gfx.present(&window)?;
            }

            //does the proper calculations and display in response to using a "punch". Also checks for a win after the punch lands.
            if input.key_down(Key::Q) && hero.moves_unlocked >= 1 && screen_tracker < 90 { 
                //punches do 18 damage, also immediately increment the turn counter.
                let punch_damage: f32 = 9.0;
                turn += 0.5;

                //calls on my function to calculate an enemies move.
                let enemy_damage = enemy_move(&enemy);
                
                if enemy_damage == -1.0 {
                    //do nothing, attack was blocked.
                } else {
                    enemy.current_health = enemy.current_health - punch_damage;
                    hero.current_health = hero.current_health - enemy_damage;
                }

                //updates the health bar visuals.
                health_bars_update(&mut gfx, &hero, &enemy);
                gfx.present(&window)?;

                //checks for win or loss.
                if enemy.current_health <= 0.0 { //win condition.
                    screen_tracker = 99;

                    gfx.clear(Color::BLACK);
                    text_box(&mut gfx, 1, &main_font, turn);
                    gfx.present(&window)?;
                } else if hero.current_health <= 0.0 { //loss condition.
                    screen_tracker = 99;
                    
                    gfx.clear(Color::BLACK);
                    text_box(&mut gfx, 2, &main_font, turn);
                    gfx.present(&window)?;
                }
            }

            //does proper calculations for a "kick" input.
            if input.key_down(Key::W) && hero.moves_unlocked >= 2 && screen_tracker < 90 { 
                let kick_damage: f32 = 20.0;
                let random = rand::thread_rng().gen_range(0..9);
                turn += 0.5;

                //calls on function to calculate an enemies move.
                let enemy_damage = enemy_move(&enemy);
                
                if enemy_damage == -1.0 {
                    //do nothing, attack was blocked.
                } else {
                    if random <= 4 { //50% chance to hit.
                        enemy.current_health = enemy.current_health - kick_damage;
                    } else {
                        //do nothing
                    }
                    hero.current_health = hero.current_health - enemy_damage;
                }

                //updates the health bar visuals.
                health_bars_update(&mut gfx, &hero, &enemy);
                gfx.present(&window)?;

                //checks for win or loss.
                if enemy.current_health <= 0.0 { //win condition.
                    screen_tracker = 99;

                    gfx.clear(Color::BLACK);
                    text_box(&mut gfx, 1, &main_font, turn);
                    gfx.present(&window)?;
                } else if hero.current_health <= 0.0 { //loss condition.
                    screen_tracker = 99;
                    
                    gfx.clear(Color::BLACK);
                    text_box(&mut gfx, 2, &main_font, turn);
                    gfx.present(&window)?;
                }
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
fn load_fight(mut gfx: &mut Graphics, background_image: &Image, hero_image: &Image, enemy_image: &Image) -> Result<()> {
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
fn health_bars_update(mut gfx: &mut Graphics, hero: &Entity, enemy: &Entity) -> Result<()> {
    //create the widths for the red and green portion of the health bars.
    let hero_green_width: f32 = (hero.current_health).clone();
    let hero_red_width: f32 = (hero.max_health - hero.current_health).clone();
    let enemy_green_width: f32 = (enemy.current_health).clone();
    let enemy_red_width: f32 = (enemy.max_health - enemy.current_health).clone();
    
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

//decides enemy move.
fn enemy_move(enemy: &Entity) -> f32 {
    //creates a random number
    let random = rand::thread_rng().gen_range(0..9);
    
    //if statements use different logic based on which enemy you're facing.
    if enemy.moves_unlocked == 2 {
        if random >= 0 && random <= 4 { //50 percent change to punch.
            return 8.0;
        } else if random >= 5 && random <= 7 { //30 percent chance to block.
            return -1.0;
        } else { //20 percent chance to stumble.
            return 0.0;
        }
    } else { return 0.0; }
}

//function to put up the text box, which clears after the player interacts.
fn text_box(mut gfx: &mut Graphics, display: u8, font: &VectorFont, turn: f32) -> Result<()> {
    let black_box = Rectangle::new(Vector::new(150.0, 500.0), Vector::new(700.0, 200.0));
    gfx.fill_rect(&black_box, Color::BLACK);
    gfx.stroke_rect(&black_box, Color::WHITE);

    if display == 1 { //text box for when you win.
        let win_text = font.to_renderer(&gfx, 40.0);
        win_text?.draw(
            &mut gfx, 
            format!("You've won! In {} turns!", turn).as_str(), 
            Color::WHITE, 
            Vector::new(165.0, 550.0)
        )?;
    } else if display == 2 { //text box for when you lose.
        let lose_text = font.to_renderer(&gfx, 30.0);
        lose_text?.draw(
            &mut gfx, 
            format!("You've lost... At least you lasted {} turns!", turn).as_str(), 
            Color::WHITE, 
            Vector::new(165.0, 550.0)
        )?;
    }

    Ok(())
}

//function that loads images specific to the first fight. (fight label, available move icons)
fn fight_one(mut gfx: &mut Graphics, font: &VectorFont, punch_image: &Image, kick_image: &Image) -> Result<()> {
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
    
    Ok(())
}