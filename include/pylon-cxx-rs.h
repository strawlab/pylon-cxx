#pragma once
#include "rust/cxx.h"

namespace Pylon
{

    // Due to interaction with CXX and namespaces, I had to put this definition
    // inside the Pylon namespace. Probably there is a better way.
    using MyNodeMap = GenApi::INodeMap;

    enum TimeoutHandling
    {
        Return,
        ThrowException,
    };
    enum GrabStrategy {
        OneByOne,
        LatestImageOnly,
        LatestImages,
        UpcomingImage
    };

    std::unique_ptr<CInstantCamera> tl_factory_create_first_device();
    std::unique_ptr<CInstantCamera> tl_factory_create_device(const CDeviceInfo &);
    std::unique_ptr<std::vector<CDeviceInfo>> tl_factory_enumerate_devices();

    std::unique_ptr<CDeviceInfo> instant_camera_get_device_info(const std::unique_ptr<CInstantCamera> &);
    void instant_camera_open(const std::unique_ptr<CInstantCamera> &);
    bool instant_camera_is_open(const std::unique_ptr<CInstantCamera> &);
    void instant_camera_close(const std::unique_ptr<CInstantCamera> &);

    const MyNodeMap& instant_camera_get_node_map(const std::unique_ptr<CInstantCamera> &);
    const MyNodeMap& instant_camera_get_tl_node_map(const std::unique_ptr<CInstantCamera> &);
    const MyNodeMap& instant_camera_get_stream_grabber_node_map(const std::unique_ptr<CInstantCamera> &);
    const MyNodeMap& instant_camera_get_event_grabber_node_map(const std::unique_ptr<CInstantCamera> &);
    const MyNodeMap& instant_camera_get_instant_camera_node_map(const std::unique_ptr<CInstantCamera> &);

    void node_map_load(const MyNodeMap&, rust::String, bool);
    void node_map_save(const MyNodeMap&, rust::String);

    void node_map_load_from_string(const MyNodeMap&, rust::String, bool);
    rust::String node_map_save_to_string(const MyNodeMap&);

    void instant_camera_start_grabbing(const std::unique_ptr<CInstantCamera> &);
    void instant_camera_start_grabbing_with_strategy(const std::unique_ptr<CInstantCamera> &, GrabStrategy);
    void instant_camera_start_grabbing_with_count(const std::unique_ptr<CInstantCamera> &, uint32_t);
    void instant_camera_start_grabbing_with_count_and_strategy(const std::unique_ptr<CInstantCamera> &, uint32_t, GrabStrategy);
    void instant_camera_stop_grabbing(const std::unique_ptr<CInstantCamera> &);
    bool instant_camera_is_grabbing(const std::unique_ptr<CInstantCamera> &);

    #if defined(FEATURE_STREAM_LINUX)
    int instant_camera_wait_object_fd(const std::unique_ptr<CInstantCamera> &);
    #endif
    #if defined(FEATURE_STREAM_WINDOWS)
    std::unique_ptr<WaitObject> instant_camera_wait_object(const std::unique_ptr<CInstantCamera> &);
    #endif

    bool instant_camera_retrieve_result(const std::unique_ptr<CInstantCamera> &, uint32_t, std::unique_ptr<CGrabResultPtr> &, TimeoutHandling);

    std::unique_ptr<CBooleanParameter> node_map_get_boolean_parameter(const MyNodeMap&, rust::Str);
    std::unique_ptr<CIntegerParameter> node_map_get_integer_parameter(const MyNodeMap&, rust::Str);
    std::unique_ptr<CFloatParameter> node_map_get_float_parameter(const MyNodeMap&, rust::Str);
    std::unique_ptr<CEnumParameter> node_map_get_enum_parameter(const MyNodeMap&, rust::Str);
    std::unique_ptr<CCommandParameter> node_map_get_command_parameter(const MyNodeMap&, rust::Str);

