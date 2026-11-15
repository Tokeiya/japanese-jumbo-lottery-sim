set hosts black blue plastic clear

for host in $hosts
	scp {$host}:/home/tokeiya3/outpt.tsv ./{$host}_output.tsv
	or echo "Failed:{$host}"
end
