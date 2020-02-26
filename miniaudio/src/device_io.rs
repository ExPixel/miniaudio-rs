use crate::base::*;
use crate::resampling::ResampleAlgorithm;
use miniaudio_sys as sys;
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::ptr;
use std::ptr::NonNull;
use std::rc::Rc;

type MADeviceConfigResampling = sys::ma_device_config__bindgen_ty_1;
type MADeviceConfigPlayback = sys::ma_device_config__bindgen_ty_2;
type MADeviceConfigCapture = sys::ma_device_config__bindgen_ty_3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    WASAPI = sys::ma_backend_wasapi as _,
    DSound = sys::ma_backend_dsound as _,
    WinMM = sys::ma_backend_winmm as _,
    CoreAudio = sys::ma_backend_coreaudio as _,
    SNDIO = sys::ma_backend_sndio as _,
    Audio4 = sys::ma_backend_audio4 as _,
    OSS = sys::ma_backend_oss as _,
    PulseAudio = sys::ma_backend_pulseaudio as _,
    ALSA = sys::ma_backend_alsa as _,
    Jack = sys::ma_backend_jack as _,
    AAudio = sys::ma_backend_aaudio as _,
    OpenSL = sys::ma_backend_opensl as _,
    WebAudio = sys::ma_backend_webaudio as _,
    Null = sys::ma_backend_null as _,
}
impl_from_c!(Backend, sys::ma_backend);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadPriority {
    Idle = sys::ma_thread_priority_idle as _,
    Lowest = sys::ma_thread_priority_lowest as _,
    Low = sys::ma_thread_priority_low as _,
    Normal = sys::ma_thread_priority_normal as _,
    High = sys::ma_thread_priority_high as _,
    Highest = sys::ma_thread_priority_highest as _,
    Realtime = sys::ma_thread_priority_realtime as _,
}
impl_from_c!(ThreadPriority, sys::ma_thread_priority);

impl ThreadPriority {
    pub const DEFAULT: ThreadPriority = ThreadPriority::Highest;
}

impl Default for ThreadPriority {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Playback = sys::ma_device_type_playback as _,
    Capture = sys::ma_device_type_capture as _,
    Duplex = sys::ma_device_type_duplex as _,
    Loopback = sys::ma_device_type_loopback as _,
}
impl_from_c!(DeviceType, sys::ma_device_type);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareMode {
    Shared = sys::ma_share_mode_shared as _,
    Exclusive = sys::ma_share_mode_exclusive as _,
}
impl_from_c!(ShareMode, sys::ma_share_mode);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOSSessionCategory {
    Default = sys::ma_ios_session_category_default as _,
    None = sys::ma_ios_session_category_none as _,
    Ambient = sys::ma_ios_session_category_ambient as _,
    SoloAmbient = sys::ma_ios_session_category_solo_ambient as _,
    Playback = sys::ma_ios_session_category_playback as _,
    Record = sys::ma_ios_session_category_record as _,
    PlayAndRecord = sys::ma_ios_session_category_play_and_record as _,
    MultiRoute = sys::ma_ios_session_category_multi_route as _,
}
impl_from_c!(IOSSessionCategory, sys::ma_ios_session_category);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOSSessionCategoryOption {
    MixWithOthers = sys::ma_ios_session_category_option_mix_with_others as _,
    DuckOthers = sys::ma_ios_session_category_option_duck_others as _,
    AllowBluetooth = sys::ma_ios_session_category_option_allow_bluetooth as _,
    DefaultToSpeaker = sys::ma_ios_session_category_option_default_to_speaker as _,
    InterruptSpokenAudioAndMixWithOthers =
        sys::ma_ios_session_category_option_interrupt_spoken_audio_and_mix_with_others as _,
    AllowBluetoothA2DP = sys::ma_ios_session_category_option_allow_bluetooth_a2dp as _,
    AllowAirPlay = sys::ma_ios_session_category_option_allow_air_play as _,
}
impl_from_c!(
    IOSSessionCategoryOption,
    sys::ma_ios_session_category_option
);

// FIXME probably going to want a way to compare these???
#[repr(transparent)]
pub struct DeviceId(sys::ma_device_id);