    bool boolean_node_get_value(const std::unique_ptr<CBooleanParameter> &);
    void boolean_node_set_value(const std::unique_ptr<CBooleanParameter> &, bool);

    std::unique_ptr<std::string> integer_node_get_unit(const std::unique_ptr<CIntegerParameter> &);
    int64_t integer_node_get_value(const std::unique_ptr<CIntegerParameter> &);
    int64_t integer_node_get_min(const std::unique_ptr<CIntegerParameter> &);
    int64_t integer_node_get_max(const std::unique_ptr<CIntegerParameter> &);
    void integer_node_set_value(const std::unique_ptr<CIntegerParameter> &, int64_t);

    std::unique_ptr<std::string> float_node_get_unit(const std::unique_ptr<CFloatParameter> &);
    double float_node_get_value(const std::unique_ptr<CFloatParameter> &);
    double float_node_get_min(const std::unique_ptr<CFloatParameter> &);
    double float_node_get_max(const std::unique_ptr<CFloatParameter> &);
    void float_node_set_value(const std::unique_ptr<CFloatParameter> &, double);

    std::unique_ptr<std::string> enum_node_get_value(const std::unique_ptr<CEnumParameter> &);
    std::unique_ptr<std::vector<std::string>> enum_node_settable_values(const std::unique_ptr<CEnumParameter> &);
    void enum_node_set_value(const std::unique_ptr<CEnumParameter> &, rust::Str);

    void command_node_execute(const std::unique_ptr<CCommandParameter> &, bool);

    std::unique_ptr<CGrabResultPtr> new_grab_result_ptr();
    bool grab_result_grab_succeeded(const std::unique_ptr<CGrabResultPtr> &);
    rust::String grab_result_error_description(const std::unique_ptr<CGrabResultPtr> &);
    uint32_t grab_result_error_code(const std::unique_ptr<CGrabResultPtr> &);
    uint32_t grab_result_width(const std::unique_ptr<CGrabResultPtr> &);
    uint32_t grab_result_height(const std::unique_ptr<CGrabResultPtr> &);
    uint32_t grab_result_offset_x(const std::unique_ptr<CGrabResultPtr> &grab_result);
    uint32_t grab_result_offset_y(const std::unique_ptr<CGrabResultPtr> &grab_result);
    uint32_t grab_result_padding_x(const std::unique_ptr<CGrabResultPtr> &grab_result);
    uint32_t grab_result_padding_y(const std::unique_ptr<CGrabResultPtr> &grab_result);
    rust::Slice<const uint8_t> grab_result_buffer(const std::unique_ptr<CGrabResultPtr> &);
    uint32_t grab_result_payload_size(const std::unique_ptr<CGrabResultPtr> &);
    uint32_t grab_result_buffer_size(const std::unique_ptr<CGrabResultPtr> &);

    uint64_t grab_result_block_id(const std::unique_ptr<CGrabResultPtr> &grab_result);
    uint64_t grab_result_time_stamp(const std::unique_ptr<CGrabResultPtr> &grab_result);
    size_t grab_result_stride(const std::unique_ptr<CGrabResultPtr> &grab_result);
    uint32_t grab_result_image_size(const std::unique_ptr<CGrabResultPtr> &grab_result);
    const MyNodeMap& grab_result_get_chunk_data_node_map(const std::unique_ptr<CGrabResultPtr> &grab_result);

    std::unique_ptr<CDeviceInfo> device_info_copy(const CDeviceInfo &);
    std::unique_ptr<std::vector<std::string>> device_info_get_property_names(const std::unique_ptr<CDeviceInfo> &);
    rust::String device_info_get_property_value(const std::unique_ptr<CDeviceInfo> &, rust::Str);
    rust::String device_info_get_model_name(const std::unique_ptr<CDeviceInfo> &);

    #if defined(FEATURE_STREAM_WINDOWS)
    bool wait_object_wait(const std::unique_ptr<WaitObject>& wait_object, uint64_t timeout);
    #endif
} // namespace Pylon
