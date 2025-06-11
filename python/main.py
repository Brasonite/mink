import mink

bunny: mink.Texture
bunny_start = 0.0


def init():
    pass


def load():
    global bunny
    bunny = mink.assets.texture("bunny.png")


def update():
    global bunny_start
    if mink.input.key_down("ArrowDown"):
        bunny_start -= 100.0 * mink.time.delta()
    if mink.input.key_down("ArrowUp"):
        bunny_start += 100.0 * mink.time.delta()

    bunny_start -= mink.input.scroll().y * 50.0


def draw():
    global bunny
    global bunny_start
    for i in range(int(bunny_start), int(bunny_start) + 1000, 50):
        mink.draw.sprite(bunny, mink.Vec2(i, i), i / 200, None)

    mink.draw.sprite(bunny, mink.input.mouse_pos(), None, None)


def exit():
    pass


mink.run(init, load, update, draw, exit)