#[repr(transparent)]
pub struct DeviceInfo(sys::ma_device_info);

impl DeviceInfo {
    #[inline]
    pub fn id(&self) -> DeviceId {
        unsafe { std::mem::transmute(self.0.id) }
    }

    #[inline]
    pub fn name(&self) -> &str {
        let cstr = unsafe { std::ffi::CStr::from_ptr(self.0.name.as_ptr()) };

        // #FIXME at the moment we just return a blank string instead of invalid UTF-8
        cstr.to_str().unwrap_or("")
    }

    #[inline]
    pub fn format_count(&self) -> u32 {
        self.0.formatCount
    }

    #[inline]
    pub fn formats(&self) -> [Format; sys::ma_format_count as usize] {
        unsafe { std::mem::transmute(self.0.formats) }
    }

    #[inline]
    pub fn min_channels(&self) -> u32 {
        self.0.minChannels
    }

    #[inline]
    pub fn max_channels(&self) -> u32 {
        self.0.maxChannels
    }

    #[inline]
    pub fn min_sample_rate(&self) -> u32 {
        self.0.minSampleRate
    }

    #[inline]
    pub fn max_sample_rate(&self) -> u32 {
        self.0.maxSampleRate
    }
}

#[repr(transparent)]
pub struct DeviceConfig(sys::ma_device_config);

impl DeviceConfig {
    pub fn new(device_type: DeviceType) -> DeviceConfig {
        DeviceConfig(unsafe { sys::ma_device_config_init(device_type as _) })
    }

    pub fn device_type(&self) -> DeviceType {
        DeviceType::from_c(self.0.deviceType)
    }

    pub fn set_device_type(&mut self, device_type: DeviceType) {
        self.0.deviceType = device_type as _;
    }

    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.0.sampleRate = sample_rate
    }

    pub fn period_size_in_frames(&self) -> u32 {
        self.0.periodSizeInFrames
    }

    pub fn set_period_size_in_frames(&mut self, size: u32) {
        self.0.periodSizeInFrames = size;
    }

    pub fn period_size_in_milliseconds(&self) -> u32 {
        self.0.periodSizeInMilliseconds
    }

    pub fn set_period_size_in_milliseconds(&mut self, size: u32) {
        self.0.periodSizeInMilliseconds = size;
    }

    pub fn periods(&self) -> u32 {
        self.0.periods
    }

    pub fn set_periods(&mut self, periods: u32) {
        self.0.periods = periods
    }

    pub fn peformance_profile(&self) -> PerformanceProfile {
        PerformanceProfile::from_c(self.0.performanceProfile)
    }

    pub fn set_performance_profile(&mut self, profile: PerformanceProfile) {
        self.0.performanceProfile = profile as _
    }

    pub fn no_pre_zeroed_output_buffer(&self) -> bool {
        from_bool32(self.0.noPreZeroedOutputBuffer)
    }

    pub fn set_no_pre_zeroed_output_buffer(&mut self, value: bool) {
        self.0.noPreZeroedOutputBuffer = to_bool32(value);
    }

    pub fn no_clip(&self) -> bool {
        from_bool32(self.0.noClip)
    }

    pub fn set_no_clip(&mut self, no_clip: bool) {
        self.0.noClip = to_bool32(no_clip);
    }

    pub fn set_data_callback<F>(&mut self, callback: F)
    where
        F: FnMut(NonNull<Device>, *mut (), *const (), u32) + 'static,
    {
        let user_data = self.ensure_user_data_ptr();
        unsafe {
            (*user_data).data_callback = Some(Box::new(callback));
        }

        // The trampoline will use the user data to call the Rust function.
        self.0.dataCallback = Some(device_callback_proc_trampoline);
    }

    pub fn set_stop_callback<F>(&mut self, callback: F)
    where
        F: FnMut(NonNull<Device>) + 'static,
    {
        let user_data = self.ensure_user_data_ptr();
        unsafe {
            (*user_data).stop_callback = Some(Box::new(callback));
        }

        // The trampoline will use the user data to call the Rust function.
        self.0.stopCallback = Some(stop_proc_trampoline);
    }

    #[inline]
    pub fn playback(&self) -> &DeviceConfigPlayback {
        unsafe { std::mem::transmute(&self.0.playback) }
    }

    #[inline]
    pub fn playback_mut(&mut self) -> &mut DeviceConfigPlayback {
        unsafe { std::mem::transmute(&mut self.0.playback) }
    }

    #[inline]
    pub fn capture(&self) -> &DeviceConfigCapture {
        unsafe { std::mem::transmute(&self.0.capture) }
    }

    #[inline]
    pub fn capture_mut(&mut self) -> &mut DeviceConfigCapture {
        unsafe { std::mem::transmute(&mut self.0.capture) }
    }

    #[inline]
    pub fn resampling(&self) -> &DeviceConfigResampling {
        unsafe { std::mem::transmute(&self.0.resampling) }
    }

    #[inline]
    pub fn resampling_mut(&mut self) -> &mut DeviceConfigResampling {
        unsafe { std::mem::transmute(&mut self.0.resampling) }
    }

    /// Ensures that there is valid user data in here for allowing rust style callbacks.
    fn ensure_user_data_ptr(&mut self) -> *mut DeviceUserData {
        if self.0.pUserData.is_null() {
            self.0.pUserData = Rc::into_raw(Rc::new(DeviceUserData {
                data_callback: None,
                stop_callback: None,
            })) as *mut _;
        }
        self.0.pUserData as *mut DeviceUserData
    }

    /// Will drop our reference to the user data containing the boxed callbacks and then clear the
    /// pointer to it.
    fn drop_user_data(&mut self) {
        if !self.0.pUserData.is_null() {
            let user_data = unsafe { Rc::from_raw(self.0.pUserData as _) };
            self.0.pUserData = ptr::null_mut();
            drop(user_data);
        }
    }

    /// This will return a clone of the user data contained in this
    /// config. This will return None if there is no user data to be cloned.
    #[inline(always)]
    fn clone_user_data(&mut self) -> Option<Rc<DeviceUserData>> {
        if self.0.pUserData.is_null() {
            None
        } else {
            let user_data = unsafe { Rc::from_raw(self.0.pUserData as _) };
            let user_data_clone = Rc::clone(&user_data);
            std::mem::forget(user_data); // we don't want to drop it
            Some(user_data_clone)
        }
    }

    // FIXME implement getters/setters for wasapi/alsa/pulse config structs
}

