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
        println!("Opening device 0...");
        let ohmd_device = ohmd_context.list_open_device(id);
        ohmdHeadSet{
            context: ohmd_context,
            device: ohmd_device
        }
    }
    pub fn gen_cfg(&self) -> HMDParams{
        use math::m16_to_4x4;

        let scrw = self.device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION) as u32;
        let scrh = self.device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION) as u32;

        // Calculating HMD params
        let scr_size_w = self.device.getf(openhmd_rs::ohmd_float_value::OHMD_SCREEN_HORIZONTAL_SIZE)[0];
        let scr_size_h = self.device.getf(openhmd_rs::ohmd_float_value::OHMD_SCREEN_VERTICAL_SIZE )[0];
        let distortion_k = self.device.getf(openhmd_rs::ohmd_float_value::OHMD_UNIVERSAL_DISTORTION_K );
        let aberration_k = self.device.getf(openhmd_rs::ohmd_float_value::OHMD_UNIVERSAL_ABERRATION_K );

        let view_port_scale = [scr_size_w / 2.0, scr_size_h];

        let sep = self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_HORIZONTAL_SEPARATION )[0];
        let mut left_lens_center: [f32; 2] = [0.0, self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_VERTICAL_POSITION)[0]];
        let mut right_lens_center: [f32; 2] = [0.0, self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LENS_VERTICAL_POSITION)[0]];

        let oproj = m16_to_4x4(self.device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_PROJECTION_MATRIX));
        let oproj2 = m16_to_4x4(self.device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX));

        left_lens_center[0] = view_port_scale[0] - sep/2.0;
        right_lens_center[0] = sep/2.0;

        HMDParams{

            scr_res: (scrw, scrh),
            scr_size: (scr_size_w, scr_size_h),

            left_lens_center: left_lens_center,
            right_lens_center: right_lens_center,

            view_port_scale: view_port_scale,

            distortion_k: [distortion_k[0], distortion_k[1], distortion_k[2], distortion_k[3]],
            aberration_k: [aberration_k[0], aberration_k[1], aberration_k[2]],

            projection1: oproj,
            projection2: oproj2,
        }
    }
}
