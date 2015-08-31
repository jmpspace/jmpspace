
#ifndef JMPSPACE_SIM
#define JMPSPACE_SIM

void hello_sim();

struct SimImpl;
typedef struct SimImpl* Sim;

Sim* build_world(int a);
void apply_command(Sim* world, unsigned char* action);

#endif
