#include <stdio.h>
#include "chelp.h"

#define CHECK(ret)             \
do {                           \
    if ((ret).status == Error) { \
        goto error;            \
    }                          \
                               \
} while(0)

int main(void) {
    VersionInfo v = chelp_get_version();
    
    printf("VersionInfo(%u %u %u)\n", v.major, v.minor, v.patch);
    CResult r1 = chelp_intern_string("aaa");
    CHECK(r1);
    
    CResult r2 = chelp_intern_string("bbb");
    CHECK(r2);

    CResult r3 = chelp_concat_strings(r1.id, r2.id);
    CHECK(r3);
    
    printf("IDs: %lu,%lu,%lu\n", r1.id, r2.id, r3.id);
    CResult r = chelp_dump_db();
    CHECK(r);
    
    for (int ii = 0; ii < 10; ii++) {
        CResult r = chelp_intern_string("aaa");
        CHECK(r);
        r = chelp_concat_strings(r1.id, r2.id);
        CHECK(r);
    }
    
    return 0;
error:
    fprintf(stderr, "error\n");
    return 1;
}
