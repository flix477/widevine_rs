#ifndef BUFFER_H
#define BUFFER_H

#include <vector>
#include "../cdm_headers/content_decryption_module.h"

struct RemoteBuffer {
  void (*destroy)(void*);
  uint32_t (*capacity)(void*);
  uint8_t* (*data)(void*);
  void (*set_size)(uint32_t, void*);
  uint32_t (*size)(void*);
};

class Buffer: public cdm::Buffer {
  public:
    Buffer(RemoteBuffer* remote, void* target);
    void Destroy() override;
    uint32_t Capacity() const override;
    uint8_t* Data() override;
    void SetSize(uint32_t size) override;
    uint32_t Size() const override;
    ~Buffer() override;

  private:
    RemoteBuffer* remote;
    void* target;
};

#endif /* BUFFER_H */
