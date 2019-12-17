# Cargo.lock to ClearlyDefined

These tools allow querying [ClearlyDefined](https://docs.clearlydefined.io/)
for information about packages found in Cargo.lock.

## Install

    cargo install cl-to-cd

Also, binaries are attached to [GitHub Releases](https://github.com/jeffmendoza/cl-to-cd/releases)

## cl-to-cd

This tool takes a Cargo.lock on stdin, and outputs an array of coordinates in
ClearlyDefined format, it is meant to be used with the tools below.

## cd-to-csv

This tool takes an array of coordinates and queries ClearlyDefined for their
license information, then outputs a csv format. For example, use in your CI
system like this:

    cat Cargo.lock | cl-to-cd | cd-to-csv > deps.csv

And save `deps.csv` in your build artifacts.

## cd-to-notice

This tool takes an array of coordinates and queries ClearlyDefined to generate
a "notice" file. For example, use in your CI system like this:

    cat Cargo.lock | cl-to-cd | cd-to-notice > notice.txt

And bundle `notice.txt` with your distributions.

## Example

The [Circle CI config](https://github.com/jeffmendoza/cl-to-cd/blob/master/.circleci/config.yml#L55)
on this repo generates a notice.txt and includes it in the binary zip attached to the GitHub Releases.

## Also see

* [LICENSE](LICENSE)
* [CONTRIBUTING.md](CONTRIBUTING.md)
