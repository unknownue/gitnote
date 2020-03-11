// clang-format off
//
// Created by goksu on 4/6/19.
//

#include <algorithm>
#include <vector>
#include "rasterizer.hpp"
#include <opencv2/opencv.hpp>
#include <math.h>
#include <limits>


rst::pos_buf_id rst::rasterizer::load_positions(const std::vector<Eigen::Vector3f> &positions)
{
    auto id = get_next_id();
    pos_buf.emplace(id, positions);

    return {id};
}

rst::ind_buf_id rst::rasterizer::load_indices(const std::vector<Eigen::Vector3i> &indices)
{
    auto id = get_next_id();
    ind_buf.emplace(id, indices);

    return {id};
}

rst::col_buf_id rst::rasterizer::load_colors(const std::vector<Eigen::Vector3f> &cols)
{
    auto id = get_next_id();
    col_buf.emplace(id, cols);

    return {id};
}

auto to_vec4(const Eigen::Vector3f& v3, float w = 1.0f)
{
    return Vector4f(v3.x(), v3.y(), v3.z(), w);
}



static std::tuple<float, float, float> computeBarycentric2D(float x, float y, const Vector3f* v)
{
    float c1 = (x*(v[1].y() - v[2].y()) + (v[2].x() - v[1].x())*y + v[1].x()*v[2].y() - v[2].x()*v[1].y()) / (v[0].x()*(v[1].y() - v[2].y()) + (v[2].x() - v[1].x())*v[0].y() + v[1].x()*v[2].y() - v[2].x()*v[1].y());
    float c2 = (x*(v[2].y() - v[0].y()) + (v[0].x() - v[2].x())*y + v[2].x()*v[0].y() - v[0].x()*v[2].y()) / (v[1].x()*(v[2].y() - v[0].y()) + (v[0].x() - v[2].x())*v[1].y() + v[2].x()*v[0].y() - v[0].x()*v[2].y());
    float c3 = (x*(v[0].y() - v[1].y()) + (v[1].x() - v[0].x())*y + v[0].x()*v[1].y() - v[1].x()*v[0].y()) / (v[2].x()*(v[0].y() - v[1].y()) + (v[1].x() - v[0].x())*v[2].y() + v[0].x()*v[1].y() - v[1].x()*v[0].y());
    return {c1,c2,c3};
}

static bool insideTriangle(float x, float y, const Vector3f* _v)
{   
    // Use barycentric coordinates to judge.
    // Method borrow from https://github.com/ssloy/tinyrenderer
    auto[c1, c2, c3] = computeBarycentric2D(x, y, _v);
    return c1 > 0.0f && c2 > 0.0f && c3 > 0.0f;
}


void rst::rasterizer::draw(pos_buf_id pos_buffer, ind_buf_id ind_buffer, col_buf_id col_buffer, Primitive type)
{
    auto& buf = pos_buf[pos_buffer.pos_id];
    auto& ind = ind_buf[ind_buffer.ind_id];
    auto& col = col_buf[col_buffer.col_id];

    float f1 = (50 - 0.1) / 2.0;
    float f2 = (50 + 0.1) / 2.0;

    Eigen::Matrix4f mvp = projection * view * model;
    for (auto& i : ind)
    {
        Triangle t;
        Eigen::Vector4f v[] = {
                mvp * to_vec4(buf[i[0]], 1.0f),
                mvp * to_vec4(buf[i[1]], 1.0f),
                mvp * to_vec4(buf[i[2]], 1.0f)
        };
        //Homogeneous division
        for (auto& vec : v) {
            vec /= vec.w();
        }
        //Viewport transformation
        for (auto & vert : v)
        {
            vert.x() = 0.5*width*(vert.x()+1.0);
            vert.y() = 0.5*height*(vert.y()+1.0);
            vert.z() = vert.z() * f1 + f2;
        }

        for (int i = 0; i < 3; ++i)
        {
            t.setVertex(i, v[i].head<3>());
            t.setVertex(i, v[i].head<3>());
            t.setVertex(i, v[i].head<3>());
        }

        auto col_x = col[i[0]];
        auto col_y = col[i[1]];
        auto col_z = col[i[2]];

        t.setColor(0, col_x[0], col_x[1], col_x[2]);
        t.setColor(1, col_y[0], col_y[1], col_y[2]);
        t.setColor(2, col_z[0], col_z[1], col_z[2]);

        rasterize_triangle(t);
    }
}

