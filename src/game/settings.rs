pub struct Settings{
    pub full_screen: bool,
    pub screen_resx: u32,
    pub screen_resy: u32,
    pub output_display: u32,
    pub vr_mode: bool,
    pub server_ip: String
}
impl Settings{
    pub fn new() -> Settings{
        use clap::{Arg, App};
        println!("Hello, world!");
        let matches = App::new("OpenHMD-Chat")
            .version("0.1")
            .author("The HellBox <thehellbox11@gmail.com>")
            .about("Online chat for VR")
            .arg(Arg::with_name("ip")
                .short("d")
                .long("ip")
                .help("Sets ip to connect to")
                .takes_value(true))
            .arg(Arg::with_name("vr")
                .short("v")
                .long("vr")
                .help("VR mode"))
            .arg(Arg::with_name("fullscreen")
                .short("f")
                .long("full_screen")
                .help("Full screen mode"))
            .arg(Arg::with_name("output_display")
                .short("o")
                .long("out_disp")
                .help("Sets screen to render picture")
                .takes_value(true))
            .get_matches();

        let server_ip = matches.value_of("ip").unwrap_or("127.0.0.1:4460").to_string();
        let vr_mode = matches.values_of_lossy("vr").is_some();
        let full_screen = matches.values_of_lossy("fullscreen").is_some();

        Settings{
            full_screen,
            screen_resx: 1024,
            screen_resy: 768,
            output_display: 0,
            vr_mode,
            server_ip
        }
    }
}
