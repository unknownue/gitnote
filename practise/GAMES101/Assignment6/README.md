

| Task                      | Status |
| ------------------------- | ------ |
| Bounding box Intersection | Done   |
| BVH Intersection          | Done   |
| SAH                       | Done   |



SAE:

实现了一个新方法, 计算与给定Objects相交的cost，cost使用Objects的包围盒的表面积估计：

```cpp
// BVH.cpp
int BVHAccel::recursiveCost(std::vector<Object *> objects, Bounds3 centroidBounds, int dim) const;
```

修改了`recursiveBuild`的实现，使其可以根据`splitmedhod`来调整使用SAH与否:

```cpp
BVHBuildNode* BVHAccel::recursiveBuild(std::vector<Object*> objects);
```

同时修改了`Vector3f`的`[]`运算符的实现：

```cpp
inline float Vector3f::operator[](int index) const {
    switch (index) {
    case 0: return x;
    case 1: return y;
    case 2: return z;
    }
    return 0;
}
```

使用SAH 加速的结果不明显，根据测试划分的partition line数量的不同，渲染时间在11～15s不等。