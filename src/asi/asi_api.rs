#![allow(dead_code)]

use super::{asicamera2::*, bytes_to_chars, chars_to_string};
use std::{error::Error, ffi::CStr, fmt::Display};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ASIBayerPattern {
    RG = 0,
    BG = 1,
    GR = 2,
    GB = 3,
}

impl ASIBayerPattern {
    pub fn from_raw(bayer_pattern: ASI_BAYER_PATTERN) -> Self {
        match bayer_pattern {
            ASI_BAYER_PATTERN_ASI_BAYER_RG => Self::RG,
            ASI_BAYER_PATTERN_ASI_BAYER_BG => Self::BG,
            ASI_BAYER_PATTERN_ASI_BAYER_GR => Self::GR,
            ASI_BAYER_PATTERN_ASI_BAYER_GB => Self::GB,
            _ => panic!("Unknown bayer pattern code. {}", bayer_pattern),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASIImageType {
    Raw8 = ASI_IMG_TYPE_ASI_IMG_RAW8,
    Rgb24 = ASI_IMG_TYPE_ASI_IMG_RGB24,
    Raw16 = ASI_IMG_TYPE_ASI_IMG_RAW16,
    Y8 = ASI_IMG_TYPE_ASI_IMG_Y8,
    End = ASI_IMG_TYPE_ASI_IMG_END,
}

impl ASIImageType {
    pub fn from_raw(image_type: ASI_IMG_TYPE) -> Self {
        match image_type {
            ASI_IMG_TYPE_ASI_IMG_RAW8 => Self::Raw8,
            ASI_IMG_TYPE_ASI_IMG_RGB24 => Self::Rgb24,
            ASI_IMG_TYPE_ASI_IMG_RAW16 => Self::Raw16,
            ASI_IMG_TYPE_ASI_IMG_Y8 => Self::Y8,
            ASI_IMG_TYPE_ASI_IMG_END => Self::End,
            _ => panic!("Unknown image type code. {}", image_type),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASIGuideDirection {
    North = ASI_GUIDE_DIRECTION_ASI_GUIDE_NORTH,
    South = ASI_GUIDE_DIRECTION_ASI_GUIDE_SOUTH,
    East = ASI_GUIDE_DIRECTION_ASI_GUIDE_EAST,
    West = ASI_GUIDE_DIRECTION_ASI_GUIDE_WEST,
}

impl ASIGuideDirection {
    pub fn from_raw(guide_direction: ASI_GUIDE_DIRECTION) -> Self {
        match guide_direction {
            ASI_GUIDE_DIRECTION_ASI_GUIDE_NORTH => Self::North,
            ASI_GUIDE_DIRECTION_ASI_GUIDE_SOUTH => Self::South,
            ASI_GUIDE_DIRECTION_ASI_GUIDE_EAST => Self::East,
            ASI_GUIDE_DIRECTION_ASI_GUIDE_WEST => Self::West,
            _ => panic!("Unknown guide direction code. {}", guide_direction),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASIFlipStatus {
    None = ASI_FLIP_STATUS_ASI_FLIP_NONE,
    Horizontal = ASI_FLIP_STATUS_ASI_FLIP_HORIZ,
    Vertical = ASI_FLIP_STATUS_ASI_FLIP_VERT,
    Both = ASI_FLIP_STATUS_ASI_FLIP_BOTH,
}

impl ASIFlipStatus {
    pub fn from_raw(flip_status: ASI_FLIP_STATUS) -> Self {
        match flip_status {
            ASI_FLIP_STATUS_ASI_FLIP_NONE => Self::None,
            ASI_FLIP_STATUS_ASI_FLIP_HORIZ => Self::Horizontal,
            ASI_FLIP_STATUS_ASI_FLIP_VERT => Self::Vertical,
            ASI_FLIP_STATUS_ASI_FLIP_BOTH => Self::Both,
            _ => panic!("Unknown flip status code. {}", flip_status),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASICameraMode {
    Normal = ASI_CAMERA_MODE_ASI_MODE_NORMAL,
    TrigSoftEdge = ASI_CAMERA_MODE_ASI_MODE_TRIG_SOFT_EDGE,
    TrigRiseEdge = ASI_CAMERA_MODE_ASI_MODE_TRIG_RISE_EDGE,
    TrigFallEdge = ASI_CAMERA_MODE_ASI_MODE_TRIG_FALL_EDGE,
    TrigSoftLevel = ASI_CAMERA_MODE_ASI_MODE_TRIG_SOFT_LEVEL,
    TrigHighLevel = ASI_CAMERA_MODE_ASI_MODE_TRIG_HIGH_LEVEL,
    TrigLowLevel = ASI_CAMERA_MODE_ASI_MODE_TRIG_LOW_LEVEL,
    End = ASI_CAMERA_MODE_ASI_MODE_END,
}

impl ASICameraMode {
    pub fn from_raw(camera_mode: ASI_CAMERA_MODE) -> Self {
        match camera_mode {
            ASI_CAMERA_MODE_ASI_MODE_TRIG_SOFT_EDGE => Self::TrigSoftEdge,
            ASI_CAMERA_MODE_ASI_MODE_TRIG_RISE_EDGE => Self::TrigRiseEdge,
            ASI_CAMERA_MODE_ASI_MODE_TRIG_FALL_EDGE => Self::TrigFallEdge,
            ASI_CAMERA_MODE_ASI_MODE_TRIG_SOFT_LEVEL => Self::TrigSoftLevel,
            ASI_CAMERA_MODE_ASI_MODE_TRIG_HIGH_LEVEL => Self::TrigHighLevel,
            ASI_CAMERA_MODE_ASI_MODE_TRIG_LOW_LEVEL => Self::TrigLowLevel,
            ASI_CAMERA_MODE_ASI_MODE_END => Self::End,
            _ => panic!("Unknown camera mode code. {}", camera_mode),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASITrigOutput {
    PinA = ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_PINA,
    PinB = ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_PINB,
    None = ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_NONE,
}

impl ASITrigOutput {
    pub fn from_raw(trig_output: ASI_TRIG_OUTPUT) -> Self {
        match trig_output {
            ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_PINA => Self::PinA,
            ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_PINB => Self::PinB,
            ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_NONE => Self::None,
            _ => panic!("Unknown trig output pin code. {}", trig_output),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASIError {
    InvalidIndex = ASI_ERROR_CODE_ASI_ERROR_INVALID_INDEX,
    InvalidID = ASI_ERROR_CODE_ASI_ERROR_INVALID_ID,
    InvalidControlType = ASI_ERROR_CODE_ASI_ERROR_INVALID_CONTROL_TYPE,
    CameraClosed = ASI_ERROR_CODE_ASI_ERROR_CAMERA_CLOSED,
    CameraRemoved = ASI_ERROR_CODE_ASI_ERROR_CAMERA_REMOVED,
    InvalidPath = ASI_ERROR_CODE_ASI_ERROR_INVALID_PATH,
    InvalidFileformat = ASI_ERROR_CODE_ASI_ERROR_INVALID_FILEFORMAT,
    InvalidSize = ASI_ERROR_CODE_ASI_ERROR_INVALID_SIZE,
    InvalidImgtype = ASI_ERROR_CODE_ASI_ERROR_INVALID_IMGTYPE,
    OutofBoundary = ASI_ERROR_CODE_ASI_ERROR_OUTOF_BOUNDARY,
    Timeout = ASI_ERROR_CODE_ASI_ERROR_TIMEOUT,
    InvalidSequence = ASI_ERROR_CODE_ASI_ERROR_INVALID_SEQUENCE,
    BufferTooSmall = ASI_ERROR_CODE_ASI_ERROR_BUFFER_TOO_SMALL,
    VideoModeActive = ASI_ERROR_CODE_ASI_ERROR_VIDEO_MODE_ACTIVE,
    ExposureInProgress = ASI_ERROR_CODE_ASI_ERROR_EXPOSURE_IN_PROGRESS,
    GeneralError = ASI_ERROR_CODE_ASI_ERROR_GENERAL_ERROR,
    InvalidMode = ASI_ERROR_CODE_ASI_ERROR_INVALID_MODE,
    End = ASI_ERROR_CODE_ASI_ERROR_END,
}

impl Display for ASIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASIError::InvalidIndex => write!(f, "Invalid index. asi error code {}", *self as u32),
            ASIError::InvalidID => write!(f, "Invalid id. asi error code {}", *self as u32),
            ASIError::InvalidControlType => {
                write!(f, "Invalid control type. asi error code {}", *self as u32)
            }
            ASIError::CameraClosed => write!(f, "Camera closed. asi error code {}", *self as u32),
            ASIError::CameraRemoved => write!(f, "Camera removed. asi error code {}", *self as u32),
            ASIError::InvalidPath => write!(f, "Invalid path. asi error code {}", *self as u32),
            ASIError::InvalidFileformat => {
                write!(f, "Invalid file format. asi error code {}", *self as u32)
            }
            ASIError::InvalidSize => write!(f, "Invalid size. asi error code {}", *self as u32),
            ASIError::InvalidImgtype => {
                write!(f, "Invalid image type. asi error code {}", *self as u32)
            }
            ASIError::OutofBoundary => {
                write!(f, "Out of boundary. asi error code {}", *self as u32)
            }
            ASIError::Timeout => write!(f, "Timeout. asi error code {}", *self as u32),
            ASIError::InvalidSequence => {
                write!(f, "Invalid sequence. asi error code {}", *self as u32)
            }
            ASIError::BufferTooSmall => {
                write!(f, "Buffer too small. asi error code {}", *self as u32)
            }
            ASIError::VideoModeActive => {
                write!(f, "Video mode active. asi error code {}", *self as u32)
            }
            ASIError::ExposureInProgress => {
                write!(f, "Exposure in progress. asi error code {}", *self as u32)
            }
            ASIError::GeneralError => write!(f, "General error. asi error code {}", *self as u32),
            ASIError::InvalidMode => write!(f, "Invalid mode. asi error code {}", *self as u32),
            ASIError::End => write!(f, "End. asi error code {}", *self as u32),
        }
    }
}

impl Error for ASIError {}

impl ASIError {
    pub fn from_raw(error_code: ASI_ERROR_CODE) -> Result<(), Self> {
        match error_code {
            ASI_ERROR_CODE_ASI_SUCCESS => Ok(()),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_INDEX => Err(Self::InvalidIndex),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_ID => Err(Self::InvalidID),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_CONTROL_TYPE => Err(Self::InvalidControlType),
            ASI_ERROR_CODE_ASI_ERROR_CAMERA_CLOSED => Err(Self::CameraClosed),
            ASI_ERROR_CODE_ASI_ERROR_CAMERA_REMOVED => Err(Self::CameraRemoved),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_PATH => Err(Self::InvalidPath),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_FILEFORMAT => Err(Self::InvalidFileformat),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_SIZE => Err(Self::InvalidSize),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_IMGTYPE => Err(Self::InvalidImgtype),
            ASI_ERROR_CODE_ASI_ERROR_OUTOF_BOUNDARY => Err(Self::OutofBoundary),
            ASI_ERROR_CODE_ASI_ERROR_TIMEOUT => Err(Self::Timeout),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_SEQUENCE => Err(Self::InvalidSequence),
            ASI_ERROR_CODE_ASI_ERROR_BUFFER_TOO_SMALL => Err(Self::BufferTooSmall),
            ASI_ERROR_CODE_ASI_ERROR_VIDEO_MODE_ACTIVE => Err(Self::VideoModeActive),
            ASI_ERROR_CODE_ASI_ERROR_EXPOSURE_IN_PROGRESS => Err(Self::ExposureInProgress),
            ASI_ERROR_CODE_ASI_ERROR_GENERAL_ERROR => Err(Self::GeneralError),
            ASI_ERROR_CODE_ASI_ERROR_INVALID_MODE => Err(Self::InvalidMode),
            ASI_ERROR_CODE_ASI_ERROR_END => Err(Self::End),
            _ => panic!("Unknown error code. {}", error_code),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASIBool {
    False = ASI_BOOL_ASI_FALSE,
    True = ASI_BOOL_ASI_TRUE,
}

impl ASIBool {
    pub fn from_raw(asi_bool: ASI_BOOL) -> Self {
        match asi_bool {
            ASI_BOOL_ASI_FALSE => Self::False,
            ASI_BOOL_ASI_TRUE => Self::True,
            _ => panic!("Unknown bool code. {}", asi_bool),
        }
    }

    pub fn to_bool(self) -> bool {
        match self {
            Self::False => false,
            Self::True => true,
        }
    }

    pub fn from_bool(b: bool) -> ASIBool {
        match b {
            true => Self::True,
            false => Self::False,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ASICameraInfo {
    pub name: String,
    pub camera_id: i32,
    pub max_height: i32,
    pub max_width: i32,
    pub is_color_cam: bool,
    pub bayer_pattern: ASIBayerPattern,
    pub supported_bins: Vec<i32>,
    pub supported_video_format: Vec<ASIImageType>,
    pub pixel_size: f64,
    pub mechanical_shutter: bool,
    pub st4_port: bool,
    pub is_cooler_cam: bool,
    pub is_usb3_host: bool,
    pub is_usb3_camera: bool,
    pub elec_per_adu: f32,
    pub bit_depth: i32,
    pub is_trigger_cam: bool,
}

impl ASICameraInfo {
    pub fn from_raw(raw: ASI_CAMERA_INFO) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self {
            name: chars_to_string(&raw.Name)?,
            camera_id: raw.CameraID,
            max_height: raw.MaxHeight,
            max_width: raw.MaxWidth,
            is_color_cam: ASIBool::from_raw(raw.IsColorCam).to_bool(),
            bayer_pattern: ASIBayerPattern::from_raw(raw.BayerPattern),
            supported_bins: Vec::with_capacity(16),
            supported_video_format: Vec::with_capacity(8),
            pixel_size: raw.PixelSize,
            mechanical_shutter: ASIBool::from_raw(raw.MechanicalShutter).to_bool(),
            st4_port: ASIBool::from_raw(raw.ST4Port).to_bool(),
            is_cooler_cam: ASIBool::from_raw(raw.IsCoolerCam).to_bool(),
            is_usb3_host: ASIBool::from_raw(raw.IsUSB3Host).to_bool(),
            is_usb3_camera: ASIBool::from_raw(raw.IsUSB3Camera).to_bool(),
            elec_per_adu: raw.ElecPerADU,
            bit_depth: raw.BitDepth,
            is_trigger_cam: ASIBool::from_raw(raw.IsTriggerCam).to_bool(),
        };

        for bin in raw.SupportedBins {
            if bin == 0 {
                break;
            }
            ret.supported_bins.push(bin);
        }

        for video_format in raw.SupportedVideoFormat {
            if video_format == ASIImageType::End as i32 {
                break;
            }
            ret.supported_video_format
                .push(ASIImageType::from_raw(video_format));
        }
        Ok(ret)
    }

    pub fn to_raw(&self) -> ASI_CAMERA_INFO {
        let mut name = [0 as ::std::os::raw::c_char; 64];
        name[..self.name.len()].copy_from_slice(bytes_to_chars(self.name.as_bytes()));

        let mut supported_bins = [0; 16];
        supported_bins.copy_from_slice(&self.supported_bins);

        let mut supported_video_format = [0; 8];
        let slice_raw: Vec<ASI_IMG_TYPE> = self
            .supported_video_format
            .iter()
            .map(|x| *x as i32)
            .collect();
        supported_video_format[..slice_raw.len()].copy_from_slice(&slice_raw);

        ASI_CAMERA_INFO {
            Name: name,
            CameraID: self.camera_id,
            MaxHeight: self.max_height as ::std::os::raw::c_long,
            MaxWidth: self.max_width as ::std::os::raw::c_long,
            IsColorCam: ASIBool::from_bool(self.is_color_cam) as ASI_BOOL,
            BayerPattern: self.bayer_pattern as ASI_BAYER_PATTERN,
            SupportedBins: supported_bins,
            SupportedVideoFormat: supported_video_format,
            PixelSize: self.pixel_size,
            MechanicalShutter: ASIBool::from_bool(self.mechanical_shutter) as ASI_BOOL,
            ST4Port: ASIBool::from_bool(self.st4_port) as ASI_BOOL,
            IsCoolerCam: ASIBool::from_bool(self.is_cooler_cam) as ASI_BOOL,
            IsUSB3Host: ASIBool::from_bool(self.is_usb3_host) as ASI_BOOL,
            IsUSB3Camera: ASIBool::from_bool(self.is_usb3_camera) as ASI_BOOL,
            ElecPerADU: self.elec_per_adu,
            BitDepth: self.bit_depth,
            IsTriggerCam: ASIBool::from_bool(self.is_trigger_cam) as ASI_BOOL,
            Unused: [0 as ::std::os::raw::c_char; 16usize], //pad
        }
    }
}

impl Default for ASICameraInfo {
    fn default() -> Self {
        Self {
            name: Default::default(),
            camera_id: Default::default(),
            max_height: Default::default(),
            max_width: Default::default(),
            is_color_cam: Default::default(),
            bayer_pattern: ASIBayerPattern::RG,
            supported_bins: Vec::with_capacity(16),
            supported_video_format: Vec::with_capacity(8),
            pixel_size: Default::default(),
            mechanical_shutter: Default::default(),
            st4_port: Default::default(),
            is_cooler_cam: Default::default(),
            is_usb3_host: Default::default(),
            is_usb3_camera: Default::default(),
            elec_per_adu: Default::default(),
            bit_depth: Default::default(),
            is_trigger_cam: Default::default(),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASIControlType {
    Gain = ASI_CONTROL_TYPE_ASI_GAIN,
    Exposure = ASI_CONTROL_TYPE_ASI_EXPOSURE,
    Gamma = ASI_CONTROL_TYPE_ASI_GAMMA,
    WbR = ASI_CONTROL_TYPE_ASI_WB_R,
    WbB = ASI_CONTROL_TYPE_ASI_WB_B,
    Offset = ASI_CONTROL_TYPE_ASI_OFFSET,
    BandwidthOverload = ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD,
    OverClock = ASI_CONTROL_TYPE_ASI_OVERCLOCK,
    Temperature = ASI_CONTROL_TYPE_ASI_TEMPERATURE,
    Flip = ASI_CONTROL_TYPE_ASI_FLIP,
    AutoMaxGain = ASI_CONTROL_TYPE_ASI_AUTO_MAX_GAIN,
    AutoMaxExp = ASI_CONTROL_TYPE_ASI_AUTO_MAX_EXP,
    AutoTargetBrightness = ASI_CONTROL_TYPE_ASI_AUTO_TARGET_BRIGHTNESS,
    HardwareBin = ASI_CONTROL_TYPE_ASI_HARDWARE_BIN,
    HighSpeedMode = ASI_CONTROL_TYPE_ASI_HIGH_SPEED_MODE,
    CoolerPowerPerc = ASI_CONTROL_TYPE_ASI_COOLER_POWER_PERC,
    TargetTemp = ASI_CONTROL_TYPE_ASI_TARGET_TEMP,
    CoolerOn = ASI_CONTROL_TYPE_ASI_COOLER_ON,
    MonoBin = ASI_CONTROL_TYPE_ASI_MONO_BIN,
    FanOn = ASI_CONTROL_TYPE_ASI_FAN_ON,
    PatternAdjust = ASI_CONTROL_TYPE_ASI_PATTERN_ADJUST,
    AntiDewHeater = ASI_CONTROL_TYPE_ASI_ANTI_DEW_HEATER,
}

impl ASIControlType {
    pub fn from_raw(control_type: ASI_CONTROL_TYPE) -> Self {
        match control_type {
            ASI_CONTROL_TYPE_ASI_GAIN => Self::Gain,
            ASI_CONTROL_TYPE_ASI_EXPOSURE => Self::Exposure,
            ASI_CONTROL_TYPE_ASI_GAMMA => Self::Gamma,
            ASI_CONTROL_TYPE_ASI_WB_R => Self::WbR,
            ASI_CONTROL_TYPE_ASI_WB_B => Self::WbB,
            ASI_CONTROL_TYPE_ASI_OFFSET => Self::Offset,
            ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD => Self::BandwidthOverload,
            ASI_CONTROL_TYPE_ASI_OVERCLOCK => Self::OverClock,
            ASI_CONTROL_TYPE_ASI_TEMPERATURE => Self::Temperature,
            ASI_CONTROL_TYPE_ASI_FLIP => Self::Flip,
            ASI_CONTROL_TYPE_ASI_AUTO_MAX_GAIN => Self::AutoMaxGain,
            ASI_CONTROL_TYPE_ASI_AUTO_MAX_EXP => Self::AutoMaxExp,
            ASI_CONTROL_TYPE_ASI_AUTO_TARGET_BRIGHTNESS => Self::AutoTargetBrightness,
            ASI_CONTROL_TYPE_ASI_HARDWARE_BIN => Self::HardwareBin,
            ASI_CONTROL_TYPE_ASI_HIGH_SPEED_MODE => Self::HighSpeedMode,
            ASI_CONTROL_TYPE_ASI_COOLER_POWER_PERC => Self::CoolerPowerPerc,
            ASI_CONTROL_TYPE_ASI_TARGET_TEMP => Self::TargetTemp,
            ASI_CONTROL_TYPE_ASI_COOLER_ON => Self::CoolerOn,
            ASI_CONTROL_TYPE_ASI_MONO_BIN => Self::MonoBin,
            ASI_CONTROL_TYPE_ASI_FAN_ON => Self::FanOn,
            ASI_CONTROL_TYPE_ASI_PATTERN_ADJUST => Self::PatternAdjust,
            ASI_CONTROL_TYPE_ASI_ANTI_DEW_HEATER => Self::AntiDewHeater,
            _ => panic!("Unknown control type code. {}", control_type),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ASIControlCaps {
    pub name: String,
    pub description: String,
    pub max_value: i32,
    pub min_value: i32,
    pub default_value: i32,
    pub is_auto_supported: bool,
    pub is_writable: bool,
    pub control_type: ASIControlType,
}

impl ASIControlCaps {
    pub fn from_raw(raw: ASI_CONTROL_CAPS) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: chars_to_string(&raw.Name)?,
            description: chars_to_string(&raw.Description)?,
            max_value: raw.MaxValue,
            min_value: raw.MinValue,
            default_value: raw.DefaultValue,
            is_auto_supported: ASIBool::from_raw(raw.IsAutoSupported).to_bool(),
            is_writable: ASIBool::from_raw(raw.IsWritable).to_bool(),
            control_type: ASIControlType::from_raw(raw.ControlType),
        })
    }

    pub fn to_raw(&self) -> ASI_CONTROL_CAPS {
        let mut name = [0 as ::std::os::raw::c_char; 64];
        name[..self.name.len()].copy_from_slice(bytes_to_chars(self.name.as_bytes()));
        let mut description = [0 as ::std::os::raw::c_char; 128];
        description[..self.description.len()]
            .copy_from_slice(bytes_to_chars(self.description.as_bytes()));
        ASI_CONTROL_CAPS {
            Name: name,
            Description: description,
            MaxValue: self.max_value as ::std::os::raw::c_long,
            MinValue: self.min_value as ::std::os::raw::c_long,
            DefaultValue: self.default_value as ::std::os::raw::c_long,
            IsAutoSupported: ASIBool::from_bool(self.is_auto_supported) as ASI_BOOL,
            IsWritable: ASIBool::from_bool(self.is_writable) as ASI_BOOL,
            ControlType: self.control_type as ASI_CONTROL_TYPE,
            Unused: [0 as ::std::os::raw::c_char; 32usize], //pad
        }
    }
}

impl Default for ASIControlCaps {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            max_value: Default::default(),
            min_value: Default::default(),
            default_value: Default::default(),
            is_auto_supported: Default::default(),
            is_writable: Default::default(),
            control_type: ASIControlType::Gain,
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ASIExposureStatus {
    Idle = ASI_EXPOSURE_STATUS_ASI_EXP_IDLE,
    Working = ASI_EXPOSURE_STATUS_ASI_EXP_WORKING,
    Success = ASI_EXPOSURE_STATUS_ASI_EXP_SUCCESS,
    Failed = ASI_EXPOSURE_STATUS_ASI_EXP_FAILED,
}

impl ASIExposureStatus {
    pub fn from_raw(exposure_status: ASI_EXPOSURE_STATUS) -> Self {
        match exposure_status {
            ASI_EXPOSURE_STATUS_ASI_EXP_IDLE => Self::Idle,
            ASI_EXPOSURE_STATUS_ASI_EXP_WORKING => Self::Working,
            ASI_EXPOSURE_STATUS_ASI_EXP_SUCCESS => Self::Success,
            ASI_EXPOSURE_STATUS_ASI_EXP_FAILED => Self::Failed,
            _ => panic!("Unknown exposure status code. {}", exposure_status),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
pub struct ASIID {
    pub id: [u8; 8],
}

impl ASIID {
    pub fn from_raw(raw: ASI_ID) -> Self {
        Self { id: raw.id }
    }

    pub fn to_raw(self) -> ASI_ID {
        ASI_ID { id: self.id }
    }
}

#[derive(Debug, Clone)]
pub struct ASISupportedMode {
    pub supported_camera_mode: Vec<ASICameraMode>,
}

impl ASISupportedMode {
    pub fn from_raw(raw: ASI_SUPPORTED_MODE) -> Self {
        let mut ret = Self {
            supported_camera_mode: Vec::with_capacity(16),
        };

        for cam_mode in raw.SupportedCameraMode {
            if cam_mode == ASICameraMode::End as i32 {
                break;
            }
            ret.supported_camera_mode
                .push(ASICameraMode::from_raw(cam_mode));
        }
        ret
    }

    pub fn to_raw(&self) -> ASI_SUPPORTED_MODE {
        let mut supported_camera_mode = [0; 16];
        let slice_raw: Vec<ASI_CAMERA_MODE> = self
            .supported_camera_mode
            .iter()
            .map(|x| *x as i32)
            .collect();
        supported_camera_mode[..self.supported_camera_mode.len()].copy_from_slice(&slice_raw);
        ASI_SUPPORTED_MODE {
            SupportedCameraMode: supported_camera_mode,
        }
    }
}

/// This should be the first API to be called.
/// Get number of connected ASI cameras.
pub fn get_num_of_connected_cameras() -> i32 {
    unsafe { ASIGetNumOfConnectedCameras() }
}

/// Check if the device is ASI Camera.
pub fn camera_check(i_vid: i32, i_pid: i32) -> bool {
    unsafe { ASIBool::from_raw(ASICameraCheck(i_vid, i_pid)).to_bool() }
}

/// Get the property of the connected cameras, you can do this without open the camera.
pub fn get_camera_property(index: i32) -> Result<ASICameraInfo, Box<dyn Error>> {
    let mut info_raw = ASICameraInfo::default().to_raw();
    unsafe { ASIError::from_raw(ASIGetCameraProperty(&mut info_raw, index)) }?;
    ASICameraInfo::from_raw(info_raw)
}

/// Get the property of the connected cameras by ID.
pub fn get_camera_property_by_id(id: i32) -> Result<ASICameraInfo, Box<dyn Error>> {
    let mut info_raw = ASICameraInfo::default().to_raw();
    unsafe { ASIError::from_raw(ASIGetCameraPropertyByID(id, &mut info_raw)) }?;
    ASICameraInfo::from_raw(info_raw)
}

/// Open the camera before any operation to the camera, this will not affect the camera which is capturing.
pub fn open_camera(id: i32) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIOpenCamera(id)) }
}

/// Initialise the camera after open, this function may take some while, this will affect the camera which is capturing.
pub fn init_camera(id: i32) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIInitCamera(id)) }
}

/// You need to close the camera to free all the resource.
pub fn close_camera(id: i32) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASICloseCamera(id)) }
}

/// Get number of controls available for this camera. the camera need be opened at first.
pub fn get_num_of_controls(id: i32) -> Result<i32, ASIError> {
    let mut num = 0;
    unsafe { ASIError::from_raw(ASIGetNumOfControls(id, &mut num)) }?;
    Ok(num)
}

/// Get controls property available for this camera. the camera need be opened at first.
/// user need to malloc and maintain the buffer.
pub fn get_control_caps(id: i32, control_index: i32) -> Result<ASIControlCaps, Box<dyn Error>> {
    let mut control_cap_raw = ASIControlCaps::default().to_raw();
    unsafe { ASIError::from_raw(ASIGetControlCaps(id, control_index, &mut control_cap_raw)) }?;
    ASIControlCaps::from_raw(control_cap_raw)
}

/// Get controls property value and auto value
/// note:the value of the temperature is the float value * 10 to convert it to long type, control name is \"Temperature\"
/// because long is the only type for control(except cooler's target temperature, because it is an integer)
pub fn get_control_value(id: i32, control_type: ASIControlType) -> Result<(i32, bool), ASIError> {
    let mut pl_value = 0;
    let mut pb_auto = 0;
    unsafe {
        ASIError::from_raw(ASIGetControlValue(
            id,
            control_type as i32,
            &mut pl_value,
            &mut pb_auto,
        ))
    }?;
    Ok((pl_value, ASIBool::from_raw(pb_auto).to_bool()))
}

/// Set the ROI area before capture.
/// You must stop capture before call it.
/// The width and height is the value after binning.
/// ie. you need to set width to 640 and height to 480 if you want to run at 640X480@BIN2
/// Specially, ASI120's data size must be times of 1024 which means width*height%1024=0.
pub fn set_roi_format(
    id: i32,
    i_width: i32,
    i_height: i32,
    i_bin: i32,
    image_type: ASIImageType,
) -> Result<(), ASIError> {
    unsafe {
        ASIError::from_raw(ASISetROIFormat(
            id,
            i_width,
            i_height,
            i_bin,
            image_type as i32,
        ))
    }
}

/// Get the current ROI area setting .
pub fn get_roi_format(id: i32) -> Result<(i32, i32, i32, ASIImageType), ASIError> {
    let mut i_width = 0;
    let mut i_height = 0;
    let mut i_bin = 0;
    let mut img_type_raw = 0;
    unsafe {
        ASIError::from_raw(ASIGetROIFormat(
            id,
            &mut i_width,
            &mut i_height,
            &mut i_bin,
            &mut img_type_raw,
        ))
    }?;
    Ok((
        i_width,
        i_height,
        i_bin,
        ASIImageType::from_raw(img_type_raw),
    ))
}

/// Set the start position of the ROI area.
/// you can call this API to move the ROI area when video is streaming.
/// the camera will set the ROI area to the center of the full image as default.
/// at bin2 or bin3 mode, the position is relative to the image after binning.
pub fn set_start_pos(id: i32, i_start_x: i32, i_start_y: i32) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASISetStartPos(id, i_start_x, i_start_y)) }
}

/// Get the start position of current ROI area.
pub fn get_start_pos(id: i32) -> Result<(i32, i32), ASIError> {
    let mut start_x = 0;
    let mut start_y = 0;
    unsafe { ASIError::from_raw(ASIGetStartPos(id, &mut start_x, &mut start_y)) }?;
    Ok((start_x, start_y))
}

/// Get the droped frames .
pub fn get_dropped_frames(id: i32) -> Result<i32, ASIError> {
    let mut count = 0;
    unsafe { ASIError::from_raw(ASIGetDroppedFrames(id, &mut count)) }?;
    Ok(count)
}

/// provide a dark file's path to the function and enable dark subtract
/// this is used when there is hot pixel or need to do long exposure
/// you'd better make this dark file from the  \"dark subtract\" funtion
/// of the \"video capture filter\" directshow page.
/// the dark file's size should be the same of camera's max width and height
/// and should be RGB8 raw format.it will on even you changed the ROI setting
/// it only correct the hot pixels if out put isn't 16bit.
/// it will be remembered in registry. so \"Dark subtract\" is on next time if you close your app.
pub fn enable_dark_subtract(id: i32, path: &str) -> Result<(), ASIError> {
    let mut path_buf = vec![0u8; path.len() + 1];
    path_buf[..path.len()].copy_from_slice(path.as_bytes());
    unsafe {
        ASIError::from_raw(ASIEnableDarkSubtract(
            id,
            std::mem::transmute(path_buf.as_mut_ptr()),
        ))
    }
}

/// Disable the dark subtract function.
/// you'd better call it at start if you don't want to use it.
/// because dark subtract function is remembered on windows platform
pub fn disable_dark_subtract(id: i32) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIDisableDarkSubtract(id)) }
}

/// Start video capture
/// then you can get the data from the API ASIGetVideoData
pub fn start_video_capture(id: i32) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIStartVideoCapture(id)) }
}

/// Stop video capture
pub fn stop_video_capture(id: i32) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIStopVideoCapture(id)) }
}

/// get data from the video buffer.the buffer is very small
/// you need to call this API as fast as possible, otherwise frame will be discarded
/// so the best way is maintain one buffer loop and call this API in a loop
/// please make sure the buffer size is biger enough to hold one image
/// otherwise the this API will crash
pub fn get_video_data(id: i32, mut buffer: Vec<u8>, waitms: i32) -> Result<Vec<u8>, ASIError> {
    unsafe {
        ASIError::from_raw(ASIGetVideoData(
            id,
            buffer.as_mut_ptr(),
            buffer.len() as ::std::os::raw::c_long,
            waitms,
        ))?;
        Ok(buffer)
    }
}

/// PulseGuide of the ST4 port on. this function only work on the module which have ST4 port
pub fn pulse_guide_on(id: i32, direction: ASIGuideDirection) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIPulseGuideOn(id, direction as i32)) }
}

/// PulseGuide of the ST4 port off. this function only work on the module which have ST4 port
/// make sure where is ASIPulseGuideOn and there is ASIPulseGuideOff
pub fn pulse_guide_off(id: i32, direction: ASIGuideDirection) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIPulseGuideOff(id, direction as i32)) }
}

