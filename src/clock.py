import datetime
import os
import typing

import pygame
import pytz

pygame.font.init()


class Clock:
    timezone: typing.Optional[pytz.BaseTzInfo]
    am_pm: bool  # Poiuu You are Fucking Shit
    name: str
    date_font = pygame.font.Font(os.getcwd() + "/src/fonts/digital.ttf", 40)
    normal_font = pygame.font.Font(pygame.font.get_default_font(), 20)
    revert: bool

    def __init__(self, config: str, am_pm: bool, revert: bool) -> None:
        if config.lower() != "local":
            self.timezone = pytz.timezone(config)
            self.name = config
        else:
            self.timezone = None
            self.name = "Local time"
        self.am_pm = am_pm
        self.revert = revert

    def convert_time_to_tz(self) -> datetime.datetime:
        now = datetime.datetime.now(self.timezone)
        return now

    def render(
        self, time: datetime.datetime, surface: pygame.Surface
    ) -> pygame.Surface:
        if not self.am_pm:
            timer = time.strftime("%H:%M:%S")
        else:
            timer = time.strftime("%I:%M:%S %p")
        middle_y = self.__get_middle(surface)
        self.__center(timer, surface, self.date_font, middle_y)
        if self.timezone:
            offset = self.name
        else:
            offset = f"local time."
        zone_thing = time.strftime("%z")
        z_negative_or_positive = zone_thing[:1]
        z_hour = zone_thing[1:3]
        z_minute = zone_thing[3:5]
        if z_negative_or_positive == "-":
            zone = f"{z_hour}:{z_minute} hours behind"
        elif z_negative_or_positive == "+":
            zone = f"{z_hour}:{z_minute} hours forward"
        else:
            zone = "Local time"
        try:
            if int(time.strftime("%Z")) < 0 or int(time.strftime("%Z")) > 0:
                date_thing = f"{time.strftime('%A %d/%B/%Y UTC%Z')} ({zone})"
        except ValueError:
            date_thing = f"{time.strftime('%A %d/%B/%Y %Z')} ({zone})"

        offset_middle_y = middle_y + 90
        self.__center(f"Currently {offset}", surface, self.normal_font, offset_middle_y)
        self.__center(f"The date is {date_thing}", surface, self.normal_font, offset_middle_y + 40)  # type: ignore
        return surface

    def __get_middle(self, surface: pygame.Surface) -> int:
        _, h = surface.get_size()
        return h // 2

    def __center(
        self,
        text: str,
        window: pygame.Surface,
        font: pygame.font.Font,
        y: typing.Optional[int],
    ):
        max_width, _ = window.get_size()
        wrapped_lines = self.__word_wrap(text, max_width, font)

        total_height = len(wrapped_lines) * font.get_height()
        y_centered = y - total_height // 2  # type: ignore

        for line in wrapped_lines:
            rendered = self.__render_font(font, line)
            rect = rendered.get_rect()
            rect.topleft = (
                max_width - rect.width
            ) // 2, y_centered  # Center horizontally
            self.__to_screen(rendered, window, rect=rect)
            y_centered += font.get_height()

    def __render_font(self, font: pygame.font.Font, text: str) -> pygame.Surface:
        return font.render(text, True, "white" if not self.revert else "black")

    def __to_screen(
        self,
        text: pygame.Surface,
        window: pygame.Surface,
        dest: typing.Optional[tuple[int, int]] = None,
        rect: typing.Optional[pygame.Rect] = None,
    ) -> None:
        window.blit(text, rect if rect is not None else dest)  # type: ignore

    def __word_wrap(self, text: str, max_width: int, font: pygame.font.Font):
        words = text.split(" ")
        lines = []
        current_line = ""

        for word in words:
            test_line = current_line + word + " "
            test_width, _ = font.size(test_line)

            if test_width <= max_width:
                current_line = test_line
            else:
                lines.append(current_line.rstrip())
                current_line = word + " "

        lines.append(current_line.rstrip())

        return lines
