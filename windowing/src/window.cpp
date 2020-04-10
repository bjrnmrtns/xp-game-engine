#include <SDL.h>
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

struct color_t {
    unsigned char r, g, b, a;
};

enum class InputEventTag : uint32_t {
    Quit,
    MouseMotion,
    NotImplemented,
    NoEvent,
};

struct InputEventQuit { InputEventTag tag; };
struct InputEventMouseMotion { InputEventTag tag; };
struct InputEventNotImplemented { InputEventTag tag; };
struct InputEventNoEvent { InputEventTag tag; };

union InputEventEnum {
    InputEventQuit quit;
    InputEventMouseMotion mouse_motion;
    InputEventNotImplemented not_implemented;
    InputEventNoEvent no_event;
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

InputEventEnum window_poll_event(const void* self)
{
    const context_t* context = static_cast<const context_t*>(self);
    SDL_Event e;
    if(SDL_PollEvent(&e))
    {
        switch(e.type) {
            case SDL_QUIT: {
                InputEventEnum event;
                event.quit.tag = InputEventTag::Quit;
                return event;
            }
            case SDL_MOUSEMOTION: {
                InputEventEnum event;
                event.mouse_motion.tag = InputEventTag::MouseMotion;
                return event;
            }
            default: {
                InputEventEnum event;
                event.not_implemented.tag = InputEventTag::NotImplemented;
                return event;
            }
        }

    }
    InputEventEnum event;
    event.no_event.tag = InputEventTag::NoEvent;
    return event;
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
