use {
    crate::*,
    std::convert::TryInto,
};

// DE = distance estimation?
// MC = monte carlo?

pub enum Bokeh {
    Todo(u8),
}

pub enum Julia {
    Disabled,
    Julia(Vec4<f64>),
}

pub enum SofterShadows {
    Disabled,
    Enabled(f32),
}

pub struct HardShadows {
    automatic: bool,
    max_length_calc: f32,
    softer: SofterShadows,
    func_to_cos: bool,
    lights: [bool; 6],
    calculated: [bool; 6],
}

pub enum AmbientOcclusion {
    SSAO15 {
        zr_threshold: f32,
        threshold_to_0: bool,
    },
    SSAO24 {
        zr_threshold: f32,
        threshold_to_0: bool,
        border_size: f32,
    },
    SSAO24Random {
        zr_threshold: f32,
        threshold_to_0: bool,
        border_size: f32,
        calc_count: usize,
    },
    DEAO {
        dithering_scale: u8,
        first_step_random: bool,
        max_l: f32,
        rays: usize,
    },
}

pub struct AmbientShadows {
    automatic: bool,
    ambient_occlusion: AmbientOcclusion,
}

pub struct ReflectionsTransparency {
    automatic: bool,
    amount: f32,
    depth: usize,
    transparency: bool,
    only_dif: bool,
    absorption: f32,
    refractive_index: f32,
    scattering: f32,
    selection_only: bool,
}

pub struct DepthOfField {
    automatic: bool,
    z1_sharp: f32,
    z2_sharp: f32,
    aperture: f32,
    clipping_r: f32,
    forward_calculation: bool,
    passes: u8,
}

pub struct Calculation {
    de_stop: f32,
    vary_de_stop_on_fov: u8,
    ray_step_multiplier: f64,
    step_width_limiter: f32,
    step_count: usize,
    smooth_normals: usize,
    normals_on_de: u8,
    first_step_random: bool,
    ray_step_sub_de_stop: u8,
}

pub enum ChoiceMode {
    OrbitTrap,
    LastLengthIncrease,
    RoutAngleXY,
    RoutAngleXZ,
    RoutAngleYZ,
    MapOnOutputVector,
}

pub enum ColorOnIt {
    Disabled,
    OnIt(usize),
}

pub enum FogOrVol {
    DynFogOnIt(u32),
    VolumeLight(u8),
}

pub struct Coloring {
    choice_mode: ChoiceMode,
    multiplier: f32,
    color_on_it: ColorOnIt,
    fog_or_vol: FogOrVol,
}

pub enum Lense {
    Common,
    Rectilinear,
    Panorama360,
}

pub struct Camera {
    fovy: f64,
    lense: Lense,
}

pub struct Infos {
    author: String,
    moderator: String,
    avg_ray_steps: f32,
    avg_iterations: f32,
    max_iterations: u32,
    main_calc_time: f32,
    hs_calc_time: f32,
    ao_calc_time: f32,
    reflects_time: f32,
}

pub struct Stereo {
    screen_distance: f32,
    screen_width: f32,
    minimal_distance: f32,
}

pub struct Light {
    option: u8,
    function: u8,
    amp: u16,
    color: [u8; 3],
    light_map_nr: u16,
    pos: Vec3<f64>,
    additional_byte_ex: u8,
    free_byte: u8,
}

pub struct LCol {
    pos: u16,
    diff: [u8; 4],
    spec: [u8; 4],
}

pub struct ICol {
    pos: u16,
    col: [u8; 4],
}

pub struct Lighting {
    var_col_z_pos: i16,
    roughness_factor: u8,
    color_map: u8,
    dyn_fog_col2: [u8; 3],
    additional_options: u8,
    tb_pos: [i32; 9],
    tb_options: u32,
    fine_col_adj1: u8,
    fine_col_adj2: u8,
    pic_offset: Vec3<u8>,
    dyn_fog_col: [u8; 3],
    amb_col: [u8; 3],
    amb_col2: [u8; 3],
    depth_col: [u8; 3],
    depth_col2: [u8; 3],
    lights: [Light; 6],
    lcols: [LCol; 10],
    icols: [ICol; 4],
}

pub struct MB3D {
    id: u8,  // "MandId"
    size: Vec2<usize>,  // output dimensions
    iterations: usize,  // number of iterations
    options: u16,  // SmoothNs: (SpinEdit2.Value shl 6) or FirstStepRandom=bit1 or StepSubDEstop=bit3
    use_quaternions: bool,
    z_range: [f64; 2],
    mid: Vec3<f64>,
    rot: Vec3<f64>,
    zoom: f64,
    r_stop: f64,
    f_mix_pow: f32,  // for formula DE mix combs
    use_hdr: bool,
    secant_search: bool,
    auto_clip: bool,
    aa_box: bool,
    bokeh: Bokeh,
    new_mc_record_yuv: bool,
    mc_diff_reflects: u8,

