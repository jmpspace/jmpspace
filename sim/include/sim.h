
#ifndef JMPSPACE_SIM
#define JMPSPACE_SIM

#include <stdio.h>
#include <stdint.h>

void hello_sim();

struct SimImpl;
typedef struct SimImpl* Sim;

Sim* build_world();
int apply_action(Sim *sim, const uint8_t *buf, size_t len);
void update_world(Sim *sim);
void snapshot_world(Sim *sim);

#endif
