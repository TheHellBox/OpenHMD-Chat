use openhmd_rs;
use math;

pub struct HMDParams{
    pub scr_res: (u32, u32),
    pub scr_size: (f32, f32),
    pub left_lens_center: [f32; 2],
    pub right_lens_center: [f32; 2],
    pub view_port_scale: [f32; 2],
    pub distortion_k: [f32; 4],
    pub aberration_k: [f32; 3],
    pub projection1: [[f32;4]; 4],
    pub projection2: [[f32;4]; 4],
}

pub struct ohmdHeadSet{
    pub context: openhmd_rs::Context,
    pub device: openhmd_rs::Device,
}

impl ohmdHeadSet{
    pub fn new(id: i32) -> ohmdHeadSet{
        println!("VR mode");
        let ohmd_context = openhmd_rs::Context::new();
        ohmd_context.probe();

        println!("\nDevice description: ");
        println!("Vendor:   {}", ohmd_context.list_gets(id, openhmd_rs::ohmd_string_value::OHMD_VENDOR));
        println!("Product:  {}", ohmd_context.list_gets(id, openhmd_rs::ohmd_string_value::OHMD_PRODUCT));
        println!("Path:     {}\n", ohmd_context.list_gets(id, openhmd_rs::ohmd_string_value::OHMD_PATH));
        println!("Opening device {}...", id);

        let ohmd_device = ohmd_context.list_open_device(id);
        ohmdHeadSet{
            context: ohmd_context,
            device: ohmd_device
        }
    }
    pub fn gen_cfg(&self) -> HMDParams{
        use math::m16_to_4x4;

        let scrw = match self.device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION){
            Some(x) => x,
            _ => 1280
        } as u32;
        let scrh = match self.device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION){
            Some(x) => x,
            _ => 800
        } as u32;

        // Calculating HMD params
        let scr_size_w = match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_SCREEN_HORIZONTAL_SIZE){
            Some(x) => x[0],
            _ => 0.149760
        };
        let scr_size_h = match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_SCREEN_VERTICAL_SIZE ){
            Some(x) => x[0],
            _ => 0.093600
        };
        let distortion_k = match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_UNIVERSAL_DISTORTION_K ){
            Some(x) => [x[0], x[1], x[2], x[3]],
            _ => [0.0,0.0,0.0,1.0]
        };
        let aberration_k = match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_UNIVERSAL_ABERRATION_K ){
            Some(x) =>  [x[0], x[1], x[2]],
            _ => [0.0,0.0,1.0]
        };

        let view_port_scale = [scr_size_w / 2.0, scr_size_h];

        let sep = match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_HORIZONTAL_SEPARATION ){
            Some(x) => x[0],
            _ => 0.063500
        };
        let mut left_lens_center: [f32; 2] = [0.0, match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_VERTICAL_POSITION){
            Some(x) => x[0],
            _ => 0.046800
        }];
        let mut right_lens_center: [f32; 2] = [0.0, match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_VERTICAL_POSITION){
            Some(x) => x[0],
            _ => 0.046800
        }];

        let oproj = m16_to_4x4( match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_PROJECTION_MATRIX){
            Some(x) => x,
            _ => [0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0]
        });
        let oproj2 = m16_to_4x4(match self.device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX){
            Some(x) => x,
            _ => [0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0]
        });

        left_lens_center[0] = view_port_scale[0] - sep/2.0;
        right_lens_center[0] = sep/2.0;

        HMDParams{

            scr_res: (scrw, scrh),
            scr_size: (scr_size_w, scr_size_h),

            left_lens_center: left_lens_center,
            right_lens_center: right_lens_center,

            view_port_scale: view_port_scale,

            distortion_k: distortion_k,
            aberration_k: aberration_k,

            projection1: oproj,
            projection2: oproj2,
        }
    }
}