    hard_shadows: HardShadows,
    ambient_shadows: AmbientShadows,
    reflections_transparency: ReflectionsTransparency,
    depth_of_field: DepthOfField,

    calculation: Calculation,
    julia: Julia,
    cutting: Vec3<f64>,
    coloring: Coloring,
    camera: Camera,
    infos: Infos,
    stereo: Stereo,
    lighting: Lighting,

    minimum_iterations: usize,
    mc_last_y: u16,
    planar_optic: u8,  // planar optic 0/1, 2: sphere pano, 3: dome?
    navi_min_dist: f32,
    cut_option: u8,
    z_step_div: f32,
    mc_depth: u8,
    image_scale: u8,
    v_grads: Mat3x3<f64>,
    mc_saturation: u8,
    y_calc_ns_on_zbuf_auto: u8,
    calc3d: u8,
    slice_calc: u8,
    de_comb_s: f32,
    h_custom_f: [u32; 6],
    cf_addon: u32,
    max_its_f2: u32,
    de_mix_color_option: u8,
    mc_contrast: u8,
    m3d_version: f32,
    tiling_options: u32,
}

fn decode_byte(byte: u8) -> Result<u32,String> {
    if (byte >= 46) && (byte <= 57) { Ok((byte - 46) as u32) }
    else if (byte >= 65) && (byte <= 90) { Ok((byte - 53) as u32) }
    else if (byte >= 97) && (byte <= 122) { Ok((byte - 59) as u32) }
    else { Err(format!("byte value {:02X} out of range",byte)) }
}

fn decode_light(src: &[u8]) -> Result<Light,String> {
    Ok(Light {
        option: src[0],
        function: src[1],
        amp: u16::from_le_bytes(src[2..4].try_into().unwrap()),
        color: [src[4],src[5],src[6]],
        light_map_nr: u16::from_le_bytes(src[7..9].try_into().unwrap()),
        pos: Vec3 {
            x: f64::from_le_bytes([0,src[9],src[10],src[11],src[12],src[13],src[14],src[15]]),
            y: f64::from_le_bytes([0,src[17],src[18],src[19],src[20],src[21],src[22],src[23]]),
            z: f64::from_le_bytes([0,src[25],src[26],src[27],src[28],src[29],src[30],src[31]]),
        },
        additional_byte_ex: src[16],
        free_byte: src[24],
    })
}

fn decode_lcol(src: &[u8]) -> Result<LCol,String> {
    Ok(LCol {
        pos: u16::from_le_bytes(src[0..2].try_into().unwrap()),
        diff: [src[2],src[3],src[4],src[5]],
        spec: [src[6],src[7],src[8],src[9]],
    })
}

fn decode_icol(src: &[u8]) -> Result<ICol,String> {
    Ok(ICol {
        pos: u16::from_le_bytes(src[0..2].try_into().unwrap()),
        col: [src[2],src[3],src[4],src[5]],
    })
}

