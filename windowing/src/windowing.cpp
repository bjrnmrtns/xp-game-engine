#include <SDL.h>
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

struct color_t {
    unsigned char r, g, b, a;
};

struct context_t {
    SDL_Window* window;
    SDL_Surface* framebuffer;
};

const void* windowing_create(const color_t* const buffer, std::size_t size)
{
    // check size = resolution
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window* window = SDL_CreateWindow("framebuffer-test", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 640, 480, 0);
    SDL_Surface* framebuffer = SDL_CreateRGBSurfaceFrom(const_cast<color_t *>(buffer), 640, 480, 32, 4 * 640, 0x000000ff, 0x0000ff00, 0x00ff0000, 0xff000000);
    return new context_t { window, framebuffer };
}

void windowing_update(const void* cookie)
{
    const context_t* context = static_cast<const context_t*>(cookie);
    SDL_BlitSurface(context->framebuffer, NULL, SDL_GetWindowSurface(context->window), NULL);
    SDL_UpdateWindowSurface(context->window);
}

bool windowing_pump(const void* cookie)
{
    const context_t* context = static_cast<const context_t*>(cookie);
    SDL_Event e;
    while(SDL_PollEvent(&e)) {
        if(e.type == SDL_QUIT) {
            return false;
        }
    }
    return true;
}

void windowing_destroy(const void* cookie)
{
    const context_t* context = static_cast<const context_t*>(cookie);
    SDL_FreeSurface(context->framebuffer);
    SDL_UpdateWindowSurface(context->window);
    SDL_DestroyWindow(context->window);
    SDL_Quit();
}

#ifdef __cplusplus
}
#endif
