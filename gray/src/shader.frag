#version 450

layout(set = 0, binding = 0) uniform Uniforms {
    uvec2 window_size;
    float u_seed;
    float camera_lens_radius;
    vec3 camera_origin;
    vec3 camera_lower_left; // This too.
    vec3 camera_horizontal;
    vec3 camera_vertical; // This breaks stuff.
};

//const uvec2 window_size = uvec2(600, 300);

layout(location = 0) out vec4 f_color;

#define M_PI 3.1415926535897932384626433832795

/* camera attributes are provided by application */
float aspect_ratio = float(window_size.x) / float(window_size.y);
    // Camera

float viewport_height = 2.0;
float viewport_width = aspect_ratio * viewport_height;
float focal_length = 0.33;

vec3 origin = vec3(0, 0, 0);
vec3 horizontal = vec3(viewport_width, 0, 0);
vec3 vertical = vec3(0, viewport_height, 0);
vec3 lower_left = origin - horizontal / 2.0 - vertical / 2.0 - vec3(0, 0, focal_length);

//vec3 camera_origin = origin;
//vec3 camera_lower_left = lower_left;
// vec3 camera_horizontal = vec3(1, 0, 0);
// vec3 camera_vertical = vec3(0, 1, 0);

struct Ray {
    vec3 origin;
    vec3 direction;
};

const int mat_dielectric = 3;
const int mat_metal = 2;
const int mat_lambert = 1;

struct Material {
    vec3 albedo;
    float fuzz;
    float ref_idx;
    int scatter_function; // Matches the above
};

struct HitRecord {
    float t;
    vec3 p;
    vec3 normal;
    Material mat;
};

struct Sphere {
    vec3 center;
    float radius;
    Material mat;
};

const Material ground_mat = Material(vec3(0, 0.8, 0), 0, 0, mat_lambert);
const Material centre_mat = Material(vec3(0.1, 0.2, 0.5), 0, 0, mat_lambert);
const Material left_mat = Material(vec3(0, 0, 0), 0, 1.5, mat_dielectric);
const Material right_mat = Material(vec3(0.8, 0.6, 0.2), 0, 0, mat_metal);

const Material gray_metal = Material(vec3(0.8, 0.8, 0.8), 0.0001, 0.0, mat_metal);
const Material gold_metal = Material(vec3(0.8, 0.6, 0.2), 0.0001, 0.0, mat_metal);
const Material dielectric = Material(vec3(0), 0.0, 1.5, mat_dielectric);
const Material lambert = Material(vec3(0.8, 0.8, 0.0), 0.0, 0.0, mat_lambert);

const Sphere ground = Sphere(vec3(0, -100, -1), 97.5, centre_mat);
const Sphere centre = Sphere(vec3(0, 0, 0), 0.5,  centre_mat);
const Sphere left = Sphere(vec3(-1, 0, -1), 0.5,  centre_mat);
const Sphere right = Sphere(vec3(1, 0, -1), 0.5,  centre_mat);

const Sphere sphere1 = Sphere(vec3(1, 0, -1), 0.5, gray_metal);
const Sphere sphere2 = Sphere(vec3(-1, 0, -1), 0.5, gold_metal);

Sphere world[] = Sphere[] (ground, left, centre, right);
//Sphere world[] = Sphere[](sphere1, sphere2);

//Sphere world[] = Sphere[](ground);

/* returns a varying number between 0 and 1 */
float drand48(vec2 co) {
    return 2.0 * fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453) - 1.0;
}

vec3 random_in_unit_disk(vec2 co) {
    vec3 p;
    int n = 0;
    do {
        p = vec3(drand48(co.xy), drand48(co.yx), 0);
        n++;
    } while(dot(p, p) >= 1.0 && n < 3);
    return p;
}

float squared_length(vec3 v) {
    return v.x * v.x + v.y * v.y + v.z * v.z;
}