pub fn decode_mb3d(src: &str) -> Result<MB3D,String> {

    // strip off the header and newlines
    let src = match src.strip_prefix("Mandelbulb3Dv18{\n") {
        Some(src) => src,
        None => return Err("data does not match Mandelbulb3Dv18".to_string()),
    };
    let src = match src.split_once("}") {
        Some(src) => src.0,
        None => return Err("data does not match Mandelbulb3Dv18".to_string()),
    };
    let encoded = src.replace('\n',"");
    let encoded = encoded.as_bytes();

    // convert base64ish to normal bytes
    let mut src = vec![0u8; (encoded.len() / 4) * 3];
    for i in 0..encoded.len() / 4 {
        let a = decode_byte(encoded[i * 4 + 3])?;
        let b = decode_byte(encoded[i * 4 + 2])?;
        let c = decode_byte(encoded[i * 4 + 1])?;
        let d = decode_byte(encoded[i * 4])?;
        src[i * 3 + 2] = ((a << 2) | (b >> 4)) as u8;
        src[i * 3 + 1] = (((b << 4) & 255) | (c >> 2)) as u8;
        src[i * 3] = (((c << 6) & 255) | d) as u8;
    }

    // and extract all the things
    Ok(MB3D {
        id: src[0],
        size: Vec2 {
            x: u32::from_le_bytes(src[4..8].try_into().unwrap()) as usize,
            y: u32::from_le_bytes(src[8..12].try_into().unwrap()) as usize,
        },
        iterations: u32::from_le_bytes(src[12..16].try_into().unwrap()) as usize,
        options: u16::from_le_bytes(src[16..18].try_into().unwrap()),
        use_quaternions: (src[18] & 1) != 0,
        z_range: [
            f64::from_le_bytes(src[20..28].try_into().unwrap()),
            f64::from_le_bytes(src[28..36].try_into().unwrap()),
        ],
        mid: Vec3 {
            x: f64::from_le_bytes(src[36..44].try_into().unwrap()),
            y: f64::from_le_bytes(src[44..52].try_into().unwrap()),
            z: f64::from_le_bytes(src[52..60].try_into().unwrap()),
        },
        rot: Vec3 {
            x: f64::from_le_bytes(src[60..68].try_into().unwrap()),
            y: f64::from_le_bytes(src[68..76].try_into().unwrap()),
            z: f64::from_le_bytes(src[76..84].try_into().unwrap()),
        },
        zoom: f64::from_le_bytes(src[84..92].try_into().unwrap()),
        r_stop: f64::from_le_bytes(src[92..100].try_into().unwrap()),
        f_mix_pow: f32::from_le_bytes(src[104..108].try_into().unwrap()),
        use_hdr: (src[124] & 1) != 0,
        secant_search: (src[124] & 2) != 0,
        auto_clip: (src[124] & 4) != 0,
        aa_box: (src[124] & 8) != 0,
        bokeh: Bokeh::Todo((src[124] & 0x70) >> 4),
        new_mc_record_yuv: (src[124] & 128) != 0,
        mc_diff_reflects: src[125],
        hard_shadows: HardShadows {
            automatic: (src[133] & 1) != 0,
            max_length_calc: f32::from_le_bytes(src[226..230].try_into().unwrap()),
            softer: if src[139] != 0 {
                SofterShadows::Enabled({
                    let mut pt = src[225] as i32;
                    if pt < -25 { pt = -25 };
                    if pt > 25 { pt = 25 };
                    (src[224] as f32) * 10.0_f32.powi(pt)
                })
            } else {
                SofterShadows::Disabled
            },
            func_to_cos: (src[133] & 2) != 0,
            lights: [
                (src[133] & 4) != 0,
                (src[133] & 8) != 0,
                (src[133] & 16) != 0,
                (src[133] & 32) != 0,
                (src[133] & 64) != 0,
                (src[133] & 128) != 0,
            ],
            calculated: [
                (src[163] & 1) != 0,
                (src[163] & 2) != 0,
                (src[163] & 4) != 0,
                (src[163] & 8) != 0,
                (src[163] & 16) != 0,
                (src[163] & 32) != 0,
            ],
        },
        ambient_shadows: AmbientShadows {
            automatic: (src[149] & 1) != 0,
            ambient_occlusion: match (src[149] >> 2) & 3 {
                0 => AmbientOcclusion::SSAO15 {
                    zr_threshold: f32::from_le_bytes(src[319..323].try_into().unwrap()),
                    threshold_to_0: (src[149] & 2) != 0,
                },
                1 => AmbientOcclusion::SSAO24 {
                    zr_threshold: f32::from_le_bytes(src[319..323].try_into().unwrap()),
                    threshold_to_0: (src[149] & 2) != 0,
                    border_size: src[127] as f32,
                },
                2 => AmbientOcclusion::SSAO24Random {
                    zr_threshold: f32::from_le_bytes(src[319..323].try_into().unwrap()),
                    threshold_to_0: (src[149] & 2) != 0,
                    border_size: src[127] as f32,
                    calc_count: src[187] as usize,
                },
                3 => AmbientOcclusion::DEAO {
                    dithering_scale: src[188],
                    first_step_random: (src[149] & 128) != 0,
                    max_l: f32::from_le_bytes(src[374..378].try_into().unwrap()),
                    rays: match (src[149] >> 4) & 3 {
                        0 => 3,
                        1 => 7,
                        2 => 17,
                        3 => 33,
                        _ => { return Err("this cannot happen".to_string()) },
                    },
                },
                _ => { return Err("this cannot happen".to_string()) },
            },
        },
        reflections_transparency: ReflectionsTransparency {
            automatic: (src[336] & 1) != 0,
            amount: f32::from_le_bytes(src[332..336].try_into().unwrap()),
            depth: src[337] as usize,
            transparency: (src[336] & 2) != 0,
            only_dif: (src[336] & 4) != 0,
            absorption: f32::from_le_bytes(src[370..374].try_into().unwrap()),
            refractive_index: f32::from_le_bytes(src[116..120].try_into().unwrap()),
            scattering: f32::from_le_bytes(src[120..124].try_into().unwrap()),
            selection_only: false, // TODO: research format
        },
        depth_of_field: DepthOfField {
            automatic: (src[181] & 1) != 0,
            z1_sharp: f32::from_le_bytes(src[164..168].try_into().unwrap()),
            z2_sharp: f32::from_le_bytes(src[410..414].try_into().unwrap()),
            aperture: f32::from_le_bytes(src[172..176].try_into().unwrap()),
            clipping_r: f32::from_le_bytes(src[168..172].try_into().unwrap()),
            forward_calculation: (src[181] & 8) != 0,
            passes: (src[181] >> 1) & 3,
        },
        calculation: Calculation {
            de_stop: f32::from_le_bytes(src[177..181].try_into().unwrap()),
            vary_de_stop_on_fov: src[162],
            ray_step_multiplier: f64::from_le_bytes(src[154..162].try_into().unwrap()),
            step_width_limiter: f32::from_le_bytes(src[242..246].try_into().unwrap()), // TODO: step_width ?
            step_count: 6,  // TODO
            smooth_normals: 0, // TODO: options, something about value << 6
            normals_on_de: src[132],
            first_step_random: false, // TODO: options & 1
            ray_step_sub_de_stop: src[134], // TODO: options & 4 ?
        },
        julia: if src[190] != 0 {
            Julia::Julia(Vec4 {
                x: f64::from_le_bytes(src[191..199].try_into().unwrap()),
                y: f64::from_le_bytes(src[199..207].try_into().unwrap()),
                z: f64::from_le_bytes(src[207..215].try_into().unwrap()),
                w: f64::from_le_bytes(src[215..223].try_into().unwrap()),
            })
        } else {
            Julia::Disabled
        },
        cutting: Vec3 {
            x: f64::from_le_bytes(src[346..354].try_into().unwrap()),
            y: f64::from_le_bytes(src[354..362].try_into().unwrap()),
            z: f64::from_le_bytes(src[362..370].try_into().unwrap()),
        },
        coloring: Coloring {
            choice_mode: match src[342] {
                0 => ChoiceMode::OrbitTrap,
                1 => ChoiceMode::LastLengthIncrease,
                2 => ChoiceMode::RoutAngleXY,
                3 => ChoiceMode::RoutAngleXZ,
                4 => ChoiceMode::RoutAngleYZ,
                5 => ChoiceMode::MapOnOutputVector,
                _ => { return Err(format!("invalid coloring choice mode ({})",src[342])) }
            },
            multiplier: f32::from_le_bytes(src[338..342].try_into().unwrap()),
            color_on_it: if (src[18] & 2) != 0 {
                ColorOnIt::OnIt(src[19] as usize)
            } else {
                ColorOnIt::Disabled
            },
            fog_or_vol: if src[223] != 0 {
                FogOrVol::DynFogOnIt(src[223] as u32)
            } else {
                FogOrVol::VolumeLight(src[343])
            },
        },
        camera: Camera {
            fovy: f64::from_le_bytes(src[108..116].try_into().unwrap()),
            lense: Lense::Common,  // TODO: research
        },
        infos: Infos {
            author: String::new(),
            moderator: String::new(),
            avg_ray_steps: 0.1 * (u32::from_le_bytes(src[140..144].try_into().unwrap()) as f32),
            avg_iterations: 0.1 * (u32::from_le_bytes(src[144..148].try_into().unwrap()) as f32),
            max_iterations: u32::from_le_bytes(src[414..418].try_into().unwrap()),
            main_calc_time: 0.1 * (u32::from_le_bytes(src[323..327].try_into().unwrap()) as f32),
            hs_calc_time: 0.1 * (u32::from_le_bytes(src[327..331].try_into().unwrap()) as f32),
            ao_calc_time: 0.1 * (u32::from_le_bytes(src[128..132].try_into().unwrap()) as f32),
            reflects_time: 0.1 * (u32::from_le_bytes(src[100..104].try_into().unwrap()) as f32),
        },
        stereo: Stereo {
            screen_distance: f32::from_le_bytes(src[234..238].try_into().unwrap()),
            screen_width: f32::from_le_bytes(src[230..234].try_into().unwrap()),
            minimal_distance: f32::from_le_bytes(src[238..242].try_into().unwrap()),
        },
        minimum_iterations: u16::from_le_bytes(src[135..137].try_into().unwrap()) as usize,
        mc_last_y: u16::from_le_bytes(src[137..139].try_into().unwrap()),
        planar_optic: src[148],
        navi_min_dist: f32::from_le_bytes(src[150..154].try_into().unwrap()),
        cut_option: src[176],
        z_step_div: f32::from_le_bytes(src[182..186].try_into().unwrap()),
        mc_depth: src[186],
        image_scale: src[189],
        v_grads: Mat3x3 {
            x: Vec3 {
                x: f64::from_le_bytes(src[246..254].try_into().unwrap()),
                y: f64::from_le_bytes(src[270..278].try_into().unwrap()),
                z: f64::from_le_bytes(src[294..302].try_into().unwrap()),
            },
            y: Vec3 {
                x: f64::from_le_bytes(src[254..262].try_into().unwrap()),
                y: f64::from_le_bytes(src[278..286].try_into().unwrap()),
                z: f64::from_le_bytes(src[302..310].try_into().unwrap()),
            },
            z: Vec3 {
                x: f64::from_le_bytes(src[262..270].try_into().unwrap()),
                y: f64::from_le_bytes(src[286..294].try_into().unwrap()),
                z: f64::from_le_bytes(src[310..318].try_into().unwrap()),
            },
        },
        mc_saturation: src[318],
        y_calc_ns_on_zbuf_auto: src[331],
        calc3d: src[344],
        slice_calc: src[345],
        de_comb_s: f32::from_le_bytes(src[378..382].try_into().unwrap()),
        h_custom_f: [
            u32::from_le_bytes(src[382..386].try_into().unwrap()),
            u32::from_le_bytes(src[386..390].try_into().unwrap()),
            u32::from_le_bytes(src[390..394].try_into().unwrap()),
            u32::from_le_bytes(src[394..398].try_into().unwrap()),
            u32::from_le_bytes(src[398..402].try_into().unwrap()),
            u32::from_le_bytes(src[402..406].try_into().unwrap()),
        ],
        cf_addon: u32::from_le_bytes(src[406..410].try_into().unwrap()),
        max_its_f2: u32::from_le_bytes(src[418..422].try_into().unwrap()),
        de_mix_color_option: src[422],
        mc_contrast: src[423],
        m3d_version: f32::from_le_bytes(src[424..428].try_into().unwrap()),
        tiling_options: u32::from_le_bytes(src[428..432].try_into().unwrap()),
        lighting: Lighting {
            var_col_z_pos: i16::from_le_bytes(src[432..434].try_into().unwrap()),
            roughness_factor: src[434],
            color_map: src[435],
            dyn_fog_col2: [src[436],src[437],src[438]],
            additional_options: src[439],
            tb_pos: [
                i32::from_le_bytes(src[440..444].try_into().unwrap()),
                i32::from_le_bytes(src[444..448].try_into().unwrap()),
                i32::from_le_bytes(src[448..452].try_into().unwrap()),
                i32::from_le_bytes(src[452..456].try_into().unwrap()),
                i32::from_le_bytes(src[456..460].try_into().unwrap()),
                i32::from_le_bytes(src[460..464].try_into().unwrap()),
                i32::from_le_bytes(src[464..468].try_into().unwrap()),
                i32::from_le_bytes(src[468..472].try_into().unwrap()),
                i32::from_le_bytes(src[472..476].try_into().unwrap()),
            ],
            tb_options: u32::from_le_bytes(src[476..480].try_into().unwrap()),
            fine_col_adj1: src[480],
            fine_col_adj2: src[481],
            pic_offset: Vec3 {
                x: src[482],
                y: src[483],
                z: src[499],
            },
            dyn_fog_col: [src[487],src[491],src[495]],
            amb_col: [src[484],src[485],src[486]],
            amb_col2: [src[488],src[489],src[490]],
            depth_col: [src[492],src[493],src[494]],
            depth_col2: [src[496],src[497],src[498]],
            lights: [
                decode_light(&src[500..532])?,
                decode_light(&src[532..564])?,
                decode_light(&src[564..596])?,
                decode_light(&src[596..628])?,
                decode_light(&src[628..660])?,
                decode_light(&src[660..692])?,
            ],
            lcols: [
                decode_lcol(&src[692..702])?,
                decode_lcol(&src[702..712])?,
                decode_lcol(&src[712..722])?,
                decode_lcol(&src[722..732])?,
                decode_lcol(&src[732..742])?,
                decode_lcol(&src[742..752])?,
                decode_lcol(&src[752..762])?,
                decode_lcol(&src[762..772])?,
                decode_lcol(&src[772..782])?,
                decode_lcol(&src[782..792])?,
            ],
            icols: [
                decode_icol(&src[792..798])?,
                decode_icol(&src[798..804])?,
                decode_icol(&src[804..810])?,
                decode_icol(&src[810..816])?,
            ],
        },
    })
}

