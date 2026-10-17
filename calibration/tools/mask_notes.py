"""Mask arm-revealing references in a blinded notes file before hand labelling (plans/A2.md).

usage: python3 -I mask_notes.py NOTES.tsv OUT.tsv

Writes OUT.tsv with columns row, code, quote, note (no `asserts`), where paper line references
("line 1397", "lines 1397-1401", "l. 1397", "L1397") and the words paper, source and sources
are replaced by "[ref]". The masking is partial by design: a note's content can still reveal
that its auditor read the paper.
"""
import csv
import re
import sys

PAT = [(re.compile(r"\b(?:lines?|l\.|L)\s*\d{2,4}(?:\s*[-–]\s*\d{2,4})?", re.I), "[ref]"),
       (re.compile(r"\bsources?\b|\bpaper(?:'s)?\b", re.I), "[ref]")]


def mask(s):
    for p, r in PAT:
        s = p.sub(r, s)
    return s


rows = list(csv.reader(open(sys.argv[1]), delimiter="\t"))
w = csv.writer(open(sys.argv[2], "w"), delimiter="\t", lineterminator="\n")
w.writerow(["row", "code", "quote", "note"])
for r in rows[1:]:
    w.writerow([r[0], r[1], r[2], mask(r[3])])
print(f"{len(rows) - 1} rows masked")
