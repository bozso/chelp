#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  Ok,
  Error,
} CStatus;

typedef struct {
  uint16_t major;
  uint16_t minor;
  uint16_t patch;
} VersionInfo;

typedef uint64_t ID;

typedef struct {
  ID id;
  CStatus status;
} CResult;

#define OK 0

VersionInfo chelp_get_version(void);

Default *chelp_new_service(void);

/**
 *  * Intern the string pointed to by `ptr` into the database. Returns the  * id of the string.
 */
CResult chelp_string_intern(const char *ptr);

CResult chelp_string_concat(ID one, ID two);

CResult chelp_string_remove(ID id);

CResult chelp_file_open(const char *ptr);

CResult chelp_file_close(ID id);

CResult chelp_dump_db(void);
