import argparse


def yank_block(code: str, start: int):
    open_brace = code.find("{", start)
    assert open_brace >= 0
    depth = 1
    for i in range(open_brace + 1, len(code)):
        if depth == 0:
            return i
        if code[i] == "{":
            depth += 1
        elif code[i] == "}":
            depth -= 1
    return len(code)


def chop(code: str):
    ret = []
    start = 0
    while True:
        fn = code.find("fn", start)
        if fn < 0:
            break
        open_paren = code.find("(", fn + 2)
        assert open_paren >= 0
        name = code[fn + 2 : open_paren].strip()
        end = yank_block(code, fn)
        src = code[fn:end]
        start = end
        ret.append((name, "use prusti_contracts::*;\n\n" + src))
    return ret


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file")
    args = parser.parse_args()
    with open(args.file) as f:
        code = f.read()
    for fn, src in chop(code):
        print(f"// {fn}")
        print(src)


if __name__ == "__main__":
    main()
