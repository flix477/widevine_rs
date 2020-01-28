#include "cdm_headers/content_decryption_module.h"
#include "implementation/host.h"
#include "implementation/decrypted_block.h"

typedef void (*InitFunc)();
typedef void* (*CreateCDMInstanceFunc)(int, const char*, uint32_t, GetCdmHostFunc, void*);
typedef const char* (*CDMVersionFunc)();

struct Library {
  InitFunc initialize_module;
  InitFunc deinitialize_module;
  CreateCDMInstanceFunc create_cdm_instance;
  CDMVersionFunc get_cdm_version;
  void* handle;
};

struct InputBuffer {
  const uint8_t* data;
  uint32_t data_size;
  cdm::EncryptionScheme encryption_scheme;
  const uint8_t* key_id;
  uint32_t key_id_size;
  const uint8_t* iv;
  uint32_t iv_size;
  const struct cdm::SubsampleEntry* subsamples;
  uint32_t num_subsamples;
  cdm::Pattern pattern;
  int64_t timestamp;
};

struct DecryptionResult {
  cdm::Status status;
  uint8_t* data;
  uint32_t capacity;
  uint32_t size;
};

extern "C" {
  Library* GetLibraryHandle();
  cdm::ContentDecryptionModule_10* GetCDM(Library* lib, Host_10* host);
  void CDM_Initialize(cdm::ContentDecryptionModule_10* cdm);
  void CDM_SetServerCertificate(
    cdm::ContentDecryptionModule_10* cdm,
    uint32_t promise_id,
    const uint8_t* server_certificate_data,
    uint32_t server_certificate_data_size
  );
  void CDM_CreateSessionAndGenerateRequest(
    cdm::ContentDecryptionModule_10* cdm,
    uint32_t promise_id,
    cdm::SessionType session_type,
    cdm::InitDataType init_data_type,
    const uint8_t* init_data,
    uint32_t init_data_size
  );
  void CDM_UpdateSession(
    cdm::ContentDecryptionModule_10* cdm,
    uint32_t promise_id,
    const char* session_id,
    uint32_t session_id_size,
    const uint8_t* response,
    uint32_t response_size
  );
  DecryptionResult CDM_Decrypt(
    cdm::ContentDecryptionModule_10* cdm,
    InputBuffer encrypted_buffer
  );
  Host_10* CreateHost(void* target, HostCallback* callback, RemoteBuffer* remote_buffer);
  void DeinitializeCDM(cdm::ContentDecryptionModule_10* cdm);
  void DeinitializeLibrary(Library* lib);
  void DeinitializeHost(Host_10* host);
}
