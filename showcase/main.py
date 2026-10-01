"""Standalone Python greeting template. No packages or API keys required."""


def greeting(name: str = "Lucy") -> str:
    return f"Hello from {name}!"


if __name__ == "__main__":
    print(greeting())
