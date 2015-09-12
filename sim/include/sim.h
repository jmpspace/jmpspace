
#ifndef JMPSPACE_SIM
#define JMPSPACE_SIM

#include <stdio.h>
#include <stdint.h>

void hello_sim();

struct BufferImpl {
  size_t length;
  uint8_t *buf;
};
typedef struct BufferImpl Buffer;

struct SimImpl;
typedef struct SimImpl* Sim;

Sim* build_world();
int apply_action(Sim *sim, int client, Buffer buffer);
void update_world(Sim *sim);
Buffer snapshot_world(Sim *sim);

#endif
