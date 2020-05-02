#include <iostream>
#include <vector>
#include "buffer.h"

Buffer::Buffer(RemoteBuffer* remote, void* target)
  : remote(remote), target(target) {}

void Buffer::Destroy() {
  this->remote->destroy(this->target);
  delete this;
}

uint32_t Buffer::Capacity() const {
  return this->remote->capacity(this->target);
}

uint8_t* Buffer::Data() {
  return this->remote->data(this->target);
}

void Buffer::SetSize(uint32_t size) {
  this->remote->set_size(size, this->target);
}

uint32_t Buffer::Size() const {
  return this->remote->size(this->target);
}

Buffer::~Buffer() {
}