vec3 random_in_unit_sphere(vec3 p) {
    int n = 0;
    do {
        p = vec3(drand48(p.xy), drand48(p.zy), drand48(p.xz));
        n++;
    } while(squared_length(p) >= 1.0 && n < 3);
    return p;
}

bool lambertian_scatter(in Material mat, in Ray r, in HitRecord hit, out vec3 attenuation, out Ray scattered) {
    vec3 target = hit.p + hit.normal + random_in_unit_sphere(hit.p);
    scattered = Ray(hit.p, target - hit.p);
    attenuation = mat.albedo;
    return true;
}

vec3 c_reflect(in vec3 v, in vec3 n) {
    return v - 2.0 * dot(v, n) * n;
}

bool metal_scatter(in Material mat, in Ray r, in HitRecord hit, out vec3 attenuation, out Ray scattered) {
    vec3 reflected = c_reflect(normalize(r.direction), hit.normal);
    scattered = Ray(hit.p, reflected + mat.fuzz * random_in_unit_sphere(hit.p));
    attenuation = mat.albedo;
    return (dot(scattered.direction, hit.normal) > 0.0);
}

float schlick(in float cosine, in float ref_idx) {
    float r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * pow((1.0 - cosine), 5.0);
}

bool refract(in vec3 v, in vec3 n, in float ni_over_nt, out vec3 refracted) {
    vec3 uv = normalize(v);
    float dt = dot(uv, n);
    float discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if(discriminant > 0.0) {
        refracted = ni_over_nt * (uv - n * dt) - n * sqrt(discriminant);
        return true;
    } else {
        return false;
    }
}

bool dielectric_scatter(in Material mat, in Ray r, in HitRecord hit, out vec3 attenuation, out Ray scattered) {
    vec3 outward_normal;
    vec3 reflected = c_reflect(r.direction, hit.normal);
    float ni_over_nt;
    attenuation = vec3(1.0, 1.0, 1.0);
    vec3 refracted;
    float reflect_prob;
    float cosine;
    if(dot(r.direction, hit.normal) > 0.0) {
        outward_normal = -hit.normal;
        ni_over_nt = mat.ref_idx;
        cosine = mat.ref_idx * dot(r.direction, hit.normal) / length(r.direction);
    } else {
        outward_normal = hit.normal;
        ni_over_nt = 1.0 / mat.ref_idx;
        cosine = -dot(r.direction, hit.normal) / length(r.direction);
    }
    if(refract(r.direction, outward_normal, ni_over_nt, refracted)) {
        reflect_prob = schlick(cosine, mat.ref_idx);
    } else {
        reflect_prob = 1.0;
    }

    if(drand48(r.direction.xy) < reflect_prob) {
        scattered = Ray(hit.p, reflected);
    } else {
        scattered = Ray(hit.p, refracted);
    }
    return true;
}

bool dispatch_scatter(in Ray r, HitRecord hit, out vec3 attenuation, out Ray scattered) {
    if(hit.mat.scatter_function == mat_dielectric) {
        return dielectric_scatter(hit.mat, r, hit, attenuation, scattered);
    } else if(hit.mat.scatter_function == mat_metal) {
        return metal_scatter(hit.mat, r, hit, attenuation, scattered);
    } else {
        return lambertian_scatter(hit.mat, r, hit, attenuation, scattered);
    }
}

Ray get_ray(float s, float t) {
    vec3 rd = camera_lens_radius * random_in_unit_disk(vec2(s, t));
    vec3 offset = vec3(s * rd.x, t * rd.y, 0);
    return Ray(camera_origin + offset, camera_lower_left + s * camera_horizontal + t * camera_vertical - camera_origin - offset);
}

vec3 point_at_parameter(Ray r, float t) {
    return r.origin + t * r.direction;
}

