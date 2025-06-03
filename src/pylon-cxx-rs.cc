#if defined(FEATURE_STREAM_LINUX)
#include <unistd.h>
#include <fcntl.h>
#endif
#include <memory>
#include "pylon/PylonIncludes.h"
#include "pylon-cxx-rs.h"

std::unique_ptr<std::vector<std::string>> to_std_vec_str(const Pylon::StringList_t& names)
{
    auto result = std::make_unique<std::vector<std::string>>();

    for (Pylon::StringList_t::const_iterator it = names.begin(); it != names.end(); ++it)
    {
        result->push_back(std::string(it->c_str()));
    }

    return result;
}

namespace Pylon
{
    EGrabStrategy convert_grab_strategy(GrabStrategy strategy)
    {
        EGrabStrategy es = GrabStrategy_OneByOne;
        if (strategy == GrabStrategy::OneByOne)
        {
            es = GrabStrategy_OneByOne;
        }
        else if (strategy == GrabStrategy::LatestImageOnly)
        {
            es = GrabStrategy_LatestImageOnly;
        }
        else if (strategy == GrabStrategy::LatestImages)
        {
            es = GrabStrategy_LatestImages;
        }
        else if (strategy == GrabStrategy::UpcomingImage)
        {
            es = GrabStrategy_UpcomingImage;
        }
        else
        {
            throw std::exception();
        }
        return es;
    }

    std::unique_ptr<CInstantCamera> tl_factory_create_first_device()
    {
        // Create an instant camera object with the camera device found first.
        return std::make_unique<CInstantCamera>(CTlFactory::GetInstance().CreateFirstDevice());
    }

    std::unique_ptr<CInstantCamera> tl_factory_create_device(const CDeviceInfo &device_info)
    {
        return std::make_unique<CInstantCamera>(CTlFactory::GetInstance().CreateDevice(device_info));
    }

    std::unique_ptr<std::vector<CDeviceInfo>> tl_factory_enumerate_devices()
    {
        Pylon::DeviceInfoList_t devices;

        CTlFactory::GetInstance().EnumerateDevices(devices);

        auto result = std::make_unique<std::vector<CDeviceInfo>>();

        for (Pylon::DeviceInfoList_t::iterator it = devices.begin(); it != devices.end(); ++it)
        {
            result->push_back(Pylon::CDeviceInfo(*it)); // make copy
        }
        return result;
    }

    std::unique_ptr<CDeviceInfo> instant_camera_get_device_info(const std::unique_ptr<CInstantCamera> &camera)
    {
        // According to InstantCamera.h, `GetDeviceInfo()` does not throw C++ exceptions.
        return std::make_unique<CDeviceInfo>(camera->GetDeviceInfo());
    }

    void instant_camera_open(const std::unique_ptr<CInstantCamera> &camera)
    {
        camera->Open();
    }

    bool instant_camera_is_open(const std::unique_ptr<CInstantCamera> &camera)
    {
        return camera->IsOpen();
    }

    void instant_camera_close(const std::unique_ptr<CInstantCamera> &camera)
    {
        camera->Close();
    }

    const MyNodeMap& instant_camera_get_node_map(const std::unique_ptr<CInstantCamera> &camera)
    {
        return camera->GetNodeMap();
    }

    const MyNodeMap& instant_camera_get_tl_node_map(const std::unique_ptr<CInstantCamera> &camera)
    {
        return camera->GetTLNodeMap();
    }

    const MyNodeMap& instant_camera_get_stream_grabber_node_map(const std::unique_ptr<CInstantCamera> &camera)
    {
        return camera->GetStreamGrabberNodeMap();
    }

    const MyNodeMap& instant_camera_get_event_grabber_node_map(const std::unique_ptr<CInstantCamera> &camera)
    {
        return camera->GetEventGrabberNodeMap();
    }

    const MyNodeMap& instant_camera_get_instant_camera_node_map(const std::unique_ptr<CInstantCamera> &camera)
    {
        return camera->GetInstantCameraNodeMap();
    }

