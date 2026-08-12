/*
 * TontooCoreIcon - C Header
 * TontooOS Icon Framework
 *
 * This header provides C bindings for the CoreIcon library.
 */

#ifndef TONTOO_COREICON_H
#define TONTOO_COREICON_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ======================== */
/* Color                    */
/* ======================== */

/**
 * Create a color from RGBA float components (0.0 - 1.0).
 */
typedef struct {
    float r;
    float g;
    float b;
    float a;
} coreicon_color_t;

/**
 * Create a color from a hex string (e.g., "#RRGGBB" or "#RRGGBBAA").
 *
 * @param hex The hex color string
 * @param color Output color
 * @return 1 on success, 0 on invalid input
 */
int coreicon_color_from_hex(const char *hex, coreicon_color_t *color);

/**
 * Create a color from 8-bit RGB components.
 *
 * @param r Red (0-255)
 * @param g Green (0-255)
 * @param b Blue (0-255)
 * @return The color (alpha = 1.0)
 */
coreicon_color_t coreicon_color_from_rgb(uint8_t r, uint8_t g, uint8_t b);

/**
 * Create a color from 8-bit RGBA components.
 *
 * @param r Red (0-255)
 * @param g Green (0-255)
 * @param b Blue (0-255)
 * @param a Alpha (0-255)
 * @return The color
 */
coreicon_color_t coreicon_color_from_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a);

/* ======================== */
/* Gradient                 */
/* ======================== */

/**
 * Gradient direction.
 */
typedef enum {
    COREICON_GRADIENT_TOP_TO_BOTTOM = 0,
    COREICON_GRADIENT_BOTTOM_TO_TOP = 1,
    COREICON_GRADIENT_LEFT_TO_RIGHT = 2,
    COREICON_GRADIENT_RIGHT_TO_LEFT = 3,
    COREICON_GRADIENT_TOP_LEADING_TO_BOTTOM_TRAILING = 4,
    COREICON_GRADIENT_TOP_TRAILING_TO_BOTTOM_LEADING = 5,
    COREICON_GRADIENT_CENTER_RADIAL = 6,
} coreicon_gradient_direction_t;

/**
 * A single gradient stop.
 */
typedef struct {
    coreicon_color_t color;
    float position;
} coreicon_gradient_stop_t;

/**
 * Opaque gradient handle.
 */
typedef struct coreicon_gradient coreicon_gradient_t;

/**
 * Create a gradient with two stops.
 *
 * @param direction The gradient direction
 * @param from The starting color
 * @param to The ending color
 * @return A new gradient handle (free with coreicon_gradient_free)
 */
coreicon_gradient_t *coreicon_gradient_linear(coreicon_gradient_direction_t direction,
                                              coreicon_color_t from,
                                              coreicon_color_t to);

/**
 * Create a gradient with multiple stops.
 *
 * @param direction The gradient direction
 * @param stops The array of stops
 * @param count The number of stops
 * @return A new gradient handle (free with coreicon_gradient_free)
 */
coreicon_gradient_t *coreicon_gradient_new(coreicon_gradient_direction_t direction,
                                           const coreicon_gradient_stop_t *stops,
                                           size_t count);

/**
 * Free a gradient handle.
 *
 * @param gradient The gradient to free
 */
void coreicon_gradient_free(coreicon_gradient_t *gradient);

/* ======================== */
/* IconCanvas               */
/* ======================== */

/**
 * Opaque canvas handle.
 */
typedef struct coreicon_canvas coreicon_canvas_t;

/**
 * Create a new icon canvas (1024x1024).
 *
 * @return A new canvas handle (free with coreicon_canvas_free)
 */
coreicon_canvas_t *coreicon_canvas_new(void);

/**
 * Free a canvas handle.
 *
 * @param canvas The canvas to free
 */
void coreicon_canvas_free(coreicon_canvas_t *canvas);

/**
 * Set the background to a solid color.
 *
 * @param canvas The canvas
 * @param color The background color
 */
void coreicon_canvas_set_background_color(coreicon_canvas_t *canvas, coreicon_color_t color);

/**
 * Set the background to a gradient.
 *
 * @param canvas The canvas
 * @param gradient The gradient (ownership NOT transferred)
 */
void coreicon_canvas_set_background_gradient(coreicon_canvas_t *canvas, coreicon_gradient_t *gradient);

/**
 * Set the corner radius (0 for none, 256 for iOS-style).
 *
 * @param canvas The canvas
 * @param radius The corner radius in pixels
 */
void coreicon_canvas_set_corner_radius(coreicon_canvas_t *canvas, float radius);

/**
 * Set the frosted-glass overlay opacity (0.0 - 1.0).
 *
 * @param canvas The canvas
 * @param opacity The frost opacity
 */
void coreicon_canvas_set_frosted(coreicon_canvas_t *canvas, float opacity);

/**
 * Set the glossy specular highlight opacity (0.0 - 1.0).
 *
 * @param canvas The canvas
 * @param opacity The specular opacity
 */
void coreicon_canvas_set_specular(coreicon_canvas_t *canvas, float opacity);

/**
 * Set the raised-button inner depth shadow.
 *
 * @param canvas The canvas
 * @param blur The shadow blur in pixels
 * @param opacity The shadow opacity (0.0 - 1.0)
 */
void coreicon_canvas_set_inner_depth(coreicon_canvas_t *canvas, float blur, float opacity);

/**
 * Set the directional edge highlight.
 *
 * @param canvas The canvas
 * @param width The highlight width in pixels
 * @param opacity The highlight opacity (0.0 - 1.0)
 */
void coreicon_canvas_set_edge_highlight(coreicon_canvas_t *canvas, float width, float opacity);

/**
 * Set the assets directory for SF Symbol PNGs.
 *
 * @param path The assets directory path
 */
