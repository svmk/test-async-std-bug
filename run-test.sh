#!/usr/bin/env bash
wget "https://www.sec.gov/Archives/edgar/full-index/1994/QTR1/company.idx" -O expected.idx || exit 1
cargo run || exit 1
echo "expected.ids md5sum:      `md5sum expected.idx`" || exit 1
echo "company.sync.ids md5sum:  `md5sum company.sync.idx`" || exit 1
echo "company.async.ids md5sum: `md5sum company.async.idx`" || exit 1
echo 'Files must be equal!!! But `company.async.ids` is differ from `expected.idx`.' || exit 1