impl Drop for DeviceConfig {
    fn drop(&mut self) {
        self.drop_user_data();
    }
}

#[repr(transparent)]
pub struct DeviceConfigPlayback(MADeviceConfigPlayback);

impl DeviceConfigPlayback {
    pub fn device_id(&self) -> *mut DeviceId {
        unsafe { std::mem::transmute(self.0.pDeviceID) }
    }

    pub fn set_device_id(&mut self, device_id: *mut DeviceId) {
        self.0.pDeviceID = unsafe { std::mem::transmute(device_id) };
    }

    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    pub fn set_channels(&mut self, channels: u32) {
        self.0.channels = channels;
    }

    pub fn channel_map(&self) -> &[Channel; sys::MA_MAX_CHANNELS as usize] {
        unsafe { std::mem::transmute(&self.0.channelMap) }
    }

    pub fn channel_map_mut(&mut self) -> &mut [Channel; sys::MA_MAX_CHANNELS as usize] {
        unsafe { std::mem::transmute(&mut self.0.channelMap) }
    }

    pub fn share_mode(&self) -> ShareMode {
        ShareMode::from_c(self.0.shareMode)
    }

    pub fn set_share_mode(&mut self, share_mode: ShareMode) {
        self.0.shareMode = share_mode as _
    }
}

#[repr(transparent)]
pub struct DeviceConfigCapture(MADeviceConfigCapture);

impl DeviceConfigCapture {
    pub fn device_id(&self) -> *mut DeviceId {
        unsafe { std::mem::transmute(self.0.pDeviceID) }
    }

    pub fn set_device_id(&mut self, device_id: *mut DeviceId) {
        self.0.pDeviceID = unsafe { std::mem::transmute(device_id) };
    }

    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    pub fn set_channels(&mut self, channels: u32) {
        self.0.channels = channels;
    }

