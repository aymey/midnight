use nannou::ease;

type EaseFn<S = f32> = fn(t: S, b: S, c: S, d: S) -> S;

#[derive(Default, Clone, Copy)]
pub enum EaseKind {
    #[default]
    None,
    LinearInOut,
    QuadInOut,
    QuadIn,
    QuadOut,
    CubicInOut,
    CubicIn,
    CubicOut,
    QuartInOut,
    QuartIn,
    QuartOut,
    QuintInOut,
    QuintIn,
    QuintOut,
    SineInOut,
    SineIn,
    SineOut,
    ExpoInOut,
    ExpoIn,
    ExpoOut,
    CircInOut,
    CircIn,
    CircOut,
    ElasticInOut,
    ElasticIn,
    ElasticOut,
    BackInOut,
    BackIn,
    BackOut,
    BounceInOut,
    BounceIn,
    BounceOut,
}

impl EaseKind {
    pub fn perform(&self, t: crate::Frame) {

    }
}

// TODO: do, maybe use a serde macro to automatically do easing functions for each variant
