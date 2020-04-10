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
    Key,
    NotImplemented,
    NoEvent,
};

enum class Key : int32_t {
    key_w,
    key_a,
    key_s,
    key_d,
    not_mapped,
};

struct InputEventQuit {};
struct InputEventMouseMotion { int32_t xrel; int32_t yrel; };
struct InputEventKey { Key key ; bool down; };
struct InputEventNotImplemented {};
struct InputEventNoEvent {};

union InputEventUnion {
    InputEventQuit quit;
    InputEventMouseMotion mouse_motion;
    InputEventKey key_event;
    InputEventNotImplemented not_implemented;
    InputEventNoEvent no_event;
};

struct InputEvent {
    InputEventTag tag;
    InputEventUnion val;
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

static Key translate_key(const SDL_Keysym& keysym) {
    Key key;
    switch(keysym.sym) {
        case SDLK_w: {
            key = Key::key_w;
            break;
        }
        case SDLK_a: {
            key = Key::key_a;
            break;
        }
        case SDLK_s: {
            key = Key::key_s;
            break;
        }
        case SDLK_d: {
            key = Key::key_d;
            break;
        }
        default: {
            key = Key::not_mapped;
        }
    }
    return key;
}

InputEvent window_poll_event(const void* self)
{
    const context_t* context = static_cast<const context_t*>(self);
    SDL_Event e;
    if(SDL_PollEvent(&e))
    {
        switch(e.type) {
            case SDL_QUIT: {
                return { .tag = InputEventTag::Quit };
            }
            case SDL_KEYDOWN: {
                return { .tag = InputEventTag::Key, { .key_event.key = translate_key(e.key.keysym),
                                                      .key_event.down = true } };
            }
            case SDL_KEYUP: {
                return { .tag = InputEventTag::Key, { .key_event.key = translate_key(e.key.keysym),
                                                      .key_event.down = false } };
            }
            case SDL_MOUSEMOTION: {
                return { .tag = InputEventTag::MouseMotion, .val = { .mouse_motion.xrel = e.motion.xrel,
                                                                     .mouse_motion.yrel = e.motion.yrel } };
            }
            default: {
                return { .tag = InputEventTag::NotImplemented };
            }
        }
    }
    return { .tag = InputEventTag::NoEvent };
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
