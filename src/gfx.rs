use nesppu;

type CharPattern = [[u8; nesppu::TILE_WIDTH_IN_PIXELS]; nesppu::TILE_HEIGHT_IN_PIXELS];

// Background attributes
pub const NORMAL_GROUND_ATTRIB: u8 = 1;
pub const NORMAL_GROUND_COLORS: [u8; 3] = [0x27, 0x17, 0x19];
pub const GRAYS_ATTRIB: u8 = 2;
pub const GRAYS_COLORS: [u8; 3] = [0x00, 0x10, 0x20];

pub const BACKGROUND_COLOR: u8 = 0x1D; // Black

pub const BLANK_PATTERN_NAME: u8 = 0;
pub const BLANK_PATTERN: [u8; nesppu::PATTERN_SIZE_IN_BYTES] = [0u8; nesppu::PATTERN_SIZE_IN_BYTES];

pub const GROUND_PATTERN_NAME: u8 = 1;
pub const GROUND_PATTERN_CHARS: CharPattern = [
    *b"....;...",
    *b";.....;.",
    *b"..;.....",
    *b".;...;..",
    *b".;......",
    *b"......;.",
    *b"..;...;.",
    *b";...;...",
];

pub const GROUND_TOP_PATTERN_NAME: u8 = 2;
pub const GROUND_TOP_PATTERN_CHARS: CharPattern = [
    *b"########",
    *b"########",
    *b"##.##.##",
    *b".;...;..",
    *b".;......",
    *b"......;.",
    *b"..;...;.",
    *b";...;...",
];

pub const NUM0_PATTERN_NAME: u8 = 30;
pub const NUM0_PATTERN_CHARS: CharPattern = [
    *b" #####  ",
    *b"##   ## ",
    *b"##   ## ",
    *b"##   ## ",
    *b"##   ## ",
    *b"##   ## ",
    *b" #####  ",
    *b"        ",
];
pub const NUM1_PATTERN_NAME: u8 = 31;
pub const NUM1_PATTERN_CHARS: CharPattern = [
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"        ",
];
pub const NUM2_PATTERN_NAME: u8 = 32;
pub const NUM2_PATTERN_CHARS: CharPattern = [
    *b" #####  ",
    *b"     ## ",
    *b"     ## ",
    *b" #####  ",
    *b"##      ",
    *b"##      ",
    *b" #####  ",
    *b"        ",
];
pub const NUM3_PATTERN_NAME: u8 = 33;
pub const NUM3_PATTERN_CHARS: CharPattern = [
    *b"######  ",
    *b"     ## ",
    *b"     ## ",
    *b"######  ",
    *b"     ## ",
    *b"     ## ",
    *b"######  ",
    *b"        ",
];
pub const NUM4_PATTERN_NAME: u8 = 34;
pub const NUM4_PATTERN_CHARS: CharPattern = [
    *b"##   ## ",
    *b"##   ## ",
    *b"##   ## ",
    *b" ###### ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"        ",
];
pub const NUM5_PATTERN_NAME: u8 = 35;
pub const NUM5_PATTERN_CHARS: CharPattern = [
    *b"######  ",
    *b"##      ",
    *b"##      ",
    *b" #####  ",
    *b"     ## ",
    *b"     ## ",
    *b" #####  ",
    *b"        ",
];
pub const NUM6_PATTERN_NAME: u8 = 36;
pub const NUM6_PATTERN_CHARS: CharPattern = [
    *b" ###### ",
    *b"##      ",
    *b"##      ",
    *b"######  ",
    *b"##   ## ",
    *b"##   ## ",
    *b" #####  ",
    *b"        ",
];
pub const NUM7_PATTERN_NAME: u8 = 37;
pub const NUM7_PATTERN_CHARS: CharPattern = [
    *b" ###### ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"     ## ",
    *b"        ",
];
pub const NUM8_PATTERN_NAME: u8 = 38;
pub const NUM8_PATTERN_CHARS: CharPattern = [
    *b" #####  ",
    *b"##   ## ",
    *b"##   ## ",
    *b" #####  ",
    *b"##   ## ",
    *b"##   ## ",
    *b" #####  ",
    *b"        ",
];
pub const NUM9_PATTERN_NAME: u8 = 39;
pub const NUM9_PATTERN_CHARS: CharPattern = [
    *b" #####  ",
    *b"##   ## ",
    *b"##   ## ",
    *b" ###### ",
    *b"     ## ",
    *b"     ## ",
    *b"######  ",
    *b"        ",
];

pub const RAINSPLASH_PATTERN_NAME: u8 = 249;
pub const RAINSPLASH_PATTERN_CHARS: CharPattern = [
    *b"        ",
    *b"        ",
    *b"    .   ",
    *b"    .   ",
    *b" .  .  .",
    *b"  . . . ",
    *b" . . . .",
    *b"   ...  ",
];

pub const RAIN_PATTERN_NAME: u8 = 250;
pub const RAIN_PATTERN_CHARS: CharPattern = [
    *b"    .   ",
    *b"    .   ",
    *b"    .   ",
    *b"    .   ",
    *b"    .   ",
    *b"    .   ",
    *b"    .   ",
    *b"    .   ",
];

pub const LIGHTNING_PATTERN_NAME: u8 = 251;
pub const LIGHTNING_PATTERN_CHARS: CharPattern = [
    *b"    .   ",
    *b"   ..   ",
    *b"  ...   ",
    *b" ....   ",
    *b"    ....",
    *b"    ... ",
    *b"    ..  ",
    *b"    .   ",
];

pub const CLOUD_LEFT_PATTERN_NAME: u8 = 252;
pub const CLOUD_LEFT_PATTERN_CHARS: CharPattern = [
    *b"      ..",
    *b"  .. ...",
    *b" .......",
    *b"........",
    *b"........",
    *b"........",
    *b" .......",
    *b"    ....",
];

