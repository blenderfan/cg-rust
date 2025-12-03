#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct CVec2f {
  float x;
  float y;
};

template<typename T>
struct PArray {
  const T *data;
  size_t size;
};

extern "C" {

void cg_rust_free_array_float2(PArray<CVec2f> *a);

void cg_rust_free_array_long(PArray<size_t> *a);

void cg_rust_free_array_int(PArray<int> *a);

PArray<size_t> *cg_rust_polygon_triangulate(const PArray<CVec2f> *points);

PArray<CVec2f> *cg_rust_polygon_regular(CVec2f center, float radius, size_t corners);

}  // extern "C"