    pub fn channel_map(&self) -> &[Channel; sys::MA_MAX_CHANNELS as usize] {
        unsafe { std::mem::transmute(&self.0.channelMap) }
    }

    pub fn channel_map_mut(&mut self) -> &mut [Channel; sys::MA_MAX_CHANNELS as usize] {
        unsafe { std::mem::transmute(&mut self.0.channelMap) }
    }

    pub fn share_mode(&self) -> ShareMode {
        ShareMode::from_c(self.0.shareMode)
    }

    pub fn set_share_mode(&mut self, share_mode: ShareMode) {
        self.0.shareMode = share_mode as _
    }
}

#[repr(transparent)]
pub struct DeviceConfigResampling(MADeviceConfigResampling);

impl DeviceConfigResampling {
    pub fn algorithm(&self) -> ResampleAlgorithm {
        ResampleAlgorithm::from_c(self.0.algorithm)
    }

    pub fn set_algorithm(&mut self, algorithm: ResampleAlgorithm) {
        self.0.algorithm = algorithm as _;
    }

    pub fn linear_lpf_poles(&self) -> u32 {
        self.0.linear.lpfPoles
    }

    pub fn set_linear_lpf_poles(&mut self, lpf_poles: u32) {
        self.0.linear.lpfPoles = lpf_poles;
    }

    pub fn speex_quality(&self) -> i32 {
        self.0.speex.quality as i32
    }

    pub fn set_speex_quality(&mut self, quality: i32) {
        self.0.speex.quality = quality as _;
    }
}

struct DeviceUserData {
    data_callback: Option<Box<dyn FnMut(NonNull<Device>, *mut (), *const (), u32)>>,
    stop_callback: Option<Box<dyn FnMut(NonNull<Device>)>>,
}

unsafe extern "C" fn device_callback_proc_trampoline(
    device: *mut sys::ma_device,
    output: *mut c_void,
    input: *const c_void,
    frame_count: u32,
) {
    todo!("device_callback_proc_trampoline"); // FIXME
}

unsafe extern "C" fn stop_proc_trampoline(device: *mut sys::ma_device) {
    todo!("stop_proc_trampoline"); // FIXME
}

#[repr(transparent)]
pub struct ContextConfig(sys::ma_context_config);

impl ContextConfig {
    /// Initializes a ContextConfig object.
    pub fn new() -> ContextConfig {
        ContextConfig(unsafe { sys::ma_context_config_init() })
    }
    // FIXME implement some stuff for context config.
    //       I just don't need it at the moement, so...
}

#[repr(transparent)]
pub struct Context(sys::ma_context);

impl Context {
    pub fn alloc(
        backends: Option<&[Backend]>,
        config: Option<&ContextConfig>,
    ) -> Result<Box<Context>, Error> {
        let mut context = Box::new(MaybeUninit::<sys::ma_context>::uninit());

        let result = unsafe {
            sys::ma_context_init(
                backends
                    .map(|b| b.as_ptr() as *const _)
                    .unwrap_or(ptr::null()),
                backends.map(|b| b.len() as u32).unwrap_or(0),
                config.map(|c| &c.0 as *const _).unwrap_or(ptr::null()),
                context.as_mut_ptr(),
            )
        };

        map_result!(result, unsafe { std::mem::transmute(context) })
    }

    pub fn backend(&self) -> Backend {
        Backend::from_c(self.0.backend)
    }

    pub fn set_backend(&mut self, backend: Backend) {
        self.0.backend = backend as _;
    }

    pub fn thread_priority(&self) -> ThreadPriority {
        ThreadPriority::from_c(self.0.threadPriority)
    }

    pub fn set_thread_priority(&mut self, priority: ThreadPriority) {
        self.0.threadPriority = priority as _;
    }

    pub fn device_info_capacity(&self) -> u32 {
        self.0.deviceInfoCapacity
    }

    pub fn playback_device_info_count(&self) -> u32 {
        self.0.playbackDeviceInfoCount
    }

    pub fn capture_device_info_count(&self) -> u32 {
        self.0.captureDeviceInfoCount
    }