/* Check hit between sphere and ray */
bool sphere_hit(Sphere sp, Ray r, float t_min, float t_max, out HitRecord hit) {
    vec3 oc = r.origin - sp.center;
    float a = dot(r.direction, r.direction);
    float b = dot(oc, r.direction);
    float c = dot(oc, oc) - sp.radius * sp.radius;
    float discriminant = b * b - a * c;
    if(discriminant > 0.0) {
        float temp = (-b - sqrt(b * b - a * c)) / a;
        if(temp < t_max && temp > t_min) {
            hit.t = temp;
            hit.p = point_at_parameter(r, hit.t);
            hit.normal = (hit.p - sp.center) / sp.radius;
            hit.mat = sp.mat;
            return true;
        }
        temp = (-b + sqrt(b * b - a * c)) / a;
        if(temp < t_max && temp > t_min) {
            hit.t = temp;
            hit.p = point_at_parameter(r, hit.t);
            hit.normal = (hit.p - sp.center) / sp.radius;
            hit.mat = sp.mat;
            return true;
        }
    }
    return false;
}

bool plane_hit(Ray r, float t_min, float t_max, out HitRecord hit) {
    float t = (-0.5 - r.origin.y) / r.direction.y;
    if(t < t_min || t > t_max)
        return false;
    hit.t = t;
    hit.p = point_at_parameter(r, t);
    hit.mat = gray_metal;
    hit.normal = vec3(0, 1, 0);
    return true;
}

/* Check all objects in world for hit with ray */
bool world_hit(Ray r, float t_min, float t_max, out HitRecord hit) {
    HitRecord temp_hit;
    bool hit_anything = false;
    float closest_so_far = t_max;

    for(int i = 0; i < world.length(); i++) {
        if(sphere_hit(world[i], r, t_min, closest_so_far, temp_hit)) {
            hit_anything = true;
            hit = temp_hit;
            closest_so_far = temp_hit.t;
        }
    }
    if(plane_hit(r, t_min, closest_so_far, temp_hit)) {
        hit_anything = true;
        hit = temp_hit;
    }

    return hit_anything;
}

vec3 color(Ray r) {
    HitRecord hit;
    vec3 col = vec3(0, 0, 0); /* visible color */
    vec3 total_attenuation = vec3(1.0, 1.0, 1.0); /* reduction of light transmission */

    for(int bounce = 0; bounce < 4; bounce++) {

        if(world_hit(r, 0.001, 1.0 / 0.0, hit)) {
      /* create a new reflected ray */
            Ray scattered;
            vec3 local_attenuation;

            if(dispatch_scatter(r, hit, local_attenuation, scattered)) {
                total_attenuation *= local_attenuation;
                r = scattered;
            } else {
                total_attenuation *= vec3(0, 0, 0);
            }
        } else {
      /* background hit (light source) */
            vec3 unit_dir = normalize(r.direction);
            float t = 0.5 * (unit_dir.y + 1.0);
            col = total_attenuation * ((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0));
            break;
        }
    }
    return col;
}

void main() {
    vec3 col = vec3(0, 0, 0);
    float u, v;
    const int nsamples = 32;
    Ray r;

    for(int s = 0; s < nsamples; s++) {
        u = (gl_FragCoord.x + drand48(col.xy + s)) / window_size.x;
        v = (gl_FragCoord.y + drand48(col.xz + s)) / window_size.y;
        r = get_ray(u, v);
        col += color(r);
    }

    col = col / float(nsamples);
    col = vec3(sqrt(col.x), sqrt(col.y), sqrt(col.z));

    f_color = vec4(col, 1.0);
}

// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
//     // Normalized pixel coordinates (from 0 to 1)
//     vec2 uv = fragCoord/iResolution.xy;

//     // Time varying pixel color
//     vec3 col = vec3(0,0,0);

//     Ray r = get_ray(uv.x, uv.y);
//     col = color(r);

//     // Output to screen
//     fragColor = vec4(col,1.0);
// }