/// Start camera exposure. the following 4 API is usually used when long exposure required
/// start exposure  and check the exposure status then get the data
pub fn start_exposure(id: i32, is_dark: bool) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIStartExposure(id, ASIBool::from_bool(is_dark) as i32)) }
}

/// to cancel the long exposure which is on.
pub fn stop_exposure(id: i32) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASIStopExposure(id)) }
}

/// to get the exposure status, work with ASIStartExposure.
/// you can read the data if get ASI_EXP_SUCCESS. or have to restart exposure again
/// if get ASI_EXP_FAILED
pub fn get_exp_status(id: i32) -> Result<ASIExposureStatus, ASIError> {
    let mut stat_raw = 0;
    unsafe { ASIError::from_raw(ASIGetExpStatus(id, &mut stat_raw)) }?;
    Ok(ASIExposureStatus::from_raw(stat_raw))
}

/// get data after exposure.
/// please make sure the buffer size is biger enough to hold one image
/// otherwise the this API will crash
pub fn get_data_after_exp(id: i32, mut buffer: Vec<u8>) -> Result<Vec<u8>, ASIError> {
    unsafe {
        ASIError::from_raw(ASIGetDataAfterExp(
            id,
            buffer.as_mut_ptr(),
            buffer.len() as ::std::os::raw::c_long,
        ))?;
        Ok(buffer)
    }
}

