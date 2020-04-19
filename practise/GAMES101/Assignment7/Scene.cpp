//
// Created by Göksu Güvendiren on 2019-05-14.
//

#include "Scene.hpp"


void Scene::buildBVH() {
    printf(" - Generating BVH...\n\n");
    this->bvh = new BVHAccel(objects, 1, BVHAccel::SplitMethod::NAIVE);
}

Intersection Scene::intersect(const Ray &ray) const
{
    return this->bvh->Intersect(ray);
}

void Scene::sampleLight(Intersection &pos, float &pdf) const
{
    float emit_area_sum = 0;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        if (objects[k]->hasEmit()){
            emit_area_sum += objects[k]->getArea();
        }
    }
    float p = get_random_float() * emit_area_sum;
    emit_area_sum = 0;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        if (objects[k]->hasEmit()){
            emit_area_sum += objects[k]->getArea();
            if (p <= emit_area_sum){
                objects[k]->Sample(pos, pdf);
                break;
            }
        }
    }
}

bool Scene::trace(
        const Ray &ray,
        const std::vector<Object*> &objects,
        float &tNear, uint32_t &index, Object **hitObject)
{
    *hitObject = nullptr;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        float tNearK = kInfinity;
        uint32_t indexK;
        Vector2f uvK;
        if (objects[k]->intersect(ray, tNearK, indexK) && tNearK < tNear) {
            *hitObject = objects[k];
            tNear = tNearK;
            index = indexK;
        }
    }


    return (*hitObject != nullptr);
}

// Implementation of Path Tracing
Vector3f Scene::castRay(const Ray &ray, int depth) const
{
    Intersection inter = this->intersect(ray);
    
    if (inter.happened == false)
        return Vector3f(0.0f, 0.0f, 0.0f);
    
    Vector3f p = inter.coords;
    Vector3f N = inter.normal.normalized();
    Material * material = inter.m; // the material on hit surface
    Object * hit_obj = inter.obj;
    
    Vector3f L_dir = Vector3f(0.0f, 0.0f, 0.0f);
    if (material->hasEmission()) {
        // This ray is casted from light source to camera directly
        L_dir += material->getEmission();
    }
    
    // Sample light from light source
    float pdf_light = 0.0;
    Intersection light_inter;
    sampleLight(light_inter, pdf_light);
    
    Vector3f x = light_inter.coords;
    
    // Test if the ray is blocked between light and p
    // Update the position of p to avoid some confusing artifacts.
    Vector3f shaded_p = dotProduct(x - p, N) < 0 ? p - N * EPSILON : p + N * EPSILON;
    Ray light_shadow_ray = Ray(shaded_p, (x - shaded_p).normalized());
    Intersection shadow_inter = this->intersect(light_shadow_ray);
    
    Vector3f wo = -ray.direction;
    Vector3f ws = light_shadow_ray.direction;
    Vector3f NN = light_inter.normal;
    
    // bool is_blocked = shadow_inter.happened && shadow_inter.obj == light_inter.obj;
    bool is_blocked = std::fabs(shadow_inter.distance - (x - shaded_p).norm()) >= EPSILON * 10.0f;
    // bool is_blocked = shadow_inter.happened;
    
    // Update the contribution from light
    if (is_blocked == false) {
        L_dir = light_inter.emit * material->eval(wo, ws, N) * dotProduct(ws, N) * dotProduct(-ws, NN) / (x - shaded_p).norm_squared() / pdf_light;
    }
    
    
    // Try Russian Roulette
    Vector3f L_indir = Vector3f(0.0f, 0.0f, 0.0f);
    if (get_random_float() < this->RussianRoulette) {
        // Sample wi from hemisphere
        Vector3f wi = material->sample(wo, N).normalized();
    
        // Cast a new ray
        Ray shadow_ray = Ray(shaded_p, wi);
        Intersection nonemit_inter = this->intersect(shadow_ray);
        // Test if this new ray hit an non-emitting object
        if (nonemit_inter.happened && nonemit_inter.m->hasEmission() == false) {
            // If hit, update the contribution from indirect light source
            float pdf = material->pdf(wo, wi, N);
            Ray new_ray = Ray(nonemit_inter.coords, wi);
            L_indir = this->castRay(new_ray, depth + 1) * material->eval(wo, wi, N) * dotProduct(wi, N) / pdf / this->RussianRoulette;
        }
    }
    
    return L_dir + L_indir;

}

