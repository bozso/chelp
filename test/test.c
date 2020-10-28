#include <stdio.h>
#include "chelp.h"

#define CHECK(ret)                                                   \
do {                                                                 \
    if ((ret).status == Error) {                                     \
        printf("Error at file '%s' line: %d\n", __FILE__, __LINE__); \
        goto error;                                                  \
    }                                                                \
                                                                     \
} while(0)

int main(void) {
    VersionInfo v = chelp_get_version();
    
    printf("VersionInfo(%u %u %u)\n", v.major, v.minor, v.patch);
    CResult r1 = chelp_string_intern("aaa"); CHECK(r1);
    
    CResult r2 = chelp_string_intern("bbb"); CHECK(r2);

    CResult r3 = chelp_string_concat(r1.id, r2.id); CHECK(r3);
    
    printf("IDs: %lu,%lu,%lu\n", r1.id, r2.id, r3.id);
    CResult r = chelp_dump_db(); CHECK(r);
    
    for (int ii = 0; ii < 10; ii++) {
        CResult r0 = chelp_string_intern("s"); CHECK(r);
        CResult r = chelp_dump_db(); CHECK(r);
        r = chelp_string_remove(r0.id); CHECK(r);
        r = chelp_dump_db(); CHECK(r);
        
        //r = chelp_string_concat(r1.id, r2.id); CHECK(r);
        r = chelp_string_remove(r.id); CHECK(r);
    }
    
    return 0;
error:
    fprintf(stderr, "error\n");
    return 1;
}
