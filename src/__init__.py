import json
import os
import typing

# import cProfile
import pygame

from . import clock

flags = pygame.RESIZABLE | pygame.HWACCEL | pygame.DOUBLEBUF | pygame.HWSURFACE


def run():
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

    try:
        with open(os.getcwd() + "/config.json") as fp:
            x = fp.read()
    except FileNotFoundError:
        with open(os.getcwd() + "/config.json", "w") as fp:
            fp.write(json.dumps({"clocks": ["local"], "am_pm": False, "revert": False}))
        exit()
    else:
        config = get_config(x)

    revert = config["revert"]
    frame_cap = 0
    while running:
        clock.tick(frame_cap)  # cpu won't be fucked over

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

        # cProfile.runctx("apply(config, window)", globals(), locals())
        clear(window, revert)
        apply(config, window, revert)
        center("Clockery", window, font2, 100, revert)
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
                f"Capped: {str(round(frame_cap, 2)) + ' FPS' if frame_cap else 'Unlimited FPS'}",
                revert,
            ),
            window,
            (0, 45),
        )
        # print(f"FPS: {round(fps, 2)}")
        pygame.display.update()


def to_screen(
    text: pygame.Surface,
    window: pygame.Surface,
    dest: typing.Optional[tuple[int, int]] = None,
    rect: typing.Optional[pygame.Rect] = None,
) -> None:
    window.blit(text, rect if rect is not None else dest)  # type: ignore


def clear(window: pygame.Surface, revert: bool):
    window.fill("black") if not revert else window.fill((255, 255, 255))


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


def apply(config: dict, window: pygame.Surface, revert: bool) -> None:
    num_rectangles = len(config["clocks"])
    rectangles = create_surfaces(num_rectangles, window, revert)
    yes: list[pygame.Surface] = []
    poses = calculate_positions(num_rectangles, window)
    w, h = window.get_size()
    for c, i in zip(config["clocks"], rectangles):
        # pygame.draw.rect(i, (255,255,255), (100 - 5, 75 - 5, w + 2 * 5, h + 2 * 5), 5)
        if "gmt" in c.lower() or "utc" in c.lower():
            c = "Etc/" + c
        c_class = clock.Clock(c, config["am_pm"], config["revert"])
        res = c_class.render(c_class.convert_time_to_tz(), i)
        yes.append(res)
    for x, pos in zip(yes, poses):
        window.blit(x, pos)


def create_surfaces(num_surfaces: int, window: pygame.Surface, revert: bool):
    w, h = window.get_size()
    surface_width = w // num_surfaces
    background_color = (0, 0, 0) if not revert else (255, 255, 255)
    x: list[pygame.Surface] = []
    for _ in range(num_surfaces):
        s = pygame.Surface((surface_width, h))
        s.fill(background_color)
        x.append(s)
    return x
    # return [pygame.Surface((surface_width, h)).fill(background_color) for _ in range(num_surfaces)]


def calculate_positions(num_surfaces: int, window: pygame.Surface):
    w, _ = window.get_size()
    surface_width = w // num_surfaces
    return [(i * surface_width, 0) for i in range(num_surfaces)]
