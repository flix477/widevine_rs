#ifndef DECRYPTED_BLOCK_H
#define DECRYPTED_BLOCK_H

#include "../cdm_headers/content_decryption_module.h"

class DecryptedBlock: public cdm::DecryptedBlock {
  public:
    DecryptedBlock();
    ~DecryptedBlock() override;
    void SetDecryptedBuffer(cdm::Buffer* buffer) override;
    cdm::Buffer* DecryptedBuffer() override;
    void SetTimestamp(int64_t timestamp) override;
    int64_t Timestamp() const override;

  private:
    cdm::Buffer* buffer;
    int64_t timestamp;
};

#endif /* DECRYPTED_BLOCK_H */