void coreicon_set_assets_dir(const char *path);

/* ======================== */
/* Layers                   */
/* ======================== */

/**
 * Layer content type.
 */
typedef enum {
    COREICON_CONTENT_ICON = 0,
    COREICON_CONTENT_RECT = 1,
    COREICON_CONTENT_CIRCLE = 2,
    COREICON_CONTENT_IMAGE = 3,
    COREICON_CONTENT_TEXT = 4,
} coreicon_content_type_t;

/**
 * Opaque layer handle.
 */
typedef struct coreicon_layer coreicon_layer_t;

/**
 * Create a new SF Symbol layer.
 *
 * @param symbol_name The SF Symbol name (e.g., "message.fill")
 * @return A new layer handle (free with coreicon_layer_free)
 */
coreicon_layer_t *coreicon_layer_icon(const char *symbol_name);

/**
 * Create a new rounded rectangle layer.
 *
 * @param width The rectangle width
 * @param height The rectangle height
 * @param corner_radius The corner radius
 * @return A new layer handle (free with coreicon_layer_free)
 */
coreicon_layer_t *coreicon_layer_rect(float width, float height, float corner_radius);

/**
 * Create a new circle layer.
 *
 * @param diameter The circle diameter
 * @return A new layer handle (free with coreicon_layer_free)
 */
coreicon_layer_t *coreicon_layer_circle(float diameter);

/**
 * Create a new image layer.
 *
 * @param path The image file path
 * @return A new layer handle (free with coreicon_layer_free)
 */
coreicon_layer_t *coreicon_layer_image(const char *path);

/**
 * Create a new text layer.
 *
 * @param content The text content
 * @param font_size The font size in pixels
 * @return A new layer handle (free with coreicon_layer_free)
 */
coreicon_layer_t *coreicon_layer_text(const char *content, float font_size);

/**
 * Free a layer handle.
 *
 * @param layer The layer to free
 */
void coreicon_layer_free(coreicon_layer_t *layer);

/**
 * Set the layer position.
 *
 * @param layer The layer
 * @param x The x position
 * @param y The y position
 */
void coreicon_layer_set_position(coreicon_layer_t *layer, float x, float y);

/**
 * Set the layer size.
 *
 * @param layer The layer
 * @param width The width
 * @param height The height
 */
void coreicon_layer_set_size(coreicon_layer_t *layer, float width, float height);

/**
 * Set the layer fill color (tint).
 *
 * @param layer The layer
 * @param color The fill color
 */
void coreicon_layer_set_tint(coreicon_layer_t *layer, coreicon_color_t color);

/**
 * Set the layer gradient fill.
 *
 * @param layer The layer
 * @param gradient The gradient (ownership NOT transferred)
 */
void coreicon_layer_set_gradient(coreicon_layer_t *layer, coreicon_gradient_t *gradient);

/**
 * Set the layer opacity (0.0 - 1.0).
 *
 * @param layer The layer
 * @param opacity The opacity
 */
void coreicon_layer_set_opacity(coreicon_layer_t *layer, float opacity);

/**
 * Set the outer shadow.
 *
 * @param layer The layer
 * @param offset_x The shadow x offset
 * @param offset_y The shadow y offset
 * @param blur The shadow blur
 * @param color The shadow color
 * @param opacity The shadow opacity (0.0 - 1.0)
 */
void coreicon_layer_set_shadow(coreicon_layer_t *layer,
                               float offset_x, float offset_y, float blur,
                               coreicon_color_t color, float opacity);

/**
 * Set the inner shadow.
 *
 * @param layer The layer
 * @param blur The shadow blur
 * @param color The shadow color
 * @param opacity The shadow opacity (0.0 - 1.0)
 */
void coreicon_layer_set_inner_shadow(coreicon_layer_t *layer,
                                     float blur,
                                     coreicon_color_t color,
                                     float opacity);

/**
 * Add a layer to the canvas (drawn in order — last = on top).
 *
 * @param canvas The canvas
 * @param layer The layer (ownership NOT transferred)
 */
void coreicon_canvas_add_layer(coreicon_canvas_t *canvas, coreicon_layer_t *layer);

/* ======================== */
/* Rendering                */
/* ======================== */

/**
 * Render the icon and save it as a PNG file.
 *
 * @param canvas The canvas
 * @param path The output PNG path
 * @return 0 on success, negative on error
 */
int coreicon_canvas_save(coreicon_canvas_t *canvas, const char *path);

/**
 * Render the icon into a raw RGBA8 pixel buffer (1024x1024).
 *
 * @param canvas The canvas
 * @param out_size Receives the buffer size in bytes (1024*1024*4)
 * @return A new buffer (must be freed with coreicon_free), or NULL on error
 */
uint8_t *coreicon_canvas_render(coreicon_canvas_t *canvas, size_t *out_size);

/* ======================== */
/* Memory Management        */
/* ======================== */

/**
 * Free a buffer returned by coreicon_canvas_render.
 *
 * @param ptr The buffer to free
 */
void coreicon_free(void *ptr);

/* ======================== */
/* Version                  */
/* ======================== */

/**
 * Get the library version string.
 *
 * @return The version string (do NOT free)
 */
const char *coreicon_version(void);

/* ======================== */
/* SF Symbols               */
/* ======================== */

/**
 * Get the canonical name of an SF Symbol from its index.
 *
 * @param index The symbol index (0-based)
 * @return The symbol name (do NOT free), or NULL if out of bounds
 */
const char *coreicon_symbol_name(int index);

/**
 * Get the total number of available SF Symbols.
 *
 * @return The symbol count
 */
int coreicon_symbol_count(void);

#ifdef __cplusplus
}
#endif

#endif /* TONTOO_COREICON_H */
