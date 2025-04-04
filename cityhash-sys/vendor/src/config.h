/* config.h.  Manual configuration for cityhash-sys.  */

/* Define if the compiler has the __builtin_expect intrinsic. */
#ifdef __GNUC__
#define HAVE_BUILTIN_EXPECT 1
#else
#define HAVE_BUILTIN_EXPECT 0
#endif

/* Define to 1 if the system is big endian. */
#if defined(__BYTE_ORDER__) && __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
#define WORDS_BIGENDIAN 1
#else
#undef WORDS_BIGENDIAN
#endif