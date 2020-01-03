#include <SDL.h>

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

const void* windowing_create()
{
    SDL_Init(SDL_INIT_VIDEO | SDL_INIT_EVENTS);
    SDL_Window* window = SDL_CreateWindow("framebuffer-test", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 640, 480, 0);
    SDL_Surface* screen = SDL_GetWindowSurface(window);
    color_t colors[640 * 480];
    SDL_Surface* framebuffer = SDL_CreateRGBSurfaceFrom(&colors, 640, 480, 32, 4 * 640, 0x000000ff, 0x0000ff00, 0x00ff0000, 0xff000000);
    for(size_t i = 0; i < 640 * 240; i++) {
        colors[i].r = 255;
        colors[i].g = 0;
        colors[i].b = 0;
        colors[i].a = 255;
    }
    for(size_t i = 640 * 240; i < 640 * 480; i++) {
        colors[i].r = 0;
        colors[i].g = 0;
        colors[i].b = 255;
        colors[i].a = 255;
    }
    SDL_BlitSurface(framebuffer, NULL, screen, NULL);
    return new context_t { window, framebuffer };
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
    SDL_Delay(2000);
    SDL_DestroyWindow(context->window);
    SDL_Quit();
}

#ifdef __cplusplus
}
#endif
