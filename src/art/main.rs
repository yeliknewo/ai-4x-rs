use art::{Name, RenderType, Size, Tint};

pub const NAME: Name = "main.png";
pub const SIZE: Size = &[410.0, 304.0];
pub const DEFAULT_TINT: Tint = &[0.5, 0.5, 0.5, 1.0];
pub const ID: RenderType = 0;

pub mod yellow {
    use art::Sprite;

    const Y: f32 = 4.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod dark_yellow {
    use art::Sprite;

    const Y: f32 = 37.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod mint_choc {
    use art::Sprite;

    const Y: f32 = 70.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod dark_green {
    use art::Sprite;

    const Y: f32 = 103.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod green {
    use art::Sprite;

    const Y: f32 = 136.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod blue {
    use art::Sprite;

    const Y: f32 = 169.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod blue_green {
    use art::Sprite;

    const Y: f32 = 202.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod dark_purple {
    use art::Sprite;

    const Y: f32 = 235.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod purple {
    use art::Sprite;

    const Y: f32 = 268.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[4.0, Y, 32.0, 31.5];
    pub const X: Sprite = &[37.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[70.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[103.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[136.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[169.0, Y, 32.0, 31.5];
}

pub mod purple_red {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 4.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}

pub mod pink {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 37.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}

pub mod red {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 70.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}

pub mod dark_red {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 103.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}

pub mod dark_orange {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 136.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}

pub mod orange {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 169.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}

pub mod light_grey {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 202.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}

pub mod grey {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 235.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}

pub mod dark_grey {
    use art::Sprite;

    const X_OFFSET: f32 = 209.0;
    const Y: f32 = 268.0;

    pub const ARRAY: &[Sprite; 6] = &[BLANK, X, CIRCLE, SQUARE, TRIANGLE, STAR];

    pub const BLANK: Sprite = &[X_OFFSET, Y, 32.0, 31.5];
    pub const X: Sprite = &[X_OFFSET + 33.0, Y, 32.0, 31.5];
    pub const CIRCLE: Sprite = &[X_OFFSET + 66.0, Y, 32.0, 31.5];
    pub const SQUARE: Sprite = &[X_OFFSET + 99.0, Y, 32.0, 31.5];
    pub const TRIANGLE: Sprite = &[X_OFFSET + 132.0, Y, 32.0, 31.5];
    pub const STAR: Sprite = &[X_OFFSET + 165.0, Y, 32.0, 31.5];
}
