#!/usr/bin/env fish

argparse 'c/count=' 'h/help' -- $argv
or exit 1

if set -q _flag_help
    echo "Usage: test.fish --count N"
    exit 0
end

set -l count 1

if set -q _flag_count
	set count $_flag_count
end

for i in (seq 1 $count)
	echo run $i
	cargo run -p driver --bin grp_survey --release -- -p {$MACHINE_ID}
	mv ./grp_100.tsv ./{$hostname}_{$i}_grp_100.tsv
end

