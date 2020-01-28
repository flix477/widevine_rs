#include "decrypted_block.h"

DecryptedBlock::DecryptedBlock()
  : buffer(nullptr), timestamp(0) {}

DecryptedBlock::~DecryptedBlock() {
}

void DecryptedBlock::SetDecryptedBuffer(cdm::Buffer* buffer) {
  this->buffer = buffer;
}

cdm::Buffer* DecryptedBlock::DecryptedBuffer() {
  return buffer;
}

void DecryptedBlock::SetTimestamp(int64_t timestamp) {
  this->timestamp = timestamp;
}

int64_t DecryptedBlock::Timestamp() const {
  return timestamp;
}
