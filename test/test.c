#include <stdio.h>
#include "chelp.h"

int main(void) {
    VersionInfo v = chelp_get_version();
    
    printf("VersionInfo(%u %u %u)\n", v.major, v.minor, v.patch);
    CResult r1 = chelp_intern_string("aaa");
    
    if (r1.status == Error) {
        fprintf(stderr, "error\n");
        return 1;
    }
    
    CResult r2 = chelp_intern_string("bbb");

    if (r2.status == Error) {
        fprintf(stderr, "error\n");
        return 1;
    }

    CResult r3 = chelp_concat_strings(r1.id, r2.id);

    if (r3.status == Error) {
        fprintf(stderr, "error\n");
        return 1;
    }
    
    printf("IDs: %lu,%lu,%lu\n", r1.id, r2.id, r3.id);
    CResult r = chelp_dump_db();

    if (r.status == Error) {
        fprintf(stderr, "error\n");
        return 1;
    }
    
    return 0;
}
