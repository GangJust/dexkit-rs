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
}
