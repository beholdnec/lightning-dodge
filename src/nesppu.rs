// NES/Famicom PPU-like rendering module.
// Tries to emulate the quirks and limitations of the original NES/Famicom
// pixel processing unit. One exception is that more sprites are supported.

extern crate image;

use image::{Rgba, RgbaImage, Pixel};

// FIXME: NTSC hardware had only 224 lines. The top and bottom 8 pixels were cut off. (??? TODO: verify)
pub const DISPLAY_WIDTH: usize = 256;
pub const DISPLAY_HEIGHT: usize = 240;

//pub const NUM_SPRITES: usize = 64;
//pub const MAX_SPRITES_ON_LINE: usize = 8;
// XXX: sprite capabilities are dramatically increased from the original hardware!
pub const NUM_SPRITES: usize = 1024;
pub const MAX_SPRITES_ON_LINE: usize = 256;

// A tile is also known as a "pattern" in NES-dev parlance.
// A tilemap is also known as a "nametable".
// Conceptually, the NES had a 64x60 tilemap, enough for 4 full screens. However,
// there was only enough VRAM for 2 screens. Depending on the cartridge hardware, the
// NES's tilemap had a mirrored arrangement where the screens were duplicated horizontally
// or vertically. Alternately, the cartridge could include its own VRAM to extend the tilemap
// to the full size.
pub const TILE_WIDTH_IN_PIXELS: usize = 8;
pub const TILE_HEIGHT_IN_PIXELS: usize = 8;
pub const TILEMAP_WIDTH_IN_TILES: usize = 64;
pub const TILEMAP_HEIGHT_IN_TILES: usize = 60;
pub const PATTERN_SIZE_IN_BYTES: usize = 16;

pub const DISPLAY_WIDTH_IN_TILES: usize = DISPLAY_WIDTH / TILE_WIDTH_IN_PIXELS;
pub const DISPLAY_HEIGHT_IN_TILES: usize = DISPLAY_HEIGHT / TILE_HEIGHT_IN_PIXELS;

const TILEMAP_SIZE_IN_BYTES: usize = TILEMAP_WIDTH_IN_TILES * TILEMAP_HEIGHT_IN_TILES;
const ATTRMAP_WIDTH_IN_METATILES: usize = TILEMAP_WIDTH_IN_TILES / 2;
const ATTRMAP_HEIGHT_IN_METATILES: usize = TILEMAP_HEIGHT_IN_TILES / 2;
const ATTRMAP_SIZE_IN_BYTES: usize = ATTRMAP_WIDTH_IN_METATILES * ATTRMAP_HEIGHT_IN_METATILES;
const PATTERN_TABLE_SIZE_IN_BYTES: usize = 0x2000;
const PALETTE_SIZE_IN_BYTES: usize = 0x20;

#[derive(Copy, Clone)]
struct Sprite {
    x: u8,
    y: u8,
    tile: u8,
    attrib: u8,
    // TODO: priority, size
    flip_horiz: bool, // TODO: implement
    flip_vert: bool, // TODO: implement
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            x: 0,
            y: 255, // Sprites at line 255 are effectively disabled.
            tile: 0,
            attrib: 0,
            flip_horiz: false,
            flip_vert: false,
        }
    }
}

pub struct Ppu {
    tilemap: [u8; TILEMAP_SIZE_IN_BYTES],
    attrmap: [u8; ATTRMAP_SIZE_IN_BYTES],
    pattern_table: [u8; PATTERN_TABLE_SIZE_IN_BYTES],
    palette: [u8; PALETTE_SIZE_IN_BYTES], // Identical to Memory Map at <https://wiki.nesdev.com/w/index.php/PPU_palettes>
    sprites: [Sprite; NUM_SPRITES],
    scroll_x: u32,
    scroll_y: u32,
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu {
            tilemap: [0; TILEMAP_SIZE_IN_BYTES],
            attrmap: [0; ATTRMAP_SIZE_IN_BYTES],
            pattern_table: [0; PATTERN_TABLE_SIZE_IN_BYTES],
            palette: [0; PALETTE_SIZE_IN_BYTES],
            sprites: [Sprite::default(); NUM_SPRITES],
            scroll_x: 0,
            scroll_y: 0,
        }
    }
}

