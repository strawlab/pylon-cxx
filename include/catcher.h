#pragma once
#include <string>

#define THROW_RUST(prefix, exc) { \
    std::stringstream ss;\
    ss << (prefix) << exc.what();\
    auto msg = ss.str();\
    fail(msg.c_str());\
}

namespace rust
{
    namespace behavior
    {
        // This must be kept in sync with error.rs. In error.rs, the string
        // generated here will be parsed to the appropriate rust-side error. (A
        // [`cxx::Exception`](https://docs.rs/cxx/*/cxx/struct.Exception.html)
        // will be generated which supports only an error message string rather
        // than custom types, so we encode our error here in a string which is
        // parsed by the code in error.rs.)
        template <typename Try, typename Fail>
        void trycatch(Try &&func, Fail &&fail) noexcept
        try
        {
            func();
        }
        catch (const ::std::exception &e)
        {
            THROW_RUST("std::exception: ", e);
        }
        catch (const Pylon::AccessException &e)
        {
            THROW_RUST("Pylon::AccessException: ", e);
        }
        #ifdef _MSC_VER
        // Pylon::AviWriterFatalException seems only defined in Windows.
        catch (const Pylon::AviWriterFatalException &e)
        {
            THROW_RUST("Pylon::AviWriterFatalException: ", e);
        }
        #endif
        catch (const Pylon::BadAllocException &e)
        {
            THROW_RUST("Pylon::BadAllocException: ", e);
        }
        catch (const Pylon::DynamicCastException &e)
        {
            THROW_RUST("Pylon::DynamicCastException: ", e);
        }
        catch (const Pylon::InvalidArgumentException &e)
        {
            THROW_RUST("Pylon::InvalidArgumentException: ", e);
        }
        catch (const Pylon::LogicalErrorException &e)
        {
            THROW_RUST("Pylon::LogicalErrorException: ", e);
        }
        catch (const Pylon::OutOfRangeException &e)
        {
            THROW_RUST("Pylon::OutOfRangeException: ", e);
        }
        catch (const Pylon::PropertyException &e)
        {
            THROW_RUST("Pylon::PropertyException: ", e);
        }
        catch (const Pylon::RuntimeException &e)
        {
            THROW_RUST("Pylon::RuntimeException: ", e);
        }
        catch (const Pylon::TimeoutException &e)
        {
            THROW_RUST("Pylon::TimeoutException: ", e);
        }
        catch (const Pylon::GenericException &e)
        {
            THROW_RUST("Pylon::GenericException: ", e);
        }
    } // namespace behavior
} // namespace rust
