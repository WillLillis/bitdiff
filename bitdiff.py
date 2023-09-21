#! /usr/bin/python3
# finds number of bits different in 2 hex string
#
# use bittdif -h to display a short help message
#
import argparse

parser = argparse.ArgumentParser(
    description="Count the number of positions where corresponding bits differ in 2 hex strings/files"
)
parser.add_argument(
    "-f",
    action="store_true",
    default=None,
    help="flag to indicate that the passed strings are file paths",
)
parser.add_argument("inputs", type=str, nargs=2, help="hex strings or hex file paths")
parser.add_argument(
    "-s",
    default=0,
    type=int,
    help="the number of bytes to skip before starting comparision",
)

args = parser.parse_args()

first, second = args.inputs

if args.f:
    with open(first, 'rb') as f:
        first_bin = f.read()
        first = first_bin.hex()
        #print(first)
    with open(second, 'rb') as f:
        second_bin = f.read()
        second = second_bin.hex()
        #print(second)

offset = 8 * args.s

aa = bytearray.fromhex(first)
bb = bytearray.fromhex(second)

aa_bin = bin(int.from_bytes(aa, "big")).split("0b")[1][offset:]
bb_bin = bin(int.from_bytes(bb, "big")).split("0b")[1][offset:]


larger_len = max(len(aa_bin), len(bb_bin))
aa_bin = aa_bin.zfill(larger_len)
bb_bin = bb_bin.zfill(larger_len)
# print(aa_bin)
# print(bb_bin)

diff = sum([1 if i != j else 0 for i, j in zip(aa_bin, bb_bin)])
print(f"# Diff: {diff}, % len: {100.0 * diff / larger_len}")

