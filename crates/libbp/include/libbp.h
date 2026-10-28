#ifndef BP_H
#define BP_H

#include <stdint.h>
#include <sys/socket.h>

/**
 * Address family number for Bundle Protocol. Randomly chosen.
 */
#define AF_BP 60518

/**
 * Bundle Protocol endpoint ID scheme.
 */
typedef enum bp_scheme_t {
  BP_SCHEME_T_IPN = 1,
  BP_SCHEME_T_DTN = 2,
} bp_scheme_t;

typedef struct sbp_ipn_t {
  uint32_t node;
  uint32_t service;
} sbp_ipn_t;

typedef char sbp_dtn_t[255];

typedef union sbp_addr_t {
  struct sbp_ipn_t ipn;
  sbp_dtn_t dtn;
} sbp_addr_t;

typedef struct sockaddr_bp {
  sa_family_t sbp_family;
  /**
   * A bp_scheme_t value (BP_SCHEME_T_IPN or BP_SCHEME_T_DTN).
   */
  uint32_t sbp_scheme;
  union sbp_addr_t sbp_addr;
} sockaddr_bp;

#endif  /* BP_H */
