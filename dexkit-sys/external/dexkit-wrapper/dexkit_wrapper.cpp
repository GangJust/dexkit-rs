#include "dexkit.h"

template <typename T>
const T *From(const void *buf)
{
    return ::flatbuffers::GetRoot<T>(buf);
}

using dexkit::DexKit;
using dexkit::Error;

extern "C"
{
    void *dexkit_new()
    {
        return new dexkit::DexKit();
    }

    void dexkit_free(void *handle)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        delete dexkit;
    }

    int dexkit_add_zip_path(void *handle, const char *apk_path, int unzip_thread_num)
    {
        std::string filePathStr(apk_path);
        auto dexkit = static_cast<DexKit *>(handle);
        auto ret = dexkit->AddZipPath(filePathStr, unzip_thread_num);
        if (ret != Error::SUCCESS)
        {
            // delete dexkit;
            return FALSE;
        }

        return TRUE;
    }

    void dexkit_set_thread_num(void *handle, int thread_num)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        dexkit->SetThreadNum(thread_num);
    }

    int dexkit_init_full_cache(void *handle)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto ret = dexkit->InitFullCache();

        if (ret != Error::SUCCESS)
        {
            return FALSE;
        }

        return TRUE;
    }

    int dexkit_get_dex_num(void *handle)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        return dexkit->GetDexNum();
    }

    int dexkit_export_dex_file(void *handle, const char *out_dir)
    {
        std::string outDirStr(out_dir);
        auto dexkit = static_cast<DexKit *>(handle);
        auto ret = dexkit->ExportDexFile(outDirStr);
        if (ret != Error::SUCCESS)
        {
            return FALSE;
        }
        return TRUE;
    }

    void dexkit_find_class(void *handle, void *buffer, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto query = From<dexkit::schema::FindClass>(buffer);
        auto result = dexkit->FindClass(query);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_find_class_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_find_method(void *handle, void *buffer, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto query = From<dexkit::schema::FindMethod>(buffer);
        auto result = dexkit->FindMethod(query);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_find_method_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_find_field(void *handle, void *buffer, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto query = From<dexkit::schema::FindField>(buffer);
        auto result = dexkit->FindField(query);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_find_field_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_class_by_ids(void *handle, void *encode_id_array, size_t ids_len, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto id_ptr = static_cast<int64_t *>(encode_id_array);
        std::vector<int64_t> ids_vec(id_ptr, id_ptr + ids_len);
        auto result = dexkit->GetClassByIds(ids_vec);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_class_by_ids_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_method_by_ids(void *handle, void *encode_id_array, size_t ids_len, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto id_ptr = static_cast<int64_t *>(encode_id_array);
        std::vector<int64_t> ids_vec(id_ptr, id_ptr + ids_len);
        auto result = dexkit->GetMethodByIds(ids_vec);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_method_by_ids_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_field_by_ids(void *handle, void *encode_id_array, size_t ids_len, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto id_ptr = static_cast<int64_t *>(encode_id_array);
        std::vector<int64_t> ids_vec(id_ptr, id_ptr + ids_len);
        auto result = dexkit->GetFieldByIds(ids_vec);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_field_by_ids_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_class_annotations(void *handle, int64_t encode_class_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetClassAnnotations(encode_class_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_class_annotations_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_field_annotations(void *handle, int64_t encode_field_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetFieldAnnotations(encode_field_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_field_annotations_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_field_get_methods(void *handle, int64_t encode_field_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->FieldGetMethods(encode_field_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_field_get_methods_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_field_put_methods(void *handle, int64_t encode_field_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->FieldPutMethods(encode_field_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_field_put_methods_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_parameter_names(void *handle, int64_t encode_method_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetParameterNames(encode_method_id); // std::optional<std::vector<std::optional<std::string_view>>>
        if (!result.has_value())
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        size_t n = result->size();
        char **arr = static_cast<char **>(malloc(n * sizeof(char *)));
        for (size_t i = 0; i < n; ++i)
        {
            const auto &opt_str = result->at(i);
            if (!opt_str.has_value())
            {
                arr[i] = nullptr;
            }
            else
            {
                const auto &sv = opt_str.value();
                arr[i] = static_cast<char *>(malloc(sv.size() + 1));
                memcpy(arr[i], sv.data(), sv.size());
                arr[i][sv.size()] = '\0';
            }
        }
        *out_buf = arr;
        *out_len = n;
    }
    void dexkit_get_parameter_names_free(char **out_buf, size_t len)
    {
        if (!out_buf)
            return;
        for (size_t i = 0; i < len; ++i)
        {
            if (out_buf[i])
                free(out_buf[i]);
        }
        free(out_buf);
    }

    void dexkit_get_method_annotations(void *handle, int64_t encode_method_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetMethodAnnotations(encode_method_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_method_annotations_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_parameter_annotations(void *handle, int64_t encode_method_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetParameterAnnotations(encode_method_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_parameter_annotations_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_method_op_codes(void *handle, int64_t encode_method_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetMethodOpCodes(encode_method_id); // std::vector<uint8_t>

        if (result.empty())
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        size_t size = result.size();
        uint8_t *buf = static_cast<uint8_t *>(malloc(size));
        if (buf != nullptr)
        {
            memcpy(buf, result.data(), size);
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;
    }
    void dexkit_get_method_op_codes_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_call_methods(void *handle, int64_t encode_method_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetCallMethods(encode_method_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_call_methods_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_invoke_methods(void *handle, int64_t encode_method_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetInvokeMethods(encode_method_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_invoke_methods_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_method_using_strings(void *handle, int64_t encode_method_id, char ***out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetUsingStrings(encode_method_id); // std::vector<std::string_view>

        if (result.empty())
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        size_t n = result.size();
        char **arr = static_cast<char **>(malloc(n * sizeof(char *)));
        for (size_t i = 0; i < n; ++i)
        {
            const auto &sv = result[i];
            arr[i] = static_cast<char *>(malloc(sv.size() + 1));
            memcpy(arr[i], sv.data(), sv.size());
            arr[i][sv.size()] = '\0';
        }
        *out_buf = arr;
        *out_len = n;
    }
    void dexkit_get_method_using_strings_free(char **out_buf, size_t len)
    {
        if (!out_buf)
            return;
        for (size_t i = 0; i < len; ++i)
        {
            if (out_buf[i])
                free(out_buf[i]);
        }
        free(out_buf);
    }

    void dexkit_get_method_using_fields(void *handle, int64_t encode_method_id, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetUsingFields(encode_method_id);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_method_using_fields_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_class_data(void *handle, char *dex_descriptor, void **out_buf, size_t *out_len)
    {
        std::string dexDescriptorStr(dex_descriptor);
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetClassData(dexDescriptorStr);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_class_data_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_method_data(void *handle, char *method_descriptor, void **out_buf, size_t *out_len)
    {
        std::string methodDescriptorStr(method_descriptor);
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetMethodData(methodDescriptorStr);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_method_data_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_get_field_data(void *handle, char *field_descriptor, void **out_buf, size_t *out_len)
    {
        std::string fieldDescriptorStr(field_descriptor);
        auto dexkit = static_cast<DexKit *>(handle);
        auto result = dexkit->GetFieldData(fieldDescriptorStr);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_get_field_data_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_batch_find_class_using_strings(void *handle, void *buffer, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto query = From<dexkit::schema::BatchFindClassUsingStrings>(buffer);
        auto result = dexkit->BatchFindClassUsingStrings(query);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_batch_find_class_using_strings_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }

    void dexkit_batch_find_method_using_strings(void *handle, void *buffer, void **out_buf, size_t *out_len)
    {
        auto dexkit = static_cast<DexKit *>(handle);
        auto query = From<dexkit::schema::BatchFindMethodUsingStrings>(buffer);
        auto result = dexkit->BatchFindMethodUsingStrings(query);
        if (result == nullptr)
        {
            *out_buf = nullptr;
            *out_len = 0;
            return;
        }

        // Copy the buffer to a new memory location
        size_t size = result->GetSize();
        void *buf = nullptr;
        if (size > 0)
        {
            buf = malloc(size);
            if (buf != nullptr)
            {
                memcpy(buf, result->GetBufferPointer(), size);
            }
        }
        *out_buf = buf;
        *out_len = (buf != nullptr) ? size : 0;

        // Release the FlatBufferBuilder to free its internal memory
        result->Release();
    }
    void dexkit_batch_find_method_using_strings_free(void **out_buf, size_t out_len)
    {
        if (!out_buf)
            return;
        free(*out_buf);
        *out_buf = nullptr;
    }
}
