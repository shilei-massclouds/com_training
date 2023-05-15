/* Helper for init_calls_[start/end] */

typedef unsigned long u64;

typedef void (*initcall_t)(void);
extern initcall_t init_calls_start[];
extern initcall_t init_calls_end[];

#define __READ_ONCE(x)  (*(const volatile u64 *)&(x))

u64 initcalls_start() {
    u64 addr = (u64) init_calls_start;
    return __READ_ONCE(addr);
}

u64 initcalls_end() {
    u64 addr = (u64) init_calls_end;
    return __READ_ONCE(addr);
}
