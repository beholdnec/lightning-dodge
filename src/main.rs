extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate cgmath;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use graphics::{Image, clear, rectangle, Transformed};
use graphics::draw_state::DrawState;
use opengl_graphics as ogl;
use image::{ImageBuffer, Rgba, RgbaImage};
use cgmath::*;
use rand::Rng;

mod nesppu;
mod gfx;

use nesppu::Ppu;
use gfx::*;

type Vec2f = Vector2<f32>;
type Vec2i = Vector2<i32>;

const METATILE_WIDTH_IN_PIXELS: usize = 16;
const METATILE_HEIGHT_IN_PIXELS: usize = 16;
const X_METATILES: usize = nesppu::DISPLAY_WIDTH / METATILE_WIDTH_IN_PIXELS;
const Y_METATILES: usize = nesppu::DISPLAY_HEIGHT / METATILE_HEIGHT_IN_PIXELS;

// Metatiles are 16x16 pixels. Metatiles contain collision information.
#[derive(Copy, Clone)]
enum Metatile {
    Blank,
    Solid,
}
type MetatileArray = [[Metatile; X_METATILES]; Y_METATILES];

enum CloudDirection {
    Left,
    Right,
}

enum PrecipitationType {
    Rain,
    RainSplash,
    Lightning,
}

struct Precipitation {
    type_: PrecipitationType,
    dead: bool,
    pos: Vec2f,
    timer: u32,
}

struct Cloud {
    direction: CloudDirection,
    pos: Vec2f,
    speed: f32,
    lightning_n: u32, // Cloud has a 1/N chance of spawning a lightning bolt.
    precipitation_period: u32, // Cloud emits precipitation every X frames.
    timer: u32,
}

#[derive(Default)]
struct DeathState {
    timer: u32,
}

struct App {
    gl: ogl::GlGraphics,
    ppu: Ppu,
    player_pos: Vec2f,
    metatiles: MetatileArray,
    clouds: Vec<Cloud>,
    precipitation: Vec<Precipitation>,
    caught_rain: u32,
    lightning_color_cycle_timer: u32,
    lightning_color_set: usize,
    death_state: Option<DeathState>,
}

// TODO:
// - max sprite limit
// - tuning
// - fancier player graphics and collision detection
// - die when hit by lightning
const PRECIPITATION_PERIOD: u32 = 1 * 60;
const RAINSPLASH_TIME: u32 = 60 / 2;
const RAINFALL_SPEED: f32 = 1.0;
const CLOUD_SPEED: f32 = 1.0;
const CLOUD_LEFT_BOUND: f32 = 8.0;
const CLOUD_RIGHT_BOUND: f32 = 228.0;
const GROUND_Y: f32 = ((Y_METATILES - 2) * METATILE_HEIGHT_IN_PIXELS) as f32;
const PLAYER_Y: f32 = GROUND_Y - 8.0;
const PLAYER_SPEED: f32 = 2.0;
const NEW_CLOUD_SCORE: u32 = 5;
const LIGHTNING_COLOR_CYCLE_TIME: u32 = 5;
const DEATH_TIME: u32 = 4 * 60;

