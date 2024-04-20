/* The Computer Language Benchmarks Game
 * https://salsa.debian.org/benchmarksgame-team/benchmarksgame/
 *
 * contributed by Christoph Bauer
 *
 */

#include "wasm.h"

#define pi 3.141592653589793f
#define solar_mass (4.0f * pi * pi)
#define days_per_year 365.24f

struct planet {
  float x, y, z;
  float vx, vy, vz;
  float mass;
};

void advance(int nbodies, struct planet *bodies, float dt) {
  int i, j;

  for (i = 0; i < nbodies; i++) {
    struct planet *b = &(bodies[i]);
    for (j = i + 1; j < nbodies; j++) {
      struct planet *b2 = &(bodies[j]);
      float dx = b->x - b2->x;
      float dy = b->y - b2->y;
      float dz = b->z - b2->z;
      float distance = __builtin_sqrtf(dx * dx + dy * dy + dz * dz);
      float mag = dt / (distance * distance * distance);
      b->vx -= dx * b2->mass * mag;
      b->vy -= dy * b2->mass * mag;
      b->vz -= dz * b2->mass * mag;
      b2->vx += dx * b->mass * mag;
      b2->vy += dy * b->mass * mag;
      b2->vz += dz * b->mass * mag;
    }
  }
  for (i = 0; i < nbodies; i++) {
    struct planet *b = &(bodies[i]);
    b->x += dt * b->vx;
    b->y += dt * b->vy;
    b->z += dt * b->vz;
  }
}

float energy(int nbodies, struct planet *bodies) {
  float e;
  int i, j;

  e = 0.0f;
  for (i = 0; i < nbodies; i++) {
    struct planet *b = &(bodies[i]);
    e += 0.5f * b->mass * (b->vx * b->vx + b->vy * b->vy + b->vz * b->vz);
    for (j = i + 1; j < nbodies; j++) {
      struct planet *b2 = &(bodies[j]);
      float dx = b->x - b2->x;
      float dy = b->y - b2->y;
      float dz = b->z - b2->z;
      float distance = __builtin_sqrtf(dx * dx + dy * dy + dz * dz);
      e -= (b->mass * b2->mass) / distance;
    }
  }
  return e;
}

void offset_momentum(int nbodies, struct planet *bodies) {
  float px = 0.0, py = 0.0, pz = 0.0;
  int i;
  for (i = 0; i < nbodies; i++) {
    px += bodies[i].vx * bodies[i].mass;
    py += bodies[i].vy * bodies[i].mass;
    pz += bodies[i].vz * bodies[i].mass;
  }
  bodies[0].vx = -px / solar_mass;
  bodies[0].vy = -py / solar_mass;
  bodies[0].vz = -pz / solar_mass;
}

#define NBODIES 5
struct planet bodies[NBODIES] = {
    {/* sun */
     0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, solar_mass},
    {/* jupiter */
     4.84143144246472090e+00f, -1.16032004402742839e+00f,
     -1.03622044471123109e-01f, 1.66007664274403694e-03f * days_per_year,
     7.69901118419740425e-03f * days_per_year,
     -6.90460016972063023e-05f * days_per_year,
     9.54791938424326609e-04f * solar_mass},
    {/* saturn */
     8.34336671824457987e+00f, 4.12479856412430479e+00f,
     -4.03523417114321381e-01f, -2.76742510726862411e-03f * days_per_year,
     4.99852801234917238e-03f * days_per_year,
     2.30417297573763929e-05f * days_per_year,
     2.85885980666130812e-04f * solar_mass},
    {/* uranus */
     1.28943695621391310e+01f, -1.51111514016986312e+01f,
     -2.23307578892655734e-01f, 2.96460137564761618e-03f * days_per_year,
     2.37847173959480950e-03f * days_per_year,
     -2.96589568540237556e-05f * days_per_year,
     4.36624404335156298e-05f * solar_mass},
    {/* neptune */
     1.53796971148509165e+01f, -2.59193146099879641e+01f,
     1.79258772950371181e-01f, 2.68067772490389322e-03f * days_per_year,
     1.62824170038242295e-03f * days_per_year,
     -9.51592254519715870e-05f * days_per_year,
     5.15138902046611451e-05f * solar_mass}};

float *WASM_EXPORT(nbody)(int n) {
  int i;

  offset_momentum(NBODIES, bodies);
  float energy_start = energy(NBODIES, bodies);
  for (i = 1; i <= n; i++)
    advance(NBODIES, bodies, 0.01f);

  float energy_end = energy(NBODIES, bodies);

  float *out = (float *)&__heap_base;
  out[0] = energy_start;
  out[1] = energy_end;
  return out;
}
