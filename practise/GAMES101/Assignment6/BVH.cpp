#include <algorithm>
#include <cassert>
#include "BVH.hpp"

BVHAccel::BVHAccel(std::vector<Object*> p, int maxPrimsInNode,
                   SplitMethod splitMethod)
    : maxPrimsInNode(std::min(255, maxPrimsInNode)), splitMethod(splitMethod),
      primitives(std::move(p))
{
    time_t start, stop;
    time(&start);
    if (primitives.empty())
        return;

    root = recursiveBuild(primitives);

    time(&stop);
    double diff = difftime(stop, start);
    int hrs = (int)diff / 3600;
    int mins = ((int)diff / 60) - (hrs * 60);
    int secs = (int)diff - (hrs * 3600) - (mins * 60);

    printf(
        "\rBVH Generation complete: \nTime Taken: %i hrs, %i mins, %i secs\n\n",
        hrs, mins, secs);
}


BVHBuildNode* BVHAccel::recursiveBuild(std::vector<Object*> objects)
{
    BVHBuildNode* node = new BVHBuildNode();

    // Compute bounds of all primitives in BVH node
    Bounds3 bounds;
    for (int i = 0; i < objects.size(); ++i)
        bounds = Union(bounds, objects[i]->getBounds());
    if (objects.size() == 1) {
        // Create leaf _BVHBuildNode_
        node->bounds = objects[0]->getBounds();
        node->object = objects[0];
        node->left  = nullptr;
        node->right = nullptr;
        return node;
    }
    else if (objects.size() == 2) {
        node->left  = recursiveBuild(std::vector{objects[0]});
        node->right = recursiveBuild(std::vector{objects[1]});

        node->bounds = Union(node->left->bounds, node->right->bounds);
        return node;
    }
    else {
        Bounds3 centroidBounds;
        for (int i = 0; i < objects.size(); ++i)
            centroidBounds =
                Union(centroidBounds, objects[i]->getBounds().Centroid());
        int dim = centroidBounds.maxExtent();
        switch (dim) {
        case 0:
            std::sort(objects.begin(), objects.end(), [](auto f1, auto f2) {
                return f1->getBounds().Centroid().x <
                       f2->getBounds().Centroid().x;
            });
            break;
        case 1:
            std::sort(objects.begin(), objects.end(), [](auto f1, auto f2) {
                return f1->getBounds().Centroid().y <
                       f2->getBounds().Centroid().y;
            });
            break;
        case 2:
            std::sort(objects.begin(), objects.end(), [](auto f1, auto f2) {
                return f1->getBounds().Centroid().z <
                       f2->getBounds().Centroid().z;
            });
            break;
        }

        int partition_index;
        switch (this->splitMethod) {
            case SplitMethod::NAIVE: {
                partition_index = int(objects.size() / 2);
                break;
            }
            case SplitMethod::SAH: {
                // Ignore the return value since it does not need in recursiveBuild method
                partition_index = this->recursiveCost(objects, centroidBounds, dim);
                break;
            }
        }
        if (this->splitMethod == SplitMethod::NAIVE) {
            partition_index = int(objects.size() / 2);
        } else if (this->splitMethod == SplitMethod::SAH) {
        }

        auto beginning = objects.begin();
        auto middling = objects.begin() + partition_index;
        auto ending = objects.end();

        auto leftshapes = std::vector<Object*>(beginning, middling);
        auto rightshapes = std::vector<Object*>(middling, ending);

        assert(objects.size() == (leftshapes.size() + rightshapes.size()));

        node->left = recursiveBuild(leftshapes);
        node->right = recursiveBuild(rightshapes);

        node->bounds = Union(node->left->bounds, node->right->bounds);
    }

    return node;
}

int BVHAccel::recursiveCost(std::vector<Object *> objects, Bounds3 centroidBounds, int dim) const {
    // best_split_index is the secondary return value
    //      indicating the index of best partition position in objects
    //
    // dim is the dimension to partition
    // objects has been sorted along the dim axis
    //
    // Return the partition index for objects
    
    if (objects.empty()) { return 0; }
    
    int best_split_index = 0;
    
    if (objects.size() == 1) {
        // Use surface area or bbox as cost
        best_split_index = 0;
    } else if (objects.size() == 2) {
        float left_cost  = objects[0]->getBounds().SurfaceArea();
        float right_cost = objects[1]->getBounds().SurfaceArea();
    
        best_split_index = 1;
    } else {
        const int SPACE_SPLIT = 10;

        // Split the space at dim-th axis
        float dim_min = centroidBounds.pMin[dim];
        float dim_max = centroidBounds.pMax[dim];
    
        // Try SPACE_SPLIT count of different partition line
        float lowest_cost = std::numeric_limits<float>::infinity(); 
    
        for(int i = 1; i <= SPACE_SPLIT; i++) {
            float split_position = dim_min + (float)i * (dim_max - dim_min) / (SPACE_SPLIT + 1);
    
            // The objects has been sorted by dim-th dimension in the recursiveBuild method
            // Here just find the middle one
    
            auto beginning = objects.cbegin();
            auto ending = objects.cend();
    
            std::vector<Object *> left_split  = {};
            std::vector<Object *> right_split = {};
            
            int split_index = 0;
            for (auto middling = objects.cbegin(); middling != ending; middling++, split_index++) {
                Vector3f centroid = (*middling)->getBounds().Centroid();
                if (centroid[dim] > split_position) {
                    left_split  = std::vector<Object*>(beginning, middling);
                    right_split = std::vector<Object*>(middling, ending);
                    break;
                }
            }

            auto union_bbox = [](const std::vector<Object *> & objects) -> Bounds3 {
                Bounds3 bbox;
                for (auto & object : objects) {
                    bbox = Union(bbox, object->getBounds());
                }
                return bbox;
            };

            // std::cout << "Left : " << left_split.size() << std::endl;
            // std::cout << "Right: " << right_split.size() << std::endl;

            float left_cost  = union_bbox(left_split).SurfaceArea();
            float right_cost = union_bbox(right_split).SurfaceArea();

            // Update best partition location
            if (left_cost + right_cost < lowest_cost) {
                best_split_index = split_index;
                lowest_cost = left_cost + right_cost;
            }
        }
    }
    
    return best_split_index;
}

Intersection BVHAccel::Intersect(const Ray& ray) const
{
    Intersection isect;
    if (!root)
        return isect;
    isect = BVHAccel::getIntersection(root, ray);
    return isect;
}

Intersection BVHAccel::getIntersection(BVHBuildNode* node, const Ray& ray) const
{
    // TODO Traverse the BVH to find intersection
    
    Intersection inter;
    std::array<int, 3> dirIsNeg = {
        int(ray.direction.x > 0.0f),
        int(ray.direction.y > 0.0f),
        int(ray.direction.z > 0.0f),
    };
    
    // If ray misses current node's bbox
    if (node->bounds.IntersectP(ray, ray.direction_inv, dirIsNeg) == false) {
        // std::cout << "Missed " << std::endl;
        return inter;
    }


    // If current node is leaf node
    if (node->left == nullptr && node->right == nullptr) {
        return node->object->getIntersection(ray);
    }

    // Return the cloest intersection of sub nodes
    if (node->left) {
        inter = this->getIntersection(node->left, ray);
    }
    if (node->right) {
        auto inter_tmp = this->getIntersection(node->right, ray);
        if (inter_tmp.distance < inter.distance) {
            inter = inter_tmp;
        }
    }

    return inter;
}