/// get camera id stored in flash, only available for USB3.0 camera
pub fn get_id(id: i32) -> Result<ASIID, ASIError> {
    let mut raw_id = ASI_ID { id: [0u8; 8] };
    unsafe { ASIError::from_raw(ASIGetID(id, &mut raw_id)) }?;
    Ok(ASIID::from_raw(raw_id))
}

/// write camera id to flash, only available for USB3.0 camera
pub fn set_id(id: i32, new_id: ASIID) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASISetID(id, new_id.to_raw())) }
}

/// get pre-setting parameter
pub fn get_gain_offset(id: i32) -> Result<(i32, i32, i32, i32), ASIError> {
    let mut highest_dr = 0;
    let mut unity_gain = 0;
    let mut gain_lowest_rn = 0;
    let mut offset_lowest_rn = 0;
    unsafe {
        ASIError::from_raw(ASIGetGainOffset(
            id,
            &mut highest_dr,
            &mut unity_gain,
            &mut gain_lowest_rn,
            &mut offset_lowest_rn,
        ))
    }?;
    Ok((highest_dr, unity_gain, gain_lowest_rn, offset_lowest_rn))
}

/// get the frequently-used gain and offset
pub fn get_lmh_gain_offset(id: i32) -> Result<(i32, i32, i32, i32), ASIError> {
    let mut l_gain = 0;
    let mut m_gain = 0;
    let mut h_gain = 0;
    let mut h_offset = 0;
    unsafe {
        ASIError::from_raw(ASIGetLMHGainOffset(
            id,
            &mut l_gain,
            &mut m_gain,
            &mut h_gain,
            &mut h_offset,
        ))
    }?;
    Ok((l_gain, m_gain, h_gain, h_offset))
}

