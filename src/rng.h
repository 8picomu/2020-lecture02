#ifndef _CLASS_RNG_
#define _CLASS_RNG_

// *Really* minimal PCG32 code / (c) 2014 M.E. O'Neill / pcg-random.org
// Licensed under Apache License 2.0 (NO WARRANTY, etc. see website)

typedef struct { uint64_t state;  uint64_t inc; } pcg32_random_t;

uint32_t pcg32_random_r(pcg32_random_t* rng)
{
    uint64_t oldstate = rng->state;
    // Advance internal state
    rng->state = oldstate * 6364136223846793005ULL + (rng->inc|1);
    // Calculate output function (XSH RR), uses old state for max ILP
    uint32_t xorshifted = ((oldstate >> 18u) ^ oldstate) >> 27u;
    uint32_t rot = oldstate >> 59u;
    return (xorshifted >> rot) | (xorshifted << ((-rot) & 31));
}

class rng
{
private:
    pcg32_random_t pcg32;
public:
    rng(uint64_t seed) {
        pcg32.state = seed;
        pcg32.inc = 1;
    }

    float makeValue() {
        return pcg32_random_r(&pcg32) / (float)std::numeric_limits<uint32_t>::max();
    }

    ~rng() {}
};


#endif // !_CLASS_RNG_