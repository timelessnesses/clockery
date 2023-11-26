"""
partially generated by chatgpt on draw_clock function because i had no idea what am i doing and my math isn't THAT advanced
"""

import math
import sys
from datetime import datetime

import pygame

pygame.init()
clock = pygame.time.Clock()
width, height = 500, 500
window = pygame.display.set_mode((width, height))
pygame.display.set_caption("Analog Clock")


def draw_clock(hour, minute, second):
    window.fill((255, 255, 255))
    pygame.draw.circle(window, (0, 0, 0), (width // 2, height // 2), 200)
    draw_hand(hour * 30 + minute * 0.5, 80, 10)
    draw_hand(minute * 6, 120, 5)
    draw_hand(second * 6, 150, 2)
    pygame.draw.circle(window, (0, 0, 0), (width // 2, height // 2), 5)
    for i in range(0, 360, 30):
        angle = math.radians(i)
        x1 = width // 2 + 180 * math.cos(angle)
        y1 = height // 2 + 180 * math.sin(angle)
        x2 = width // 2 + 170 * math.cos(angle)
        y2 = height // 2 + 170 * math.sin(angle)
        pygame.draw.line(window, (100, 100, 100), (x1, y1), (x2, y2), 2)


def draw_hand(angle, length, thickness):
    x = width // 2 + length * math.cos(math.radians(angle))
    y = height // 2 + length * math.sin(math.radians(angle))
    pygame.draw.line(
        window, (255, 255, 255), (width // 2, height // 2), (x, y), thickness
    )


while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()
    now = datetime.now()
    hour = now.hour % 12
    minute = now.minute
    second = now.second
    draw_clock(hour, minute, second)
    pygame.display.flip()
    clock.tick(1)