// Returns a bool with a 1/N chance of being true.
fn random_bool(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

// Returns a random u32 in the open range [lo, hi).
fn random_u32(lo: u32, hi: u32) -> u32 {
    rand::thread_rng().gen_range(lo, hi)
}

// Returns a random f32 in the open range [lo, hi).
fn random_f32(lo: f32, hi: f32) -> f32 {
    rand::thread_rng().gen_range(lo, hi)
}

impl App {
    fn new(opengl: glutin_window::OpenGL) -> Self {
        let mut this = App {
            gl: ogl::GlGraphics::new(opengl),
            ppu: Default::default(),
            player_pos: Vec2f::new(nesppu::DISPLAY_WIDTH as f32 / 2.0, PLAYER_Y),
            metatiles: [[Metatile::Blank; X_METATILES]; Y_METATILES],
            clouds: Vec::new(),
            precipitation: Vec::new(),
            caught_rain: 0,
            lightning_color_cycle_timer: 0,
            lightning_color_set: 0,
            death_state: None,
        };

        this.reset();

        this
    }

    fn reset(&mut self) {
        self.ppu = Default::default();
        self.player_pos = Vec2f::new(nesppu::DISPLAY_WIDTH as f32 / 2.0, PLAYER_Y);
        self.metatiles = [[Metatile::Blank; X_METATILES]; Y_METATILES];
        self.clouds = Vec::new();
        self.precipitation = Vec::new();
        self.caught_rain = 0;
        self.lightning_color_cycle_timer = 0;
        self.lightning_color_set = 0;
        self.death_state = None;

        // Clear PPU
        for y in 0..nesppu::TILEMAP_HEIGHT_IN_TILES {
            for x in 0..nesppu::TILEMAP_WIDTH_IN_TILES {
                self.ppu.set_tile(x, y, BLANK_PATTERN_NAME);
                self.ppu.set_attribute(x, y, 0);
            }
        }

        // Load patterns
        load_all_patterns(&mut self.ppu);

        // Set background color to black
        self.ppu.set_common_bg_color(BACKGROUND_COLOR);

        // Set background palettes
        self.ppu.set_bg_colors(NORMAL_GROUND_ATTRIB, NORMAL_GROUND_COLORS);
        self.ppu.set_bg_colors(GRAYS_ATTRIB, GRAYS_COLORS);

        // Set sprite palettes
        self.ppu.set_sprite_colors(PLAYER_ATTRIB, PLAYER_COLORS);
        self.ppu.set_sprite_colors(CLOUD_ATTRIB, CLOUD_COLORS);
        self.ppu.set_sprite_colors(LIGHTNING_ATTRIB, LIGHTNING_COLOR_SETS[self.lightning_color_set]);
        self.ppu.set_sprite_colors(RAIN_ATTRIB, RAIN_COLORS);

        // Draw ground on bottom two rows
        for y in Y_METATILES-2..Y_METATILES {
            for x in 0..X_METATILES {
                self.metatiles[y][x] = Metatile::Solid;
                // Draw ground tiles in the 2x2-tile area covered by each metatile
                for sy in 0..2 {
                    for sx in 0..2 {
                        self.ppu.set_tile(x*2+sx, y*2+sy, GROUND_PATTERN_NAME);
                        self.ppu.set_attribute(x*2+sx, y*2+sy, NORMAL_GROUND_ATTRIB);
                    }
                }
            }
        }
        // Draw top of ground
        for x in 0..nesppu::DISPLAY_WIDTH_IN_TILES {
            self.ppu.set_tile(x, (Y_METATILES-2) * 2, GROUND_TOP_PATTERN_NAME);
            self.ppu.set_attribute(x, (Y_METATILES-2) * 2, NORMAL_GROUND_ATTRIB);
        }

        // Spawn the first cloud
        self.spawn_cloud(Vec2f::new(50.0, 20.0));
    }

    fn spawn_cloud(&mut self, pos: Vec2f) {
        self.clouds.push(Cloud {
            direction: if random_bool(2) { CloudDirection::Left } else { CloudDirection::Right },
            pos: pos,
            speed: CLOUD_SPEED,
            lightning_n: 4, // TODO: configurable lightning frequency
            precipitation_period: PRECIPITATION_PERIOD, // TODO: configurable precipitation period
            timer: 0,
        });
    }

    fn spawn_lightning(&mut self, pos: Vec2f) {
        self.precipitation.push(Precipitation {
            type_: PrecipitationType::Lightning,
            dead: false,
            pos: pos,
            timer: 0,
        });
    }

    fn spawn_raindrop(&mut self, pos: Vec2f) {
        self.precipitation.push(Precipitation {
            type_: PrecipitationType::Rain,
            dead: false,
            pos: pos,
            timer: 0,
        });
    }

    fn advance_frame_playing(&mut self, direction: Vec2f) {
        // Move the player
        self.player_pos.x += direction.x * PLAYER_SPEED;

        // Drive lightning color cycles
        self.lightning_color_cycle_timer += 1;
        if self.lightning_color_cycle_timer >= LIGHTNING_COLOR_CYCLE_TIME {
            self.lightning_color_cycle_timer = 0;
            self.lightning_color_set += 1;
            if self.lightning_color_set >= NUM_LIGHTNING_COLOR_SETS {
                self.lightning_color_set = 0;
            }

            self.ppu.set_sprite_colors(LIGHTNING_ATTRIB,
                                       LIGHTNING_COLOR_SETS[self.lightning_color_set]);
        }

        self.ppu.clear_sprites();

        let mut sprite_index = 0;

        self.draw_player(&mut sprite_index);

        // Simulate clouds
        for cn in 0..self.clouds.len() {
            // FIXME: it's unfortunate that I have to type self.clouds[cn] every time I want to
            //        to modify the cloud's state, but I don't know any other way to make the
            //        borrow checker happy.
            //        If I make a mut reference to the current cloud, it considers "self" to be
            //        borrowed, which interferes with self.spawn_lightning and self.spawn_raindrop.
            self.clouds[cn].pos.x += match self.clouds[cn].direction {
                CloudDirection::Left => -self.clouds[cn].speed,
                CloudDirection::Right => self.clouds[cn].speed,
            };

            if self.clouds[cn].pos.x < CLOUD_LEFT_BOUND {
                self.clouds[cn].pos.x = CLOUD_LEFT_BOUND;
                self.clouds[cn].direction = CloudDirection::Right;
            } else if self.clouds[cn].pos.x > CLOUD_RIGHT_BOUND {
                self.clouds[cn].pos.x = CLOUD_RIGHT_BOUND;
                self.clouds[cn].direction = CloudDirection::Left;
            }

            self.ppu.set_sprite(sprite_index, self.clouds[cn].pos.x as i32, self.clouds[cn].pos.y as i32,
                                CLOUD_LEFT_PATTERN_NAME, CLOUD_ATTRIB);
            sprite_index += 1;
            self.ppu.set_sprite(sprite_index, self.clouds[cn].pos.x as i32 + 8, self.clouds[cn].pos.y as i32,
                                CLOUD_RIGHT_PATTERN_NAME, CLOUD_ATTRIB);
            sprite_index += 1;

            // Spawn new raindrops and/or lightning bolts
            self.clouds[cn].timer += 1;
            if self.clouds[cn].timer >= self.clouds[cn].precipitation_period {
                self.clouds[cn].timer = 0;
                if random_bool(self.clouds[cn].lightning_n) {
                    let pos = self.clouds[cn].pos.clone();
                    self.spawn_lightning(pos);
                    // FIXME: speaking of weird borrow checker restrictions, why can't I write this??
                    // self.spawn_lightning(self.clouds[cn].pos.clone());
                } else {
                    let pos = self.clouds[cn].pos.clone();
                    self.spawn_raindrop(pos);
                }
            }
        }

        // Simulate precipitation
        let mut num_new_clouds = 0;
        for p in self.precipitation.iter_mut() {
            if let PrecipitationType::RainSplash = p.type_ {
                p.timer += 1;
                if p.timer >= RAINSPLASH_TIME {
                    p.dead = true;
                    continue;
                }
            } else {
                p.pos.y += RAINFALL_SPEED;
                if p.pos.y > GROUND_Y - 8.0 {
                    p.pos.y = GROUND_Y - 8.0;
                    if let PrecipitationType::Rain = p.type_ {
                        p.type_ = PrecipitationType::RainSplash;
                        p.timer = 0;
                    } else {
                        p.dead = true;
                        continue;
                    }
                }
            }

            // Check if player caught rain/lightning

            // Compute player hitbox
            let (pl, pt, pr, pb) = (self.player_pos.x, self.player_pos.y,
                                    self.player_pos.x + 8.0, self.player_pos.y + 8.0);

            // Compute rain hitbox (or hitPOINT, really.)
            let (rx, ry) = (p.pos.x + 4.0, p.pos.y + 8.0);

            // Check for hit
            if rx >= pl && rx <= pr && ry >= pt && ry <= pb {
                if let PrecipitationType::Rain = p.type_ {
                    self.caught_rain += 1;
                    if self.caught_rain % NEW_CLOUD_SCORE == 0 {
                        num_new_clouds += 1;
                    }
                    p.dead = true;
                    continue;
                } else if let PrecipitationType::Lightning = p.type_ {
                    // NOTE: this isn't elegant, but setting DeathState will trigger the death
                    //       sequence on the next frame.
                    self.death_state = Some(DeathState::default());
                }
            }

            let (pattern_name, attrib) = match p.type_ {
                PrecipitationType::Rain => (RAIN_PATTERN_NAME, RAIN_ATTRIB),
                PrecipitationType::RainSplash => (RAINSPLASH_PATTERN_NAME, RAIN_ATTRIB),
                PrecipitationType::Lightning => (LIGHTNING_PATTERN_NAME, LIGHTNING_ATTRIB),
            };
            self.ppu.set_sprite(sprite_index, p.pos.x as i32, p.pos.y as i32, pattern_name, attrib);
            sprite_index += 1;
        }

        // Clean out dead precipitation
        self.precipitation.retain(|item| {
            !item.dead
        });

        // Spawn new clouds if score increased enough
        for i in 0..num_new_clouds {
            let x = random_f32(CLOUD_LEFT_BOUND, CLOUD_RIGHT_BOUND);
            let y = random_f32(10.0, 60.0);
            self.spawn_cloud(Vec2f::new(x, y));
        }

        self.draw_scorebar();
    }

    fn advance_frame_death(&mut self, direction: Vec2f) {
        self.ppu.clear_sprites();

        let mut sprite_index = 0;

        self.draw_player(&mut sprite_index);

        self.draw_scorebar();

        let mut please_reset = false;

        {
            let death_state = self.death_state.as_mut().unwrap();

            if death_state.timer % 30 < 15 && death_state.timer < 2 * 60 {
                self.ppu.set_common_bg_color(0x20);
            } else {
                self.ppu.set_common_bg_color(BACKGROUND_COLOR);
            }

            death_state.timer += 1;
            if death_state.timer >= DEATH_TIME {
                // Reset the game
                // Note that I can't call self.reset here, as death_state is borrowed.
                please_reset = true;
            }
        }

        if please_reset {
            self.reset();
        }
    }

    fn advance_frame(&mut self, direction: Vec2f) {
        let mut direction = direction;
        if direction.magnitude2() > 1.0 {
            direction = direction.normalize();
        }

        if self.death_state.is_some() {
            self.advance_frame_death(direction);
        } else {
            self.advance_frame_playing(direction);
        }
    }

    fn draw_player(&mut self, sprite_index: &mut usize) {
        // Draw player
        self.ppu.set_sprite(*sprite_index, self.player_pos.x as i32, self.player_pos.y as i32,
                            PLAYER_PATTERN_NAME, PLAYER_ATTRIB);
        *sprite_index += 1;
        self.ppu.set_sprite(*sprite_index, self.player_pos.x as i32, self.player_pos.y as i32 - 8,
                            PLAYER_TOP_PATTERN_NAME, PLAYER_ATTRIB);
        *sprite_index += 1;
    }

    fn draw_scorebar(&mut self) {
        for x in 0..nesppu::DISPLAY_WIDTH_IN_TILES {
            self.ppu.set_tile(x, nesppu::DISPLAY_HEIGHT_IN_TILES - 1, BLANK_PATTERN_NAME);
            self.ppu.set_attribute(x, nesppu::DISPLAY_HEIGHT_IN_TILES - 1, GRAYS_ATTRIB);
        }

        let mut score = self.caught_rain;
        let mut score_x = nesppu::DISPLAY_WIDTH_IN_TILES - 1;
        let mut draw_score = true;
        while draw_score {
            let digit = score % 10;
            self.ppu.set_tile(score_x, nesppu::DISPLAY_HEIGHT_IN_TILES - 1,
                              NUM0_PATTERN_NAME + digit as u8);
            score /= 10;
            score_x -= 1;
            draw_score = score != 0;
        }
    }

    // Note: update must be called once every 60th of a second.
    //       It will not check the time for accuracy.
    fn update(&mut self, args: &UpdateArgs, direction: Vec2f) {
        self.advance_frame(direction)
    }

    fn render(&mut self, args: &RenderArgs) {
        let image = Image::new().rect([0.0, 0.0, args.width as f64, args.height as f64]);

        let ppu = &self.ppu; // TODO: ask an expert why this works, while referencing self.ppu
                             //       in the closure below doesn't work.

        self.gl.draw(args.viewport(), |c, gl| {
            let mut ppu_image = RgbaImage::new(nesppu::DISPLAY_WIDTH as u32, nesppu::DISPLAY_HEIGHT as u32);
            ppu.draw_image(&mut ppu_image);
            let mut texture_settings = ogl::TextureSettings::new();
            texture_settings.set_filter(ogl::Filter::Nearest);
            // TODO: update a texture in-place instead of creating a new one every frame.
            let texture = ogl::Texture::from_image(&ppu_image, &texture_settings);
            image.draw(&texture, &Default::default(), c.transform, gl);
        });
    }
}

fn main() {
    let opengl = ogl::OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "Lightning Dodge",
            [1024, 768]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);

    let mut left_state = false;
    let mut right_state = false;
    let mut up_state = false;
    let mut down_state = false;

    let mut events = window.events().max_fps(60).ups(60);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            let mut direction = Vec2f::zero();
            if left_state {
                direction.x -= 1.0
            }
            if right_state {
                direction.x += 1.0
            }
            if up_state {
                direction.y -= 1.0
            }
            if down_state {
                direction.y += 1.0
            }

            app.update(&u, direction);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => left_state = true,
                Key::Right => right_state = true,
                Key::Up => up_state = true,
                Key::Down => down_state = true,
                _ => {},
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Left => left_state = false,
                Key::Right => right_state = false,
                Key::Up => up_state = false,
                Key::Down => down_state = false,
                _ => {},
            }
        }
    }
}
