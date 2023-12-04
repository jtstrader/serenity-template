"""Update the template bot name."""

from os import path
import sys
import re

TEMPLATE_BOT_NAME_REGEX = "enter_bot_name_here"
FILES_TO_UPDATE = ["Cargo.toml", "Dockerfile"]


def main():
    """
    Update `enter_bot_name_here` to the selected name.

    Ensure that the crate name is valid before attempting to change the bot name.
    """
    while True:
        bot_name = input("Enter new bot name: ")
        verify_char = input("Are you sure? [Y/N]: ").lower()
        if verify_char == "y":
            break

    root_dir = path.join(path.dirname(path.abspath(sys.argv[0])), "..")
    target_updates = [path.join(root_dir, file) for file in FILES_TO_UPDATE]

    # Replace all instances of the template name.
    for file in target_updates:
        with open(file, "r", encoding="utf-8") as f:
            file_text = f.read()
        file_text = re.sub(TEMPLATE_BOT_NAME_REGEX, bot_name, file_text)
        with open(file, "w", encoding="utf-8") as f:
            f.write(file_text)


if __name__ == "__main__":
    main()
