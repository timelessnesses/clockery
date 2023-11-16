import datetime
import typing
import os
import pygame
import pytz


class Clock:
    timezone: typing.Optional[pytz.BaseTzInfo]
    am_pm: bool # Poiuu You are Fucking Shit
    name: str

    def __init__(self, config: str, am_pm: bool) -> None:
        if config.lower() != "local":
            self.timezone = pytz.timezone(config)
            self.name = config.capitalize()
        else:
            self.timezone = None
            self.name = "Local time"
        self.am_pm = am_pm

    def convert_time_to_tz(self) -> datetime.datetime:
        now = datetime.datetime.now(self.timezone)
        return now

    def render(
        self, time: datetime.datetime, surface: pygame.Surface
    ) -> pygame.Surface:
        date_font = pygame.font.Font(os.getcwd() + "/src/fonts/digital.ttf", 40)
        normal_font = pygame.font.Font(pygame.font.get_default_font(), 20)
        if not self.am_pm:
            date = time.strftime("%H:%M:%S")
        else:
            date = time.strftime("%I:%M:%S %p")
        self.__center(date, surface, date_font, self.__get_middle(surface))
        if self.timezone:
            offset = f"{self.timezone._utcoffset if self.timezone.zone is not None else 'local time'}."
        else:
            offset = f"local time."
        self.__center(f"Currently {offset}", surface, normal_font, self.__get_middle(surface) + 90)
        return surface

    def __get_middle(self, surface: pygame.Surface) -> int:
        w, h = surface.get_size()
        return h // 2

    def __center(self, text: str, window: pygame.Surface, font: pygame.font.Font, y: typing.Optional[int]):
        rendered = self.__render_font(font, text)
        middle = self.__get_middle_surface(rendered, window, y)
        self.__to_screen(rendered, window, rect=middle)

    def __get_middle_surface(self, surface: pygame.Surface, window: pygame.Surface, y: typing.Optional[int]):
        w, h = window.get_size()
        r = surface.get_rect(center=(w / 2, y))
        return r

    def __render_font(self, font: pygame.font.Font, text: str) -> pygame.Surface:
        return font.render(text, True, (255, 255, 255))

    def __to_screen(
        self,
        text: pygame.Surface,
        window: pygame.Surface,
        dest: typing.Optional[tuple[int, int]] = None,
        rect: typing.Optional[pygame.Rect] = None,
    ) -> None:
        window.blit(text, rect if rect is not None else dest)  # type: ignore
