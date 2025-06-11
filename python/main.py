import math
import mink

bunny: mink.Texture
bunny_start = 0.0
camera = mink.Camera()


def init():
    pass


def load():
    global bunny
    bunny = mink.assets.texture("bunny.png")


def update():
    global bunny_start
    global camera

    if mink.input.key_down("ArrowDown"):
        bunny_start -= 100.0 * mink.time.delta()
    if mink.input.key_down("ArrowUp"):
        bunny_start += 100.0 * mink.time.delta()

    bunny_start -= mink.input.scroll().y * 50.0

    if mink.input.key_down("ArrowLeft"):
        camera.rotation -= math.pi * mink.time.delta()
    if mink.input.key_down("ArrowRight"):
        camera.rotation += math.pi * mink.time.delta()

    if mink.input.key_down("Minus"):
        camera.zoom -= 1.0 * mink.time.delta()
    if mink.input.key_down("Equal"):
        camera.zoom += 1.0 * mink.time.delta()


def draw():
    global bunny
    global bunny_start
    global camera

    mink.draw.set_camera(camera)

    for i in range(int(bunny_start), int(bunny_start) + 1000, 50):
        mink.draw.sprite(bunny, mink.Vec2(i, i), i / 200, None)

    mink.draw.sprite(bunny, mink.input.mouse_pos(), None, None)


def exit():
    pass


mink.run(init, load, update, draw, exit)
