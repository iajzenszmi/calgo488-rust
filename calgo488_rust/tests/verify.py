#!/usr/bin/env python3
"""Compare the compiled Fortran and Rust implementations on several seeds.

Requires gfortran and rustc/cargo. Run: python3 tests/verify.py
"""
import math
import pathlib
import shutil
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
FORTRAN = ROOT / 'fortran/grand_ref'
RUST = ROOT / 'target/release/calgo488'


def run(args):
    return subprocess.run(args, cwd=ROOT, check=True, capture_output=True, text=True).stdout


def main():
    if not shutil.which('gfortran'):
        print('ERROR: gfortran not found', file=sys.stderr)
        return 2
    subprocess.run(['gfortran', '-std=f2008', '-O2', '-ffp-contract=off',
                    'fortran/grand_ref.f90', '-o', str(FORTRAN)], cwd=ROOT, check=True)
    if not shutil.which('cargo'):
        print('ERROR: cargo not installed; Fortran baseline is available, Rust comparison not run.', file=sys.stderr)
        return 2
    subprocess.run(['cargo', 'build', '--release', '--offline'], cwd=ROOT, check=True)

    total = 0
    max_err = 0.0
    for seed in (0, 1, 42, 12345, 4294967295):
        n = 1000
        f_lines = run([str(FORTRAN), str(seed), str(n)]).splitlines()
        r_lines = run([str(RUST), str(seed), str(n)]).splitlines()
        assert len(f_lines) == len(r_lines) == n
        for pos, (f, r) in enumerate(zip(f_lines, r_lines), start=1):
            ff, rr = float(f), float(r)
            err = abs(ff - rr)
            max_err = max(max_err, err)
            if err > 0.000001:
                raise AssertionError(f'Local mismatch seed={seed}, index={pos}: {ff}, {rr}, diff={err}')
            total += 1
        print(f'seed={seed}: {n} samples within 1e-6; exact printed lines = {sum(a==b for a,b in zip(f_lines,r_lines))}/{n}')
    print(f'PASS: {total} samples, max abs error={max_err:.9g}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
