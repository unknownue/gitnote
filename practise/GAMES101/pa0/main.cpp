
#include<cmath>
#include<eigen3/Eigen/Core>
#include<eigen3/Eigen/Dense>
#include<iostream>

int main() {

    using std::cos;
    using std::sin;

    const float PI = std::acos(-1.0f);

    Eigen::Vector3f p = Eigen::Vector3f(2.0f, 1.0f, 1.0f);

    Eigen::Matrix3f rotate = Eigen::Matrix3f();
    float degree = PI / 4.0f;
    rotate << cos(degree), -sin(degree), 0.0, sin(degree), cos(degree), 0.0, 0.0, 0.0, 1.0;

    Eigen::Matrix3f translate = Eigen::Matrix3f();
    translate << 1.0, 0.0, 1.0, 0.0, 1.0, 2.0, 0.0, 0.0, 1.0;

    Eigen::Vector3f p_transformed = translate * rotate * p;
    p_transformed(0) = p_transformed(0) / p_transformed(2);
    p_transformed(1) = p_transformed(1) / p_transformed(2);

    std::cout << "Before transformation: \n" << p << std::endl;  // output: (2, 1, 1)
    std::cout << "After  transformation: \n" << p_transformed << std::endl; // output: (1.7, 4.1, 1)

    return 0;
}

