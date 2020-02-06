#include <iostream>
#include <ctime>
#include "host.h"

Host_10::Host_10(void* target, HostCallback* callback, RemoteBuffer* remote_buffer) {
  this->target = target;
  this->callback = callback;
  this->remote_buffer = remote_buffer;
}

cdm::Buffer* Host_10::Allocate(uint32_t capacity) {
  void* target = this->callback->allocate(capacity);
  return new Buffer(this->remote_buffer, target);
}

void Host_10::SetTimer(int64_t delay_ms, void* context) {
  this->callback->set_timer(delay_ms, context, this->target);
}

cdm::Time Host_10::GetCurrentWallTime() {
  return time(0);
}

void Host_10::OnInitialized(bool success) {
  this->callback->on_initialized(success, this->target);
}

void Host_10::OnResolveKeyStatusPromise(
  uint32_t promise_id,
  cdm::KeyStatus key_status
) {
  std::cout << "OnResolveKeyStatusPromise";
}

void Host_10::OnResolveNewSessionPromise(
  uint32_t promise_id,
  const char* session_id,
  uint32_t session_id_size
) {
  this->callback->on_resolve_new_session(
    promise_id,
    session_id,
    session_id_size,
    this->target
  );
}

void Host_10::OnResolvePromise(uint32_t promise_id) {
  this->callback->on_resolve(promise_id, this->target);
}

void Host_10::OnRejectPromise(
  uint32_t promise_id,
  cdm::Exception exception,
  uint32_t system_code,
  const char* error_message,
  uint32_t error_message_size
) {
  this->callback->on_reject(
    promise_id,
    exception,
    system_code,
    error_message,
    error_message_size,
    this->target
  );
}

void Host_10::OnSessionMessage(
  const char* session_id,
  uint32_t session_id_size,
  cdm::MessageType message_type,
  const char* message,
  uint32_t message_size
) {
  this->callback->on_session_message(
    session_id,
    session_id_size,
    message_type,
    message,
    message_size,
    this->target
  );
}

void Host_10::OnSessionKeysChange(
  const char* session_id,
  uint32_t session_id_size,
  bool has_additional_usable_key,
  const cdm::KeyInformation* keys_info,
  uint32_t keys_info_count
) {
  this->callback->on_session_keys_change(
    session_id,
    session_id_size,
    has_additional_usable_key,
    keys_info,
    keys_info_count,
    this->target
  );
}

void Host_10::OnExpirationChange(
  const char* session_id,
  uint32_t session_id_size,
  cdm::Time new_expiry_time
) {
  this->callback->on_expiration_change(
    session_id,
    session_id_size,
    new_expiry_time,
    this->target
  );
}

void Host_10::OnSessionClosed(
  const char* session_id,
  uint32_t session_id_size
) {
  std::cout << "OnSessionClosed";
}

void Host_10::SendPlatformChallenge(
  const char* service_id,
  uint32_t service_id_size,
  const char* challenge,
  uint32_t challenge_size
) {
  std::cout << "SendPlatformChallenge";
}

void Host_10::EnableOutputProtection(uint32_t desired_protection_mask) {
  std::cout << "EnableOutputProtection";
}

void Host_10::QueryOutputProtectionStatus() {
  std::cout << "QueryOutputProtectionStatus";
}

void Host_10::OnDeferredInitializationDone(
  cdm::StreamType stream_type,
  cdm::Status decoder_status
) {
  std::cout << "OnDeferredInitializationDone";
}

cdm::FileIO* Host_10::CreateFileIO(cdm::FileIOClient* client) {
  std::cout << "CreateFileIO";
  return nullptr;
}

void Host_10::RequestStorageId(uint32_t version) {
  std::cout << "RequestStorageId";
}

Host_10::~Host_10() {
}
