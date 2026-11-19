set hosts black blue plastic clear dell_eth

for host in $hosts
	scp {$host}:/home/tokeiya3/15output_players.tsv /mnt/wsl/data/{$host}_players_output.tsv
	scp {$host}:/home/tokeiya3/15output_outcome.tsv /mnt/wsl/data/{$host}_outcome_output.tsv
	or echo "Failed:{$host}"
end