const NTSC_PALETTE: &'static [u8; 64 * 3] = include_bytes!("ntscpalette.pal");

fn get_color_rgba(color: u8) -> Rgba<u8> {
    let rgb = NTSC_PALETTE.chunks_exact(3).nth((color & 0x3f) as usize).unwrap();
    Rgba::<u8>::from([rgb[0], rgb[1], rgb[2], 255])
}


pub fn get_pixel_from_pattern(pattern: &[u8; PATTERN_SIZE_IN_BYTES], x: usize, y: usize) -> u8 {
    (pattern[y * 2 + (x / 4)] >> (2 * (3 - (x % 4)))) & 0x3
}

pub fn set_pixel_in_pattern(pattern: &mut [u8; PATTERN_SIZE_IN_BYTES], x: usize, y: usize, pixel: u8) {
    let b = &mut pattern[y * 2 + (x / 4)];
    *b &= !(0x3 << (2 * (3 - (x % 4))));
    *b |= pixel << (2 * (3 - (x % 4)));
}

impl Ppu {
    pub fn draw_image(&self, image: &mut RgbaImage) {
        for (dy, line_chunk) in (0..DISPLAY_HEIGHT).zip(image.chunks_mut(4 * DISPLAY_WIDTH)) {
            let world_y = dy + self.scroll_y as usize;
            let tile_y = world_y / 8;
            let subtile_y = world_y % 8;

            let mut sprites_on_line: [Option<&Sprite>; MAX_SPRITES_ON_LINE] =
                [None; MAX_SPRITES_ON_LINE];
            let mut num_sprites_on_line = 0;
            for i in 0..NUM_SPRITES {
                let sprite = &self.sprites[i];
                // TODO: support 8x16 sprites
                if dy >= sprite.y as usize && dy < sprite.y as usize + TILE_HEIGHT_IN_PIXELS {
                    sprites_on_line[num_sprites_on_line] = Some(&sprite);
                    num_sprites_on_line += 1;
                    if num_sprites_on_line >= MAX_SPRITES_ON_LINE {
                        break;
                    }
                }
            }

            // Make sprite variables immutable
            let sprites_on_line = sprites_on_line;
            let num_sprites_on_line = num_sprites_on_line;

            for (dx, rgba_chunk) in (0..DISPLAY_WIDTH).zip(line_chunk.chunks_exact_mut(4)) {
                let world_x = dx + self.scroll_x as usize;
                let tile_x = world_x / 8;
                let subtile_x = world_x % 8;

                let mut palette_index = 0;
                let mut sprite_drawn = false;

                for i in 0..num_sprites_on_line {
                    let sprite = sprites_on_line[i].unwrap();
                    if dx >= sprite.x as usize && dx < sprite.x as usize + TILE_WIDTH_IN_PIXELS {
                        let sprite_col = dx - sprite.x as usize;
                        let sprite_row = dy - sprite.y as usize;
                        let sprite_pattern = self.get_pattern(sprite.tile);
                        let pixel = get_pixel_from_pattern(&sprite_pattern, sprite_col, sprite_row);
                        if pixel != 0 { // Color 0 is transparent
                            // Draw sprite
                            palette_index = self.get_sprite_color(pixel, sprite.attrib);
                            sprite_drawn = true;
                            break;
                        }
                    }
                }

                if !sprite_drawn {
                    // Draw background
                    let tile = self.get_tile(tile_x, tile_y);
                    let pattern = self.get_pattern(tile);
                    let pixel = get_pixel_from_pattern(&pattern, subtile_x, subtile_y);
                    let attrib = self.get_attribute(tile_x, tile_y);
                    palette_index = self.get_bg_color(pixel, attrib);
                }

                let rgba = get_color_rgba(palette_index);
                rgba_chunk.copy_from_slice(&rgba.channels());
            }
        }
    }

    fn get_tile(&self, tile_x: usize, tile_y: usize) -> u8 {
        self.tilemap[tile_y * TILEMAP_WIDTH_IN_TILES + tile_x]
    }

