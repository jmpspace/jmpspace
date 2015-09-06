
#ifndef JMPSPACE_SIM
#define JMPSPACE_SIM

void hello_sim();

struct SimImpl;
typedef struct SimImpl* Sim;

Sim* build_world(int a);
void apply_action(Sim* sim, int a);
void update_world(Sim* sim);
void snapshot_world(Sim* sim);

#endif