pub const CLOUD_RIGHT_PATTERN_NAME: u8 = 253;
pub const CLOUD_RIGHT_PATTERN_CHARS: CharPattern = [
    *b"..      ",
    *b"... ..  ",
    *b"....... ",
    *b"........",
    *b"........",
    *b"....... ",
    *b".....   ",
    *b".       ",
];

pub const PLAYER_TOP_PATTERN_NAME: u8 = 254;
pub const PLAYER_TOP_PATTERN_CHARS: CharPattern = [
    *b"        ",
    *b"        ",
    *b"  ....  ",
    *b" .    . ",
    *b" .    . ",
    *b".      .",
    *b".      .",
    *b".      .",
];

pub const PLAYER_PATTERN_NAME: u8 = 255;
pub const PLAYER_PATTERN_CHARS: CharPattern = [
    *b"........",
    *b"........",
    *b"..;..;..",
    *b"........",
    *b"..;..;..",
    *b"...;;...",
    *b"........",
    *b"##   ## ",
];

pub const PLAYER_ATTRIB: u8 = 0;
pub const PLAYER_COLORS: [u8; 3] = [0x2D, 0x1D, 0x07];
pub const CLOUD_ATTRIB: u8 = 1;
pub const CLOUD_COLORS: [u8; 3] = [0x30, 0x10, 0x00];
pub const LIGHTNING_ATTRIB: u8 = 2;
pub const NUM_LIGHTNING_COLOR_SETS: usize = 3;
pub const LIGHTNING_COLOR_SETS: [[u8; 3]; NUM_LIGHTNING_COLOR_SETS] = [
    [0x18, 0x28, 0x18],
    [0x28, 0x28, 0x18],
    [0x38, 0x28, 0x18],
];
pub const RAIN_ATTRIB: u8 = 3;
pub const RAIN_COLORS: [u8; 3] = [0x11, 0x21, 0x01];

pub fn decode_pattern_chars(cp: CharPattern) -> [u8; nesppu::PATTERN_SIZE_IN_BYTES] {
    let mut pattern = [0u8; nesppu::PATTERN_SIZE_IN_BYTES];
    for y in 0..nesppu::TILE_HEIGHT_IN_PIXELS {
        for x in 0..nesppu::TILE_WIDTH_IN_PIXELS {
            let c = cp[y][x];
            let pixel = match c {
                b'#' => 3,
                b';' => 2,
                b'.' => 1,
                _ => 0,
            };
            nesppu::set_pixel_in_pattern(&mut pattern, x, y, pixel);
        }
    }
    pattern
}

pub fn load_all_patterns(ppu: &mut nesppu::Ppu) {
    ppu.set_pattern(BLANK_PATTERN_NAME as usize, BLANK_PATTERN);
    ppu.set_pattern(GROUND_PATTERN_NAME as usize, decode_pattern_chars(GROUND_PATTERN_CHARS));
    ppu.set_pattern(GROUND_TOP_PATTERN_NAME as usize, decode_pattern_chars(GROUND_TOP_PATTERN_CHARS));
    ppu.set_pattern(NUM0_PATTERN_NAME as usize, decode_pattern_chars(NUM0_PATTERN_CHARS));
    ppu.set_pattern(NUM1_PATTERN_NAME as usize, decode_pattern_chars(NUM1_PATTERN_CHARS));
    ppu.set_pattern(NUM2_PATTERN_NAME as usize, decode_pattern_chars(NUM2_PATTERN_CHARS));
    ppu.set_pattern(NUM3_PATTERN_NAME as usize, decode_pattern_chars(NUM3_PATTERN_CHARS));
    ppu.set_pattern(NUM4_PATTERN_NAME as usize, decode_pattern_chars(NUM4_PATTERN_CHARS));
    ppu.set_pattern(NUM5_PATTERN_NAME as usize, decode_pattern_chars(NUM5_PATTERN_CHARS));
    ppu.set_pattern(NUM6_PATTERN_NAME as usize, decode_pattern_chars(NUM6_PATTERN_CHARS));
    ppu.set_pattern(NUM7_PATTERN_NAME as usize, decode_pattern_chars(NUM7_PATTERN_CHARS));
    ppu.set_pattern(NUM8_PATTERN_NAME as usize, decode_pattern_chars(NUM8_PATTERN_CHARS));
    ppu.set_pattern(NUM9_PATTERN_NAME as usize, decode_pattern_chars(NUM9_PATTERN_CHARS));
    ppu.set_pattern(RAINSPLASH_PATTERN_NAME as usize, decode_pattern_chars(RAINSPLASH_PATTERN_CHARS));
    ppu.set_pattern(RAIN_PATTERN_NAME as usize, decode_pattern_chars(RAIN_PATTERN_CHARS));
    ppu.set_pattern(LIGHTNING_PATTERN_NAME as usize, decode_pattern_chars(LIGHTNING_PATTERN_CHARS));
    ppu.set_pattern(CLOUD_LEFT_PATTERN_NAME as usize, decode_pattern_chars(CLOUD_LEFT_PATTERN_CHARS));
    ppu.set_pattern(CLOUD_RIGHT_PATTERN_NAME as usize, decode_pattern_chars(CLOUD_RIGHT_PATTERN_CHARS));
    ppu.set_pattern(PLAYER_TOP_PATTERN_NAME as usize, decode_pattern_chars(PLAYER_TOP_PATTERN_CHARS));
    ppu.set_pattern(PLAYER_PATTERN_NAME as usize, decode_pattern_chars(PLAYER_PATTERN_CHARS));
}
