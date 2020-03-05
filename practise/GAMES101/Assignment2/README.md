Done Anti-aliasing!

Modified:
1. Change the definition of `depth_buf` from `std::vector<float, 4>` to `std::vector<std::array<float, 4>>`.
2. The implementation of `rst::rasterizer::clear`.
