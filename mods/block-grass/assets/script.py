from argparse import ArgumentParser
from PIL import Image, ImageChops, ImageColor


def applica_tinta(percorso_input: str, percorso_output: str, colore: str) -> None:
    immagine = Image.open(percorso_input).convert("RGBA")
    rosso, verde, blu = ImageColor.getrgb(colore)

    # Minecraft colora la texture moltiplicandone i canali RGB per la tinta.
    base_rgb = immagine.convert("RGB")
    tinta = Image.new("RGB", immagine.size, (rosso, verde, blu))
    risultato_rgb = ImageChops.multiply(base_rgb, tinta)

    risultato = Image.merge(
        "RGBA",
        (*risultato_rgb.split(), immagine.getchannel("A"))
    )

    risultato.save(percorso_output)
    print(f"Creata: {percorso_output}")


if __name__ == "__main__":
    parser = ArgumentParser(
        description="Colora di verde una texture grigia di Minecraft."
    )
    parser.add_argument("input", help="Texture PNG grigia")
    parser.add_argument("output", help="PNG verde da creare")
    parser.add_argument(
        "--colore",
        default="#91BD59",
        help="Tinta esadecimale, predefinita: #91BD59, Pianure"
    )

    argomenti = parser.parse_args()
    applica_tinta(argomenti.input, argomenti.output, argomenti.colore)