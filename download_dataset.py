# /// script
# requires-python = ">=3.10"
# dependencies = ["polars"]
# ///

import polars as pl


def download_dataset(path: str, key: str) -> None:
    print(f"Downloading {key} split from {path}...")
    output_path = f"./data/{key}.parquet"
    pl.scan_parquet(path).sink_parquet(
        output_path,
    )
    print(f"Downloaded {key}")
    print(pl.scan_parquet(output_path).head(10).collect())


def main() -> None:
    base_url = "hf://datasets/Ritvik19/Sudoku-Dataset/"
    splits = {
        "validation": "**/valid_*.parquet",
        "train": "**/train_*.parquet",
    }

    for key, split in splits.items():
        print(f"Loading {key}...")
        path = base_url + split
        print(f"Path: {path}")
        # Download the dataset
        download_dataset(path, key)
        print(f"Downloaded {key}.")

    print("All datasets loaded and saved successfully.")


if __name__ == "__main__":
    main()