    pub fn playback_device_infos(&self) -> &[DeviceInfo] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.pDeviceInfos as _,
                self.0.playbackDeviceInfoCount as usize,
            )
        }
    }

    pub fn capture_device_info(&self) -> &[DeviceInfo] {
        unsafe {
            std::slice::from_raw_parts(
                self.0
                    .pDeviceInfos
                    .add(self.0.playbackDeviceInfoCount as usize) as _,
                self.0.captureDeviceInfoCount as usize,
            )
        }
    }

    pub fn is_backend_asynchronous(&self) -> bool {
        from_bool32(self.0.isBackendAsynchronous())
    }

    /// Retrieves basic information about every active playback and capture device. This function
    /// will allocate memory internally for device lists.
    /// This function will not call the closure if an error occurred.
    pub fn with_devices<F>(&mut self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&[DeviceInfo], &[DeviceInfo]),
    {
        let mut playback_ptr: *mut sys::ma_device_info = ptr::null_mut();
        let mut playback_count: u32 = 0;

        let mut capture_ptr: *mut sys::ma_device_info = ptr::null_mut();
        let mut capture_count = 0;

        unsafe {
            let result = sys::ma_context_get_devices(
                &mut self.0,
                &mut playback_ptr,
                &mut playback_count,
                &mut capture_ptr,
                &mut capture_count,
            );

            if Error::is_c_error(result) {
                return Err(Error::from_c_error(result));
            }

            f(
                std::slice::from_raw_parts(
                    std::mem::transmute::<_, *mut DeviceInfo>(playback_ptr),
                    playback_count as usize,
                ),
                std::slice::from_raw_parts(
                    std::mem::transmute::<_, *mut DeviceInfo>(capture_ptr),
                    capture_count as usize,
                ),
            );
        }

        return Ok(());
    }

    /// Retrieves basic information about every active playback device. This function
    /// will allocate memory internally for device lists.
    /// This function will not call the closure if an error occurred.
    pub fn with_playback_devices<F>(&mut self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&[DeviceInfo]),
    {
        let mut playback_ptr: *mut sys::ma_device_info = ptr::null_mut();
        let mut playback_count: u32 = 0;

        unsafe {
            let result = sys::ma_context_get_devices(
                &mut self.0,
                &mut playback_ptr,
                &mut playback_count,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if Error::is_c_error(result) {
                return Err(Error::from_c_error(result));
            }

            f(std::slice::from_raw_parts(
                std::mem::transmute::<_, *mut DeviceInfo>(playback_ptr),
                playback_count as usize,
            ));
        }

        return Ok(());
    }

    /// Retrieves basic information about every active capture device. This function
    /// will allocate memory internally for device lists.
    /// This function will not call the closure if an error occurred.
    pub fn with_capture_devices<F>(&mut self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&[DeviceInfo]),
    {
        let mut capture_ptr: *mut sys::ma_device_info = ptr::null_mut();
        let mut capture_count = 0;

        unsafe {
            let result = sys::ma_context_get_devices(
                &mut self.0,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut capture_ptr,
                &mut capture_count,
            );

            if Error::is_c_error(result) {
                return Err(Error::from_c_error(result));
            }

            f(std::slice::from_raw_parts(
                std::mem::transmute::<_, *mut DeviceInfo>(capture_ptr),
                capture_count as usize,
            ));
        }

        return Ok(());
    }

    // FIXME implement ma_context_enumerate_devices

    // FIXME implement user data / callbacks
    // See `DeviceConfig` for how to do that.
}

impl Drop for Context {
    fn drop(&mut self) {
        Error::from_c_result(unsafe { sys::ma_context_uninit(&mut self.0) })
            .expect("failed to uninit context");
    }
}

#[repr(transparent)]
pub struct Device(sys::ma_device);

impl Device {
    /// Will drop our reference to the user data containing the boxed callbacks and then clear the
    /// pointer to it.
    fn drop_user_data(&mut self) {
        if !self.0.pUserData.is_null() {
            let user_data = unsafe { Rc::from_raw(self.0.pUserData as _) };
            self.0.pUserData = ptr::null_mut();
            drop(user_data);
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.drop_user_data();
    }
}
