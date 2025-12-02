#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct PArray {
  const T *data;
  size_t size;
};

struct CVec2f {
  float x;
  float y;
};

extern "C" {

PArray<size_t> *cg_rust_polygon_triangulate(const PArray<CVec2f> *points);

PArray<CVec2f> *cg_rust_polygon_regular(CVec2f center, float radius, size_t corners);

}  // extern "C"
