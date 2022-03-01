

#[derive(Copy,Clone,Debug)]
#[repr(C)]
pub struct Time64 {
    fractions :u32,
    seconds :u32,
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
pub struct CineHeader {
    kind :u16,
    headersize :u16,
    compression :u16,
    version :u16,
    first_movie_image :i32,
    total_image_count :u32,
    first_image_number :i32,
    image_count :u32,
    offset_to_image_header :u32,
    offset_to_setup_header :u32,
    offset_to_image_offsets :u32,
    trigger_time :Time64,
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
pub struct BitmapHeader {
    bi_size :u32,
    width :i32,
    height :i32,
    planes :u16,
    bit_count :u16,
    compression :u32,
    size_image :u32,
    x_pels_per_meter :i32,
    y_pels_per_meter :i32,
    color_used :u32,
    color_important :u32,
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
pub struct ImFilter {
    dim :i32,
    shifts :i32,
    bias :i32,
    coef :[i32;25],
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
pub struct WbGain {
    r :f32,
    b :f32,
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
pub struct Rect {
    left :i32,
    right :i32,
    top :i32,
    bottom :i32,
}

#[derive(Copy,Clone,Debug)]
#[repr(C,packed)]
pub struct SetupHeader {
    fps16 :u16,
    shutter16 :u16,
    post_trigger16 :u16,
    frame_delay16 :u16,
    aspect_ratio :u16,
    res7 :u16,
    res8 :u16,
    res9 :u8,
    res10 :u8,
    res11 :u8,
    trig_frame :u8,
    res12 :u8,
    description_old :[u8;121],
    mark :u16,
    length :u16,
    res13 :u16,
    sig_option :u16,
    bin_channesl :i16,
    samples_per_image :u8,
    bin_name :[[u8;11];8],
    ana_option :u16,
    ana_channels :i16,
    res6 :u8,
    ana_board :u8,
    ch_option :[i16;8],
    ana_gain :[f32;8],
    ana_unit :[[u8;6];8],
    ana_name :[[u8;11];8],
    l_first_image :i32,
    dw_image_count :u32,
    nq_factor :i16,
    w_cine_file_type :u16,
    sz_cine_path :[[u8;65];4],
    res14 :u16,
    res15 :u8,
    res16 :u8,
    res17 :u16,
    res18 :f64,
    res19 :f64,
    res20 :u16,
    res1 :i32,
    res2 :i32,
    res3 :i32,
    imwidth :u16,
    imheight :u16,
    edr_shutter16 :u16,
    serial :u32,
    saturation :i32,
    res5 :u8,
    auto_exposure :u32,
    b_flip_h :i32,
    b_flip_v :i32,
    grid :u32,
    frame_rate :u32,
    shutter :u32,
    edr_shutter :u32,
    post_trigger :u32,
    frame_delay :u32,
    b_enable_color :i32,
    camera_version :u32,
    firmware_version :u32,
    software_version :u32,
    recording_time_zone :i32,
    cfa :u32,
    bright :i32,
    contrast :i32,
    gamma :i32,
    res21 :u32,
    auto_exp_level :u32,
    auto_exp_speed :u32,
    auto_exp_rect :Rect,
    wb_gain :[WbGain;4],
    rotate :i32,
    wb_view :WbGain,
    real_bpp :u32,
    conv8min :u32,
    conv8max :u32,
    filter_code :i32,
    filter_param :i32,
    uf :ImFilter,
    black_cal_s_ver :u32,
    white_cal_s_ver :u32,
    gray_cal_s_ver :u32,
    b_stamp_time :i32,
    sound_dest :u32,
    frp_steps :u32,
    frp_image_number :[i32;16],
    frp_rate :[u32;16],
    frp_exp :[u32;16],
    mcc_nt :i32,
    mc_percent :[f32;64],
    ci_calib :u32,
    calib_width :u32,
    calib_height :u32,
    calib_rate :u32,
    calib_exp :u32,
    calib_edr :u32,
    calib_temp :u32,
    head_serial :[u32;4],
    range_code :u32,
    range_size :u32,
    decimation :u32,
    master_serial :u32,
    sensor :u32,
    shutter_ns :u32,
    edr_shutter_ns :u32,
    frame_delay_ns :u32,
    im_pos_x_acq :u32,
    im_pos_y_acq :u32,
    im_width_acq :u32,
    im_height_acq :u32,
    description :[u8;4096],
    rising_edge :i32,
    filter_time :u32,
    long_ready :i32,
    shutter_off :i32,
    /* no further fields in this structure from the video files we are using */
}

impl CineHeader {

    pub fn get_image_count(&self) -> u32 {
        self.image_count
    }

    pub fn get_images_array_offset(&self) -> u64 {
        self.offset_to_image_offsets as u64
    }
}

impl BitmapHeader {

    pub fn get_size_in_bytes(&self) -> usize {
        self.size_image as usize
    }

    pub fn get_bit_depth(&self) -> u16 {
        self.bit_count
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

}
