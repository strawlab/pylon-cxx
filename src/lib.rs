#![cfg_attr(feature = "backtrace", feature(error_generic_member_access))]

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

#[cfg(feature = "stream")]
pub mod stream;

#[cfg(feature = "stream")]
use std::cell::RefCell;

#[derive(Debug)]
pub struct PylonError {
    msg: String,
    #[cfg(feature = "backtrace")]
    backtrace: Backtrace,
}

impl From<cxx::Exception> for PylonError {
    fn from(orig: cxx::Exception) -> PylonError {
        PylonError {
            msg: orig.what().into(),
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<std::str::Utf8Error> for PylonError {
    fn from(_: std::str::Utf8Error) -> PylonError {
        PylonError {
            msg: "Cannot convert C++ string to UTF-8".to_string(),
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<std::io::Error> for PylonError {
    fn from(orig: std::io::Error) -> PylonError {
        PylonError {
            msg: orig.to_string(),
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
        }
    }
}

impl std::fmt::Display for PylonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PylonError({})", self.msg)
    }
}

impl std::error::Error for PylonError {
    #[cfg(feature = "backtrace")]
    fn provide<'a>(&'a self, req: &mut std::error::Request<'a>) {
        req.provide_ref::<std::backtrace::Backtrace>(&self.backtrace);
    }
}

pub type PylonResult<T> = Result<T, PylonError>;

#[cxx::bridge(namespace = Pylon)]
mod ffi {
    #[repr(u32)]
    enum TimeoutHandling {
        Return,
        ThrowException,
    }
    #[repr(u32)]
    enum GrabStrategy {
        OneByOne,
        LatestImageOnly,
        LatestImages,
        UpcomingImage,
    }

    unsafe extern "C++" {
        include!("pylon/PylonIncludes.h");
        include!("pylon/gige/BaslerGigECamera.h");
        include!("catcher.h");
        include!("pylon-cxx-rs.h");

        type CInstantCamera;
        type CDeviceInfo;
        type CGrabResultPtr;
        type TimeoutHandling;
        type GrabStrategy;
        type CBooleanParameter;
        type CIntegerParameter;
        type CFloatParameter;
        type CEnumParameter;
        type CCommandParameter;

        type MyNodeMap;

        fn PylonInitialize();
        fn PylonTerminate(ShutDownLogging: bool);
        unsafe fn GetPylonVersion(
            major: *mut u32,
            minor: *mut u32,
            subminor: *mut u32,
            build: *mut u32,
        );

        fn tl_factory_create_first_device() -> Result<UniquePtr<CInstantCamera>>;
        fn tl_factory_create_device(device_info: &CDeviceInfo)
            -> Result<UniquePtr<CInstantCamera>>;
        fn tl_factory_enumerate_devices() -> Result<UniquePtr<CxxVector<CDeviceInfo>>>;

        fn instant_camera_get_device_info(
            camera: &UniquePtr<CInstantCamera>,
        ) -> UniquePtr<CDeviceInfo>;
        fn instant_camera_open(camera: &UniquePtr<CInstantCamera>) -> Result<()>;
        fn instant_camera_is_open(camera: &UniquePtr<CInstantCamera>) -> Result<bool>;
        fn instant_camera_close(camera: &UniquePtr<CInstantCamera>) -> Result<()>;
        fn instant_camera_start_grabbing(camera: &UniquePtr<CInstantCamera>) -> Result<()>;
        fn instant_camera_stop_grabbing(camera: &UniquePtr<CInstantCamera>) -> Result<()>;
        fn instant_camera_start_grabbing_with_strategy(
            camera: &UniquePtr<CInstantCamera>,
            grab_strategy: GrabStrategy,
        ) -> Result<()>;
        fn instant_camera_start_grabbing_with_count(
            camera: &UniquePtr<CInstantCamera>,
            count: u32,
        ) -> Result<()>;
        fn instant_camera_start_grabbing_with_count_and_strategy(
            camera: &UniquePtr<CInstantCamera>,
            count: u32,
            grab_strategy: GrabStrategy,
        ) -> Result<()>;
        fn instant_camera_is_grabbing(camera: &UniquePtr<CInstantCamera>) -> bool;
        #[cfg(feature = "stream")]
        fn instant_camera_wait_object_fd(camera: &UniquePtr<CInstantCamera>) -> i32;
        fn instant_camera_retrieve_result(
            camera: &UniquePtr<CInstantCamera>,
            timeout_ms: u32,
            grab_result: &mut UniquePtr<CGrabResultPtr>,
            timeout_handling: TimeoutHandling,
        ) -> Result<bool>;

        fn instant_camera_get_node_map(camera: &UniquePtr<CInstantCamera>) -> Result<&MyNodeMap>;
        fn instant_camera_get_tl_node_map(camera: &UniquePtr<CInstantCamera>)
            -> Result<&MyNodeMap>;
        fn instant_camera_get_stream_grabber_node_map(
            camera: &UniquePtr<CInstantCamera>,
        ) -> Result<&MyNodeMap>;
        fn instant_camera_get_event_grabber_node_map(
            camera: &UniquePtr<CInstantCamera>,
        ) -> Result<&MyNodeMap>;
        fn instant_camera_get_instant_camera_node_map(
            camera: &UniquePtr<CInstantCamera>,
        ) -> Result<&MyNodeMap>;

        fn node_map_load(node_map: &MyNodeMap, filename: String, validate: bool) -> Result<()>;
        fn node_map_save(node_map: &MyNodeMap, filename: String) -> Result<()>;
        fn node_map_load_from_string(
            node_map: &MyNodeMap,
            features: String,
            validate: bool,
        ) -> Result<()>;
        fn node_map_save_to_string(node_map: &MyNodeMap) -> Result<String>;

        fn node_map_get_boolean_parameter(
            node_map: &MyNodeMap,
            name: &str,
        ) -> Result<UniquePtr<CBooleanParameter>>;
        fn node_map_get_integer_parameter(
            node_map: &MyNodeMap,
            name: &str,
        ) -> Result<UniquePtr<CIntegerParameter>>;
        fn node_map_get_float_parameter(
            node_map: &MyNodeMap,
            name: &str,
        ) -> Result<UniquePtr<CFloatParameter>>;
        fn node_map_get_enum_parameter(
            node_map: &MyNodeMap,
            name: &str,
        ) -> Result<UniquePtr<CEnumParameter>>;

        fn node_map_get_command_parameter(
            node_map: &MyNodeMap,
            name: &str,
        ) -> Result<UniquePtr<CCommandParameter>>;

        fn boolean_node_get_value(boolean_node: &UniquePtr<CBooleanParameter>) -> Result<bool>;
        fn boolean_node_set_value(
            boolean_node: &UniquePtr<CBooleanParameter>,
            value: bool,
        ) -> Result<()>;

        fn integer_node_get_unit(
            node: &UniquePtr<CIntegerParameter>,
        ) -> Result<UniquePtr<CxxString>>;
        fn integer_node_get_value(node: &UniquePtr<CIntegerParameter>) -> Result<i64>;
        fn integer_node_get_min(node: &UniquePtr<CIntegerParameter>) -> Result<i64>;
        fn integer_node_get_max(node: &UniquePtr<CIntegerParameter>) -> Result<i64>;
        fn integer_node_set_value(node: &UniquePtr<CIntegerParameter>, value: i64) -> Result<()>;

        fn float_node_get_unit(node: &UniquePtr<CFloatParameter>) -> Result<UniquePtr<CxxString>>;
        fn float_node_get_value(node: &UniquePtr<CFloatParameter>) -> Result<f64>;
        fn float_node_get_min(node: &UniquePtr<CFloatParameter>) -> Result<f64>;
        fn float_node_get_max(node: &UniquePtr<CFloatParameter>) -> Result<f64>;
        fn float_node_set_value(node: &UniquePtr<CFloatParameter>, value: f64) -> Result<()>;

        fn enum_node_get_value(node: &UniquePtr<CEnumParameter>) -> Result<UniquePtr<CxxString>>;
        fn enum_node_settable_values(
            enum_node: &UniquePtr<CEnumParameter>,
        ) -> Result<UniquePtr<CxxVector<CxxString>>>;
        fn enum_node_set_value(enum_node: &UniquePtr<CEnumParameter>, value: &str) -> Result<()>;

        fn command_node_execute(node: &UniquePtr<CCommandParameter>, verify: bool) -> Result<()>;

        fn new_grab_result_ptr() -> Result<UniquePtr<CGrabResultPtr>>;
        fn grab_result_grab_succeeded(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<bool>;
        fn grab_result_error_description(grab_result: &UniquePtr<CGrabResultPtr>)
            -> Result<String>;
        fn grab_result_error_code(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_width(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_height(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_offset_x(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_offset_y(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_padding_x(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_padding_y(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_buffer(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<&[u8]>;
        fn grab_result_payload_size(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_buffer_size(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;
        fn grab_result_block_id(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u64>;
        fn grab_result_time_stamp(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u64>;
        fn grab_result_stride(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<usize>;
        fn grab_result_image_size(grab_result: &UniquePtr<CGrabResultPtr>) -> Result<u32>;

        fn device_info_copy(device_info: &CDeviceInfo) -> UniquePtr<CDeviceInfo>;
        fn device_info_get_property_names(
            device_info: &UniquePtr<CDeviceInfo>,
        ) -> Result<UniquePtr<CxxVector<CxxString>>>;
        fn device_info_get_property_value(
            device_info: &UniquePtr<CDeviceInfo>,
            name: &str,
        ) -> Result<String>;
        fn device_info_get_model_name(device_info: &UniquePtr<CDeviceInfo>) -> Result<String>;
    }
}
pub use ffi::GrabStrategy;
pub use ffi::TimeoutHandling;

pub struct Pylon {}

impl Pylon {
    pub fn new() -> Self {
        ffi::PylonInitialize();
        Self {}
    }
}

impl Default for Pylon {
    fn default() -> Self {
        ffi::PylonInitialize();
        Self {}
    }
}

impl Drop for Pylon {
    fn drop(&mut self) {
        ffi::PylonTerminate(true);
    }
}

#[derive(Debug)]
pub struct PylonVersion {
    pub major: u32,
    pub minor: u32,
    pub subminor: u32,
    pub build: u32,
}

pub fn pylon_version() -> PylonVersion {
    let mut major = 0;
    let mut minor = 0;
    let mut subminor = 0;
    let mut build = 0;
    unsafe {
        ffi::GetPylonVersion(&mut major, &mut minor, &mut subminor, &mut build);
    }
    PylonVersion {
        major,
        minor,
        subminor,
        build,
    }
}

/// Terminate the Pylon library.
///
/// # Safety
///
/// You should prefer dropping the [Pylon] instance instead. This is unsafe
/// because the API cannot guarantee the Pylon library has been instantiated
/// exactly once and will not be terminated again.
pub unsafe fn terminate(shutdown_logging: bool) {
    ffi::PylonTerminate(shutdown_logging);
}

/// Wrap the CTlFactory type
// Since in C++ `CTlFactory::GetInstance()` merely returns a reference to
// a static object, here we don't store anything and instead get the
// reference when needed.
pub struct TlFactory<'a> {
    lib: &'a Pylon,
}

impl<'a> TlFactory<'a> {
    pub fn instance(lib: &'a Pylon) -> Self {
        Self { lib }
    }
    pub fn create_first_device(&self) -> PylonResult<InstantCamera<'a>> {
        let inner = ffi::tl_factory_create_first_device()?;
        Ok(InstantCamera::new(self.lib, inner))
    }
    pub fn create_device(&self, device_info: &DeviceInfo) -> PylonResult<InstantCamera<'a>> {
        let inner = ffi::tl_factory_create_device(&device_info.inner)?;
        Ok(InstantCamera::new(self.lib, inner))
    }
    pub fn enumerate_devices(&self) -> PylonResult<Vec<DeviceInfo>> {
        let devs: cxx::UniquePtr<cxx::CxxVector<ffi::CDeviceInfo>> =
            ffi::tl_factory_enumerate_devices()?;
        Ok(devs
            .into_iter()
            .map(|cdev: &ffi::CDeviceInfo| DeviceInfo {
                inner: ffi::device_info_copy(cdev),
            })
            .collect())
    }
}

/// Wrap the CInstantCamera type
pub struct InstantCamera<'a> {
    #[allow(dead_code)]
    lib: &'a Pylon,
    inner: cxx::UniquePtr<ffi::CInstantCamera>,
    #[cfg(feature = "stream")]
    fd: RefCell<Option<tokio::io::unix::AsyncFd<std::os::unix::io::RawFd>>>,
}

/// Wrap the `GenApi::INodeMap` type.
///
/// This provides access to the various nodes (boolean, integer, float, enum,
/// and command nodes). Also allows loading all node values from and saving all
/// values to either a file or a [String].
///
/// The `'parent` lifetime refers to the object, such as an [`InstantCamera`],
/// from which the node map is generated. The reference to the nodemap itself
/// has the `'map` lifetime. The `'parent` lifetime lives at least as long as
/// the `'map` lifetime.
pub struct NodeMap<'map, 'parent: 'map> {
    inner: &'map ffi::MyNodeMap,
    parent: std::marker::PhantomData<&'parent u8>,
}

impl<'map, 'parent: 'map> NodeMap<'map, 'parent> {
    /// Load all values from the file at `path` into the nodemap.
    pub fn load<P: AsRef<std::path::Path>>(&self, path: P, validate: bool) -> PylonResult<()> {
        let filename = path_to_string(path)?;
        ffi::node_map_load(self.inner, filename, validate).into_rust()
    }
    /// Save all values to file at `path` from the nodemap.
    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> PylonResult<()> {
        let filename = path_to_string(path)?;
        ffi::node_map_save(self.inner, filename).into_rust()
    }
    /// Load all values from the `features` string into the nodemap.
    pub fn load_from_string(&self, features: String, validate: bool) -> PylonResult<()> {
        ffi::node_map_load_from_string(self.inner, features, validate).into_rust()
    }
    /// Save all values to [String] from the nodemap.
    pub fn save_to_string(&self) -> PylonResult<String> {
        ffi::node_map_save_to_string(self.inner).into_rust()
    }
    pub fn boolean_node(&self, name: &str) -> PylonResult<BooleanNode> {
        let name = name.to_string();
        let inner = ffi::node_map_get_boolean_parameter(self.inner, &name)?;
        Ok(BooleanNode { name, inner })
    }
    pub fn integer_node(&self, name: &str) -> PylonResult<IntegerNode> {
        let name = name.to_string();
        let inner = ffi::node_map_get_integer_parameter(self.inner, &name)?;
        Ok(IntegerNode { name, inner })
    }
    pub fn float_node(&self, name: &str) -> PylonResult<FloatNode> {
        let name = name.to_string();
        let inner = ffi::node_map_get_float_parameter(self.inner, &name)?;
        Ok(FloatNode { name, inner })
    }
    pub fn enum_node(&self, name: &str) -> PylonResult<EnumNode> {
        let name = name.to_string();
        let inner = ffi::node_map_get_enum_parameter(self.inner, &name)?;
        Ok(EnumNode { name, inner })
    }
    pub fn command_node(&self, name: &str) -> PylonResult<CommandNode> {
        let name = name.to_string();
        let inner = ffi::node_map_get_command_parameter(self.inner, &name)?;
        Ok(CommandNode { name, inner })
    }
}

/// Options passed to `start_grabbing`.
#[derive(Default)]
pub struct GrabOptions {
    count: Option<u32>,
    strategy: Option<GrabStrategy>,
}

impl GrabOptions {
    pub fn count(self, count: u32) -> GrabOptions {
        Self {
            count: Some(count),
            ..self
        }
    }

    pub fn strategy(self, strategy: GrabStrategy) -> GrabOptions {
        Self {
            strategy: Some(strategy),
            ..self
        }
    }
}

pub struct BooleanNode {
    name: String,
    inner: cxx::UniquePtr<ffi::CBooleanParameter>,
}

impl BooleanNode {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> PylonResult<bool> {
        ffi::boolean_node_get_value(&self.inner).into_rust()
    }

    pub fn set_value(&mut self, value: bool) -> PylonResult<()> {
        ffi::boolean_node_set_value(&self.inner, value).into_rust()
    }
}

pub struct IntegerNode {
    name: String,
    inner: cxx::UniquePtr<ffi::CIntegerParameter>,
}

impl IntegerNode {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn unit(&self) -> PylonResult<String> {
        let cstr = ffi::integer_node_get_unit(&self.inner)?;
        Ok(cstr.to_str()?.to_string())
    }

    pub fn value(&self) -> PylonResult<i64> {
        ffi::integer_node_get_value(&self.inner).into_rust()
    }

    pub fn min(&self) -> PylonResult<i64> {
        ffi::integer_node_get_min(&self.inner).into_rust()
    }

    pub fn max(&self) -> PylonResult<i64> {
        ffi::integer_node_get_max(&self.inner).into_rust()
    }

    pub fn set_value(&mut self, value: i64) -> PylonResult<()> {
        ffi::integer_node_set_value(&self.inner, value).into_rust()
    }
}

pub struct FloatNode {
    name: String,
    inner: cxx::UniquePtr<ffi::CFloatParameter>,
}

impl FloatNode {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn unit(&self) -> PylonResult<String> {
        let cstr = ffi::float_node_get_unit(&self.inner)?;
        Ok(cstr.to_str()?.to_string())
    }

    pub fn value(&self) -> PylonResult<f64> {
        ffi::float_node_get_value(&self.inner).into_rust()
    }

    pub fn min(&self) -> PylonResult<f64> {
        ffi::float_node_get_min(&self.inner).into_rust()
    }

    pub fn max(&self) -> PylonResult<f64> {
        ffi::float_node_get_max(&self.inner).into_rust()
    }

    pub fn set_value(&mut self, value: f64) -> PylonResult<()> {
        ffi::float_node_set_value(&self.inner, value).into_rust()
    }
}

pub struct EnumNode {
    name: String,
    inner: cxx::UniquePtr<ffi::CEnumParameter>,
}

impl EnumNode {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> PylonResult<String> {
        let cstr = ffi::enum_node_get_value(&self.inner)?;
        Ok(cstr.to_str()?.to_string())
    }
    pub fn settable_values(&self) -> PylonResult<Vec<String>> {
        ffi::enum_node_settable_values(&self.inner)?.into_rust()
    }
    pub fn set_value(&mut self, value: &str) -> PylonResult<()> {
        ffi::enum_node_set_value(&self.inner, value).into_rust()
    }
}

pub struct CommandNode {
    name: String,
    inner: cxx::UniquePtr<ffi::CCommandParameter>,
}

impl CommandNode {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn execute(&self, verify: bool) -> PylonResult<()> {
        ffi::command_node_execute(&self.inner, verify).into_rust()
    }
}

unsafe impl<'a> Send for InstantCamera<'a> {}

impl<'a> InstantCamera<'a> {
    pub fn new(lib: &'a Pylon, inner: cxx::UniquePtr<ffi::CInstantCamera>) -> Self {
        InstantCamera {
            lib,
            inner,
            #[cfg(feature = "stream")]
            fd: RefCell::new(None),
        }
    }

    pub fn device_info(&self) -> DeviceInfo {
        // According to InstantCamera.h, `GetDeviceInfo()` does not throw C++ exceptions.
        let di = ffi::instant_camera_get_device_info(&self.inner);
        DeviceInfo { inner: di }
    }

    pub fn open(&self) -> PylonResult<()> {
        ffi::instant_camera_open(&self.inner).into_rust()
    }

    pub fn is_open(&self) -> PylonResult<bool> {
        ffi::instant_camera_is_open(&self.inner).into_rust()
    }

    pub fn close(&self) -> PylonResult<()> {
        ffi::instant_camera_close(&self.inner).into_rust()
    }

    pub fn start_grabbing(&self, options: &GrabOptions) -> PylonResult<()> {
        // we assign the waitobject fd here for using it in the stream to poll for progress
        #[cfg(feature = "stream")]
        {
            if tokio::runtime::Handle::try_current().is_ok() {
                self.fd.replace(Some(tokio::io::unix::AsyncFd::new(
                    self.get_grab_result_fd()?,
                )?));
            }
        }

        match (options.count, options.strategy) {
            (Some(count), Some(strategy)) => {
                ffi::instant_camera_start_grabbing_with_count_and_strategy(
                    &self.inner,
                    count,
                    strategy,
                )
                .into_rust()
            }
            (Some(count), None) => {
                ffi::instant_camera_start_grabbing_with_count(&self.inner, count).into_rust()
            }
            (None, Some(strategy)) => {
                ffi::instant_camera_start_grabbing_with_strategy(&self.inner, strategy).into_rust()
            }
            (None, None) => ffi::instant_camera_start_grabbing(&self.inner).into_rust(),
        }
    }

    pub fn stop_grabbing(&self) -> PylonResult<()> {
        ffi::instant_camera_stop_grabbing(&self.inner).into_rust()?;
        #[cfg(feature = "stream")]
        self.fd.replace(None);
        Ok(())
    }

    pub fn is_grabbing(&self) -> bool {
        // According to InstantCamera.h, `IsGrabbing()` does not throw C++ exceptions.
        ffi::instant_camera_is_grabbing(&self.inner)
    }

    pub fn retrieve_result(
        &self,
        timeout_ms: u32,
        grab_result: &mut GrabResult,
        timeout_handling: TimeoutHandling,
    ) -> PylonResult<bool> {
        ffi::instant_camera_retrieve_result(
            &self.inner,
            timeout_ms,
            &mut grab_result.inner,
            timeout_handling,
        )
        .into_rust()
    }

    #[cfg(feature = "stream")]
    pub fn get_grab_result_fd(&self) -> PylonResult<std::os::unix::io::RawFd> {
        Ok(ffi::instant_camera_wait_object_fd(&self.inner))
    }
}

/// These methods return the various node maps.
impl<'a> InstantCamera<'a> {
    pub fn node_map<'map>(&'a self) -> PylonResult<NodeMap<'map, 'a>> {
        Ok(NodeMap {
            inner: ffi::instant_camera_get_node_map(&self.inner)?,
            parent: std::marker::PhantomData,
        })
    }
    pub fn tl_node_map<'map>(&'a self) -> PylonResult<NodeMap<'map, 'a>> {
        Ok(NodeMap {
            inner: ffi::instant_camera_get_tl_node_map(&self.inner)?,
            parent: std::marker::PhantomData,
        })
    }
    pub fn stream_grabber_node_map<'map>(&'a self) -> PylonResult<NodeMap<'map, 'a>> {
        Ok(NodeMap {
            inner: ffi::instant_camera_get_stream_grabber_node_map(&self.inner)?,
            parent: std::marker::PhantomData,
        })
    }
    pub fn event_grabber_node_map<'map>(&'a self) -> PylonResult<NodeMap<'map, 'a>> {
        Ok(NodeMap {
            inner: ffi::instant_camera_get_event_grabber_node_map(&self.inner)?,
            parent: std::marker::PhantomData,
        })
    }
    pub fn instant_camera_node_map<'map>(&'a self) -> PylonResult<NodeMap<'map, 'a>> {
        Ok(NodeMap {
            inner: ffi::instant_camera_get_instant_camera_node_map(&self.inner)?,
            parent: std::marker::PhantomData,
        })
    }
}

pub struct GrabResult {
    inner: cxx::UniquePtr<ffi::CGrabResultPtr>,
}

unsafe impl Send for GrabResult {}

impl GrabResult {
    pub fn new() -> PylonResult<Self> {
        Ok(Self {
            inner: ffi::new_grab_result_ptr()?,
        })
    }

    pub fn grab_succeeded(&self) -> PylonResult<bool> {
        ffi::grab_result_grab_succeeded(&self.inner).into_rust()
    }

    pub fn error_description(&self) -> PylonResult<String> {
        ffi::grab_result_error_description(&self.inner).into_rust()
    }

    pub fn error_code(&self) -> PylonResult<u32> {
        ffi::grab_result_error_code(&self.inner).into_rust()
    }

    pub fn width(&self) -> PylonResult<u32> {
        ffi::grab_result_width(&self.inner).into_rust()
    }

    pub fn height(&self) -> PylonResult<u32> {
        ffi::grab_result_height(&self.inner).into_rust()
    }

    pub fn offset_x(&self) -> PylonResult<u32> {
        ffi::grab_result_offset_x(&self.inner).into_rust()
    }

    pub fn offset_y(&self) -> PylonResult<u32> {
        ffi::grab_result_offset_y(&self.inner).into_rust()
    }

    pub fn padding_x(&self) -> PylonResult<u32> {
        ffi::grab_result_padding_x(&self.inner).into_rust()
    }

    pub fn padding_y(&self) -> PylonResult<u32> {
        ffi::grab_result_padding_y(&self.inner).into_rust()
    }

    pub fn buffer(&self) -> PylonResult<&[u8]> {
        ffi::grab_result_buffer(&self.inner).into_rust()
    }

    pub fn payload_size(&self) -> PylonResult<u32> {
        ffi::grab_result_payload_size(&self.inner).into_rust()
    }

    pub fn buffer_size(&self) -> PylonResult<u32> {
        ffi::grab_result_buffer_size(&self.inner).into_rust()
    }

    pub fn block_id(&self) -> PylonResult<u64> {
        ffi::grab_result_block_id(&self.inner).into_rust()
    }

    pub fn time_stamp(&self) -> PylonResult<u64> {
        ffi::grab_result_time_stamp(&self.inner).into_rust()
    }

    pub fn stride(&self) -> PylonResult<usize> {
        // ffi::grab_result_stride(&self.inner).into_rust()
        ffi::grab_result_stride(&self.inner).into_rust()
    }

    pub fn image_size(&self) -> PylonResult<u32> {
        ffi::grab_result_image_size(&self.inner).into_rust()
    }
}

trait CxxResultExt {
    type RustResult;
    fn into_rust(self) -> Self::RustResult;
}

impl CxxResultExt for cxx::UniquePtr<cxx::CxxVector<cxx::CxxString>> {
    type RustResult = PylonResult<Vec<String>>;
    fn into_rust(self) -> Self::RustResult {
        // This needs to return a Result (and cannot move the data, but rather
        // copy) because we need to ensure the strings are correct UTF8.
        Ok(self
            .into_iter()
            .map(|name| name.to_str().map(String::from))
            .collect::<Result<_, std::str::Utf8Error>>()?)
    }
}

impl<T> CxxResultExt for Result<T, cxx::Exception> {
    type RustResult = PylonResult<T>;
    fn into_rust(self) -> Self::RustResult {
        self.map_err(PylonError::from)
    }
}

// ---------------------------
// HasProperties trait

pub trait HasProperties {
    fn property_names(&self) -> PylonResult<Vec<String>>;
    fn property_value(&self, name: &str) -> PylonResult<String>;
}

impl HasProperties for DeviceInfo {
    fn property_names(&self) -> PylonResult<Vec<String>> {
        ffi::device_info_get_property_names(&self.inner)?.into_rust()
    }

    fn property_value(&self, name: &str) -> PylonResult<String> {
        Ok(ffi::device_info_get_property_value(&self.inner, name)?)
    }
}

impl DeviceInfo {
    pub fn model_name(&self) -> PylonResult<String> {
        ffi::device_info_get_model_name(&self.inner).into_rust()
    }
}

pub struct DeviceInfo {
    inner: cxx::UniquePtr<ffi::CDeviceInfo>,
}

impl Clone for DeviceInfo {
    fn clone(&self) -> DeviceInfo {
        DeviceInfo {
            inner: ffi::device_info_copy(&self.inner),
        }
    }
}

unsafe impl Send for DeviceInfo {}

fn path_to_string<P: AsRef<std::path::Path>>(path: P) -> PylonResult<String> {
    match path.as_ref().to_str() {
        Some(filename) => Ok(filename.into()),
        None => Err(PylonError {
            msg: "Cannot convert path to UTF-8".to_string(),
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
        }),
    }
}
