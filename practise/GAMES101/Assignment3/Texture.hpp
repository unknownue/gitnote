//
// Created by LEI XU on 4/27/19.
//

#ifndef RASTERIZER_TEXTURE_H
#define RASTERIZER_TEXTURE_H
#include "global.hpp"
#include <eigen3/Eigen/Eigen>
#include <opencv2/opencv.hpp>
class Texture{
private:
    cv::Mat image_data;

public:
    Texture(const std::string& name)
    {
        image_data = cv::imread(name);
        cv::cvtColor(image_data, image_data, cv::COLOR_RGB2BGR);
        width = image_data.cols;
        height = image_data.rows;
    }

    int width, height;

    Eigen::Vector3f getColor(float u, float v)
    {
        auto u_img = u * width;
        auto v_img = (1 - v) * height;
        auto color = image_data.at<cv::Vec3b>(v_img, u_img);
        return Eigen::Vector3f(color[0], color[1], color[2]);
    }

    Eigen::Vector3f getColorBilinear(float u, float v)
    {
        using std::max, std::min, std::round;
        u = min(max(0.0f, u), 1.0f);
        v = min(max(0.0f, v), 1.0f);

        auto u_img = u * width;
        auto v_img = (1 - v) * height;

        auto lerp = [&](float x, const cv::Vec3b & v0, const cv::Vec3b & v1) -> cv::Vec3b {
            return v0 + x * (v1 - v0);
        };
        
        float center_x = round(u_img);
        float center_y = round(v_img);
        
        float offset_x[2] = { max(0.0f, center_x - 0.5f), min((float)this->width,  center_x + 0.5f) };
        float offset_y[2] = { max(0.0f, center_y - 0.5f), min((float)this->height, center_y + 0.5f) };
        
        auto u00 = image_data.at<cv::Vec3b>(offset_y[0], offset_x[0]);
        auto u01 = image_data.at<cv::Vec3b>(offset_y[1], offset_x[0]);
        auto u10 = image_data.at<cv::Vec3b>(offset_y[0], offset_x[1]);
        auto u11 = image_data.at<cv::Vec3b>(offset_y[1], offset_x[1]);
        
        float s = u_img - (center_x - 0.5f);
        float t = v_img - (center_y - 0.5f);
        
        auto u0 = lerp(s, u00, u10);
        auto u1 = lerp(s, u01, u11);
        auto color = lerp(t, u0, u1);
        // auto color = image_data.at<cv::Vec3b>(v_img, u_img);
        return Eigen::Vector3f(color[0], color[1], color[2]);
    }

};
#endif //RASTERIZER_TEXTURE_H

