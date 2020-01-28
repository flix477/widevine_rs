#include <cstring>
#include <iostream>
#include <dlfcn.h>
#include "api.h"

const int VERSION = 10;
const char* KEY_SYSTEM = "com.widevine.alpha";
const int KEY_SYSTEM_LENGTH = std::strlen(KEY_SYSTEM);

Library* GetLibraryHandle() {
  void* handle = dlopen("libwidevinecdm.dylib", RTLD_LAZY);
  if (!handle) {
    return nullptr;
  }

  InitFunc initialize_module = (InitFunc)dlsym(handle, "InitializeCdmModule_4");
  InitFunc deinitialize_module = (InitFunc)dlsym(handle, "DeinitializeCdmModule");
  CreateCDMInstanceFunc create_cdm_instance = (CreateCDMInstanceFunc)dlsym(handle, "CreateCdmInstance");
  CDMVersionFunc get_cdm_version = (CDMVersionFunc)dlsym(handle, "GetCdmVersion");
  if (
    !initialize_module ||
    !deinitialize_module ||
    !create_cdm_instance ||
    !get_cdm_version
  ) {
    return nullptr;
  }

  Library* lib = (Library*) malloc(sizeof(Library));
  lib->initialize_module = initialize_module;
  lib->deinitialize_module = deinitialize_module;
  lib->create_cdm_instance = create_cdm_instance;
  lib->get_cdm_version = get_cdm_version;
  lib->handle = handle;

  lib->initialize_module();

  return lib;
}

void* GetCDMHost(int host_interface_version, void* user_data) {
  if (host_interface_version != VERSION)
    return nullptr;
  return user_data;
}

cdm::ContentDecryptionModule_10* GetCDM(Library* lib, Host_10* host) {
  if (!lib || !host)
    return nullptr;

  void* ptr = lib->create_cdm_instance(
    VERSION,
    KEY_SYSTEM,
    KEY_SYSTEM_LENGTH,
    &GetCDMHost,
    host
  );
  return static_cast<cdm::ContentDecryptionModule_10*>(ptr);
}

void CDM_Initialize(cdm::ContentDecryptionModule_10* cdm) {
  if (!cdm) return;
  cdm->Initialize(false, false, false);
}

void CDM_SetServerCertificate(
  cdm::ContentDecryptionModule_10* cdm,
  uint32_t promise_id,
  const uint8_t* server_certificate_data,
  uint32_t server_certificate_data_length
) {
  if (!cdm) return;
  cdm->SetServerCertificate(
    promise_id,
    server_certificate_data,
    server_certificate_data_length
  );
}

void CDM_CreateSessionAndGenerateRequest(
  cdm::ContentDecryptionModule_10* cdm,
  uint32_t promise_id,
  cdm::SessionType session_type,
  cdm::InitDataType init_data_type,
  const uint8_t* init_data,
  uint32_t init_data_size
) {
  if (!cdm) return;
  cdm->CreateSessionAndGenerateRequest(
    promise_id,
    session_type,
    init_data_type,
    init_data,
    init_data_size
  );
}

void CDM_UpdateSession(
  cdm::ContentDecryptionModule_10* cdm,
  uint32_t promise_id,
  const char* session_id,
  uint32_t session_id_size,
  const uint8_t* response,
  uint32_t response_size
) {
  if (!cdm) return;
  cdm->UpdateSession(
    promise_id,
    session_id,
    session_id_size,
    response,
    response_size
  );
}

cdm::InputBuffer_2 RustBufferToCDM(InputBuffer buffer) {
  cdm::InputBuffer_2 buf = {
    buffer.data,
    buffer.data_size,
    buffer.encryption_scheme,
    buffer.key_id,
    buffer.key_id_size,
    buffer.iv,
    buffer.iv_size,
    buffer.subsamples,
    buffer.num_subsamples,
    buffer.pattern,
    buffer.timestamp
  };

  return buf;
}

DecryptionResult CDM_Decrypt(
  cdm::ContentDecryptionModule_10* cdm,
  InputBuffer encrypted_buffer
) {
  if (!cdm) {
    DecryptionResult result = { cdm::kInitializationError, nullptr, 0, 0 };
    return result;
  }
  cdm::InputBuffer_2 buf = RustBufferToCDM(encrypted_buffer);
  DecryptedBlock* decrypted = new DecryptedBlock();
  cdm::Status status = cdm->Decrypt(buf, decrypted);
  Buffer* buffer = static_cast<Buffer*>(decrypted->DecryptedBuffer());
  DecryptionResult result = { status, buffer->Data(), buffer->Capacity(), buffer->Size() };
  delete decrypted;
  delete buffer;
  return result;
}

Host_10* CreateHost(void* target, HostCallback* callback, RemoteBuffer* remote_buffer) {
  if (!target)
    return nullptr;

  Host_10* host = new Host_10(target, callback, remote_buffer);
  return host;
}

void DeinitializeCDM(cdm::ContentDecryptionModule_10* cdm) {
  if (!cdm) return;
  cdm->Destroy();
}

void DeinitializeLibrary(Library* lib) {
  if (!lib)
    return;

  lib->deinitialize_module();
  dlclose(lib->handle);
  free(lib);
}

void DeinitializeHost(Host_10* host) {
  if (!host)
    return;

  delete host;
}
