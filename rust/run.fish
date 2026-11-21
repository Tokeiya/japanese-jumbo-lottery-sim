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
	cargo run -p driver --bin 1_5million --release
	mv ./15output_players.tsv ./{$hostname}_{$i}_players.tsv
	mv ./15output_outcome.tsv ./{$hostname}_{$i}_outcome.tsv
end

