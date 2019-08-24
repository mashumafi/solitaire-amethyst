tmpl = """        ( x: {}, y: {}, width: 71, height: 96 )"""

sprites = []

for y in range(0, 4):
    for x in range(0, 13):
        sprites.append(tmpl.format(x * 70, y * 95))

output = """(
    texture_width: 981,
    texture_height: 381,
    sprites: [
{}
    ]
)"""

with open("../resources/sprites/faces.ron","w+") as file:
    file.write(output.format(",\n".join(sprites)))