/// get version string, like \"1, 13, 0503\"
pub fn get_sdk_version() -> Result<String, Box<dyn Error>> {
    Ok(unsafe {
        let raw_char = ASIGetSDKVersion();
        CStr::from_ptr(raw_char).to_str()
    }?
    .to_owned())
}

/// Get the camera supported mode, only need to call when the IsTriggerCam in the CameraInfo is true.
pub fn get_camera_support_mode(id: i32) -> Result<ASISupportedMode, ASIError> {
    let mut raw_supported_mode = ASI_SUPPORTED_MODE {
        SupportedCameraMode: [0; 16usize],
    };
    unsafe { ASIError::from_raw(ASIGetCameraSupportMode(id, &mut raw_supported_mode)) }?;
    Ok(ASISupportedMode::from_raw(raw_supported_mode))
}

/// Get the camera current mode, only need to call when the IsTriggerCam in the CameraInfo is true
pub fn get_camera_mode(id: i32) -> Result<ASICameraMode, ASIError> {
    let mut raw_mode = 0;
    unsafe { ASIError::from_raw(ASIGetCameraMode(id, &mut raw_mode)) }?;
    Ok(ASICameraMode::from_raw(raw_mode))
}

/// Set the camera mode, only need to call when the IsTriggerCam in the CameraInfo is true
pub fn set_camera_mode(id: i32, mode: ASICameraMode) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASISetCameraMode(id, mode as i32)) }
}