    pub fn set_tile(&mut self, tile_x: usize, tile_y: usize, name: u8) {
        self.tilemap[tile_y * TILEMAP_WIDTH_IN_TILES + tile_x] = name;
    }

    pub fn clear_sprites(&mut self) {
        self.sprites = [Sprite::default(); NUM_SPRITES];
    }

    pub fn set_sprite(&mut self, num: usize, x: i32, y: i32, tile: u8, attrib: u8) {
        let mut new_sprite = Sprite::default();
        if x >= 0 && x < 256 && y >= 0 && y < 256 {
            new_sprite.x = x as u8;
            new_sprite.y = y as u8;
            new_sprite.tile = tile;
            new_sprite.attrib = attrib;
        }
        self.sprites[num] = new_sprite;
    }

    pub fn set_pattern(&mut self, tile: usize, pattern: [u8; PATTERN_SIZE_IN_BYTES]) {
        let addr = tile as usize * PATTERN_SIZE_IN_BYTES;
        self.pattern_table[addr .. addr + PATTERN_SIZE_IN_BYTES].copy_from_slice(&pattern);
    }

    fn get_attribute(&self, tile_x: usize, tile_y: usize) -> u8 {
        let attr_x = tile_x / 2;
        let attr_y = tile_y / 2;
        let sub_x = tile_x % 2;
        let sub_y = tile_y % 2;
        let index = sub_y * 2 + sub_x;
        let b = self.attrmap[attr_y * (TILEMAP_WIDTH_IN_TILES / 2) + attr_x];
        (b >> (2 * (3 - index))) & 0x3
    }

    pub fn set_attribute(&mut self, tile_x: usize, tile_y: usize, attrib: u8) {
        let attr_x = tile_x / 2;
        let attr_y = tile_y / 2;
        let sub_x = tile_x % 2;
        let sub_y = tile_y % 2;
        let index = sub_y * 2 + sub_x;
        let b = &mut self.attrmap[attr_y * ATTRMAP_WIDTH_IN_METATILES + attr_x];
        *b &= !(0x3 << (2 * (3 - index)));
        *b |= attrib << (2 * (3 - index));
    }

    fn get_pattern(&self, name: u8) -> [u8; PATTERN_SIZE_IN_BYTES] {
        let mut result = [0u8; PATTERN_SIZE_IN_BYTES];
        let addr = name as usize * PATTERN_SIZE_IN_BYTES;
        result.copy_from_slice(&self.pattern_table[addr .. addr + PATTERN_SIZE_IN_BYTES]);
        result
    }

    // Get color of tilemap background
    fn get_bg_color(&self, pixel: u8, attrib: u8) -> u8 {
        if pixel == 0 {
            self.palette[0]
        } else {
            self.palette[attrib as usize * 4 + (pixel & 0x3) as usize]
        }
    }

    fn get_sprite_color(&self, pixel: u8, attrib: u8) -> u8 {
        // Note that sprite color 0 is transparent. This function does not handle transparency.
        self.palette[attrib as usize * 4 + 0x10 + (pixel & 0x3) as usize]
    }

    // Set universal background color
    pub fn set_common_bg_color(&mut self, color: u8) {
        self.palette[0] = color;
    }

    // Set the three non-transparent colors of background palette 0..3
    pub fn set_bg_colors(&mut self, attrib: u8, colors: [u8; 3]) {
        self.palette[attrib as usize * 4 + 1] = colors[0];
        self.palette[attrib as usize * 4 + 2] = colors[1];
        self.palette[attrib as usize * 4 + 3] = colors[2];
    }

    // Set the three non-transparent colors sprite palette 0..3
    pub fn set_sprite_colors(&mut self, attrib: u8, colors: [u8; 3]) {
        self.palette[attrib as usize * 4 + 0x10 + 1] = colors[0];
        self.palette[attrib as usize * 4 + 0x10 + 2] = colors[1];
        self.palette[attrib as usize * 4 + 0x10 + 3] = colors[2];
    }
}
