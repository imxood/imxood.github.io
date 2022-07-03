/*******************************************************************************
 * Size: 16 px
 * Bpp: 4
 * Opts: --bpp 4 --size 16 -o Core/Src/lv_custom_font_16.c --format lvgl --no-kerning --no-prefilter --font ../../../program/rust/code/fonts/DroidSansFallbackFull.ttf --symbols 你好
 ******************************************************************************/

#ifdef LV_LVGL_H_INCLUDE_SIMPLE
#include "lvgl.h"
#else
#include "lvgl/lvgl.h"
#endif

#ifndef LV_CUSTOM_FONT_16
#define LV_CUSTOM_FONT_16 1
#endif

#if LV_CUSTOM_FONT_16

/*-----------------
 *    BITMAPS
 *----------------*/

/*Store the image of the glyphs*/
static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+4F60 "你" */
    0x0, 0xce, 0x80, 0xaa, 0x0, 0xfe, 0x1f, 0x40,
    0xd8, 0x0, 0xfe, 0x9c, 0x3, 0xf8, 0x55, 0xf0,
    0x0, 0xfd, 0x2, 0xff, 0x77, 0xbb, 0xc0, 0x1b,
    0xe2, 0x9e, 0xc0, 0x56, 0x3, 0xae, 0x15, 0xde,
    0x33, 0x60, 0x3, 0xf1, 0x5, 0x3, 0xe4, 0xf1,
    0x0, 0x31, 0x1f, 0x8a, 0x28, 0x0, 0x8f, 0xc4,
    0x1, 0xe8, 0x7e, 0x2f, 0xa0, 0x11, 0xf8, 0x82,
    0x78, 0x9f, 0x89, 0x79, 0x80, 0xf, 0xc4, 0x2a,
    0xc0, 0xfc, 0x43, 0x5c, 0x0, 0x7e, 0x25, 0xea,
    0x7, 0xe2, 0x15, 0x60, 0x3, 0xf1, 0x6c, 0x0,
    0x1f, 0x88, 0x3d, 0x0, 0xf, 0xc4, 0x0, 0x46,
    0xbe, 0x1, 0xe3, 0xf1, 0x0, 0x47, 0xfa, 0xc0,
    0x3f, 0xf8, 0xe0,

    /* U+597D "好" */
    0x0, 0x98, 0x40, 0x3f, 0xf8, 0x5e, 0x40, 0xe,
    0xff, 0xf2, 0x0, 0x9, 0x7c, 0x88, 0x4, 0x67,
    0xb3, 0x44, 0xf, 0xff, 0x90, 0x2, 0x1c, 0xc0,
    0x80, 0x43, 0x90, 0x3e, 0x60, 0x16, 0x50, 0x7,
    0xbc, 0xcb, 0xc4, 0x2, 0xd4, 0x0, 0xe4, 0xf0,
    0x4e, 0xad, 0xdb, 0xfb, 0x76, 0x30, 0x8b, 0x8,
    0xb4, 0x55, 0xba, 0x15, 0x61, 0x5, 0xf4, 0xc7,
    0x0, 0xda, 0x80, 0x1e, 0x6f, 0xf1, 0x0, 0x6d,
    0x40, 0xf, 0xb7, 0xcc, 0x3, 0x6a, 0x0, 0x7a,
    0x7a, 0xb8, 0x40, 0x2d, 0x40, 0xe, 0x9f, 0x30,
    0xa2, 0x2, 0x27, 0x20, 0x7, 0x62, 0x0, 0x74,
    0x7e, 0x88, 0x7, 0xff, 0x20
};


/*---------------------
 *  GLYPH DESCRIPTION
 *--------------------*/

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 256, .box_w = 15, .box_h = 15, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 99, .adv_w = 256, .box_w = 16, .box_h = 15, .ofs_x = 0, .ofs_y = -2}
};

/*---------------------
 *  CHARACTER MAPPING
 *--------------------*/

static const uint16_t unicode_list_0[] = {
    0x0, 0xa1d
};

/*Collect the unicode lists and glyph_id offsets*/
static const lv_font_fmt_txt_cmap_t cmaps[] =
{
    {
        .range_start = 20320, .range_length = 2590, .glyph_id_start = 1,
        .unicode_list = unicode_list_0, .glyph_id_ofs_list = NULL, .list_length = 2, .type = LV_FONT_FMT_TXT_CMAP_SPARSE_TINY
    }
};



/*--------------------
 *  ALL CUSTOM DATA
 *--------------------*/

#if LV_VERSION_CHECK(8, 0, 0)
/*Store all the custom data of the font*/
static  lv_font_fmt_txt_glyph_cache_t cache;
static const lv_font_fmt_txt_dsc_t font_dsc = {
#else
static lv_font_fmt_txt_dsc_t font_dsc = {
#endif
    .glyph_bitmap = glyph_bitmap,
    .glyph_dsc = glyph_dsc,
    .cmaps = cmaps,
    .kern_dsc = NULL,
    .kern_scale = 0,
    .cmap_num = 1,
    .bpp = 4,
    .kern_classes = 0,
    .bitmap_format = 2,
#if LV_VERSION_CHECK(8, 0, 0)
    .cache = &cache
#endif
};


/*-----------------
 *  PUBLIC FONT
 *----------------*/

/*Initialize a public general font descriptor*/
#if LV_VERSION_CHECK(8, 0, 0)
const lv_font_t lv_custom_font_16 = {
#else
lv_font_t lv_custom_font_16 = {
#endif
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,    /*Function pointer to get glyph's data*/
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,    /*Function pointer to get glyph's bitmap*/
    .line_height = 15,          /*The maximum line height required by the font*/
    .base_line = 2,             /*Baseline measured from the bottom of the line*/
#if !(LVGL_VERSION_MAJOR == 6 && LVGL_VERSION_MINOR == 0)
    .subpx = LV_FONT_SUBPX_NONE,
#endif
#if LV_VERSION_CHECK(7, 4, 0) || LVGL_VERSION_MAJOR >= 8
    .underline_position = -1,
    .underline_thickness = 1,
#endif
    .dsc = &font_dsc           /*The custom font data. Will be accessed by `get_glyph_bitmap/dsc` */
};



#endif /*#if LV_CUSTOM_FONT_16*/

