import json
import os
import random
import typing

import pygame

from . import clock

flags = pygame.RESIZABLE | pygame.HWACCEL | pygame.DOUBLEBUF | pygame.HWSURFACE


def run():
    pygame.init()

    window = pygame.display.set_mode((800, 800), flags)
    pygame.display.set_caption("Clockery")
    window.fill((0, 0, 0))
    pygame.font.init()

    font = pygame.font.SysFont(pygame.font.get_default_font(), 20)
    font2 = pygame.font.SysFont(pygame.font.get_default_font(), 50)

    running = True
    clock = pygame.time.Clock()

    max_fps = 0
    min_fps = 0

    while running:
        clock.tick(0)  # cpu won't be fucked over

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = not running
                continue
            # elif event.type == pygame.VIDEORESIZE:
            #     w, h = event.size
            #     should_change = False
            #     if w < 800:
            #         w = 800
            #         should_change = True
            #     elif h < 800:
            #         h = 800
            #         should_change = True
            #     if should_change:
            #         window = pygame.display.set_mode((w,h), pygame.RESIZABLE)

        fps = clock.get_fps()
        if fps >= max_fps:
            max_fps = fps
        elif fps <= min_fps and int(fps) != 0:
            min_fps = fps

        clear(window)

        center("Fucking Shit!", window, font2, 30)

        try:
            with open(os.getcwd() + "/config.json") as fp:
                x = fp.read()
        except FileNotFoundError:
            with open(os.getcwd() + "/config.json", "w") as fp:
                fp.write(json.dumps({"clocks": ["local"], "am_pm": False}))
            exit()
        else:
            config = get_config(x)
            apply(config, window)
            pass



        to_screen(render_font(font, f"FPS: {round(fps, 2)}"), window, (0, 0))
        to_screen(render_font(font, f"Max: {round(max_fps, 2)}"), window, (0, 15))
        to_screen(render_font(font, f"Min: {round(min_fps, 2)}"), window, (0, 30))

        # print(f"FPS: {round(fps, 2)}")
        pygame.display.flip()


def to_screen(
    text: pygame.Surface,
    window: pygame.Surface,
    dest: typing.Optional[tuple[int, int]] = None,
    rect: typing.Optional[pygame.Rect] = None,
) -> None:
    window.blit(text, rect if rect is not None else dest)  # type: ignore


def clear(window: pygame.Surface):
    window.fill("black")


def center(
    text: str, window: pygame.Surface, font: pygame.font.Font, y: typing.Optional[int]
):
    rendered = render_font(font, text)
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


def render_font(font: pygame.font.Font, text: str) -> pygame.Surface:
    return font.render(text, True, (255, 255, 255))


def get_config(raw: str | bytes) -> dict:
    return json.loads(raw)


def apply(config: dict, window: pygame.Surface) -> None:
    w, h = window.get_size()
    num_rectangles = len(config["clocks"])
    rectangles = create_surfaces(num_rectangles, window)
    yes: list[pygame.Surface] = []
    poses = calculate_positions(num_rectangles, window)
    for c in config["clocks"]:
        c_class = clock.Clock(c, config["am_pm"])
        draw = random.choice(rectangles)
        rectangles.remove(draw)
        res = c_class.render(c_class.convert_time_to_tz(), draw)
        yes.append(res)
    for x, pos in zip(yes, poses):
        window.blit(x, pos)


def create_surfaces(num_surfaces: int, window: pygame.Surface):
    w, h = window.get_size()
    surface_width = w // num_surfaces
    return [pygame.Surface((surface_width, h)) for _ in range(num_surfaces)]


def calculate_positions(num_surfaces: int, window: pygame.Surface):
    w, _ = window.get_size()
    surface_width = w // num_surfaces
    return [(i * surface_width, 0) for i in range(num_surfaces)]
