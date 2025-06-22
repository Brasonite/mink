import math
import mink

bunny: mink.Texture
bunny_start = 0.0
camera = mink.Camera()
drumloop: mink.Sound


def init():
    pass


def load():
    global bunny
    global drumloop

    bunny = mink.assets.texture("bunny.png")
    drumloop = mink.assets.sound("goofy_drumloop.mp3")

    mink.audio.play(drumloop)


def update():
    global bunny_start
    global camera

    if mink.input.key_down("KeyS"):
        camera.position.y -= 100.0 * mink.time.delta()
    if mink.input.key_down("KeyW"):
        camera.position.y += 100.0 * mink.time.delta()
    if mink.input.key_down("KeyA"):
        camera.position.x -= 100.0 * mink.time.delta()
    if mink.input.key_down("KeyD"):
        camera.position.x = camera.position.x + 100.0 * mink.time.delta()

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
        mink.draw.sprite(bunny, mink.Vec2(i, i), i / 200, None, None)

    mink.draw.sprite(
        bunny,
        camera.project(mink.input.mouse_pos(), mink.window.size()),
        None,
        None,
        mink.Color.RED,
    )


def exit():
    pass


mink.run(init, load, update, draw, exit)
