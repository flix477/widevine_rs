#ifndef HOST_H
#define HOST_H

#include "../cdm_headers/content_decryption_module.h"
#include "buffer.h"

struct HostCallback {
  void (*on_initialized)(bool, void*);
  void (*on_resolve)(uint32_t, void*);
  void (*on_reject)(uint32_t, cdm::Exception, uint32_t, const char*, uint32_t, void*);
  void (*on_resolve_new_session)(uint32_t, const char*, uint32_t, void*);
  void (*on_session_message)(const char*, uint32_t, cdm::MessageType, const char*, uint32_t, void*);
  void* (*allocate)(uint32_t);
  void (*on_expiration_change)(const char*, uint32_t, cdm::Time, void*);
  void (*on_session_keys_change)(const char*, uint32_t, bool, const cdm::KeyInformation*, uint32_t, void*);
};

class Host_10: public cdm::Host_10 {
  public:
    Host_10(void* target, HostCallback* callback, RemoteBuffer* remote_buffer);
    cdm::Buffer* Allocate(uint32_t capacity) override;
    void SetTimer(int64_t delay_ms, void* context) override;
    cdm::Time GetCurrentWallTime() override;
    void OnInitialized(bool success) override;
    void OnResolveKeyStatusPromise(uint32_t promise_id,
                                   cdm::KeyStatus key_status) override;
    void OnResolveNewSessionPromise(uint32_t promise_id,
                                    const char* session_id,
                                    uint32_t session_id_size) override;
    void OnResolvePromise(uint32_t promise_id) override;
    void OnRejectPromise(uint32_t promise_id,
                         cdm::Exception exception,
                         uint32_t system_code,
                         const char* error_message,
                         uint32_t error_message_size) override;
    void OnSessionMessage(const char* session_id,
                          uint32_t session_id_size,
                          cdm::MessageType message_type,
                          const char* message,
                          uint32_t message_size) override;
    void OnSessionKeysChange(const char* session_id,
                             uint32_t session_id_size,
                             bool has_additional_usable_key,
                             const cdm::KeyInformation* keys_info,
                             uint32_t keys_info_count) override;
    void OnExpirationChange(const char* session_id,
                            uint32_t session_id_size,
                            cdm::Time new_expiry_time) override;
    void OnSessionClosed(const char* session_id,
                         uint32_t session_id_size) override;
    void SendPlatformChallenge(const char* service_id,
                               uint32_t service_id_size,
                               const char* challenge,
                               uint32_t challenge_size) override;
    void EnableOutputProtection(uint32_t desired_protection_mask) override;
    void QueryOutputProtectionStatus() override;
    void OnDeferredInitializationDone(cdm::StreamType stream_type,
                                      cdm::Status decoder_status) override;
    cdm::FileIO* CreateFileIO(cdm::FileIOClient* client) override;
    void RequestStorageId(uint32_t version) override;
    ~Host_10();
  
  private:
    void* target;
    HostCallback* callback;
    RemoteBuffer* remote_buffer;
};

#endif /* HOST_H */
