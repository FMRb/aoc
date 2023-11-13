import argparse
from pathlib import Path
import shutil
from urllib.error import HTTPError
from urllib.request import urlopen, Request

def main():
    parser = argparse.ArgumentParser(description="Parse AdventOfCode exercise year and day")
    group = parser.add_mutually_exclusive_group()
    group.add_argument("-n","--dry-run", help="dry run - no write file", action="store_true")
    group.add_argument("-f","--force", help="force write file even if exists", action="store_true")
    parser.add_argument("-y", "--year", type=int, default=2022)
    parser.add_argument("-d", "--day", type=int, default=1)
    args = parser.parse_args()

    dry_run, force = args.dry_run, args.force
    year, day = args.year, args.day
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    file = f"day{day}.txt"
    if not force and Path(file).exists():
        print(f"{file} already exists")
        return

    print(f"{file} = {url}")

    if dry_run:
        return

    request = Request(
        url,
        headers={
            "User-Agent":
            "https://github.com/FMRb/aoc2023",
            "Cookie": "session=53616c7465645f5fdf11e47300bfaa2c14edc240c2a52541e8a3fe6983644c05f6b8b367b65d24ef567e57f73010f017a555093da5d0a48629538c1ed8e9c2ad"
        }
    )

    response = urlopen(request)
    if response.status != 200:
        raise HTTPError(response.url, response.code, response.msg,
                        response.headers, None)

    with response:
        with open(file, "wb") as f:
            shutil.copyfileobj(response, f)

if __name__ == "__main__":
    main()
