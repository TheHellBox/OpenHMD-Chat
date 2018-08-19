#[derive(Serialize, Deserialize)]
pub enum AudioEvent{
    Play(Vec<i16>, i32, &'static str),
    AddSource(String),
}