/// Send out a softTrigger. For edge trigger, it only need to set true which means send a
/// rising trigger to start exposure. For level trigger, it need to set true first means
/// start exposure, and set false means stop exposure.it only need to call when the
/// IsTriggerCam in the CameraInfo is true
pub fn send_soft_trigger(id: i32, start: bool) -> Result<(), ASIError> {
    unsafe { ASIError::from_raw(ASISendSoftTrigger(id, ASIBool::from_bool(start) as i32)) }
}

/// Get a serial number from a camera.
/// It is 8 ASCII characters, you need to print it in hexadecimal.
pub fn get_serial_number(id: i32) -> Result<ASIID, ASIError> {
    let mut asiid_raw = ASI_ID { id: [0u8; 8] };
    unsafe { ASIError::from_raw(ASIGetSerialNumber(id, &mut asiid_raw)) }?;
    Ok(ASIID::from_raw(asiid_raw))
}

/// Config the output pin (A or B) of Trigger port. If lDuration <= 0, this output pin will be closed.
/// Only need to call when the IsTriggerCam in the CameraInfo is true
pub fn set_trigger_output_io_conf(
    id: i32,
    pin: ASITrigOutput,
    pin_high: bool,
    delay: i32,
    duration: i32,
) -> Result<(), ASIError> {
    unsafe {
        ASIError::from_raw(ASISetTriggerOutputIOConf(
            id,
            pin as i32,
            ASIBool::from_bool(pin_high) as i32,
            delay as ::std::os::raw::c_long,
            duration as ::std::os::raw::c_long,
        ))
    }
}

/// Get the output pin configuration, only need to call when the IsTriggerCam in the CameraInfo is true
pub fn get_trigger_output_io_conf(
    id: i32,
    pin: ASITrigOutput,
) -> Result<(bool, i32, i32), ASIError> {
    let mut pin_high_raw = 0;
    let mut delay = 0;
    let mut duration = 0;
    unsafe {
        ASIError::from_raw(ASIGetTriggerOutputIOConf(
            id,
            pin as i32,
            &mut pin_high_raw,
            &mut delay,
            &mut duration,
        ))
    }?;
    Ok((
        ASIBool::from_raw(pin_high_raw).to_bool(),
        delay as ::std::os::raw::c_int,
        duration as ::std::os::raw::c_int,
    ))
}
