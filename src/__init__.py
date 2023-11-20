import json
import os
import typing
import asyncio
import cProfile
import pygame

from . import clock

flags = pygame.RESIZABLE | pygame.HWACCEL | pygame.DOUBLEBUF | pygame.HWSURFACE

thing = '{"clocks": ["local", "Australia/Sydney", "America/Argentina/Buenos_Aires", "America/Vancouver"], "am_pm": false, "revert": false, "enable_bg": true}'

async def run(fps_cap: int):
    pygame.init()

    window = pygame.display.set_mode((800, 600), flags)  # type: ignore
    pygame.display.set_caption("Clockery")
    window.fill((0, 0, 0))
    pygame.font.init()

    font = pygame.font.SysFont(pygame.font.get_default_font(), 20)
    font2 = pygame.font.SysFont(pygame.font.get_default_font(), 50)

    running = True
    clock = pygame.time.Clock()

    max_fps = 0
    min_fps = 0

    config = get_config(thing) # type: ignore

    revert:bool = config["revert"]
    am_pm: bool = config["am_pm"]
    
    num_rectangles = len(config["clocks"])
    rectangles = create_surfaces(num_rectangles, window, revert)
    while running:
        clock.tick(fps_cap)  # cpu won't be fucked over

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = not running
                continue
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_r:
                    revert = not revert
                elif event.key == pygame.K_a:
                    am_pm = not am_pm
            elif event.type == pygame.VIDEORESIZE:
                print("Screen resolution is being changed!",event.size)
                w, h = event.size
                should_change = False
                if w < 800:
                    print("Screen's Width is less wider than 800 pixels. Enabling Screen Changing Resolution Flag")
                    w = 800
                    should_change = True
                if h < 600:
                    print("Screen's Height is less higher than 600 pixels. Enabling Screen Changing Resolution Flag")
                    h = 600
                    should_change = True
                if should_change:
                    print("Screen resolution changed! Changing Resolution.")
                    window = pygame.display.set_mode((w,h), flags)
                    should_change = False
                rectangles = create_surfaces(num_rectangles, window, revert)
        fps = clock.get_fps()
        if fps >= max_fps:
            max_fps = fps
        elif fps <= min_fps and int(fps) != 0:
            min_fps = fps

        # cProfile.runctx("apply(config, rectangles, revert, am_pm, window)", globals(), locals())
        clear(window, revert)
        apply(config, rectangles, revert, am_pm, window)
        center("Clockery", window, font2, 50, revert)
        to_screen(render_font(font, f"FPS: {round(fps, 2)}", revert), window, (0, 0))
        to_screen(
            render_font(font, f"Max: {round(max_fps, 2)}", revert), window, (0, 15)
        )
        to_screen(
            render_font(font, f"Min: {round(min_fps, 2)}", revert), window, (0, 30)
        )
        to_screen(
            render_font(
                font,
                f"Capped: {str(round(fps_cap, 2)) + ' FPS' if fps_cap else 'Unlimited FPS'}",
                revert,
            ),
            window,
            (0, 45),
        )
        # print(f"FPS: {round(fps, 2)}")
        pygame.display.update()
        await asyncio.sleep(0)
    with open(os.getcwd() + "/config.json", "w") as fp:
        fp.write(json.dumps({
            **config,
            "revert": revert,
            "am_pm": am_pm
        }))


def to_screen(
    text: pygame.Surface,
    window: pygame.Surface,
    dest: typing.Optional[tuple[int, int]] = None,
    rect: typing.Optional[pygame.Rect] = None,
) -> None:
    window.blit(text, rect if rect is not None else dest)  # type: ignore


def clear(window: pygame.Surface, revert: bool):
    window.fill("black") if not revert else window.fill("white")


def center(
    text: str,
    window: pygame.Surface,
    font: pygame.font.Font,
    y: typing.Optional[int],
    revert: bool,
):
    rendered = render_font(font, text, revert)
    middle = get_middle_surface(rendered, window, y)
    to_screen(rendered, window, rect=middle)


def get_middle_surface(
    surface: pygame.Surface, window: pygame.Surface, y: typing.Optional[int]
):
    w, h = window.get_size()
    if y:
        r = surface.get_rect(center=(w / 2, y))
    else:
        r = surface.get_rect(center=(w / 2, h / 2))
    return r


def render_font(font: pygame.font.Font, text: str, revert: bool) -> pygame.Surface:
    return font.render(text, True, (255, 255, 255) if not revert else (0, 0, 0))


def get_config(raw: str | bytes) -> dict:
    return json.loads(raw)


def apply(config: dict, surfaces: list[tuple[pygame.Rect, pygame.Surface]], revert: bool, am_pm: bool, window: pygame.Surface) -> None:

    yes: list[pygame.Surface] = []
    # w, h = window.get_size()
    for c, i in zip(config["clocks"], surfaces):
        # pygame.draw.rect(i, (255,255,255), (100 - 5, 75 - 5, w + 2 * 5, h + 2 * 5), 5)
        if "gmt" in c.lower() or "utc" in c.lower():
            c = "Etc/" + c
        c_class = clock.Clock(c, am_pm, revert)
        clear(i[1], revert)
        res = c_class.render(c_class.convert_time_to_tz(), i[1])
        yes.append(res)
    for pos, x in surfaces:
        window.blit(x, pos)

def create_surfaces(num_corners: int, window: pygame.Surface, revert: bool) :
    w, h = window.get_size()
    background_color = (0, 0, 0) if not revert else (255, 255, 255)
    
    x: list[tuple[pygame.Rect, pygame.Surface]] = []
    
    # Calculate the number of rows and columns needed
    num_rows = int(num_corners ** 0.5)
    num_cols = (num_corners + num_rows - 1) // num_rows
    
    # Calculate the size of each surface
    surface_width = w // num_cols
    surface_height = h // num_rows
    
    # Create surfaces for each corner
    for i in range(num_rows):
        for j in range(num_cols):
            index = i * num_cols + j
            if index < num_corners:
                s = pygame.Surface((surface_width, surface_height))
                s.fill(background_color)
                
                # Position the surfaces
                s_rect = s.get_rect(topleft=(surface_width * j, surface_height * i))
                
                x.append((s_rect, s))
    
    return x



def calculate_positions(num_surfaces: int, window: pygame.Surface):
    w, _ = window.get_size()
    surface_width = w // num_surfaces
    return [(i * surface_width, 0) for i in range(num_surfaces)]