    void node_map_load(const MyNodeMap& node_map, rust::String filename, bool validate)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        const char *filename_c = filename.c_str();
        CFeaturePersistence::Load(filename_c, &nodemap, validate);
    }

    void node_map_save(const MyNodeMap& node_map, rust::String filename)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        const char *filename_c = filename.c_str();
        CFeaturePersistence::Save(filename_c, &nodemap);
    }

    void node_map_load_from_string(const MyNodeMap& node_map, rust::String features, bool validate)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        const char *features_c = features.c_str();
        CFeaturePersistence::LoadFromString(features_c, &nodemap, validate);
    }

    rust::String node_map_save_to_string(const MyNodeMap& node_map)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        Pylon::String_t result;
        CFeaturePersistence::SaveToString(result, &nodemap);
        return rust::String(result.c_str(), result.length());
    }

    void instant_camera_start_grabbing(const std::unique_ptr<CInstantCamera> &camera)
    {
        camera->StartGrabbing();
    }

    void instant_camera_start_grabbing_with_strategy(const std::unique_ptr<CInstantCamera> &camera, GrabStrategy strategy)
    {
        camera->StartGrabbing(convert_grab_strategy(strategy));
    }

    void instant_camera_start_grabbing_with_count(const std::unique_ptr<CInstantCamera> &camera, uint32_t count)
    {
        camera->StartGrabbing(count);
    }

    void instant_camera_start_grabbing_with_count_and_strategy(const std::unique_ptr<CInstantCamera> &camera, uint32_t count, GrabStrategy strategy)
    {
        camera->StartGrabbing(count, convert_grab_strategy(strategy));
    }

    void instant_camera_stop_grabbing(const std::unique_ptr<CInstantCamera> &camera)
    {
        camera->StopGrabbing();
    }

    bool instant_camera_is_grabbing(const std::unique_ptr<CInstantCamera> &camera)
    {
        // According to InstantCamera.h, `IsGrabbing()` does not throw C++ exceptions.
        return camera->IsGrabbing();
    }

    #if defined(FEATURE_STREAM_LINUX)
    int instant_camera_wait_object_fd(const std::unique_ptr<CInstantCamera> &camera) {
      return camera->GetGrabResultWaitObject().GetFd();
    }
    #endif

    #if defined(FEATURE_STREAM_WINDOWS)
    std::unique_ptr<WaitObject> instant_camera_wait_object(const std::unique_ptr<CInstantCamera> &camera) {
      return std::make_unique<WaitObject>(camera->GetGrabResultWaitObject());
    }
    #endif

    bool instant_camera_retrieve_result(const std::unique_ptr<CInstantCamera> &camera, uint32_t timeout, std::unique_ptr<CGrabResultPtr> &result, TimeoutHandling timeout_handling)
    {
        ETimeoutHandling eth;

        if (timeout_handling == TimeoutHandling::ThrowException)
        {
            eth = TimeoutHandling_ThrowException;
        }
        else if (timeout_handling == TimeoutHandling::Return)
        {
            eth = TimeoutHandling_Return;
        }
        else
        {
            throw std::exception();
        }
        return camera->RetrieveResult(timeout, *result, eth);
    }

    std::unique_ptr<CBooleanParameter> node_map_get_boolean_parameter(const MyNodeMap& node_map, rust::Str c_name)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        Pylon::String_t name = Pylon::String_t(c_name.data(), c_name.length());
        return std::make_unique<CBooleanParameter>(CBooleanParameter(nodemap, name));
    }

    std::unique_ptr<CIntegerParameter> node_map_get_integer_parameter(const MyNodeMap& node_map, rust::Str c_name)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        Pylon::String_t name = Pylon::String_t(c_name.data(), c_name.length());
        return std::make_unique<CIntegerParameter>(CIntegerParameter(nodemap, name));
    }

    std::unique_ptr<CFloatParameter> node_map_get_float_parameter(const MyNodeMap& node_map, rust::Str c_name)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        Pylon::String_t name = Pylon::String_t(c_name.data(), c_name.length());
        return std::make_unique<CFloatParameter>(CFloatParameter(nodemap, name));
    }

    std::unique_ptr<CEnumParameter> node_map_get_enum_parameter(const MyNodeMap& node_map, rust::Str c_name)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        Pylon::String_t name = Pylon::String_t(c_name.data(), c_name.length());
        return std::make_unique<CEnumParameter>(CEnumParameter(nodemap, name));
    }

    std::unique_ptr<CCommandParameter> node_map_get_command_parameter(const MyNodeMap& node_map, rust::Str c_name)
    {
        GenApi::INodeMap& nodemap = (GenApi::INodeMap&)node_map;
        Pylon::String_t name = Pylon::String_t(c_name.data(), c_name.length());
        return std::make_unique<CCommandParameter>(CCommandParameter(nodemap, name));
    }

    bool boolean_node_get_value(const std::unique_ptr<CBooleanParameter> &node)
    {
        return node->GetValue();
    }

    void boolean_node_set_value(const std::unique_ptr<CBooleanParameter> &boolean_node, bool value)
    {
        boolean_node->SetValue(value);
    }

    std::unique_ptr<std::string> integer_node_get_unit(const std::unique_ptr<CIntegerParameter> &node)
    {
        return std::make_unique<std::string>(node->GetUnit());
    }

    int64_t integer_node_get_value(const std::unique_ptr<CIntegerParameter> &node)
    {
        return node->GetValue();
    }

    int64_t integer_node_get_min(const std::unique_ptr<CIntegerParameter> &node)
    {
        return node->GetMin();
    }

    int64_t integer_node_get_max(const std::unique_ptr<CIntegerParameter> &node)
    {
        return node->GetMax();
    }

    void integer_node_set_value(const std::unique_ptr<CIntegerParameter> &node, int64_t value)
    {
        node->SetValue(value);
    }

    std::unique_ptr<std::string> float_node_get_unit(const std::unique_ptr<CFloatParameter> &node)
    {
        return std::make_unique<std::string>(node->GetUnit());
    }

    double float_node_get_value(const std::unique_ptr<CFloatParameter> &node)
    {
        return node->GetValue();
    }

    double float_node_get_min(const std::unique_ptr<CFloatParameter> &node)
    {
        return node->GetMin();
    }

    double float_node_get_max(const std::unique_ptr<CFloatParameter> &node)
    {
        return node->GetMax();
    }

    void float_node_set_value(const std::unique_ptr<CFloatParameter> &node, double value)
    {
        node->SetValue(value);
    }

    std::unique_ptr<std::string> enum_node_get_value(const std::unique_ptr<CEnumParameter> &node)
    {
        return std::make_unique<std::string>(node->GetValue());
    }

    std::unique_ptr<std::vector<std::string>> enum_node_settable_values(const std::unique_ptr<CEnumParameter> &enum_node)
    {
        Pylon::StringList_t names;
        enum_node->GetSettableValues(names);
        return to_std_vec_str(names);
    }

    void enum_node_set_value(const std::unique_ptr<CEnumParameter> &enum_node, rust::Str c_value)
    {
        Pylon::String_t value = Pylon::String_t(c_value.data(), c_value.length());
        enum_node->SetValue(value);
    }

    void command_node_execute(const std::unique_ptr<CCommandParameter> &command_node, bool verify)
    {
        command_node->Execute(verify);
    }

    // CGrabResultPtr
    std::unique_ptr<CGrabResultPtr> new_grab_result_ptr()
    {
        return std::make_unique<CGrabResultPtr>();
    }

    bool grab_result_grab_succeeded(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GrabSucceeded();
    }

    rust::String grab_result_error_description(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        // This copies the data.
        return rust::String((*grab_result)->GetErrorDescription());
    }

    uint32_t grab_result_error_code(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetErrorCode();
    }

    /*
        /// Get the current payload type.
        EPayloadType GetPayloadType() const;


        /// Get the current pixel type.
        EPixelType GetPixelType() const;*

    // questions:
    // what is return value (bool) of GetStride? (Why is the pattern of, e.g. width, used?)
    // what is the difference between GetImageSize and GetBufferSize?
    // what is payload size?

        */

    uint32_t grab_result_width(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetWidth();
    }

    uint32_t grab_result_height(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetHeight();
    }

    uint32_t grab_result_offset_x(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetOffsetX();
    }

    uint32_t grab_result_offset_y(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetOffsetY();
    }

    uint32_t grab_result_padding_x(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetPaddingX();
    }

    uint32_t grab_result_padding_y(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetPaddingY();
    }

    rust::Slice<const uint8_t> grab_result_buffer(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        auto buf = (*grab_result)->GetBuffer();
        auto sz = (*grab_result)->GetImageSize();

        return rust::Slice<const uint8_t>(reinterpret_cast<const uint8_t *>(buf),
                                          sz);
    }

    uint32_t grab_result_payload_size(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetPayloadSize();
    }

    uint32_t grab_result_buffer_size(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetImageSize();
    }

    uint64_t grab_result_block_id(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetBlockID();
    }

    uint64_t grab_result_time_stamp(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetTimeStamp();
    }

    size_t grab_result_stride(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        size_t result;
        bool hmm = (*grab_result)->GetStride(result);
        return result;
    }

    uint32_t grab_result_image_size(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetImageSize();
    }

    const MyNodeMap& grab_result_get_chunk_data_node_map(const std::unique_ptr<CGrabResultPtr> &grab_result)
    {
        return (*grab_result)->GetChunkDataNodeMap();
    }

    std::unique_ptr<CDeviceInfo> device_info_copy(const CDeviceInfo &device_info)
    {
        return std::make_unique<CDeviceInfo>(device_info);
    }

    std::unique_ptr<std::vector<std::string>> device_info_get_property_names(const std::unique_ptr<CDeviceInfo> &device_info)
    {

        Pylon::StringList_t names;
        device_info->GetPropertyNames(names);
        return to_std_vec_str(names);
    }

    rust::String device_info_get_property_value(const std::unique_ptr<CDeviceInfo> &device_info, rust::Str c_name)
    {

        Pylon::String_t result;
        Pylon::String_t name = Pylon::String_t(c_name.data(), c_name.length());

        bool ok = device_info->GetPropertyValue(name, result);
        if (!ok)
        {
            throw std::exception();
        }

        return rust::String(result.c_str(), result.length());
    }

    rust::String device_info_get_model_name(const std::unique_ptr<CDeviceInfo> &device_info)
    {
        // This copies the data.
        return rust::String(device_info->GetModelName());
    }

    #if defined(FEATURE_STREAM_WINDOWS)
    bool wait_object_wait(const std::unique_ptr<WaitObject> &wait_object, uint64_t timeout) {
        return wait_object->Wait(timeout);
    }
    #endif

} // namespace Pylon
