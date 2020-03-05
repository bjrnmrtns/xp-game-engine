#include <SDL.h>
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

struct color_t {
    unsigned char r, g, b, a;
};

enum class window_event_t : int {
    quit,
    not_implemented,
    none,
};

struct context_t {
    SDL_Window* window;
    SDL_Surface* framebuffer;
};

const void* window_create(std::size_t width, std::size_t height, const color_t* const buffer, std::size_t size)
{
    // check size = resolution
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window* window = SDL_CreateWindow("software-renderer-rs", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, width, height, 0);
    SDL_Surface* framebuffer = SDL_CreateRGBSurfaceFrom(const_cast<color_t *>(buffer), width, height, 32, 4 * width, 0x000000ff, 0x0000ff00, 0x00ff0000, 0xff000000);
    return new context_t { window, framebuffer };
}

void window_update(const void* self)
{
    const context_t* context = static_cast<const context_t*>(self);
    SDL_BlitSurface(context->framebuffer, NULL, SDL_GetWindowSurface(context->window), NULL);
    SDL_UpdateWindowSurface(context->window);
}

window_event_t window_poll_event(const void* self)
{
    const context_t* context = static_cast<const context_t*>(self);
    SDL_Event e;
    if(SDL_PollEvent(&e))
    {
        switch(e.type) {
            case SDL_QUIT: return window_event_t::quit;
            default: return window_event_t::not_implemented;
        }

    }
    return window_event_t::none;
}

void window_destroy(const void* self)
{
    const context_t* context = static_cast<const context_t*>(self);
    SDL_FreeSurface(context->framebuffer);
    SDL_UpdateWindowSurface(context->window);
    SDL_DestroyWindow(context->window);
    SDL_Quit();
}

#ifdef __cplusplus
}
#endif