pub fn dump_mb3d(mb3d: &MB3D) {
    println!("MB3D parameters:");
    println!("    id: {}",mb3d.id);
    println!("    size: {} x {}",mb3d.size.x,mb3d.size.y);
    println!("    iterations: {}",mb3d.iterations);
    println!("    options: {}",mb3d.options);
    println!("    use_quaternions: {}",mb3d.use_quaternions);
    println!("    z_range: from {} to {}",mb3d.z_range[0],mb3d.z_range[1]);
    println!("    mid: {}",mb3d.mid);
    println!("    rot: {}",mb3d.rot);
    println!("    zoom: {}",mb3d.zoom);
    println!("    r_stop: {}",mb3d.r_stop);
    println!("    f_mix_pow: {}",mb3d.f_mix_pow);
    println!("    use_hdr: {}",mb3d.use_hdr);
    println!("    secant_search: {}",mb3d.secant_search);
    println!("    auto_clip: {}",mb3d.auto_clip);
    println!("    aa_box: {}",mb3d.aa_box);
    println!("    bokeh: TODO{}",if let Bokeh::Todo(n) = mb3d.bokeh { n as i8 } else { -1 });
    println!("    new_mc_record_yuv: {}",mb3d.new_mc_record_yuv);
    println!("    mc_diff_reflects: {}",mb3d.mc_diff_reflects);
    println!("    hard shadows:");
    println!("        automatic: {}",mb3d.hard_shadows.automatic);
    println!("        max_length_calc: {}",mb3d.hard_shadows.max_length_calc);
    println!("        softer: {}",match mb3d.hard_shadows.softer {
        SofterShadows::Disabled => "disabled".to_string(),
        SofterShadows::Enabled(radius) => format!("{}",radius),
    });
    println!("        func_to_cos: {}",mb3d.hard_shadows.func_to_cos);
    println!("        lights: [{},{},{},{},{},{}]",mb3d.hard_shadows.lights[0],mb3d.hard_shadows.lights[1],mb3d.hard_shadows.lights[2],mb3d.hard_shadows.lights[3],mb3d.hard_shadows.lights[4],mb3d.hard_shadows.lights[5]);
    println!("        calculated: [{},{},{},{},{},{}]",mb3d.hard_shadows.calculated[0],mb3d.hard_shadows.calculated[1],mb3d.hard_shadows.calculated[2],mb3d.hard_shadows.calculated[3],mb3d.hard_shadows.calculated[4],mb3d.hard_shadows.calculated[5]);
    println!("    ambient shadows:");
    println!("        automatic: {}",mb3d.ambient_shadows.automatic);
    match mb3d.ambient_shadows.ambient_occlusion {
        AmbientOcclusion::SSAO15 { zr_threshold, threshold_to_0, }=> {
            println!("        type: 15-bit SSAO");
            println!("        zr_threshold: {}",zr_threshold);
            println!("        threshold_to_0: {}",threshold_to_0);
        },
        AmbientOcclusion::SSAO24 { zr_threshold, threshold_to_0, border_size, } => {
            println!("        type: 24-bit SSAO");
            println!("        zr_threshold: {}",zr_threshold);
            println!("        threshold_to_0: {}",threshold_to_0);
            println!("        border_size: {}",border_size);
        },
        AmbientOcclusion::SSAO24Random { zr_threshold, threshold_to_0, border_size, calc_count } => {
            println!("        type: random 24-bit SSAO");
            println!("        zr_threshold: {}",zr_threshold);
            println!("        threshold_to_0: {}",threshold_to_0);
            println!("        border_size: {}",border_size);
            println!("        calc_count: {}",calc_count);
        },
        AmbientOcclusion::DEAO { dithering_scale, first_step_random, max_l, rays, } => {
            println!("        type: DEAO");
            println!("        dithering_scale: {}",dithering_scale);
            println!("        first_step_random: {}",first_step_random);
            println!("        max_l: {}",max_l);
            println!("        rays: {}",rays);
        },
    }
    println!("    reflections and transparency:");
    println!("        automatic: {}",mb3d.reflections_transparency.automatic);
    println!("        amount: {}",mb3d.reflections_transparency.amount);
    println!("        depth: {}",mb3d.reflections_transparency.depth);
    println!("        transparency: {}",mb3d.reflections_transparency.transparency);
    println!("        only_dif: {}",mb3d.reflections_transparency.only_dif);
    println!("        absorption: {}",mb3d.reflections_transparency.absorption);
    println!("        refractive_index: {}",mb3d.reflections_transparency.refractive_index);
    println!("        scattering: {}",mb3d.reflections_transparency.scattering);
    println!("        selection_only: {}",mb3d.reflections_transparency.selection_only);
    println!("    depth of field:");
    println!("        automatic: {}",mb3d.depth_of_field.automatic);
    println!("        z1_sharp: {}",mb3d.depth_of_field.z1_sharp);
    println!("        z2_sharp: {}",mb3d.depth_of_field.z2_sharp);
    println!("        aperture: {}",mb3d.depth_of_field.aperture);
    println!("        clipping_r: {}",mb3d.depth_of_field.clipping_r);
    println!("        forward_calculation: {}",mb3d.depth_of_field.forward_calculation);
    println!("        passes: {}",mb3d.depth_of_field.passes);
    println!("    calculation:");
    println!("        de_stop: {}",mb3d.calculation.de_stop);
    println!("        vary_de_stop_on_fov: {}",mb3d.calculation.vary_de_stop_on_fov);
    println!("        ray_step_multiplier: {}",mb3d.calculation.ray_step_multiplier);
    println!("        step_width_limiter: {}",mb3d.calculation.step_width_limiter);
    println!("        step_count: {}",mb3d.calculation.step_count);
    println!("        smooth_normals: {}",mb3d.calculation.smooth_normals);
    println!("        normals_on_de: {}",mb3d.calculation.normals_on_de);
    println!("        first_step_random: {}",mb3d.calculation.first_step_random);
    println!("        ray_step_sub_de_stop: {}",mb3d.calculation.ray_step_sub_de_stop);
    println!("    julia: {}",match mb3d.julia {
        Julia::Disabled => "disabled".to_string(),
        Julia::Julia(v) => format!("{}",v),
    });
    println!("    cutting: {}",mb3d.cutting);
    println!("    coloring:");
    println!("        choice_mode: {}",match mb3d.coloring.choice_mode {
        ChoiceMode::OrbitTrap => "orbit trap",
        ChoiceMode::LastLengthIncrease => "last length increase",
        ChoiceMode::RoutAngleXY => "rout angle X-Y",
        ChoiceMode::RoutAngleXZ => "rout angle X-Z",
        ChoiceMode::RoutAngleYZ => "rout angle Y-Z",
        ChoiceMode::MapOnOutputVector => "map on output vector",
    });
    println!("        multiplier: {}",mb3d.coloring.multiplier);
    println!("        color_on_it: {}",match mb3d.coloring.color_on_it {
        ColorOnIt::Disabled => "disabled".to_string(),
        ColorOnIt::OnIt(n) => format!("{}",n),
    });
    println!("        fog_or_vol: {}",match mb3d.coloring.fog_or_vol {
        FogOrVol::DynFogOnIt(n) => format!("dyn fog {}",n),
        FogOrVol::VolumeLight(n) => format!("volume light {}",n),
    });
    println!("    camera:");
    println!("        fovy: {}",mb3d.camera.fovy);
    println!("        lense: {}",match mb3d.camera.lense {
        Lense::Common => "common",
        Lense::Rectilinear => "rectilinear",
        Lense::Panorama360 => "360 panorama",
    });
    println!("    infos:");
    println!("        author: {}",mb3d.infos.author);
    println!("        moderator: {}",mb3d.infos.moderator);
    println!("        average ray steps: {}",mb3d.infos.avg_ray_steps);
    println!("        average iterations: {}",mb3d.infos.avg_iterations);
    println!("        main_calc_time: {}",mb3d.infos.main_calc_time);
    println!("        hs_calc_time: {}",mb3d.infos.hs_calc_time);
    println!("        ao_calc_time: {}",mb3d.infos.ao_calc_time);
    println!("        reflects_time: {}",mb3d.infos.reflects_time);
    println!("    stereo:");
    println!("        screen_distance: {}",mb3d.stereo.screen_distance);
    println!("        screen_width: {}",mb3d.stereo.screen_width);
    println!("        minimal_distance: {}",mb3d.stereo.minimal_distance);
    println!("    lighting:");
    println!("        var_col_z_pos: {}",mb3d.lighting.var_col_z_pos);
    println!("        roughness_factor: {}",mb3d.lighting.roughness_factor);
    println!("        color_map: {}",mb3d.lighting.color_map);
    println!("        dyn_fog_col2: [{},{},{}]",mb3d.lighting.dyn_fog_col2[0],mb3d.lighting.dyn_fog_col2[1],mb3d.lighting.dyn_fog_col2[2]);
    println!("        additional_options: {}",mb3d.lighting.additional_options);
    println!("        tb_pos: [{},{},{},{},{},{},{},{},{}]",mb3d.lighting.tb_pos[0],mb3d.lighting.tb_pos[1],mb3d.lighting.tb_pos[2],mb3d.lighting.tb_pos[3],mb3d.lighting.tb_pos[4],mb3d.lighting.tb_pos[5],mb3d.lighting.tb_pos[6],mb3d.lighting.tb_pos[7],mb3d.lighting.tb_pos[8]);
    println!("        tb_options: {}",mb3d.lighting.tb_options);
    println!("        fine_col_adj1: {}",mb3d.lighting.fine_col_adj1);
    println!("        fine_col_adj2: {}",mb3d.lighting.fine_col_adj2);
    println!("        pic_offset: ({},{},{})",mb3d.lighting.pic_offset.x,mb3d.lighting.pic_offset.y,mb3d.lighting.pic_offset.z);
    println!("        dyn_fog_col: [{},{},{}]",mb3d.lighting.dyn_fog_col[0],mb3d.lighting.dyn_fog_col[1],mb3d.lighting.dyn_fog_col[2]);
    println!("        amb_col: [{},{},{}]",mb3d.lighting.amb_col[0],mb3d.lighting.amb_col[1],mb3d.lighting.amb_col[2]);
    println!("        amb_col2: [{},{},{}]",mb3d.lighting.amb_col2[0],mb3d.lighting.amb_col2[1],mb3d.lighting.amb_col2[2]);
    println!("        depth_col: [{},{},{}]",mb3d.lighting.depth_col[0],mb3d.lighting.depth_col[1],mb3d.lighting.depth_col[2]);
    println!("        depth_col2: [{},{},{}]",mb3d.lighting.depth_col2[0],mb3d.lighting.depth_col2[1],mb3d.lighting.depth_col2[2]);
    for i in 0..6 {
        let light = &mb3d.lighting.lights[i];
        println!("        light {}:",i);
        println!("            option: {}",light.option);
        println!("            function: {}",light.function);
        println!("            amp: {}",light.amp);
        println!("            color: [{},{},{}]",light.color[0],light.color[1],light.color[2]);
        println!("            light_map_nr: {}",light.light_map_nr);
        println!("            pos: {}",light.pos);
        println!("            additional_byte_ex: {}",light.additional_byte_ex);
        println!("            free_byte: {}",light.free_byte);    
    }
    for i in 0..10 {
        let lcol = &mb3d.lighting.lcols[i];
        println!("        surface color {}:",i);
        println!("            pos: {}",lcol.pos);
        println!("            diff: [{},{},{},{}]",lcol.diff[0],lcol.diff[1],lcol.diff[2],lcol.diff[3]);
        println!("            spec: [{},{},{},{}]",lcol.spec[0],lcol.spec[1],lcol.spec[2],lcol.spec[3]);
    }
    for i in 0..4 {
        let icol = &mb3d.lighting.icols[i];
        println!("        internal color {}:",i);
        println!("            pos: {}",icol.pos);
        println!("            col: [{},{},{},{}]",icol.col[0],icol.col[1],icol.col[2],icol.col[3]);
    }
    println!("    minimum_iterations: {}",mb3d.minimum_iterations);
    println!("    mc_last_y: {}",mb3d.mc_last_y);
    println!("    planar_optic: {}",mb3d.planar_optic);
    println!("    navi_min_dist: {}",mb3d.navi_min_dist);
    println!("    cut_option: {}",mb3d.cut_option);
    println!("    z_step_div: {}",mb3d.z_step_div);
    println!("    mc_depth: {}",mb3d.mc_depth);
    println!("    image_scale: {}",mb3d.image_scale);
    println!("    v_grads: {}",mb3d.v_grads);
    println!("    mc_saturation: {}",mb3d.mc_saturation);
    println!("    y_calc_ns_on_zbuf_auto: {}",mb3d.y_calc_ns_on_zbuf_auto);
    println!("    calc3d: {}",mb3d.calc3d);
    println!("    slice_calc: {}",mb3d.slice_calc);
    println!("    de_comb_s: {}",mb3d.de_comb_s);
    println!("    h_custom_f: [{},{},{},{},{},{}]",mb3d.h_custom_f[0],mb3d.h_custom_f[1],mb3d.h_custom_f[2],mb3d.h_custom_f[3],mb3d.h_custom_f[4],mb3d.h_custom_f[5]);
    println!("    cf_addon: {}",mb3d.cf_addon);
    println!("    max_its_f2: {}",mb3d.max_its_f2);
    println!("    de_mix_color_option: {}",mb3d.de_mix_color_option);
    println!("    mc_contrast: {}",mb3d.mc_contrast);
    println!("    m3d_version: {}",mb3d.m3d_version);
    println!("    tiling_options: {}",mb3d.tiling_options);
}