//Screen space rasterization
void rst::rasterizer::rasterize_triangle(const Triangle& t) {
    auto v = t.toVector4();

    // Find out the bounding box of current triangle.
    Eigen::Vector2f bbox_min = { std::numeric_limits<float>::max(), std::numeric_limits<float>::max() };
    Eigen::Vector2f bbox_max = { 0.0f, 0.0f };
    
    for (size_t i = 0; i < 3; i++) {
        const auto & vertex = t.v[i];
        if (vertex.x() < bbox_min.x()) { bbox_min.x() = vertex.x(); }
        if (vertex.y() < bbox_min.y()) { bbox_min.y() = vertex.y(); }
        if (vertex.x() > bbox_max.x()) { bbox_max.x() = vertex.x(); }
        if (vertex.y() > bbox_max.y()) { bbox_max.y() = vertex.y(); }
    }

    // Return the color of current pixel
    auto get_pixel = [&](int x, int y) -> Eigen::Vector3f {
        auto ind = (this->height - 1 - y) * this->width + x;
        return this->frame_buf[ind];
    };

    const std::array<Eigen::Vector2f, 4> offsets = {
        Eigen::Vector2f(0.25f, 0.25f),
        Eigen::Vector2f(0.75f, 0.25f),
        Eigen::Vector2f(0.25f, 0.75f),
        Eigen::Vector2f(0.75f, 0.75f),
    };
    // iterate through the pixel and find if the current pixel is inside the triangle
    for (int x = (int)bbox_min.x(); x <= (int)bbox_max.x(); x++) {
        for (int y = (int)bbox_min.y(); y <= (int)bbox_max.y(); y++) {

            // the count of subpixel occupied by current triangle in (x, y) pixel
            size_t hit_sample = 0;
            for (size_t i = 0; i < 4; i++) {
                float x_ = (float)x + offsets[i].x();
                float y_ = (float)y + offsets[i].y();

                if (insideTriangle(x_, y_, t.v)) {
                    // Get the interpolated z value.
                    auto[alpha, beta, gamma] = computeBarycentric2D(x_, y_, t.v);
                    float w_reciprocal = 1.0/(alpha / v[0].w() + beta / v[1].w() + gamma / v[2].w());
                    float z_interpolated = alpha * v[0].z() / v[0].w() + beta * v[1].z() / v[1].w() + gamma * v[2].z() / v[2].w();
                    z_interpolated *= w_reciprocal;

                    if (z_interpolated < this->depth_buf[this->get_index(x, y)][i]) {
                        this->depth_buf[this->get_index(x, y)][i] = z_interpolated;

                        hit_sample++;
                    }
                }

            }

            if (hit_sample > 0) {
                const Eigen::Vector3f pos = { (float)(this->width - x), (float)(this->height - y), 0.0f };
                auto color = t.getColor() * (float)hit_sample / 4.0f + get_pixel((this->width - x), (this->height - y));
                this->set_pixel(pos, color);
            }

        }
    }

}

void rst::rasterizer::set_model(const Eigen::Matrix4f& m)
{
    model = m;
}

void rst::rasterizer::set_view(const Eigen::Matrix4f& v)
{
    view = v;
}

void rst::rasterizer::set_projection(const Eigen::Matrix4f& p)
{
    projection = p;
}

void rst::rasterizer::clear(rst::Buffers buff)
{
    if ((buff & rst::Buffers::Color) == rst::Buffers::Color)
    {
        std::fill(frame_buf.begin(), frame_buf.end(), Eigen::Vector3f{0, 0, 0});
    }

    float infinity = std::numeric_limits<float>::infinity();
    std::array<float, 4> black = { infinity, infinity, infinity, infinity };
    if ((buff & rst::Buffers::Depth) == rst::Buffers::Depth)
    {
        std::fill(depth_buf.begin(), depth_buf.end(), black);
    }
}

rst::rasterizer::rasterizer(int w, int h) : width(w), height(h)
{
    frame_buf.resize(w * h);
    depth_buf.resize(w * h);
}

int rst::rasterizer::get_index(int x, int y)
{
    return (height-1-y)*width + x;
}

void rst::rasterizer::set_pixel(const Eigen::Vector3f& point, const Eigen::Vector3f& color)
{
    //old index: auto ind = point.y() + point.x() * width;
    auto ind = (height-1-point.y())*width + point.x();
    frame_buf[ind] = color;

}

// clang-format on
